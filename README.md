# GFA-dead-end-counter

[![License GPL v3](https://img.shields.io/badge/license-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)

This is a very simple tool to count the number of dead ends in a [GFA v1](https://gfa-spec.github.io/GFA-spec/GFA1.html) assembly graph. It's lightweight and written in [Rust](https://www.rust-lang.org), so it's fast and can be used on very large GFA files.

You can read more about assembly graph dead ends in this blog post: [Dead end count for QC of short-read assemblies](https://rrwick.github.io/2023/02/21/dead-ends-for-assembly-qc.html). If you use this tool in your research, please cite that blog post via [its Zenodo DOI](https://doi.org/10.5281/zenodo.7662683).



### Usage

The executable named `deadends` takes a GFA file (can be gzipped) as an argument, and it will print the number of dead ends in the graph to stdout. Run it like this:

```bash
deadends assembly.gfa
```

The tool only takes one file at a time, so if you need to run it on lots of assembly graphs, you can use a Bash loop like this (modify the glob as appropriate for your data):
```bash
for g in *.gfa; do
    printf $g"\t"
    deadends "$g"
done
```

Full help text:
```
GFA dead-end counter

Usage: deadends <GFA>

Arguments:
  <GFA>  Input graph file (GFA v1 format)

Options:
  -h, --help     Print help
  -V, --version  Print version
```



### Installation from pre-built binaries

GFA-dead-end-counter compiles to a single executable binary (`deadends`), which makes installation easy!

You can find pre-built binaries for common OSs/CPUs on the [releases page](https://github.com/rrwick/GFA-dead-end-counter/releases). If you use one of these OSs/CPUs, download the appropriate binary for your system and put the `deadends` file in a directory that's in your `PATH` variable, e.g. `/usr/local/bin/` or `~/.local/bin/`.

Alternatively, you don't need to install GFA-dead-end-counter at all. Instead, just run it from wherever the `deadends` executable happens to be, like this: `/some/path/to/deadends --help`.



### Installation from source

If you are using incompatible hardware or a different OS, then you'll have to build GFA-dead-end-counter from source. [Install Rust](https://www.rust-lang.org/tools/install) if you don't already have it. Then clone and build GFA-dead-end-counter like this:
```
git clone https://github.com/rrwick/GFA-dead-end-counter.git
cd GFA-dead-end-counter
cargo build --release
```

You'll find the freshly built executable in `target/release/deadends`, which you can then move to an appropriate location that's in your `PATH` variable.



### License

[GNU General Public License, version 3](https://www.gnu.org/licenses/gpl-3.0.html)
