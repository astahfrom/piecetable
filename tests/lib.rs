#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate piecetable;
extern crate quickcheck;
extern crate rand;

use quickcheck::{Arbitrary, Gen};
use rand::Rng;

use piecetable::PieceTable;

// Note: These also implicitly test the iterator.

#[derive(Clone, Debug)]
struct InsertWithIndices<T: Arbitrary> {
    data: Vec<T>,
    indices: Vec<usize>,
}

impl<T: Arbitrary> Arbitrary for InsertWithIndices<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let data: Vec<T> = Arbitrary::arbitrary(g);
        let mut indices: Vec<usize> = Vec::with_capacity(data.len());

        for i in (1..data.len()) {
            indices.push(g.gen_range(0, i));
        }

        InsertWithIndices {
            data: data,
            indices: indices,
        }
    }
}

#[derive(Clone, Debug)]
struct RemoveWithIndices<T: Arbitrary> {
    data: Vec<T>,
    indices: Vec<usize>,
}

impl<T: Arbitrary> Arbitrary for RemoveWithIndices<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let data: Vec<T> = Arbitrary::arbitrary(g);
        let mut indices: Vec<usize> = Vec::with_capacity(data.len());

        if data.len() == 0 {
            return RemoveWithIndices { data: data, indices: vec![], };
        } else if data.len() == 1 {
            return RemoveWithIndices { data: data, indices: vec![0], };
        }

        for i in (1..g.gen_range(1, data.len())).rev() {
            indices.push(g.gen_range(0, i));
        }

        RemoveWithIndices {
            data: data,
            indices: indices,
        }
    }
}

#[quickcheck]
fn insert(recipe: InsertWithIndices<i32>) -> bool {
    let mut expected = Vec::with_capacity(recipe.data.len());
    let mut table = PieceTable::new();

    for (&i, &x) in recipe.indices.iter().zip(recipe.data.iter()) {
        expected.insert(i, x);
        table.insert(i, x);
    }

    expected.iter().collect::<Vec<&i32>>() ==
        table.iter().collect::<Vec<&i32>>()
}

#[quickcheck]
fn remove(recipe: RemoveWithIndices<i32>) -> bool {
    let mut expected = recipe.data.clone();
    let mut table = PieceTable::new().src(&recipe.data);

    for &i in recipe.indices.iter() {
        expected.remove(i);
        table.remove(i);
    }

    expected.iter().collect::<Vec<&i32>>() ==
        table.iter().collect::<Vec<&i32>>()
}

#[quickcheck]
fn insert_and_remove(xs: Vec<i32>) -> bool {
    let mut expected = Vec::with_capacity(xs.len());
    let mut table = PieceTable::new();

    for (i, &x) in xs.iter().enumerate() {
        expected.insert(i / 2, x);
        table.insert(i / 2, x);

        if i % 2 == 0 {
            expected.remove(i / 3);
            table.remove(i / 3);
        }
    }

    expected.iter().collect::<Vec<&i32>>() ==
        table.iter().collect::<Vec<&i32>>()
}

#[test]
fn insert_linear_then_remove() {
    let mut table = PieceTable::new();

    table.insert(0, 1);
    table.insert(1, 2);
    table.insert(2, 3);

    table.insert(1, 27);

    table.insert(4, 4);
    table.insert(5, 5);

    table.remove(1);

    table.insert(6, 6);

    assert_eq!(vec![&1, &2, &3, &4, &5, &6],
               table.iter().collect::<Vec<&i32>>());

}

#[quickcheck]
fn indexing(recipe: InsertWithIndices<i32>) -> bool {
    let mut table = PieceTable::new();
    let mut expected = Vec::with_capacity(recipe.data.len());

    for (&i, &x) in recipe.indices.iter().zip(recipe.data.iter()) {
        expected.insert(i, x);
        table.insert(i, x);

        if table[i] != x {
            return false;
        }
    }

    for i in (0..expected.len()) {
        if table[i] != expected[i] {
            return false;
        }
    }

    true
}

// TODO: make generator for ranges to test this properly
#[test]
fn ranges() {
    let src = (0..10).collect::<Vec<i32>>();
    let mut table = PieceTable::new().src(&src);

    assert_eq!(vec![&0, &1, &2, &3, &4], table.range(0, 5).collect::<Vec<&i32>>());

    assert_eq!(vec![&7, &8, &9], table.range(7, 10).collect::<Vec<&i32>>());

    table.insert(3, 42);

    assert_eq!(vec![&2, &42, &3], table.range(2, 5).collect::<Vec<&i32>>());
}
