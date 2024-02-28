//
// Copyright (c) 2024 Jeff Garzik
//
// This file is part of the posixutils-rs project covered under
// the MIT License.  For the full license text, please see the LICENSE
// file in the root directory of this project.
// SPDX-License-Identifier: MIT
//

extern crate clap;
extern crate plib;

use clap::Parser;
use gettextrs::{bind_textdomain_codeset, textdomain};
use plib::PROJECT_NAME;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    pathname: String,
}

fn show_dirname(args: &Args) {
    let mut pb = PathBuf::from(&args.pathname);
    pb.pop();

    println!("{}", pb.to_string_lossy());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse command line arguments
    let args = Args::parse();

    textdomain(PROJECT_NAME)?;
    bind_textdomain_codeset(PROJECT_NAME, "UTF-8")?;

    show_dirname(&args);

    Ok(())
}