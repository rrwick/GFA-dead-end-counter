// Copyright 2023 Ryan Wick (rrwick@gmail.com)
// https://github.com/rrwick/GFA-Dead-End-Counter

// This file is part of GFA-Dead-End-Counter. GFA-Dead-End-Counter is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public License as published
// by the Free Software Foundation, either version 3 of the License, or (at your option) any later
// version. GFA-Dead-End-Counter is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE.  See the GNU General Public License for more details. You should have received a copy
// of the GNU General Public License along with GFA-Dead-End-Counter. If not, see
// <http://www.gnu.org/licenses/>.

use crate::misc::{quit_with_error, is_file_gzipped};

use flate2::read::GzDecoder;

use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::path::PathBuf;


/// The Link struct holds the relevant info from a GFA link line: which segments are linked and
/// on which strands.
#[derive(PartialEq, Debug)]
pub struct Link {
    pub name_a: String,
    pub strand_a: i32,
    pub name_b: String,
    pub strand_b: i32,
}


/// This function reads a GFA file and returns two vectors: segment names and links.
pub fn load_gfa(filename: &PathBuf) -> (Vec<String>, Vec<Link>) {
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
    let reader: Box<dyn Read> = match is_file_gzipped(&filename) {
        true => Box::new(GzDecoder::new(file)),
        _ => Box::new(file),
    };
    let reader = BufReader::new(reader);

    let mut segments = Vec::new();
    let mut links = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => panic!("There was a problem reading the file: {:?}", error),
        };
        if line.starts_with('S') || line.starts_with('L'){
            let mut parts = line.split('\t');
            let line_type = parts.next();
            if line_type == Some("S") {
                segments.push(line_parts_to_segment_name(parts, filename));
            } else if line_type == Some("L") {
                links.push(line_parts_to_link(parts, filename));
            }
        }
    }

    (segments, links)
}


/// Given the tab-delimited parts of a GFA segment line (minus the initial "S"), this function
/// returns the name as a string.
fn line_parts_to_segment_name(mut parts: std::str::Split<char>, filename: &PathBuf) -> String {
    let name = parts.next();
    if name.is_none() {
        quit_with_error(&format!("{:?} is not properly formatted", filename))
    }
    name.unwrap().to_string()
}


/// Given the tab-delimited parts of a GFA link line (minus the initial "L"), this function returns
/// a Link struct.
fn line_parts_to_link(mut parts: std::str::Split<char>, filename: &PathBuf) -> Link {
    let name_a = parts.next();
    let strand_a_str = parts.next();
    let name_b = parts.next();
    let strand_b_str = parts.next();
    if name_a.is_none() || strand_a_str.is_none() ||
            name_b.is_none() || strand_b_str.is_none() {
        quit_with_error(&format!("{:?} is not properly formatted", filename))
    }
    let strand_a = strand_str_to_int(strand_a_str.unwrap());
    let strand_b = strand_str_to_int(strand_b_str.unwrap());
    if strand_a == 0 || strand_b == 0 {
        quit_with_error(&format!("{:?} is not properly formatted", filename))
    }
    Link{name_a: name_a.unwrap().to_string(), strand_a,
         name_b: name_b.unwrap().to_string(), strand_b}
}


/// This function converts a strand in string form ("+" or "-") to integer form (1 or -1). Returns
/// 0 for other cases.
fn strand_str_to_int(strand_str: &str) -> i32 {
    if strand_str == "+" {
        return 1;
    }
    if strand_str == "-" {
        return -1;
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_gfa() {

        fn test_file(filename: &str) -> PathBuf {
            let mut gfa = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            gfa.push("tests");
            gfa.push(filename);
            return gfa
        }

        let (segments, links) = load_gfa(&test_file("circular.gfa"));
        assert_eq!(segments.len(), 1);
        assert_eq!(links.len(), 1);

        let (segments, links) = load_gfa(&test_file("circular.gfa.gz"));
        assert_eq!(segments.len(), 1);
        assert_eq!(links.len(), 1);

        let (segments, links) = load_gfa(&test_file("linear.gfa"));
        assert_eq!(segments.len(), 1);
        assert_eq!(links.len(), 0);

        let (segments, links) = load_gfa(&test_file("linear.gfa.gz"));
        assert_eq!(segments.len(), 1);
        assert_eq!(links.len(), 0);

        let (segments, links) = load_gfa(&test_file("empty.gfa"));
        assert_eq!(segments.len(), 0);
        assert_eq!(links.len(), 0);

        let (segments, links) = load_gfa(&test_file("seq.fasta"));
        assert_eq!(segments.len(), 0);
        assert_eq!(links.len(), 0);

        let (segments, links) = load_gfa(&test_file("seq.fastq"));
        assert_eq!(segments.len(), 0);
        assert_eq!(links.len(), 0);

        let (segments, links) = load_gfa(&test_file("circular_blank_lines.gfa"));
        assert_eq!(segments.len(), 1);
        assert_eq!(links.len(), 1);

        let (segments, links) = load_gfa(&test_file("medium.gfa.gz"));
        assert_eq!(segments.len(), 89);
        assert_eq!(links.len(), 122);

        let (segments, links) = load_gfa(&test_file("big.gfa.gz"));
        assert_eq!(segments.len(), 21816);
        assert_eq!(links.len(), 9616);
    }

    #[test]
    fn test_line_parts_to_segment_name() {
        let filename = PathBuf::new();

        let parts = "abc\tACGTACGACTAC".split('\t');
        assert_eq!(line_parts_to_segment_name(parts, &filename), "abc");

        let parts = "def\tACGTACGACTAC\tLN:i:12".split('\t');
        assert_eq!(line_parts_to_segment_name(parts, &filename), "def");
    }

    #[test]
    fn test_line_parts_to_link() {
        let filename = PathBuf::new();

        let parts = "a\t+\tb\t+\t0M".split('\t');
        assert_eq!(line_parts_to_link(parts, &filename),
                   Link{name_a: "a".to_string(), strand_a: 1,
                        name_b: "b".to_string(), strand_b: 1});

        let parts = "c\t-\td\t+\t0M".split('\t');
        assert_eq!(line_parts_to_link(parts, &filename),
                   Link{name_a: "c".to_string(), strand_a: -1,
                        name_b: "d".to_string(), strand_b: 1});

        let parts = "e\t+\tf\t-\t0M".split('\t');
        assert_eq!(line_parts_to_link(parts, &filename),
                   Link{name_a: "e".to_string(), strand_a: 1,
                        name_b: "f".to_string(), strand_b: -1});

        let parts = "g\t-\th\t-\t0M".split('\t');
        assert_eq!(line_parts_to_link(parts, &filename),
                   Link{name_a: "g".to_string(), strand_a: -1,
                        name_b: "h".to_string(), strand_b: -1});
    }

    #[test]
    fn test_strand_str_to_int() {
        assert_eq!(strand_str_to_int("+"), 1);
        assert_eq!(strand_str_to_int("-"), -1);
        assert_eq!(strand_str_to_int("a"), 0);
        assert_eq!(strand_str_to_int("b"), 0);
    }
}
