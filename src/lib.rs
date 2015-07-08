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

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct PieceTable<'a, T: 'a> {
    original: &'a [T],
    adds: Vec<T>,
    pieces: Vec<Piece>,
    reusable_piece: Option<(usize, usize)>,
}

#[derive(Debug)]
pub struct Iter<'a, T: 'a> {
    table: &'a PieceTable<'a, T>,
    idx: usize,
    piece_idx: usize,
    buffer: &'a [T],
}

#[derive(Debug)]
pub struct Range<'a, T: 'a> {
    iter: Iter<'a, T>,
    idx: usize,
    to: usize,
}

impl<'a, T: 'a> PieceTable<'a, T> {

    pub fn new() -> PieceTable<'a, T> {
        PieceTable {
            original: &[],
            adds: Vec::new(),
            pieces: Vec::new(),
            reusable_piece: None,
        }
    }

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

        if let Some(piece) = self.pieces.get(piece_idx) {
            buffer = match piece.buffer {
                Add => std::borrow::Borrow::borrow(&self.adds),
                Original => self.original,
            };
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

    pub fn iter(&'a self) -> Iter<'a, T> {
        self.make_iter(0)
    }

    pub fn range(&'a self, from: usize, to: usize) -> Range<'a, T> {
        let iter = self.make_iter(from);

        Range {
            iter: iter,
            idx: from,
            to: to,
        }
    }

    pub fn insert(&mut self, idx: usize, item: T) {
        match self.reusable_piece {
            Some((last_idx, piece_idx)) if idx == last_idx+1 => {
                let piece = self.pieces.get_mut(piece_idx).unwrap();
                self.adds.push(item);
                piece.length += 1;
                self.reusable_piece= Some((idx, piece_idx));
            }
            _ => self.raw_insert(idx, item),
        }
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

                self.reusable_piece = Some((idx, piece_idx));
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

                self.reusable_piece = Some((idx, piece_idx+1));

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

                self.reusable_piece = Some((idx, piece_idx));
            }
        }
    }

    // TODO: don't know if we want to return the deleted item
    // TODO: optimize continous deletes
    pub fn remove(&mut self, idx: usize) {
        self.reusable_piece = None;

        match self.idx_to_location(idx) {
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
                let piece_length: usize;
                let piece_start: usize;
                let piece_buffer: Buffer;
                if let Some(piece) = self.pieces.get_mut(piece_idx) {
                    piece_start = piece.start;
                    piece_length = piece.length;
                    piece_buffer = piece.buffer;
                    piece.length = norm_idx;
                } else {
                    panic!("find_piece_idx returned invalid index");
                }

                let start = norm_idx + 1;
                self.pieces.insert(piece_idx+1, Piece {
                    start: piece_start + start,
                    length: piece_length - start,
                    buffer: piece_buffer,
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

// TODO: iter_mut, move

impl<'a, T> Index<usize> for PieceTable<'a, T> {
    type Output = T;

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

    // TODO: benchmark remove

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
}
