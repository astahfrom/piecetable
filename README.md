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
test edited_insert_clustered_table        ... bench:       8,136 ns/iter (+/- 2,180)
test edited_insert_clustered_vec          ... bench:      85,083 ns/iter (+/- 34,651)
test edited_insert_first_table            ... bench:         357 ns/iter (+/- 60)
test edited_insert_first_vec              ... bench:         229 ns/iter (+/- 99)
test edited_insert_last_table             ... bench:         349 ns/iter (+/- 59)
test edited_insert_last_vec               ... bench:         193 ns/iter (+/- 105)
test edited_insert_middle_table           ... bench:         359 ns/iter (+/- 113)
test edited_insert_middle_vec             ... bench:         235 ns/iter (+/- 94)
test edited_insert_remove_clustered_table ... bench:       8,563 ns/iter (+/- 1,505)
test edited_insert_remove_clustered_vec   ... bench:      68,466 ns/iter (+/- 25,460)
test edited_insert_remove_scattered_table ... bench:     149,508 ns/iter (+/- 68,402)
test edited_insert_remove_scattered_vec   ... bench:      80,183 ns/iter (+/- 20,988)
test edited_insert_scattered_table        ... bench:     323,165 ns/iter (+/- 106,685)
test edited_insert_scattered_vec          ... bench:      78,562 ns/iter (+/- 33,350)
test edited_iter_table                    ... bench:         886 ns/iter (+/- 387)
test edited_iter_vec                      ... bench:          59 ns/iter (+/- 19)
test edited_remove_clustered_table        ... bench:       1,863 ns/iter (+/- 721)
test edited_remove_clustered_vec          ... bench:      11,007 ns/iter (+/- 3,444)
test edited_remove_scattered_table        ... bench:      15,046 ns/iter (+/- 9,205)
test edited_remove_scattered_vec          ... bench:      10,027 ns/iter (+/- 4,728)
test empty_insert_clustered_table         ... bench:       7,739 ns/iter (+/- 2,944)
test empty_insert_clustered_vec           ... bench:      20,358 ns/iter (+/- 6,492)
test empty_insert_remove_clustered_table  ... bench:       8,116 ns/iter (+/- 1,056)
test empty_insert_remove_clustered_vec    ... bench:       8,598 ns/iter (+/- 2,657)
test empty_insert_remove_scattered_table  ... bench:     148,858 ns/iter (+/- 68,658)
test empty_insert_remove_scattered_vec    ... bench:      13,747 ns/iter (+/- 1,809)
test empty_insert_scattered_table         ... bench:     317,274 ns/iter (+/- 124,235)
test empty_insert_scattered_vec           ... bench:      24,059 ns/iter (+/- 3,944)
test empty_insert_single_table            ... bench:          61 ns/iter (+/- 32)
test empty_insert_single_vec              ... bench:           0 ns/iter (+/- 0)
test fresh_insert_clustered_table         ... bench:       7,817 ns/iter (+/- 3,985)
test fresh_insert_clustered_vec           ... bench:   1,641,414 ns/iter (+/- 305,853)
test fresh_insert_first_table             ... bench:         106 ns/iter (+/- 63)
test fresh_insert_first_vec               ... bench:       2,559 ns/iter (+/- 496)
test fresh_insert_last_table              ... bench:         102 ns/iter (+/- 39)
test fresh_insert_last_vec                ... bench:       5,014 ns/iter (+/- 245)
test fresh_insert_middle_table            ... bench:         130 ns/iter (+/- 86)
test fresh_insert_middle_vec              ... bench:       1,365 ns/iter (+/- 324)
test fresh_insert_remove_clustered_table  ... bench:       7,941 ns/iter (+/- 3,212)
test fresh_insert_remove_clustered_vec    ... bench:   1,534,895 ns/iter (+/- 571,763)
test fresh_insert_remove_scattered_table  ... bench:     145,605 ns/iter (+/- 62,469)
test fresh_insert_remove_scattered_vec    ... bench:   1,303,346 ns/iter (+/- 207,086)
test fresh_insert_scattered_table         ... bench:     328,885 ns/iter (+/- 105,175)
test fresh_insert_scattered_vec           ... bench:   1,493,562 ns/iter (+/- 520,357)
test fresh_iter_table                     ... bench:       7,202 ns/iter (+/- 2,782)
test fresh_iter_vec                       ... bench:       1,067 ns/iter (+/- 189)
test fresh_remove_clustered_table         ... bench:       1,764 ns/iter (+/- 753)
test fresh_remove_clustered_vec           ... bench:       4,167 ns/iter (+/- 618)
test fresh_remove_scattered_table         ... bench:      13,770 ns/iter (+/- 5,518)
test fresh_remove_scattered_vec           ... bench:       3,406 ns/iter (+/- 656)
test push_table                           ... bench:       1,474 ns/iter (+/- 648)
test push_vec                             ... bench:         410 ns/iter (+/- 195)
```

These benchmarks are arguably doing too much, and the variances show that, but they still give a good idea of the advantage of this data structure.
Inserting clusters arbitrarily is much faster, as is removal at arbitrary points.
For scattered editing, the vector is more suitable.

# Building

I'm using nightly Rust and Cargo.
