#[cfg(test)]
extern crate quickcheck;

use std::iter::Iterator;

use Buffer::*;
use Location::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Buffer {
    Add,
    Original,
}

// TODO: possibly return Piece also
#[derive(Debug, PartialEq)]
enum Location {
    PieceHead(usize),
    PieceMid(usize, usize),
    PieceTail(usize),
    EOF,
}

#[derive(Debug)]
struct Piece {
    start: usize,
    length: usize,
    buffer: Buffer,
}

#[derive(Debug)]
pub struct PieceTree<'a, T: 'a> {
    original: &'a [T],
    adds: Vec<T>,
    pieces: Vec<Piece>, // TODO: should be a tree
}

pub struct Iter<'a, T: 'a> {
    piecetree: &'a PieceTree<'a, T>,
    idx: usize,
    piece_idx: usize,
}

impl<'a, T: 'a> PieceTree<'a, T> {

    pub fn new(src: &'a [T]) -> PieceTree<'a, T> {
        let mut pieces = Vec::new();
        pieces.push(Piece {
            start: 0,
            length: src.len(),
            buffer: Original,
        });

        PieceTree {
            original: src,
            adds: Vec::new(),
            pieces: pieces,
        }
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            piecetree: &self,
            idx: 0,
            piece_idx: 0,
        }
    }

    // TODO: inserts a single T, providing batch insert can probably be faster
    // Also, reusing pieces.
    // TODO: delete 0-length pieces
    // TODO: clean this up
    pub fn insert(&mut self, idx: usize, item: T) {
        let item_idx = self.adds.len();
        self.adds.push(item);

        match find_piece_idx(&self.pieces, idx) {
            PieceHead(piece_idx) => {
                self.pieces.insert(piece_idx, Piece {
                    start: item_idx,
                    length: 1,
                    buffer: Add,
                });
            },
            PieceMid(piece_idx, _) | PieceTail(piece_idx) => {
                let piece_length: usize;
                let piece_buffer: Buffer;

                if let Some(piece) = self.pieces.get_mut(piece_idx) {
                    piece_length = piece.length;
                    piece_buffer = piece.buffer;
                    piece.length = idx - piece.start;
                } else {
                    panic!("find_piece_idx returned invalid index.");
                }

                self.pieces.insert(piece_idx+1, Piece {
                    start: item_idx,
                    length: 1,
                    buffer: Add,
                });

                self.pieces.insert(piece_idx+2, Piece {
                    start: idx,
                    length: piece_length - idx,
                    buffer: piece_buffer,
                });
            },
            EOF => {
                self.pieces.push(Piece {
                    start: item_idx,
                    length: 1,
                    buffer: Add,
                });
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
            PieceTail(piece_idx) => {
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

    // TODO: this can be optimized
    // TODO: this doesn't handle lengths right now
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(piece) = self.piecetree.pieces.get(self.piece_idx) {

            let buffer = match piece.buffer {
                Add => std::borrow::Borrow::borrow(&self.piecetree.adds),
                Original => self.piecetree.original,
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
                delta if delta == piece.length-1 => PieceTail(i),
                delta => PieceMid(i, delta),
            };
        }

        offset += piece.length;
    }

    EOF
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::PieceTree;

    // Note: These also implicitly test the iterator.

    #[test]
    fn insert() {
        fn prop(xs: Vec<i32>) -> bool {
            let mut expected = Vec::with_capacity(xs.len());
            let mut tree = PieceTree::new(&[]);

            for (i, &x) in xs.iter().enumerate() {
                expected.insert(i, x);
                tree.insert(i, x);
            }

            expected.iter().collect::<Vec<&i32>>() ==
                tree.iter().collect::<Vec<&i32>>()
        }

        quickcheck(prop as fn(Vec<i32>) -> bool);
    }

    #[test]
    fn remove() {
        fn prop(src: Vec<i32>, indices: Vec<u8>) -> bool {
            let mut expected = src.clone();
            let mut tree = PieceTree::new(&src);

            for &i in indices.iter() {
                if (i as usize) < expected.len() {
                    expected.remove(i as usize);
                    tree.remove(i as usize);
                }
            }


            expected.iter().collect::<Vec<&i32>>() ==
                tree.iter().collect::<Vec<&i32>>()
        }

        quickcheck(prop as fn(Vec<i32>, Vec<u8>) -> bool);
    }

    #[test]
    fn insert_remove() {
        fn prop(xs: Vec<i32>, indices: Vec<u8>) -> bool {
            let mut expected = Vec::with_capacity(xs.len());
            let mut tree = PieceTree::new(&[]);

            for (i, &x) in xs.iter().enumerate() {
                expected.insert(i, x);
                tree.insert(i, x);
            }

            for &i in indices.iter() {
                if (i as usize) < expected.len() {
                    expected.remove(i as usize);
                    tree.remove(i as usize);
                }
            }

            expected.iter().collect::<Vec<&i32>>() ==
                tree.iter().collect::<Vec<&i32>>()
        }

        quickcheck(prop as fn(Vec<i32>, Vec<u8>) -> bool);
    }
}
