#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate rand;
extern crate quickcheck;
extern crate piecetable;

use rand::Rng;
use quickcheck::{Arbitrary, Gen};

use piecetable::PieceTable;

use Command::*;

// Note: These also implicitly test the iterator.

// TODO: These shrink poorly

#[derive(Clone, Copy, Debug)]
enum Command<T> {
    Insert(usize, T),
    Remove(usize),
}

#[derive(Clone, Debug)]
struct InsertScattered<T: Arbitrary> {
    commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
struct InsertClustered<T: Arbitrary> {
    commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
struct RemoveScattered<T: Arbitrary> {
    data: Vec<T>,
    commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
struct RemoveClustered<T: Arbitrary> {
    data: Vec<T>,
    commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
struct InsertRemoveScatteredEmpty<T: Arbitrary> {
    commands: Vec<Command<T>>,
    elements: usize,
}

#[derive(Clone, Debug)]
struct InsertRemoveScatteredGiven<T: Arbitrary> {
    data: Vec<T>,
    commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
struct InsertRemoveClusteredEmpty<T: Arbitrary> {
    commands: Vec<Command<T>>,
    elements: usize,
}

#[derive(Clone, Debug)]
struct InsertRemoveClusteredGiven<T: Arbitrary> {
    data: Vec<T>,
    commands: Vec<Command<T>>,
}

impl<T: Arbitrary> Arbitrary for InsertScattered<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let len = g.size();
        let mut commands = Vec::with_capacity(len);

        for i in (0 .. len) {
            let idx = g.gen_range(0, i+1);
            commands.push(Insert(idx, Arbitrary::arbitrary(g)));
        }

        InsertScattered {
            commands: commands,
        }
    }
}

impl<T: Arbitrary> Arbitrary for InsertClustered<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let len = g.size();
        let mut commands = Vec::with_capacity(len);

        let mut inserted = 0;

        while inserted < len {
            let cluster_size = g.gen_range(1, len-inserted+1);
            let idx = g.gen_range(0, inserted+1);

            for i in (0 .. cluster_size) {
                commands.push(Insert(idx+i, Arbitrary::arbitrary(g)));
            }

            inserted += cluster_size;
        }

        InsertClustered {
            commands: commands,
        }
    }
}

impl<T: Arbitrary> Arbitrary for RemoveScattered<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let data: Vec<T> = Arbitrary::arbitrary(g);
        if data.len() < 1 {
            return Self::arbitrary(g);
        }

        let removes = g.gen_range(0, data.len());
        let mut commands = Vec::with_capacity(removes);

        for i in (0 .. removes) {
            let idx = g.gen_range(0, data.len()-i);
            commands.push(Remove(idx));
        }

        RemoveScattered {
            data: data,
            commands: commands,
        }
    }
}

impl<T: Arbitrary> Arbitrary for RemoveClustered<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let data: Vec<T> = Arbitrary::arbitrary(g);
        if data.len() < 1 {
            return Self::arbitrary(g);
        }

        let removes = g.gen_range(0, data.len());
        let mut commands = Vec::with_capacity(removes);
        let mut removed = 0;

        while removed < removes {
            let idx = g.gen_range(0, data.len()-removed);
            let cluster_size = std::cmp::min(
                g.gen_range(1, removes-removed+1),
                idx+1);

            for i in (0 .. cluster_size) {
                commands.push(Remove(idx-i));
            }

            removed += cluster_size;
        }

        RemoveClustered {
            data: data,
            commands: commands,
        }
    }
}

fn insert_remove_scattered<T, G: Gen>(g: &mut G, mut elements: usize, weight: u32) -> (Vec<Command<T>>, usize)
    where T: Arbitrary
{
    let len = g.size();
    let mut commands = Vec::with_capacity(len);

    for _ in (0 .. len) {
        let cmd = if elements > 0 && g.gen_weighted_bool(weight) {
            let idx = g.gen_range(0, elements);
            elements -= 1;
            Remove(idx)
        } else {
            let idx = g.gen_range(0, elements+1);
            elements += 1;
            Insert(idx, Arbitrary::arbitrary(g))
        };

        commands.push(cmd);
    }

    (commands, elements)
}

impl<T: Arbitrary> Arbitrary for InsertRemoveScatteredEmpty<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // Delete 1/3 of the time to ensure some items in the structure
        let (commands, elements) = insert_remove_scattered(g, 0, 3);

        InsertRemoveScatteredEmpty {
            commands: commands,
            elements: elements,
        }
    }
}

impl<T: Arbitrary> Arbitrary for InsertRemoveScatteredGiven<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let data: Vec<T> = Arbitrary::arbitrary(g);
        let (commands, _) = insert_remove_scattered(g, data.len(), 2);

        InsertRemoveScatteredGiven {
            data: data,
            commands: commands,
        }
    }
}


fn insert_remove_clustered<T, G: Gen>(g: &mut G, mut elements: usize, weight: u32) -> (Vec<Command<T>>, usize)
    where T: Arbitrary
{
    let len = g.size();
    let mut commands = Vec::with_capacity(len);

    let mut n = 0;

    while n < len {
        if elements > 0 && g.gen_weighted_bool(weight) {
            let idx = g.gen_range(0, elements);
            let cluster_size = std::cmp::min(
                g.gen_range(1, len-n+1),
                idx+1);

            for i in (0 .. cluster_size) {
                commands.push(Remove(idx-i));
            }

            elements -= cluster_size;
            n += cluster_size;
        } else {
            let cluster_size = g.gen_range(1, len-n+1);
            let idx = g.gen_range(0, elements+1);

            for i in (0 .. cluster_size) {
                commands.push(Insert(idx+i, Arbitrary::arbitrary(g)));
            }

            elements += cluster_size;
            n += cluster_size;
        };
    }

    (commands, elements)
}

impl<T: Arbitrary> Arbitrary for InsertRemoveClusteredEmpty<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // Delete 1/3 of the time to ensure some items in the structure
        let (commands, elements) = insert_remove_clustered(g, 0, 3);

        InsertRemoveClusteredEmpty {
            commands: commands,
            elements: elements,
        }
    }
}

impl<T: Arbitrary> Arbitrary for InsertRemoveClusteredGiven<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let data: Vec<T> = Arbitrary::arbitrary(g);
        let (commands, _) = insert_remove_clustered(g, 0, 2);

        InsertRemoveClusteredGiven {
            data: data,
            commands: commands,
        }
    }
}

fn test_commands<T>(table: &mut PieceTable<T>,
                    expected: &mut Vec<T>,
                    commands: &[Command<T>]) -> bool
    where T: Arbitrary + PartialEq + Copy + std::fmt::Debug
{
    for &cmd in commands {
        match cmd {
            Insert(idx, value) => {
                expected.insert(idx, value);
                table.insert(idx, value);
            },
            Remove(idx) => {
                expected.remove(idx);
                table.remove(idx);
            },
        }
    }

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

// TODO: make generator for ranges to test this properly or something
#[test]
fn ranges() {
    let src = (0..10).collect::<Vec<i32>>();
    let mut table = PieceTable::new().src(&src);

    assert_eq!(vec![&0, &1, &2, &3, &4], table.range(0, 5).collect::<Vec<&i32>>());

    assert_eq!(vec![&7, &8, &9], table.range(7, 10).collect::<Vec<&i32>>());

    table.insert(3, 42);

    assert_eq!(vec![&2, &42, &3], table.range(2, 5).collect::<Vec<&i32>>());
}
