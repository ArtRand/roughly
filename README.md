## Example Smith-Waterman in Rust-lang
> Implementation of the Smith-Waterman DNA alignment algorithm, for learning purposes.

### Usage

Takes a FASTA formatted file with a pair of sequences and aligns them. Pretty-prints the output to stdout.

```
$ cargo build
$ cargo run -- -h  # see help

# run aligner
$ cargo run -- -f examples/wiki-align.fa
```


