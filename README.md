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
test edited_insert_clustered_table        ... bench:       8,430 ns/iter (+/- 1,414)
test edited_insert_clustered_vec          ... bench:      84,121 ns/iter (+/- 26,792)
test edited_insert_first_table            ... bench:         343 ns/iter (+/- 129)
test edited_insert_first_vec              ... bench:         251 ns/iter (+/- 97)
test edited_insert_last_table             ... bench:         332 ns/iter (+/- 48)
test edited_insert_last_vec               ... bench:         191 ns/iter (+/- 58)
test edited_insert_middle_table           ... bench:         350 ns/iter (+/- 138)
test edited_insert_middle_vec             ... bench:         221 ns/iter (+/- 115)
test edited_insert_remove_clustered_table ... bench:       8,315 ns/iter (+/- 2,987)
test edited_insert_remove_clustered_vec   ... bench:      67,956 ns/iter (+/- 22,672)
test edited_insert_remove_scattered_table ... bench:     153,687 ns/iter (+/- 53,704)
test edited_insert_remove_scattered_vec   ... bench:      75,101 ns/iter (+/- 28,954)
test edited_insert_scattered_table        ... bench:     332,530 ns/iter (+/- 105,369)
test edited_insert_scattered_vec          ... bench:      81,319 ns/iter (+/- 24,769)
test edited_iter_table                    ... bench:         841 ns/iter (+/- 298)
test edited_iter_vec                      ... bench:          59 ns/iter (+/- 23)
test edited_remove_clustered_table        ... bench:       1,834 ns/iter (+/- 661)
test edited_remove_clustered_vec          ... bench:      10,974 ns/iter (+/- 3,261)
test edited_remove_scattered_table        ... bench:      14,652 ns/iter (+/- 5,560)
test edited_remove_scattered_vec          ... bench:      10,864 ns/iter (+/- 3,707)
test empty_insert_clustered_table         ... bench:       9,080 ns/iter (+/- 2,974)
test empty_insert_clustered_vec           ... bench:      20,874 ns/iter (+/- 8,708)
test empty_insert_remove_clustered_table  ... bench:       8,022 ns/iter (+/- 3,076)
test empty_insert_remove_clustered_vec    ... bench:       8,548 ns/iter (+/- 2,485)
test empty_insert_remove_scattered_table  ... bench:     151,851 ns/iter (+/- 66,835)
test empty_insert_remove_scattered_vec    ... bench:      13,642 ns/iter (+/- 2,327)
test empty_insert_scattered_table         ... bench:     351,121 ns/iter (+/- 158,236)
test empty_insert_scattered_vec           ... bench:      23,444 ns/iter (+/- 9,007)
test empty_insert_single_table            ... bench:          74 ns/iter (+/- 26)
test empty_insert_single_vec              ... bench:          26 ns/iter (+/- 11)
test fresh_insert_clustered_table         ... bench:       9,155 ns/iter (+/- 3,484)
test fresh_insert_clustered_vec           ... bench:   1,699,775 ns/iter (+/- 366,498)
test fresh_insert_first_table             ... bench:          93 ns/iter (+/- 31)
test fresh_insert_first_vec               ... bench:       1,673 ns/iter (+/- 604)
test fresh_insert_last_table              ... bench:          87 ns/iter (+/- 40)
test fresh_insert_last_vec                ... bench:         245 ns/iter (+/- 56)
test fresh_insert_middle_table            ... bench:         115 ns/iter (+/- 62)
test fresh_insert_middle_vec              ... bench:       1,776 ns/iter (+/- 257)
test fresh_insert_remove_clustered_table  ... bench:       8,109 ns/iter (+/- 2,402)
test fresh_insert_remove_clustered_vec    ... bench:   1,593,900 ns/iter (+/- 530,101)
test fresh_insert_remove_scattered_table  ... bench:     151,632 ns/iter (+/- 60,862)
test fresh_insert_remove_scattered_vec    ... bench:   1,335,647 ns/iter (+/- 340,802)
test fresh_insert_scattered_table         ... bench:     317,177 ns/iter (+/- 86,529)
test fresh_insert_scattered_vec           ... bench:   1,675,689 ns/iter (+/- 300,617)
test fresh_iter_table                     ... bench:       6,980 ns/iter (+/- 2,414)
test fresh_iter_vec                       ... bench:       1,061 ns/iter (+/- 187)
test fresh_remove_clustered_table         ... bench:       1,698 ns/iter (+/- 773)
test fresh_remove_clustered_vec           ... bench:       4,041 ns/iter (+/- 1,371)
test fresh_remove_scattered_table         ... bench:      14,537 ns/iter (+/- 5,627)
test fresh_remove_scattered_vec           ... bench:       3,622 ns/iter (+/- 725)
```

These benchmarks are arguably doing too much, and the variances show that, but they still give a good idea of the advantage of this data structure.
Inserting clusters arbitrarily is much faster, as is removal at arbitrary points.
For scattered editing, the vector is more suitable.

# Building

I'm using nightly Rust and Cargo.
