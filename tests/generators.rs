extern crate rand;
extern crate quickcheck;

use std::cmp;
use self::rand::{Rng, SeedableRng, StdRng};
use self::quickcheck::{Arbitrary, Gen, StdGen};

// TODO: These shrink poorly

pub fn make_recipe<T>(seed: &[usize], size: usize) -> T
    where T: Send + Clone + Arbitrary
{
    let rng: StdRng = SeedableRng::from_seed(seed);
    let mut g = StdGen::new(rng, size);
    Arbitrary::arbitrary(&mut g)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command<T> {
    Insert(usize, T),
    Remove(usize),
}

pub use self::Command::*;

#[derive(Clone, Debug)]
pub struct InsertScattered<T: Arbitrary> {
    pub commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
pub struct InsertClustered<T: Arbitrary> {
    pub commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
pub struct RemoveScattered<T: Arbitrary> {
    pub data: Vec<T>,
    pub commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
pub struct RemoveClustered<T: Arbitrary> {
    pub data: Vec<T>,
    pub commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
pub struct InsertRemoveScatteredEmpty<T: Arbitrary> {
    pub commands: Vec<Command<T>>,
    pub elements: usize,
}

#[derive(Clone, Debug)]
pub struct InsertRemoveScatteredGiven<T: Arbitrary> {
    pub data: Vec<T>,
    pub commands: Vec<Command<T>>,
}

#[derive(Clone, Debug)]
pub struct InsertRemoveClusteredEmpty<T: Arbitrary> {
    pub commands: Vec<Command<T>>,
    pub elements: usize,
}

#[derive(Clone, Debug)]
pub struct InsertRemoveClusteredGiven<T: Arbitrary> {
    pub data: Vec<T>,
    pub commands: Vec<Command<T>>,
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
            let cluster_size = cmp::min(
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
            let cluster_size = cmp::min(
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
