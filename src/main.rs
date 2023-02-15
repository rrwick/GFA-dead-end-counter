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

mod misc;
mod gfa;

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use clap::{Parser, crate_version, crate_description};
use num_format::{Locale, ToFormattedString};


#[derive(Parser)]
#[clap(name = "GFA dead-end counter",
       version = concat!("v", crate_version!()),
       about = crate_description!())]
struct Cli {
    gfa: PathBuf,
}


fn main() {
    let cli = Cli::parse();
    misc::check_if_file_exists(&cli.gfa);
    let (segments, links) = gfa::load_gfa(&cli.gfa);

    eprintln!("{:?}", segments);  // TEMP
    eprintln!("{:?}", links);  // TEMP
}
