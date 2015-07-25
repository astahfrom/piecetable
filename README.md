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
test edited_insert_clustered_table        ... bench:       7,771 ns/iter (+/- 2,416)
test edited_insert_clustered_vec          ... bench:      81,553 ns/iter (+/- 34,085)
test edited_insert_first_table            ... bench:         312 ns/iter (+/- 34)
test edited_insert_first_vec              ... bench:         248 ns/iter (+/- 56)
test edited_insert_last_table             ... bench:         327 ns/iter (+/- 101)
test edited_insert_last_vec               ... bench:         192 ns/iter (+/- 58)
test edited_insert_middle_table           ... bench:         328 ns/iter (+/- 157)
test edited_insert_middle_vec             ... bench:         214 ns/iter (+/- 86)
test edited_insert_remove_clustered_table ... bench:       7,812 ns/iter (+/- 1,246)
test edited_insert_remove_clustered_vec   ... bench:      69,334 ns/iter (+/- 29,484)
test edited_insert_remove_scattered_table ... bench:     154,513 ns/iter (+/- 67,964)
test edited_insert_remove_scattered_vec   ... bench:      75,521 ns/iter (+/- 26,517)
test edited_insert_scattered_table        ... bench:     360,339 ns/iter (+/- 61,398)
test edited_insert_scattered_vec          ... bench:      81,349 ns/iter (+/- 33,069)
test edited_iter_table                    ... bench:         893 ns/iter (+/- 317)
test edited_iter_vec                      ... bench:          64 ns/iter (+/- 32)
test edited_remove_clustered_table        ... bench:       1,859 ns/iter (+/- 768)
test edited_remove_clustered_vec          ... bench:      10,382 ns/iter (+/- 2,166)
test edited_remove_scattered_table        ... bench:      13,159 ns/iter (+/- 3,500)
test edited_remove_scattered_vec          ... bench:       9,896 ns/iter (+/- 3,708)
test empty_insert_clustered_table         ... bench:       8,338 ns/iter (+/- 1,821)
test empty_insert_clustered_vec           ... bench:      19,693 ns/iter (+/- 7,417)
test empty_insert_remove_clustered_table  ... bench:       7,516 ns/iter (+/- 1,544)
test empty_insert_remove_clustered_vec    ... bench:       7,905 ns/iter (+/- 3,640)
test empty_insert_remove_scattered_table  ... bench:     147,570 ns/iter (+/- 57,519)
test empty_insert_remove_scattered_vec    ... bench:      12,366 ns/iter (+/- 2,175)
test empty_insert_scattered_table         ... bench:     348,681 ns/iter (+/- 56,433)
test empty_insert_scattered_vec           ... bench:      23,902 ns/iter (+/- 5,448)
test empty_insert_single_table            ... bench:          66 ns/iter (+/- 30)
test empty_insert_single_vec              ... bench:          26 ns/iter (+/- 13)
test fresh_insert_clustered_table         ... bench:       8,505 ns/iter (+/- 4,918)
test fresh_insert_clustered_vec           ... bench:   1,706,412 ns/iter (+/- 322,227)
test fresh_insert_first_table             ... bench:          97 ns/iter (+/- 19)
test fresh_insert_first_vec               ... bench:       1,691 ns/iter (+/- 786)
test fresh_insert_last_table              ... bench:          96 ns/iter (+/- 18)
test fresh_insert_last_vec                ... bench:         805 ns/iter (+/- 238)
test fresh_insert_middle_table            ... bench:          95 ns/iter (+/- 48)
test fresh_insert_middle_vec              ... bench:         989 ns/iter (+/- 315)
test fresh_insert_remove_clustered_table  ... bench:       7,185 ns/iter (+/- 2,228)
test fresh_insert_remove_clustered_vec    ... bench:   1,637,769 ns/iter (+/- 304,751)
test fresh_insert_remove_scattered_table  ... bench:     162,108 ns/iter (+/- 54,566)
test fresh_insert_remove_scattered_vec    ... bench:   1,388,342 ns/iter (+/- 349,844)
test fresh_insert_scattered_table         ... bench:     360,758 ns/iter (+/- 62,489)
test fresh_insert_scattered_vec           ... bench:   1,655,242 ns/iter (+/- 364,599)
test fresh_iter_table                     ... bench:       7,133 ns/iter (+/- 2,495)
test fresh_iter_vec                       ... bench:         988 ns/iter (+/- 520)
test fresh_remove_clustered_table         ... bench:       1,559 ns/iter (+/- 540)
test fresh_remove_clustered_vec           ... bench:       3,831 ns/iter (+/- 1,299)
test fresh_remove_scattered_table         ... bench:      12,410 ns/iter (+/- 4,604)
test fresh_remove_scattered_vec           ... bench:       3,237 ns/iter (+/- 1,614)
```

These benchmarks are arguably doing too much, and the variances show that, but they still give a good idea of the advantage of this data structure.
Inserting clusters arbitrarily is much faster, as is removal at arbitrary points.
For scattered editing, the vector is more suitable.

# Building

I'm using nightly Rust and Cargo.
