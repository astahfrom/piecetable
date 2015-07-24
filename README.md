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
test edited_insert_clustered_table        ... bench:       6,635 ns/iter (+/- 1,978)
test edited_insert_clustered_vec          ... bench:      82,753 ns/iter (+/- 27,665)
test edited_insert_first_table            ... bench:         318 ns/iter (+/- 110)
test edited_insert_first_vec              ... bench:         238 ns/iter (+/- 71)
test edited_insert_last_table             ... bench:         314 ns/iter (+/- 115)
test edited_insert_last_vec               ... bench:         195 ns/iter (+/- 104)
test edited_insert_middle_table           ... bench:         330 ns/iter (+/- 151)
test edited_insert_middle_vec             ... bench:         221 ns/iter (+/- 52)
test edited_insert_remove_clustered_table ... bench:       6,973 ns/iter (+/- 1,284)
test edited_insert_remove_clustered_vec   ... bench:      73,272 ns/iter (+/- 12,002)
test edited_insert_remove_scattered_table ... bench:     149,678 ns/iter (+/- 51,692)
test edited_insert_remove_scattered_vec   ... bench:      72,312 ns/iter (+/- 27,147)
test edited_insert_scattered_table        ... bench:     318,971 ns/iter (+/- 110,125)
test edited_insert_scattered_vec          ... bench:      79,430 ns/iter (+/- 34,462)
test edited_iter_table                    ... bench:       1,265 ns/iter (+/- 268)
test edited_iter_vec                      ... bench:          58 ns/iter (+/- 27)
test edited_remove_clustered_table        ... bench:       1,867 ns/iter (+/- 281)
test edited_remove_clustered_vec          ... bench:      10,981 ns/iter (+/- 3,534)
test edited_remove_scattered_table        ... bench:      14,375 ns/iter (+/- 5,636)
test edited_remove_scattered_vec          ... bench:      10,175 ns/iter (+/- 3,283)
test empty_insert_clustered_table         ... bench:       6,563 ns/iter (+/- 2,867)
test empty_insert_clustered_vec           ... bench:      20,088 ns/iter (+/- 6,375)
test empty_insert_remove_clustered_table  ... bench:       7,322 ns/iter (+/- 1,371)
test empty_insert_remove_clustered_vec    ... bench:       8,141 ns/iter (+/- 3,473)
test empty_insert_remove_scattered_table  ... bench:     152,549 ns/iter (+/- 63,299)
test empty_insert_remove_scattered_vec    ... bench:      13,604 ns/iter (+/- 2,509)
test empty_insert_scattered_table         ... bench:     350,957 ns/iter (+/- 124,744)
test empty_insert_scattered_vec           ... bench:      23,881 ns/iter (+/- 4,035)
test empty_insert_single_table            ... bench:          69 ns/iter (+/- 35)
test empty_insert_single_vec              ... bench:          27 ns/iter (+/- 12)
test fresh_insert_clustered_table         ... bench:       6,693 ns/iter (+/- 3,034)
test fresh_insert_clustered_vec           ... bench:   1,669,135 ns/iter (+/- 312,653)
test fresh_insert_first_table             ... bench:          87 ns/iter (+/- 33)
test fresh_insert_first_vec               ... bench:       1,665 ns/iter (+/- 646)
test fresh_insert_last_table              ... bench:          80 ns/iter (+/- 34)
test fresh_insert_last_vec                ... bench:         915 ns/iter (+/- 243)
test fresh_insert_middle_table            ... bench:          91 ns/iter (+/- 53)
test fresh_insert_middle_vec              ... bench:         764 ns/iter (+/- 229)
test fresh_insert_remove_clustered_table  ... bench:       6,915 ns/iter (+/- 2,364)
test fresh_insert_remove_clustered_vec    ... bench:   1,628,741 ns/iter (+/- 288,611)
test fresh_insert_remove_scattered_table  ... bench:     159,139 ns/iter (+/- 48,696)
test fresh_insert_remove_scattered_vec    ... bench:   1,271,398 ns/iter (+/- 329,702)
test fresh_insert_scattered_table         ... bench:     319,539 ns/iter (+/- 161,955)
test fresh_insert_scattered_vec           ... bench:   1,679,293 ns/iter (+/- 305,372)
test fresh_iter_table                     ... bench:      11,903 ns/iter (+/- 2,244)
test fresh_iter_vec                       ... bench:       1,037 ns/iter (+/- 162)
test fresh_remove_clustered_table         ... bench:       1,357 ns/iter (+/- 218)
test fresh_remove_clustered_vec           ... bench:     344,666 ns/iter (+/- 81,919)
test fresh_remove_scattered_table         ... bench:      14,124 ns/iter (+/- 7,424)
test fresh_remove_scattered_vec           ... bench:     347,406 ns/iter (+/- 62,943)
```

These benchmarks are arguably doing too much, and the variances show that, but they still give a good idea of the advantage of this data structure.
Inserting clusters arbitrarily is much faster, as is removal at arbitrary points.
For scattered editing, the vector is more suitable.

# Building

I'm using nightly Rust and Cargo.
