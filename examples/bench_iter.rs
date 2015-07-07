extern crate piecetable;

use piecetable::PieceTable;

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

fn main() {
    let src: Vec<i32> = (0..100).collect();
    let indices = scattered_insert_indices(src.len());

    let mut table = PieceTable::new();
    for (&i, &x) in indices.iter().zip(src.iter()) {
        table.insert(i, x);
    }

    let mut sum: usize = 0;

    for i in (0..100000) {
        sum += table.iter().fold(0, |acc, &x| acc + x) as usize;
        table.insert(i, 1);
    }

    println!("{:?}", sum);
}
