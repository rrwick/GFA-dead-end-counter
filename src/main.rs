// Copyright 2023 Ryan Wick (rrwick@gmail.com)
// https://github.com/rrwick/GFA-dead-end-counter

// This file is part of GFA-dead-end-counter. GFA-dead-end-counter is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License, or (at your option) any later
// version. GFA-dead-end-counter is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE.  See the GNU General Public License for more details. You should have received a copy
// of the GNU General Public License along with GFA-dead-end-counter. If not, see
// <http://www.gnu.org/licenses/>.

mod misc;
mod gfa;

use std::path::PathBuf;
use rustc_hash::FxHashSet;
use clap::{Parser, crate_version, crate_description};


#[derive(Parser)]
#[clap(name = "GFA dead-end counter",
       version = concat!("v", crate_version!()),
       about = crate_description!())]
struct Cli {
    /// Input graph file (GFA v1 format)
    gfa: PathBuf,
}


fn main() {
    let cli = Cli::parse();
    misc::check_if_file_exists(&cli.gfa);
    println!("{}", count_dead_ends(&cli.gfa));
}


fn count_dead_ends(filename: &PathBuf) -> usize {
    let (segments, links) = gfa::load_gfa(filename);

    // Each segment initially gets a dead start and a dead end.
    let mut dead_starts = FxHashSet::default();
    let mut dead_ends = FxHashSet::default();
    for name in segments {
        dead_starts.insert(name.clone());
        dead_ends.insert(name);
    }

    // Then dead starts/ends are removed based on the links.
    for link in links {
        if link.strand_a == 1 {
            dead_ends.remove(&link.name_a);
        } else {  // link.strand_a == -1
            dead_starts.remove(&link.name_a);
        }
        if link.strand_b == 1 {
            dead_starts.remove(&link.name_b);
        } else {  // link.strand_b == -1
            dead_ends.remove(&link.name_b);
        }
    }

    dead_starts.len() + dead_ends.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_dead_ends() {

        fn test_file(filename: &str) -> PathBuf {
            let mut gfa = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            gfa.push("tests");
            gfa.push(filename);
            return gfa
        }

        let dead_ends = count_dead_ends(&test_file("circular.gfa"));
        assert_eq!(dead_ends, 0);

        let dead_ends = count_dead_ends(&test_file("circular.gfa.gz"));
        assert_eq!(dead_ends, 0);

        let dead_ends = count_dead_ends(&test_file("circular_blank_lines.gfa"));
        assert_eq!(dead_ends, 0);

        let dead_ends = count_dead_ends(&test_file("linear.gfa"));
        assert_eq!(dead_ends, 2);

        let dead_ends = count_dead_ends(&test_file("linear.gfa.gz"));
        assert_eq!(dead_ends, 2);

        let dead_ends = count_dead_ends(&test_file("seq.fasta"));
        assert_eq!(dead_ends, 0);

        let dead_ends = count_dead_ends(&test_file("seq.fastq"));
        assert_eq!(dead_ends, 0);

        let dead_ends = count_dead_ends(&test_file("medium.gfa.gz"));
        assert_eq!(dead_ends, 0);

        let dead_ends = count_dead_ends(&test_file("big.gfa.gz"));
        assert_eq!(dead_ends, 29312);
    }
}
