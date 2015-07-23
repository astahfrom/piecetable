#![feature(test)]

extern crate test;
extern crate piecetable;

use test::Bencher;

use piecetable::PieceTable;

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
