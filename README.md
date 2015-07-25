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
test edited_insert_clustered_table        ... bench:       6,558 ns/iter (+/- 2,972)
test edited_insert_clustered_vec          ... bench:      81,306 ns/iter (+/- 26,984)
test edited_insert_first_table            ... bench:         324 ns/iter (+/- 62)
test edited_insert_first_vec              ... bench:         246 ns/iter (+/- 77)
test edited_insert_last_table             ... bench:         332 ns/iter (+/- 124)
test edited_insert_last_vec               ... bench:         192 ns/iter (+/- 94)
test edited_insert_middle_table           ... bench:         370 ns/iter (+/- 184)
test edited_insert_middle_vec             ... bench:         226 ns/iter (+/- 101)
test edited_insert_remove_clustered_table ... bench:       7,233 ns/iter (+/- 1,252)
test edited_insert_remove_clustered_vec   ... bench:      73,051 ns/iter (+/- 22,828)
test edited_insert_remove_scattered_table ... bench:     163,725 ns/iter (+/- 29,051)
test edited_insert_remove_scattered_vec   ... bench:      75,226 ns/iter (+/- 33,783)
test edited_insert_scattered_table        ... bench:     324,031 ns/iter (+/- 123,122)
test edited_insert_scattered_vec          ... bench:      80,923 ns/iter (+/- 26,047)
test edited_iter_table                    ... bench:         892 ns/iter (+/- 311)
test edited_iter_vec                      ... bench:          62 ns/iter (+/- 14)
test edited_remove_clustered_table        ... bench:       1,712 ns/iter (+/- 711)
test edited_remove_clustered_vec          ... bench:      10,985 ns/iter (+/- 4,331)
test edited_remove_scattered_table        ... bench:      12,472 ns/iter (+/- 2,570)
test edited_remove_scattered_vec          ... bench:       9,949 ns/iter (+/- 2,472)
test empty_insert_clustered_table         ... bench:       6,630 ns/iter (+/- 2,522)
test empty_insert_clustered_vec           ... bench:      23,092 ns/iter (+/- 5,206)
test empty_insert_remove_clustered_table  ... bench:       7,275 ns/iter (+/- 1,297)
test empty_insert_remove_clustered_vec    ... bench:       8,432 ns/iter (+/- 2,555)
test empty_insert_remove_scattered_table  ... bench:     159,384 ns/iter (+/- 29,131)
test empty_insert_remove_scattered_vec    ... bench:      15,023 ns/iter (+/- 8,358)
test empty_insert_scattered_table         ... bench:     322,700 ns/iter (+/- 91,618)
test empty_insert_scattered_vec           ... bench:      24,310 ns/iter (+/- 5,540)
test empty_insert_single_table            ... bench:          62 ns/iter (+/- 15)
test empty_insert_single_vec              ... bench:          27 ns/iter (+/- 8)
test fresh_insert_clustered_table         ... bench:       6,862 ns/iter (+/- 3,123)
test fresh_insert_clustered_vec           ... bench:   1,549,529 ns/iter (+/- 462,813)
test fresh_insert_first_table             ... bench:          91 ns/iter (+/- 46)
test fresh_insert_first_vec               ... bench:       1,614 ns/iter (+/- 368)
test fresh_insert_last_table              ... bench:          98 ns/iter (+/- 26)
test fresh_insert_last_vec                ... bench:         604 ns/iter (+/- 239)
test fresh_insert_middle_table            ... bench:         108 ns/iter (+/- 24)
test fresh_insert_middle_vec              ... bench:         588 ns/iter (+/- 222)
test fresh_insert_remove_clustered_table  ... bench:       7,262 ns/iter (+/- 1,587)
test fresh_insert_remove_clustered_vec    ... bench:   1,638,679 ns/iter (+/- 259,018)
test fresh_insert_remove_scattered_table  ... bench:     163,610 ns/iter (+/- 30,118)
test fresh_insert_remove_scattered_vec    ... bench:   1,377,960 ns/iter (+/- 226,879)
test fresh_insert_scattered_table         ... bench:     329,222 ns/iter (+/- 84,486)
test fresh_insert_scattered_vec           ... bench:   1,677,579 ns/iter (+/- 320,670)
test fresh_iter_table                     ... bench:       6,988 ns/iter (+/- 3,270)
test fresh_iter_vec                       ... bench:       1,037 ns/iter (+/- 161)
test fresh_remove_clustered_table         ... bench:       1,430 ns/iter (+/- 617)
test fresh_remove_clustered_vec           ... bench:     351,620 ns/iter (+/- 99,937)
test fresh_remove_scattered_table         ... bench:      12,645 ns/iter (+/- 5,994)
test fresh_remove_scattered_vec           ... bench:     334,327 ns/iter (+/- 101,321)
```

These benchmarks are arguably doing too much, and the variances show that, but they still give a good idea of the advantage of this data structure.
Inserting clusters arbitrarily is much faster, as is removal at arbitrary points.
For scattered editing, the vector is more suitable.

# Building

I'm using nightly Rust and Cargo.
