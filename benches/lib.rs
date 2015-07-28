#![feature(collections_bound)]
#![feature(test)]
#![feature(vec_push_all)]

extern crate test;
extern crate piecetable;

extern crate rand;
extern crate quickcheck;

mod generators;
use generators::*;

use test::Bencher;
use piecetable::PieceTable;

const SEED: &'static [usize] = &[1, 2, 3, 4];
const SIZE: usize = 1_000;
const VALUE: i32 = 42;
const SRC: &'static [i32; 10_000] = &[0; 10_000];

fn run_commands_table<T: Copy>(table: &mut PieceTable<T>, cmds: &[Command<T>]) {
    for &cmd in cmds {
        match cmd {
            Insert(idx, value) => table.insert(idx, value),
            Remove(idx) => table.remove(idx),
        }
    }
}

fn run_commands_vec<T: Copy>(vec: &mut Vec<T>, cmds: &[Command<T>]) {
    for &cmd in cmds {
        match cmd {
            Insert(idx, value) => vec.insert(idx, value),
            Remove(idx) => { vec.remove(idx); },
        }
    }
}

// EMPTY
// - insert single
// - insert scattered
// - insert clustered
// - insert/remove scattered
// - insert/remove clustered


fn run_benchmark_empty_table(b: &mut Bencher, commands: &[Command<i32>], capacity: usize) {
    b.iter(|| {
        let mut table = PieceTable::with_capacity(capacity, capacity);
        run_commands_table(&mut table, commands);
    })
}

fn run_benchmark_empty_vec(b: &mut Bencher, commands: &[Command<i32>], capacity: usize) {
    b.iter(|| {
        let mut vec = Vec::with_capacity(capacity);
        run_commands_vec(&mut vec, commands);
    })
}

// Table

#[bench]
fn empty_insert_single_table(b: &mut Bencher) {
    b.iter(|| {
        let mut table = PieceTable::with_capacity(1, 1);
        table.insert(0, VALUE);
    })
}

#[bench]
fn empty_insert_scattered_table(b: &mut Bencher) {
    let recipe: InsertScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_empty_table(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn empty_insert_clustered_table(b: &mut Bencher) {
    let recipe: InsertClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_empty_table(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn empty_insert_remove_scattered_table(b: &mut Bencher) {
    let recipe: InsertRemoveScatteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_empty_table(b, &recipe.commands, recipe.elements);
}

#[bench]
fn empty_insert_remove_clustered_table(b: &mut Bencher) {
    let recipe: InsertRemoveClusteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_empty_table(b, &recipe.commands, recipe.elements);
}

// Vec

#[bench]
fn empty_insert_single_vec(b: &mut Bencher) {
    b.iter(|| {
        let mut vec = Vec::with_capacity(1);
        vec.insert(0, VALUE);
    })
}

#[bench]
fn empty_insert_scattered_vec(b: &mut Bencher) {
    let recipe: InsertScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_empty_vec(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn empty_insert_clustered_vec(b: &mut Bencher) {
    let recipe: InsertClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_empty_vec(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn empty_insert_remove_scattered_vec(b: &mut Bencher) {
    let recipe: InsertRemoveScatteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_empty_vec(b, &recipe.commands, recipe.elements);
}

#[bench]
fn empty_insert_remove_clustered_vec(b: &mut Bencher) {
    let recipe: InsertRemoveClusteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_empty_vec(b, &recipe.commands, recipe.elements);
}

// FRESH (10k)
// - iter/sum
// - insert first
// - insert mid
// - insert last
// - insert scatter
// - insert clusters
// - remove scatter
// - remove clusters
// - insert/remove scatter
// - insert/remove clusters

fn run_benchmark_fresh_table(b: &mut Bencher, src: &[i32], commands: &[Command<i32>], capacity: usize) {
    b.iter(|| {
        let mut table = PieceTable::with_capacity(capacity, capacity).src(src);
        run_commands_table(&mut table, commands);
    })
}

fn run_benchmark_fresh_vec(b: &mut Bencher, src: &[i32], commands: &[Command<i32>], capacity: usize) {
    b.iter(|| {
        let mut vec = Vec::with_capacity(src.len() + capacity);
        vec.push_all(src); // Not sure if this is the best way to do this
        run_commands_vec(&mut vec, commands);
    })
}

// Table

#[bench]
fn fresh_iter_table(b: &mut Bencher) {
    let table = PieceTable::new().src(SRC);
    b.iter(|| table.iter().fold(0, |acc, &x| acc + x));
}

#[bench]
fn fresh_insert_first_table(b: &mut Bencher) {
    b.iter(|| {
        let mut table = PieceTable::with_capacity(1, 2).src(SRC);
        table.insert(0, VALUE)
    });
}

#[bench]
fn fresh_insert_middle_table(b: &mut Bencher) {
    b.iter(|| {
        let mut table = PieceTable::with_capacity(1, 3).src(SRC);
        table.insert(SRC.len() / 2, VALUE)
    });
}

#[bench]
fn fresh_insert_last_table(b: &mut Bencher) {
    b.iter(|| {
        let mut table = PieceTable::with_capacity(1, 2).src(SRC);
        table.insert(SRC.len(), VALUE)
    });
}

#[bench]
fn fresh_insert_scattered_table(b: &mut Bencher) {
    let recipe: InsertScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_table(b, SRC, &recipe.commands, recipe.commands.len());
}

#[bench]
fn fresh_insert_clustered_table(b: &mut Bencher) {
    let recipe: InsertClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_table(b, SRC, &recipe.commands, recipe.commands.len());
}

#[bench]
fn fresh_remove_scattered_table(b: &mut Bencher) {
    let recipe: RemoveScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_table(b, &recipe.data, &recipe.commands, recipe.data.len());
}

#[bench]
fn fresh_remove_clustered_table(b: &mut Bencher) {
    let recipe: RemoveClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_table(b, &recipe.data, &recipe.commands, recipe.data.len());
}

#[bench]
fn fresh_insert_remove_scattered_table(b: &mut Bencher) {
    let recipe: InsertRemoveScatteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_table(b, SRC, &recipe.commands, recipe.elements);
}

#[bench]
fn fresh_insert_remove_clustered_table(b: &mut Bencher) {
    let recipe: InsertRemoveClusteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_table(b, SRC, &recipe.commands, recipe.elements);
}

// Vec

#[bench]
fn fresh_iter_vec(b: &mut Bencher) {
    let mut vec = Vec::with_capacity(SRC.len());
    vec.push_all(SRC);
    b.iter(|| vec.iter().fold(0, |acc, &x| acc + x));
}

#[bench]
fn fresh_insert_first_vec(b: &mut Bencher) {
    let mut vec = Vec::with_capacity(SRC.len() + 100);
    vec.push_all(SRC);
    b.iter(|| vec.insert(0, VALUE));
}

#[bench]
fn fresh_insert_middle_vec(b: &mut Bencher) {
    let mut vec = Vec::with_capacity(SRC.len() + 100);
    vec.push_all(SRC);
    b.iter(|| vec.insert(SRC.len()/2, VALUE));
}

#[bench]
fn fresh_insert_last_vec(b: &mut Bencher) {
    let mut vec = Vec::with_capacity(SRC.len() + 100);
    vec.push_all(SRC);
    b.iter(|| vec.insert(SRC.len(), VALUE))
}

#[bench]
fn fresh_insert_scattered_vec(b: &mut Bencher) {
    let recipe: InsertScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_vec(b, SRC, &recipe.commands, recipe.commands.len());
}

#[bench]
fn fresh_insert_clustered_vec(b: &mut Bencher) {
    let recipe: InsertClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_vec(b, SRC, &recipe.commands, recipe.commands.len());
}

#[bench]
fn fresh_remove_scattered_vec(b: &mut Bencher) {
    let recipe: RemoveScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_vec(b, &recipe.data, &recipe.commands, 0);
}

#[bench]
fn fresh_remove_clustered_vec(b: &mut Bencher) {
    let recipe: RemoveClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_vec(b, &recipe.data, &recipe.commands, 0);
}

#[bench]
fn fresh_insert_remove_scattered_vec(b: &mut Bencher) {
    let recipe: InsertRemoveScatteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_vec(b, SRC, &recipe.commands, recipe.elements);
}

#[bench]
fn fresh_insert_remove_clustered_vec(b: &mut Bencher) {
    let recipe: InsertRemoveClusteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_fresh_vec(b, SRC, &recipe.commands, recipe.elements);
}

// EDITED
// - iter/sum
// - insert first
// - insert mid
// - insert last
// - insert scatter
// - insert clusters
// - remove scatter
// - remove clusters
// - insert/remove scatter
// - insert/remove clusters

fn make_edited_table<'a>(additional: usize) -> PieceTable<'a, i32> {
    let recipe: InsertClustered<i32> = make_recipe(SEED, SIZE);
    let n = recipe.commands.len() + additional;
    let mut table = PieceTable::with_capacity(n, n);
    run_commands_table(&mut table, &recipe.commands);
    table
}

fn make_edited_vec(additional: usize) -> Vec<i32> {
    let recipe: InsertClustered<i32> = make_recipe(SEED, SIZE);
    let mut vec = Vec::with_capacity(recipe.commands.len() + additional);
    run_commands_vec(&mut vec, &recipe.commands);
    vec
}

fn run_benchmark_edited_table(b: &mut Bencher, commands: &[Command<i32>], capacity: usize) {
    let table = make_edited_table(capacity);
    b.iter(|| {
        let mut table = table.clone();
        run_commands_table(&mut table, commands);
    })
}

fn run_benchmark_edited_vec(b: &mut Bencher, commands: &[Command<i32>], capacity: usize) {
    let vec = make_edited_vec(capacity);
    b.iter(|| {
        let mut vec = vec.clone();
        run_commands_vec(&mut vec, commands);
    })
}

// Table

#[bench]
fn edited_iter_table(b: &mut Bencher) {
    let table = make_edited_table(0);
    b.iter(|| table.iter().fold(0, |acc, &x| acc + x));
}

#[bench]
fn edited_insert_first_table(b: &mut Bencher) {
    let table = make_edited_table(1);
    b.iter(|| {
        let mut table = table.clone();
        table.insert(0, VALUE)
    });
}

#[bench]
fn edited_insert_middle_table(b: &mut Bencher) {
    let table = make_edited_table(1);
    let idx = table.len() / 2;
    b.iter(|| {
        let mut table = table.clone();
        table.insert(idx, VALUE)
    });
}

#[bench]
fn edited_insert_last_table(b: &mut Bencher) {
    let table = make_edited_table(1);
    let idx = table.len();
    b.iter(|| {
        let mut table = table.clone();
        table.insert(idx, VALUE)
    });
}

#[bench]
fn edited_insert_scattered_table(b: &mut Bencher) {
    let recipe: InsertScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_table(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn edited_insert_clustered_table(b: &mut Bencher) {
    let recipe: InsertClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_table(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn edited_remove_scattered_table(b: &mut Bencher) {
    let recipe: RemoveScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_table(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn edited_remove_clustered_table(b: &mut Bencher) {
    let recipe: RemoveClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_table(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn edited_insert_remove_scattered_table(b: &mut Bencher) {
    let recipe: InsertRemoveScatteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_table(b, &recipe.commands, recipe.elements);
}

#[bench]
fn edited_insert_remove_clustered_table(b: &mut Bencher) {
    let recipe: InsertRemoveClusteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_table(b, &recipe.commands, recipe.elements);
}

// Vec

#[bench]
fn edited_iter_vec(b: &mut Bencher) {
    let vec = make_edited_vec(0);
    b.iter(|| vec.iter().fold(0, |acc, &x| acc + x));
}

#[bench]
fn edited_insert_first_vec(b: &mut Bencher) {
    let vec = make_edited_vec(1);
    b.iter(|| {
        let mut vec = vec.clone();
       vec.insert(0, VALUE)
    });
}

#[bench]
fn edited_insert_middle_vec(b: &mut Bencher) {
    let vec = make_edited_vec(1);
    let idx = vec.len()/2;
    b.iter(|| {
        let mut vec = vec.clone();
       vec.insert(idx, VALUE)
    });
}

#[bench]
fn edited_insert_last_vec(b: &mut Bencher) {
    let vec = make_edited_vec(1);
    let idx = vec.len();
    b.iter(|| {
        let mut vec = vec.clone();
       vec.insert(idx, VALUE)
    });
}

#[bench]
fn edited_insert_scattered_vec(b: &mut Bencher) {
    let recipe: InsertScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_vec(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn edited_insert_clustered_vec(b: &mut Bencher) {
    let recipe: InsertClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_vec(b, &recipe.commands, recipe.commands.len());
}

#[bench]
fn edited_remove_scattered_vec(b: &mut Bencher) {
    let recipe: RemoveScattered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_vec(b, &recipe.commands, 0);
}

#[bench]
fn edited_remove_clustered_vec(b: &mut Bencher) {
    let recipe: RemoveClustered<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_vec(b, &recipe.commands, 0);
}

#[bench]
fn edited_insert_remove_scattered_vec(b: &mut Bencher) {
    let recipe: InsertRemoveScatteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_vec(b, &recipe.commands, recipe.elements);
}

#[bench]
fn edited_insert_remove_clustered_vec(b: &mut Bencher) {
    let recipe: InsertRemoveClusteredEmpty<i32> = make_recipe(SEED, SIZE);
    run_benchmark_edited_vec(b, &recipe.commands, recipe.elements);
}

#[bench]
fn push_table(b: &mut Bencher) {
    let data: Vec<i32> = make_recipe(SEED, SIZE);

    b.iter(|| {
        let mut table = PieceTable::with_capacity(data.len(), 1);

        for x in &data {
            table.push(x);
        }
    });
}

#[bench]
fn push_vec(b: &mut Bencher) {
    let data: Vec<i32> = make_recipe(SEED, SIZE);

    b.iter(|| {
        let mut vec = Vec::with_capacity(data.len());

        for x in &data {
            vec.push(x);
        }
    });
}
