#![feature(plugin)]
#![plugin(quickcheck_macros)]
#![feature(collections_bound)]

extern crate rand;
extern crate quickcheck;
extern crate piecetable;

mod generators;
use generators::*;

use piecetable::PieceTable;

// Note: These also implicitly test the iterator.

// TODO: These shrink poorly

fn run_commands<T: Copy>(table: &mut PieceTable<T>, vec: &mut Vec<T>, commands: &[Command<T>]) {
    for &cmd in commands {
        match cmd {
            Insert(idx, value) => {
                vec.insert(idx, value);
                table.insert(idx, value);
            },
            Remove(idx) => {
                vec.remove(idx);
                table.remove(idx);
            },
        }
    }
}

fn test_commands<T>(table: &mut PieceTable<T>,
                    expected: &mut Vec<T>,
                    commands: &[Command<T>]) -> bool
    where T: PartialEq + Copy
{
    run_commands(table, expected, commands);

    let expected_vec = expected.iter().collect::<Vec<&T>>();
    let table_vec = table.iter().collect::<Vec<&T>>();

    expected_vec == table_vec
}

#[quickcheck]
fn insert_scattered(recipe: InsertScattered<i32>) -> bool {
    test_commands(&mut PieceTable::new(),
                  &mut Vec::with_capacity(recipe.commands.len()),
                  &recipe.commands)
}

#[quickcheck]
fn insert_clustered(recipe: InsertClustered<i32>) -> bool {
    test_commands(&mut PieceTable::new(),
                  &mut Vec::with_capacity(recipe.commands.len()),
                  &recipe.commands)
}

#[quickcheck]
fn remove_scattered(recipe: RemoveScattered<i32>) -> bool {
    test_commands(&mut PieceTable::new().src(&recipe.data),
                  &mut recipe.data.clone(),
                  &recipe.commands)
}

#[quickcheck]
fn remove_clustered(recipe: RemoveClustered<i32>) -> bool {
    test_commands(&mut PieceTable::new().src(&recipe.data),
                  &mut recipe.data.clone(),
                  &recipe.commands)
}


#[quickcheck]
fn insert_remove_scattered_empty(recipe: InsertRemoveScatteredEmpty<i32>) -> bool {
    test_commands(&mut PieceTable::new(),
                  &mut Vec::with_capacity(recipe.elements),
                  &recipe.commands)
}

#[quickcheck]
fn insert_remove_scattered_given(recipe: InsertRemoveScatteredGiven<i32>) -> bool {
    test_commands(&mut PieceTable::new().src(&recipe.data),
                  &mut recipe.data.clone(),
                  &recipe.commands)
}

#[quickcheck]
fn insert_remove_clustered_empty(recipe: InsertRemoveClusteredEmpty<i32>) -> bool {
    test_commands(&mut PieceTable::new(),
                  &mut Vec::with_capacity(recipe.elements),
                  &recipe.commands)
}

#[quickcheck]
fn insert_remove_clustered_given(recipe: InsertRemoveClusteredGiven<i32>) -> bool {
    test_commands(&mut PieceTable::new().src(&recipe.data),
                  &mut recipe.data.clone(),
                  &recipe.commands)
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

    table.insert(5, 6);

    assert_eq!(vec![&1, &2, &3, &4, &5, &6],
               table.iter().collect::<Vec<&i32>>());

}

#[quickcheck]
fn indexing(recipe: InsertRemoveScatteredEmpty<i32>) -> bool {
    let commands = recipe.commands;

    let mut table = PieceTable::new();
    let mut expected = Vec::with_capacity(commands.len());

    for cmd in commands {
        match cmd {
            Insert(idx, value) => {
                expected.insert(idx, value);
                table.insert(idx, value);

                if table[idx] != value {
                    return false;
                }
            },
            Remove(idx) => {
                expected.remove(idx);
                table.remove(idx);
            },
        }

    }

    for i in (0..expected.len()) {
        if table[i] != expected[i] {
            return false;
        }
    }

    true
}

#[quickcheck]
fn ranges(recipe: Ranges<i32>) -> bool {
    use std::collections::Bound::*;

    let mut table = PieceTable::new();
    let mut expected = Vec::with_capacity(recipe.elements);

    run_commands(&mut table, &mut expected, &recipe.commands);

    for (from, to) in recipe.ranges {
        let table_vec = table.range(from, to).map(|&x| x).collect::<Vec<i32>>();

        let x = match from {
            Included(a) => a,
            Excluded(a) => a+1,
            Unbounded => 0,
        };

        let y = match to {
            Included(b) => b+1,
            Excluded(b) => b,
            Unbounded => expected.len(),
        };

        let expected_vec = expected[(x .. y)].to_vec();

        if table_vec != expected_vec {
            return false
        }
    }

    true
}
