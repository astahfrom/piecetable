PieceTable
----------
[Documentation](http://andreasfrom.github.io/piecetable/)

Implementation of a piece table in Rust for storing text for efficient editing.

Based on [this paper](https://www.cs.unm.edu/~crowley/papers/sds.pdf).

Tested with the excellent [quickcheck](https://github.com/BurntSushi/quickcheck), but still a work in progress.
There are some things around ownership and iterators that need more thought.

# Benchmarks

Compared with the standard Vec(tor):

```
test tests::empty_insert_100_clusters_of_100_table     ... bench:      70,960 ns/iter (+/- 31,678)
test tests::empty_insert_100_clusters_of_100_vec       ... bench:   4,244,045 ns/iter (+/- 625,171)
test tests::empty_insert_10k_linear_table              ... bench:      62,066 ns/iter (+/- 26,844)
test tests::empty_insert_10k_linear_vec                ... bench:      40,693 ns/iter (+/- 15,271)
test tests::given_10k_index_sum_table                  ... bench:      24,963 ns/iter (+/- 11,940)
test tests::given_10k_index_sum_vec                    ... bench:       1,049 ns/iter (+/- 199)
test tests::given_10k_insert_first_table               ... bench:          91 ns/iter (+/- 29)
test tests::given_10k_insert_first_vec                 ... bench:       1,676 ns/iter (+/- 681)
test tests::given_10k_insert_last_table                ... bench:          85 ns/iter (+/- 37)
test tests::given_10k_insert_last_vec                  ... bench:           3 ns/iter (+/- 1)
test tests::given_10k_insert_then_remove_100_mid_table ... bench:      19,483 ns/iter (+/- 4,885)
test tests::given_10k_insert_then_remove_100_mid_vec   ... bench:      94,378 ns/iter (+/- 36,718)
test tests::given_10k_iter_table                       ... bench:      11,982 ns/iter (+/- 3,780)
test tests::given_10k_iter_vec                         ... bench:       1,045 ns/iter (+/- 195)
test tests::given_10k_remove_mid_100_backwards_table   ... bench:      23,858 ns/iter (+/- 9,933)
test tests::given_10k_remove_mid_100_backwards_vec     ... bench:     249,770 ns/iter (+/- 75,786)
test tests::given_10k_remove_mid_100_forwards_table    ... bench:      25,588 ns/iter (+/- 4,508)
test tests::given_10k_remove_mid_100_forwards_vec      ... bench:      54,014 ns/iter (+/- 16,758)
```

These benchmarks are arguably doing too much, and the variances show that, but it stills gives a good idea of the advantage of this data structure.
Inserting clusters arbitrarily is much faster, as is removal at arbitrary points.

# Building

I'm using nightly Rust and Cargo.
