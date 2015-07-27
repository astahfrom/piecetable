//! Implementation of a piece table based on a vector.
//! A piece table provides efficient methods for inserting and removing elements sequentially, intended for use as the underlying data structure in a text editor.
//!
//! The piece table stores a read only reference to a source (if one is provided) and stores inserted elements in an append-only vector.
//! A table of pieces pointing either to the source or add-buffer is maintained, and these pieces are manipulated when inserting and removing text.
//! Asymptotics in the following are based on `p`, the number of pieces, where `p` should be strictly smaller than the number of elements when used as intended.

#![feature(collections_bound)]

use std::iter::Iterator;
use std::ops::Index;
use std::collections::Bound;

use Buffer::*;
use Location::*;

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Buffer {
    Add,
    Original,
}

#[derive(Debug, PartialEq, Clone, Copy, Hash)]
enum Location {
    PieceHead(usize),
    PieceMid(usize, usize),
    PieceTail(usize, usize),
    EOF,
}

#[derive(Debug, Clone, Copy, Hash)]
struct Piece {
    start: usize,
    length: usize,
    buffer: Buffer,
}

/// The `PieceTable` type with all relevant methods.
#[derive(Debug, Clone, Hash)]
pub struct PieceTable<'a, T: 'a> {
    original: &'a [T],
    adds: Vec<T>,
    pieces: Vec<Piece>,
    last_idx: usize,
    length: usize,
    reusable_insert: Option<(usize, bool)>,
    reusable_remove: Option<Location>,
}

/// Struct for iterating the elements of a `PieceTable`.
pub struct Iter<'a, T: 'a>
{
    table: &'a PieceTable<'a, T>,
    piece_idx: usize,
    it: std::slice::Iter<'a, T>,
}

/// Struct for iterating a range of elements in a `PieceTable`.
pub struct Range<'a, T: 'a> {
    iter: Iter<'a, T>,
    idx: usize,
    to: usize,
}

impl<'a, T: 'a> PieceTable<'a, T> {

    /// Construct a new `PieceTable`
    ///
    /// # Example
    /// ```
    /// use piecetable::PieceTable;
    /// let table: PieceTable<char> = PieceTable::new();
    /// ```
    pub fn new() -> PieceTable<'a, T> {
        Default::default()
    }

    /// Constructs a new, empty `PieceTable<T>` with the specified capacity for elements and pieces.
    /// Sequential insertion of `data_capacity` elements will be possible without reallocation.
    /// Scattered operations results in pieces being added; the created piece table will be able to store `piece_capacity` of these before reallocating.
    pub fn with_capacity(data_capacity: usize, piece_capacity: usize) -> PieceTable<'a, T> {
        PieceTable {
            original: &[],
            adds: Vec::with_capacity(data_capacity),
            pieces: Vec::with_capacity(piece_capacity),
            last_idx: 0,
            length: 0,
            reusable_insert: None,
            reusable_remove: None,
        }
    }

    /// Assign a read-only source to an existing `PieceTable`.
    ///
    /// # Example
    /// ```
    /// use piecetable::PieceTable;
    /// let src: Vec<i32> = (0..100).collect();
    /// let table = PieceTable::new().src(&src);
    /// ```
    pub fn src(mut self, src: &'a [T]) -> PieceTable<'a, T> {
        let mut pieces = Vec::new();
        if src.len() > 0 {
            pieces.push(Piece {
                start: 0,
                length: src.len(),
                buffer: Original,
            });
        }

        self.original = src;
        self.pieces = pieces;
        self.length = src.len();

        self
    }

    /// The number of elements stored in the piece table.
    pub fn len(&self) -> usize {
        self.length
    }

    // Returns `true` if the piece table contains no elements.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the number of elements sequentially inserted the piece table can hold without reallocating.
    pub fn capacity_data(&self) -> usize {
        self.adds.capacity()
    }

    /// Returns the number of pieces (created by scattered operations), the piece table can hold without reallocating.
    pub fn capacity_pieces(&self) -> usize {
        self.pieces.capacity()
    }

    /// Reserves capacity for at least `additional` more elements to be inserted.
    /// The collection may reserve more space to avoid frequent reallocations.
    pub fn reserve_data(&mut self, additional: usize) {
        self.adds.reserve(additional);
    }

    /// Reserves capacity for at least `additional` more pieces to be created.
    /// The collection may reserve more space to avoid frequent reallocations.
    pub fn reserve_piece(&mut self, additional: usize) {
        self.pieces.reserve(additional);
    }

    /// Clears the piece table, removing all elements.
    /// Also removes reference to any given `src`.
    pub fn clear(&mut self) {
        self.original = &[];
        self.adds.clear();
        self.pieces.clear();
    }

    fn make_iter(&'a self, idx: usize) -> Iter<'a, T> {
        let (piece_idx, norm_idx) = match self.idx_to_location(idx) {
            PieceHead(piece_idx) => (piece_idx, 0),
            PieceMid(piece_idx, norm_idx) |
            PieceTail(piece_idx, norm_idx) => (piece_idx, norm_idx),
            EOF => {
                // Need an iterator that just closes.
                let it = self.adds[(0 .. 0)].iter();
                return Iter {
                    table: &self,
                    piece_idx: self.pieces.len(),
                    it: it,
                }
            },
        };

        let piece = self.pieces[piece_idx];
        let buf = self.get_buffer(&piece);
        let it = buf[(piece.start + norm_idx .. piece.start + piece.length)].iter();

        Iter {
            table: &self,
            piece_idx: piece_idx,
            it: it,
        }
    }

    /// Return an iterator over all elements of the `PieceTable`.
    ///
    /// Advancing the iterator takes constant time.
    ///
    /// # Example
    /// ```
    /// use piecetable::PieceTable;
    /// let src: Vec<i32> = (0..101).collect();
    /// let table = PieceTable::new().src(&src);
    /// assert_eq!(5050, table.iter().fold(0, |acc, &x| acc + x));
    /// ```
    pub fn iter(&'a self) -> Iter<'a, T> {
        self.make_iter(0)
    }

    /// Return an iterator over the bound range in the `PieceTable`.
    /// Constructing the iterator takes `O(p)` time, but consuming it is constant time per element.
    ///
    /// # Example
    /// ```
    /// #![feature(collections_bound)]
    /// use piecetable::PieceTable;
    /// use std::collections::Bound::*;
    /// let src: Vec<i32> = (0..100).collect();
    /// let table = PieceTable::new().src(&src);
    /// assert_eq!(vec![&55, &56, &57], table.range(Included(55), Excluded(58)).collect::<Vec<&i32>>());
    /// ```
    pub fn range(&'a self, min: Bound<usize>, max: Bound<usize>) -> Range<'a, T> {
        let from = match min {
            Bound::Included(x) => x,
            Bound::Excluded(x) => x+1,
            Bound::Unbounded => 0,
        };

        let to = match max {
            Bound::Included(x) => x+1,
            Bound::Excluded(x) => x,
            Bound::Unbounded => self.length,
        };


        let iter = self.make_iter(from);

        Range {
            iter: iter,
            idx: from,
            to: to,
        }
    }

    /// Insert an element at `idx`.
    ///
    /// `O(p)` time, but sequential inserts afterwards take `O(1) `time.
    ///
    /// # Panics
    /// Panics if not `idx <= len`.
    ///
    /// # Example
    /// ```
    /// use piecetable::PieceTable;
    /// let src: Vec<i32> = (0..100).collect();
    /// let mut table = PieceTable::new().src(&src);
    /// table.insert(11, 42); // takes `O(p)` time
    /// table.insert(12, 42); // constant time
    /// table.insert(13, 42); // constant time
    /// table.insert(27, 42); // `O(p)` time
    /// table.insert(28, 42); // constant time
    /// ```
    pub fn insert(&mut self, idx: usize, item: T) {
        assert!(idx <= self.length);

        match self.reusable_insert {
            Some((piece_idx, ref mut inserted))
                if (idx == self.last_idx+1 && *inserted)
                || (idx == self.last_idx && !*inserted) => {
                    let piece = &mut self.pieces[piece_idx];
                    self.adds.push(item);
                    piece.length += 1;
                    *inserted = true;
                }
            _ => self.raw_insert(idx, item),
        }

        self.reusable_remove = None;
        self.last_idx = idx;
        self.length += 1;
    }

    fn raw_insert(&mut self, idx: usize, item: T) {
        let item_idx = self.adds.len();
        self.adds.push(item);

        match self.idx_to_location(idx) {
            PieceHead(piece_idx) => {
                self.pieces.insert(piece_idx, Piece {
                    start: item_idx,
                    length: 1,
                    buffer: Add,
                });

                self.reusable_insert = Some((piece_idx, true));
            },
            PieceMid(piece_idx, norm_idx) | PieceTail(piece_idx, norm_idx) => {
                let orig = self.pieces[piece_idx].clone();
                self.pieces[piece_idx].length = norm_idx;

                push_all_at(&mut self.pieces, piece_idx+1, &vec![
                    Piece {
                        start: item_idx,
                        length: 1,
                        buffer: Add,
                    },
                    Piece {
                        start: orig.start + norm_idx,
                        length: orig.length - norm_idx,
                        buffer: orig.buffer,
                    }]);

                self.reusable_insert = Some((piece_idx+1, true));
            },
            EOF => {
                let piece_idx = self.pieces.len();

                self.pieces.push(Piece {
                    start: item_idx,
                    length: 1,
                    buffer: Add,
                });

                self.reusable_insert = Some((piece_idx, true));
            }
        }
    }

    /// Remove the element at the given index.
    ///
    /// `O(p)` operation initially, but removing sequentially backwards afterwards is `O(1)`.
    ///
    /// # Panics
    /// Panics if not `idx < len`.
    ///
    /// # Example
    /// ```
    /// use piecetable::PieceTable;
    /// let src: Vec<i32> = (0..10).collect();
    /// let mut table = PieceTable::new().src(&src);
    /// table.remove(5); // `O(p)`
    /// table.remove(4); // `O(1)`
    /// table.remove(3); // `O(1)`
    /// table.remove(6); // `O(p)`
    /// assert_eq!(vec![&0, &1, &2, &6, &7, &8], table.iter().collect::<Vec<&i32>>());
    /// ```
    pub fn remove(&mut self, idx: usize) {
        assert!(idx < self.length);
        let remove: Option<usize>;

        match self.reusable_insert {
            Some((piece_idx, ref mut inserted))
                if (idx+1 == self.last_idx && !*inserted)
                || (idx == self.last_idx && *inserted) => {
                    let piece = &mut self.pieces[piece_idx];
                    piece.length -= 1;
                    self.adds.pop();

                    self.reusable_remove = None;
                    remove = if piece.length == 0 { Some(piece_idx) } else { None };

                    *inserted = false;
                },
            _ => {
                let location = match self.reusable_remove {
                    Some(loc) if idx+1 == self.last_idx => loc,
                    _ => self.idx_to_location(idx),
                };

                self.raw_remove(location);

                self.reusable_insert = None;
                remove = None;
            }
        }

        if let Some(piece_idx) = remove {
            self.pieces.remove(piece_idx);
            self.reusable_insert = None;

            if piece_idx > 0 {
                let idx = piece_idx-1;
                let len = self.pieces[idx].length;
                let loc = if len == 1 {
                    PieceHead(idx)
                } else {
                    PieceTail(idx, len-1)
                };

                self.reusable_remove = Some(loc);
            }
        }

        self.last_idx = idx;
        self.length -= 1;
    }

    fn raw_remove(&mut self, location: Location) {
        self.reusable_remove = None;

        match location {
            PieceHead(piece_idx) => {
                let remove = {
                    let piece = &mut self.pieces[piece_idx];
                    piece.start += 1;
                    piece.length -= 1;
                    piece.length == 0
                };

                if remove {
                    self.pieces.remove(piece_idx);
                }

                if piece_idx > 0 {
                    let idx = piece_idx-1;
                    let len = self.pieces[idx].length;
                    let loc = if len == 1 {
                        PieceHead(idx)
                    } else {
                        PieceTail(idx, len-1)
                    };

                    self.reusable_remove = Some(loc);
                }
            },
            PieceTail(piece_idx, norm_idx) => {
                self.pieces[piece_idx].length -= 1;

                if piece_idx > 0 {
                    let loc = if norm_idx-1 == 0 {
                        PieceHead(piece_idx)
                    } else {
                        PieceTail(piece_idx, norm_idx-1)
                    };
                    self.reusable_remove = Some(loc);
                }
            },
            PieceMid(piece_idx, norm_idx) => {
                let orig = self.pieces[piece_idx];
                self.pieces[piece_idx].length = norm_idx;

                let start = norm_idx + 1;
                if orig.length - start > 0 {
                    self.pieces.insert(piece_idx+1, Piece {
                        start: orig.start + start,
                        length: orig.length - start,
                        buffer: orig.buffer,
                    });
                }

                if piece_idx > 0 {
                    let loc = if norm_idx-1 == 0 {
                        PieceHead(piece_idx)
                    } else {
                        PieceMid(piece_idx, norm_idx-1)
                    };

                    self.reusable_remove = Some(loc);
                }
            },
            EOF => {},
        }
    }

    /// Appends an element to the back, efficiently and in constant time.
    pub fn push(&mut self, value: T) {
        let reuse = self.pieces.last().map_or
            (false, |last| last.buffer == Add
             && last.start+last.length == self.adds.len());

        self.adds.push(value);

        if reuse {
            self.pieces.last_mut().unwrap().length += 1;
        } else {
            self.pieces.push(Piece {
                start: self.adds.len()-1,
                length: 1,
                buffer: Add,
            });
        }
    }

    fn idx_to_location(&self, idx: usize) -> Location {
        let mut offset = 0;
        for (i, piece) in self.pieces.iter().enumerate() {
            if idx >= offset && idx < offset + piece.length {
                return match idx - offset {
                    0 => PieceHead(i),
                    delta if delta == piece.length-1 => PieceTail(i, delta),
                    delta => PieceMid(i, delta),
                };
            }

            offset += piece.length;
        }

        EOF
    }

    fn get_buffer(&'a self, piece: &Piece) -> &'a [T] {
        match piece.buffer {
            Add => &self.adds,
            Original => self.original,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.it.next() {
            Some(next)
        } else {
            self.piece_idx += 1;

            if self.piece_idx >= self.table.pieces.len() {
                None
            } else {
                let piece = self.table.pieces[self.piece_idx];
                let buf = self.table.get_buffer(&piece);

                self.it = buf[(piece.start .. piece.start + piece.length)].iter();
                self.next()
            }
        }
    }
}

impl<'a, T> Iterator for Range<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.to {
            None
        } else {
            self.idx += 1;
            self.iter.next()
        }
    }
}

impl<'a, T> std::iter::FromIterator<T> for PieceTable<'a, T> {
    fn from_iter<I>(iterable: I) -> PieceTable<'a, T> where I: IntoIterator<Item=T> {
        use std::iter::FromIterator;

        let mut table = PieceTable::new();
        table.adds = FromIterator::from_iter(iterable);

        table.pieces = vec![Piece {
            start: 0,
            length: table.adds.len(),
            buffer: Add,
        }];

        table
    }
}

impl<'a, T> std::iter::Extend<T> for PieceTable<'a, T> {
    fn extend<I>(&mut self, iterable: I) where I: IntoIterator<Item=T> {
        let start = self.adds.len();
        self.adds.extend(iterable);
        let length = self.adds.len() - start;

        self.pieces.push(Piece {
            start: start,
            length: length,
            buffer: Add,
        });
    }
}

impl<'a, T> Index<usize> for PieceTable<'a, T> {
    type Output = T;

    /// Note: Reading an index takes `O(p)` time, use iterators for fast sequential access.
    fn index<'b>(&'b self, idx: usize) -> &'b T {
        let (piece_idx, norm_idx) = match self.idx_to_location(idx) {
            PieceHead(piece_idx) => (piece_idx, 0),
            PieceMid(piece_idx, norm_idx) |
            PieceTail(piece_idx, norm_idx) => (piece_idx, norm_idx),
            EOF => panic!("PieceTable out of bounds: {}", idx),
        };

        let ref piece = self.pieces[piece_idx];
        match piece.buffer {
            Add => &self.adds[piece.start + norm_idx],
            Original => &self.original[piece.start + norm_idx],
        }
    }
}

fn push_all_at<T>(v: &mut Vec<T>, offset: usize, s: &[T]) where T: Copy {
    match (v.len(), s.len()) {
        (_, 0) => (),
        (current_len, _) => {
            v.reserve_exact(s.len());
            unsafe {
                v.set_len(current_len + s.len());
                let to_move = current_len - offset;
                let src = v.as_mut_ptr().offset(offset as isize);
                if to_move > 0 {
                    let dst = src.offset(s.len() as isize);
                    std::ptr::copy(src, dst, to_move);
                }
                std::ptr::copy_nonoverlapping(s.as_ptr(), src, s.len());
            }
        },
    }
}

impl<'a, T> Default for PieceTable<'a, T> {
    fn default() -> PieceTable<'a, T> {
        PieceTable {
            original: &[],
            adds: Vec::new(),
            pieces: Vec::new(),
            last_idx: 0,
            length: 0,
            reusable_insert: None,
            reusable_remove: None,
        }
    }
}
