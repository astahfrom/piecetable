//! Implementation of a piece table based on a vector.
//! A piece table provides efficient methods for inserting and removing elements sequentially, intended for use as the underlying data structure in a text editor.
//!
//! The piece table stores a read only reference to a source (if one is provided) and stores inserted elements in an append-only vector.
//! A table of pieces pointing either to the source or add-buffer is maintained, and these pieces are manipulated when inserting and removing text.
//! Asymptotics in the following are based on `p`, the number of pieces, where `p` should be strictly smaller than the number of elements when used as intended.

#![feature(test)]

extern crate test;

use std::iter::Iterator;
use std::ops::Index;

use Buffer::*;
use Location::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Buffer {
    Add,
    Original,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Location {
    PieceHead(usize),
    PieceMid(usize, usize),
    PieceTail(usize, usize),
    EOF,
}

#[derive(Debug, Clone)]
struct Piece {
    start: usize,
    length: usize,
    buffer: Buffer,
}

/// The `PieceTable` type with all relevant methods.
#[derive(Debug)]
pub struct PieceTable<'a, T: 'a> {
    original: &'a [T],
    adds: Vec<T>,
    pieces: Vec<Piece>,
    last_idx: usize,
    reusable_insert: Option<usize>,
}

/// Struct for iterating the elements of a `PieceTable`.
#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    table: &'a PieceTable<'a, T>,
    idx: usize,
    piece_idx: usize,
    buffer: &'a [T],
}

/// Struct for iterating a range of elements in a `PieceTable`.
#[derive(Debug)]
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
        PieceTable {
            original: &[],
            adds: Vec::new(),
            pieces: Vec::new(),
            last_idx: 0,
            reusable_insert: None,
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

        self
    }

    fn make_iter(&'a self, idx: usize) -> Iter<'a, T> {
        let buffer: &'a [T];

        let piece_idx = match self.idx_to_location(idx) {
            PieceHead(piece_idx) |
            PieceMid(piece_idx, _) |
            PieceTail(piece_idx, _) => piece_idx,
            EOF => self.adds.len(),
        };

        // TODO: can be cleaned up with option
        if let Some(piece) = self.pieces.get(piece_idx) {
            buffer = self.get_buffer(piece);
        } else {
            buffer = &[];
        }

        Iter {
            table: &self,
            idx: idx,
            piece_idx: piece_idx,
            buffer: buffer,
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

    /// Return an iterator over the range `[from, to)` in the `PieceTable`.
    /// Construction the iterator takes `O(p)` time, but consuming it is constant time per element.
    ///
    /// # Example
    /// ```
    /// use piecetable::PieceTable;
    /// let src: Vec<i32> = (0..100).collect();
    /// let table = PieceTable::new().src(&src);
    /// assert_eq!(vec![&55, &56, &57], table.range(55, 58).collect::<Vec<&i32>>());
    /// ```
    pub fn range(&'a self, from: usize, to: usize) -> Range<'a, T> {
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
        match self.reusable_insert {
            Some(piece_idx) if idx == self.last_idx+1 => {
                let piece = self.pieces.get_mut(piece_idx).unwrap();
                self.adds.push(item);
                piece.length += 1;
            }
            _ => self.raw_insert(idx, item),
        }

        self.last_idx = idx;
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

                self.reusable_insert = Some(piece_idx);
            },
            PieceMid(piece_idx, norm_idx) | PieceTail(piece_idx, norm_idx) => {
                let orig = self.pieces[piece_idx].clone();
                if let Some(piece) = self.pieces.get_mut(piece_idx) {
                    piece.length = norm_idx;
                } else {
                    panic!("find_piece_idx returned invalid index.");
                }

                self.pieces.insert(piece_idx+1, Piece {
                    start: item_idx,
                    length: 1,
                    buffer: Add,
                });

                self.reusable_insert = Some(piece_idx+1);

                self.pieces.insert(piece_idx+2, Piece {
                    start: orig.start + norm_idx,
                    length: orig.length - norm_idx,
                    buffer: orig.buffer,
                });
            },
            EOF => {
                let piece_idx = self.pieces.len();

                self.pieces.push(Piece {
                    start: item_idx,
                    length: 1,
                    buffer: Add,
                });

                self.reusable_insert = Some(piece_idx);
            }
        }
    }

    /// Remove the element at the given index.
    ///
    /// `O(p)` operation, but removing just inserted indices sequentially is `O(1)`
    /// and most remove operations are cheap.
    /// # Example
    /// ```
    /// use piecetable::PieceTable;
    /// let src: Vec<i32> = (0..10).collect();
    /// let mut table = PieceTable::new().src(&src);
    /// table.remove(5);
    /// table.remove(6);
    /// assert_eq!(vec![&0, &1, &2, &3, &4, &6, &8, &9], table.iter().collect::<Vec<&i32>>());
    /// ```
    pub fn remove(&mut self, idx: usize) {
        let mut remove: Option<usize> = None;

        match self.reusable_insert {
            Some(piece_idx) if idx == self.last_idx+1 => {
                let piece = self.pieces.get_mut(piece_idx).unwrap();
                piece.length -= 1;
                if piece.length == 0 {
                    remove = Some(piece_idx);
                }
            },
            _ => {
                self.reusable_insert = None;

                let location = self.idx_to_location(idx);
                self.raw_remove(location);
            },
        }

        if let Some(piece_idx) = remove {
            self.pieces.remove(piece_idx);
            self.reusable_insert = None;
        }

        self.last_idx = idx;
    }

    fn raw_remove(&mut self, location: Location) {
        match location {
            PieceHead(piece_idx) => {
                let remove: bool;
                if let Some(piece) = self.pieces.get_mut(piece_idx) {
                    piece.start += 1;
                    piece.length -= 1;
                    remove = piece.length == 0;
                } else {
                    panic!("find_piece_idx returned invalid index");
                }

                if remove {
                    self.pieces.remove(piece_idx);
                }
            },
            PieceTail(piece_idx, _) => {
                let remove: bool;
                if let Some(piece) = self.pieces.get_mut(piece_idx) {
                    piece.length -= 1;
                    remove = piece.length == 0;
                } else {
                    panic!("find_piece_idx returned invalid index");
                }

                if remove {
                    self.pieces.remove(piece_idx);
                }
            },
            PieceMid(piece_idx, norm_idx) => {
                let orig = self.pieces[piece_idx].clone();
                if let Some(piece) = self.pieces.get_mut(piece_idx) {
                    piece.length = norm_idx;
                } else {
                    panic!("find_piece_idx returned invalid index");
                }

                let start = norm_idx + 1;
                self.pieces.insert(piece_idx+1, Piece {
                    start: orig.start + start,
                    length: orig.length - start,
                    buffer: orig.buffer,
                });
            },
            EOF => {},
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
        self.table.pieces.get(self.piece_idx).map(|piece| {
            let ret = self.buffer.get(piece.start + self.idx).unwrap();

            self.idx += 1;
            if self.idx >= piece.length {
                self.idx = 0;
                self.piece_idx += 1;

                self.buffer = match self.table.pieces.get(self.piece_idx) {
                    Some(p) if p.buffer == Add => &self.table.adds,
                    Some(p) if p.buffer == Original => self.table.original,
                    _ => &[],
                };
            }

            ret
        })
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

// Benchmarks don't work in the tests directory
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn given_10k_iter_table(b: &mut Bencher) {
        let src: Vec<i32> = (0..10_000).collect();
        let table = PieceTable::new().src(&src);

        b.iter(|| table.iter().fold(0, |acc, &x| acc + x))
    }

    #[bench]
    fn given_10k_iter_vec(b: &mut Bencher) {
        let src: Vec<i32> = (0..10_000).collect();

        b.iter(|| src.iter().fold(0, |acc, &x| acc + x));
    }

    #[bench]
    fn given_10k_insert_last_table(b: &mut Bencher) {
        let src: Vec<i32> = (0..10_000).collect();

        b.iter(|| {
            let mut table = PieceTable::new().src(&src);
            table.insert(10_000, 42);
        })
    }

    #[bench]
    fn given_10k_insert_last_vec(b: &mut Bencher) {
        // Note: not entirely fair, because this vec will grow.
        let mut vec: Vec<i32> = (0..10_000).collect();

        b.iter(|| {
            vec.push(42);
        })
    }

    #[bench]
    fn given_10k_insert_first_table(b: &mut Bencher) {
        let src: Vec<i32> = (0..10_000).collect();

        b.iter(|| {
            let mut table = PieceTable::new().src(&src);
            table.insert(0, 42);
        })
    }

    #[bench]
    fn given_10k_insert_first_vec(b: &mut Bencher) {
        let mut vec: Vec<i32> = (0..10_000).collect();

        b.iter(|| {
            vec.insert(0, 42);
        })
    }

    #[bench]
    fn empty_insert_10k_linear_table(b: &mut Bencher) {
        b.iter(|| {
            let mut table = PieceTable::new();
            for (i, x) in (0..10_000).enumerate() {
                table.insert(i, x);
            }
        });
    }

    #[bench]
    fn empty_insert_10k_linear_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut vec = Vec::new();
            for (i, x) in (0..10_000).enumerate() {
                vec.insert(i, x);
            }
        });
    }

    fn clustered_insert_indices(clusters: usize, cluster_size: usize) -> Vec<usize> {
        let max = clusters * cluster_size;
        let mut indices: Vec<usize> = Vec::with_capacity(max);
        let mut offset = 0;

        for i in (0..max) {
            let rem = i % cluster_size;
            indices.push(rem + offset);

            if rem == cluster_size - 1 {
                offset = i / (rem + offset);
            }
        }

        indices
    }

    #[bench]
    fn empty_insert_100_clusters_of_100_table(b: &mut Bencher) {
        let indices = clustered_insert_indices(100, 100);

        b.iter(|| {
            let mut table = PieceTable::new();
            for &i in indices.iter() {
                table.insert(i, 42);
            }
        });
    }

    #[bench]
    fn empty_insert_100_clusters_of_100_vec(b: &mut Bencher) {
        let indices = clustered_insert_indices(100, 100);

        b.iter(|| {
            let mut vec = Vec::new();
            for &i in indices.iter() {
                vec.insert(i, 42);
            }
        });
    }

    #[bench]
    fn given_10k_index_sum_table(b: &mut Bencher) {
        let src: Vec<i32> = (0..10_000).collect();
        let table = PieceTable::new().src(&src);

        b.iter(|| {
            let mut sum = 0;
            for i in (0..src.len()) {
                sum += table[i];
            }
            sum
        })
    }

    #[bench]
    fn given_10k_index_sum_vec(b: &mut Bencher) {
        let vec: Vec<i32> = (0..10_000).collect();

        b.iter(|| {
            let mut sum = 0;
            for i in (0..vec.len()) {
                sum += vec[i];
            }
            sum
        })
    }

    #[bench]
    fn given_10k_remove_mid_100_backwards_table(b: &mut Bencher) {
        b.iter(|| {
            let src: Vec<i32> = (0..10_000).collect();
            let mut table = PieceTable::new().src(&src);
            for i in (5_000..6_000).rev() {
                table.remove(i);
            }
        })
    }

    #[bench]
    fn given_10k_remove_mid_100_backwards_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut vec: Vec<i32> = (0..10_000).collect();
            for i in (5_000..6_000).rev() {
                vec.remove(i);
            }
        })
    }

    #[bench]
    fn given_10k_remove_mid_100_forwards_table(b: &mut Bencher) {
        b.iter(|| {
            let src: Vec<i32> = (0..10_000).collect();
            let mut table = PieceTable::new().src(&src);
            for i in (5_000..5_100) {
                table.remove(i);
            }
        })
    }

    #[bench]
    fn given_10k_remove_mid_100_forwards_vec(b: &mut Bencher) {
        b.iter(|| {
            let mut vec: Vec<i32> = (0..10_000).collect();
            for i in (5_000..5_100) {
                vec.remove(i);
            }
        })
    }

    #[bench]
    fn given_10k_insert_then_remove_100_mid_table(b: &mut Bencher) {
        let range = 5_000 .. 5_100;
        b.iter(|| {
            let src: Vec<i32> = (0..10_000).collect();
            let mut table = PieceTable::new().src(&src);
            for i in range.clone() {
                table.insert(i, 42);
            }

            for i in range.clone().rev() {
                table.remove(i);
            }
        })
    }

    #[bench]
    fn given_10k_insert_then_remove_100_mid_vec(b: &mut Bencher) {
        let range = 5_000 .. 5_100;
        b.iter(|| {
            let mut vec: Vec<i32> = (0..10_000).collect();
            for i in range.clone() {
                vec.insert(i, 42);
            }

            for i in range.clone().rev() {
                vec.remove(i);
            }
        })
    }
}
