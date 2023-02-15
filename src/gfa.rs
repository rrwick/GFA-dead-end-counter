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

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};


#[derive(Debug)]
pub struct Link {
    name_a: String,
    strand_a: i32,
    name_b: String,
    strand_b: i32,
}


/// This function loads a GFA file and runs a few checks on the result. If everything looks good,
/// it returns a vector of segment names and a vector of links
pub fn load_gfa(filename: &PathBuf) -> (Vec<String>, Vec<Link>) {
    let load_result = if is_file_gzipped(&filename) {
        load_gfa_gzipped(&filename)
    } else {
        load_gfa_not_gzipped(&filename)
    };
    match load_result {
        Ok(_)  => (),
        Err(_) => quit_with_error(&format!("unable to load {:?}", filename)),
    }
    let (segments, links) = load_result.unwrap();
    // check_load_gfa(&fasta_seqs, &filename);
    (segments, links)
}


// /// This function looks at the result of the load_gfa function and does some checks to make sure
// /// everything looks okay. If any problems are found, it will quit with an error message.
// fn check_load_gfa(fasta_seqs: &Vec<(String, String)>, filename: &PathBuf) {
//     if fasta_seqs.len() == 0 {
//         quit_with_error(&format!("{:?} contains no sequences", filename));
//     }
//     for (name, sequence) in fasta_seqs {
//         if name.len() == 0 {
//             quit_with_error(&format!("{:?} has an unnamed sequence", filename));
//         }
//         if sequence.len() == 0 {
//             quit_with_error(&format!("{:?} has an empty sequence", filename));
//         }
//     }
//     let mut set = HashSet::new();
//     for (name, _) in fasta_seqs {
//         set.insert(name);
//     }
//     if set.len() < fasta_seqs.len() {
//         quit_with_error(&format!("{:?} has a duplicated name", filename));
//     }
// }


fn load_gfa_not_gzipped(filename: &PathBuf) -> io::Result<(Vec<String>, Vec<Link>)> {
    let mut segments = Vec::new();
    let mut links = Vec::new();
    let file = File::open(&filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let text = line?;
        if text.starts_with('S') || text.starts_with('L'){
            let mut parts = text.split('\t');
            let line_type = parts.next();
            if line_type == Some("S") {
                let name = parts.next();
                match name {
                    Some(n) => {segments.push(n.to_string());}
                    None => {quit_with_error(&format!("{:?} is not properly formatted", filename))}
                }
            }
            if line_type == Some("L") {
                let name_a = parts.next();
                let strand_a_str = parts.next();
                let name_b = parts.next();
                let strand_b_str = parts.next();
                if name_a.is_none() || strand_a_str.is_none() ||
                        name_b.is_none() || strand_b_str.is_none() {
                    quit_with_error(&format!("{:?} is not properly formatted", filename))
                }
                else {
                    let strand_a = strand_str_to_int(strand_a_str.unwrap());
                    let strand_b = strand_str_to_int(strand_b_str.unwrap());
                    if strand_a == 0 || strand_b == 0 {
                        quit_with_error(&format!("{:?} is not properly formatted", filename))
                    }
                    links.push(Link{name_a: name_a.unwrap().to_string(), strand_a,
                                    name_b: name_b.unwrap().to_string(), strand_b});
                }
            }
        }
    }
    Ok((segments, links))
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


fn load_gfa_gzipped(filename: &PathBuf) -> io::Result<(Vec<String>, Vec<Link>)> {
    let mut segments = Vec::new();
    let mut links = Vec::new();
    let file = File::open(&filename)?;
    let reader = BufReader::new(GzDecoder::new(file));
    for line in reader.lines() {
        // let text = line?;
        // if text.len() == 0 {continue;}
        // if text.starts_with('>') {
        //     if name.len() > 0 {
        //         sequence.make_ascii_uppercase();
        //         fasta_seqs.push((name, sequence));
        //         sequence = String::new();
        //     }
        //     let first_piece = text[1..].split_whitespace().next();
        //     match first_piece {
        //         Some(_) => (),
        //         None    => quit_with_error(&format!("{:?} is not correctly formatted", filename)),
        //     }
        //     name = first_piece.unwrap().to_string();
        // } else {
        //     if name.len() == 0 {
        //         quit_with_error(&format!("{:?} is not correctly formatted", filename));
        //     }
        //     sequence.push_str(&text);
        // }
    }
    Ok((segments, links))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strand_str_to_int() {
        assert_eq!(strand_str_to_int("+"), 1);
        assert_eq!(strand_str_to_int("-"), -1);
        assert_eq!(strand_str_to_int("a"), 0);
        assert_eq!(strand_str_to_int("b"), 0);
    }
}
