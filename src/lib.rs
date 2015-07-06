#![feature(test)]

extern crate test;

use std::iter::Iterator;

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

pub struct Iter<'a, T: 'a> {
    table: &'a PieceTable<'a, T>,
    idx: usize,
    piece_idx: usize,
}

impl<'a, T: 'a> PieceTable<'a, T> {

    pub fn new(src: &'a [T]) -> PieceTable<'a, T> {
        let mut pieces = Vec::new();
        pieces.push(Piece {
            start: 0,
            length: src.len(),
            buffer: Original,
        });

        PieceTable {
            original: src,
            adds: Vec::new(),
            pieces: pieces,
            reusable_piece: None,
        }
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            table: &self,
            idx: 0,
            piece_idx: 0,
        }
    }

    pub fn insert(&mut self, idx: usize, item: T) {
        match self.reusable_piece {
            Some((last_idx, piece_idx)) if idx == last_idx+1 => {
                let piece = self.pieces.get_mut(piece_idx).unwrap();
                assert_eq!(piece.start+piece.length, self.adds.len());
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

        match find_piece_idx(&self.pieces, idx) {
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
    // TODO: possibly delete range
    pub fn remove(&mut self, idx: usize) {
        match find_piece_idx(&self.pieces, idx) {
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
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(piece) = self.table.pieces.get(self.piece_idx) {

            let buffer = match piece.buffer {
                Add => std::borrow::Borrow::borrow(&self.table.adds),
                Original => self.table.original,
            };

            let ret = buffer.get(piece.start + self.idx);

            self.idx += 1;
            if self.idx >= piece.length {
                self.idx = 0;
                self.piece_idx += 1;
            }

            if ret.is_none() {
                return self.next();
            }

            ret
        } else {
            None
        }
    }
}

fn find_piece_idx(pieces: &[Piece], idx: usize) -> Location {
    let mut offset = 0;
    for (i, piece) in pieces.iter().enumerate() {
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

// Benchmarks don't work in the tests directory
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const N: i32 = 1000;

    #[bench]
    fn table_iter_original(b: &mut Bencher) {
        let src: Vec<i32> = (0..N).collect();
        let table = PieceTable::new(&src);

        b.iter(|| table.iter().fold(0, |acc, &x| acc + x))
    }

    #[bench]
    fn table_iter_inserted_linear(b: &mut Bencher) {
        let src: Vec<i32> = (0..N).collect();
        let mut table = PieceTable::new(&[]);

        for (i, &x) in src.iter().enumerate() {
            table.insert(i, x);
        }

        b.iter(|| table.iter().fold(0, |acc, &x| acc + x));
    }

    #[bench]
    fn vec_iter(b: &mut Bencher) {
        let src: Vec<i32> = (0..N).collect();

        b.iter(|| src.iter().fold(0, |acc, &x| acc + x));
    }

    // TODO: benchmark remove

    #[bench]
    fn table_insert_original(b: &mut Bencher) {
        let src: Vec<i32> = (0..N).collect();

        b.iter(|| {
            let mut table = PieceTable::new(&src);
            table.insert(N as usize +1, N+1);
        })
    }

    #[bench]
    fn table_insert_linear(b: &mut Bencher) {
        b.iter(|| {
            let mut table = PieceTable::new(&[]);
            for (i, x) in (0..N).enumerate() {
                table.insert(i, x);
            }
        });
    }

    #[bench]
    fn vec_insert_linear(b: &mut Bencher) {
        b.iter(|| {
            let mut vec = Vec::with_capacity(N as usize);
            for (i, x) in (0..N).enumerate() {
                vec.insert(i, x);
            }
        });
    }

    fn scattered_insert_indices(max: usize) -> Vec<usize> {
        let mut indices: Vec<usize> = Vec::with_capacity(max);

        for i in (1..max) {
            if i % 2 == 0 {
                indices.push(i / 2);
            } else {
                indices.push(i / 3);
            }
        }

        indices
    }

    #[bench]
    fn table_insert_scattered(b: &mut Bencher) {
        let src: Vec<i32> = (0..N).collect();
        let indices = scattered_insert_indices(src.len());

        b.iter(|| {
            let mut table = PieceTable::new(&[]);
            for (&i, &x) in indices.iter().zip(src.iter()) {
                table.insert(i, x);
            }
        })
    }

    #[bench]
    fn vec_insert_scattered(b: &mut Bencher) {
        let src: Vec<i32> = (0..N).collect();
        let indices = scattered_insert_indices(src.len()-1);

        b.iter(|| {
            let mut vec = Vec::with_capacity(src.len());
            for (&i, &x) in indices.iter().zip(src.iter()) {
                vec.insert(i, x);
            }
        })
    }
}
