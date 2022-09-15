// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::Result;
use clap::{AppSettings, Parser};
use std::{path::PathBuf, rc::Rc};

fn main() {
    if let Err(err) = real_main() {
        eprintln!("{}", err);
    }
}

fn real_main() -> Result<()> {
    let config = Config::parse();
    let uxo1 = uxf::parse_options(
        &config.file1.to_string_lossy(),
        if config.equivalent {
            uxf::ParserOptions::AS_STANDALONE
        } else {
            uxf::ParserOptions::DEFAULT
        },
        Some(Rc::new(|_event| {})), // ignore lints
    )?;
    let uxo2 = uxf::parse_options(
        &config.file2.to_string_lossy(),
        if config.equivalent {
            uxf::ParserOptions::AS_STANDALONE
        } else {
            uxf::ParserOptions::DEFAULT
        },
        Some(Rc::new(|_event| {})), // ignore lints
    )?;
    let eq = if config.equivalent {
        if uxo1.is_equivalent(&uxo2, uxf::Compare::EQUIVALENT) {
            "Equivalent"
        } else {
            "Unequivalent"
        }
    } else {
        if uxo1 == uxo2 {
            "Equal"
        } else {
            "Unequal"
        }
    };
    println!("{}: {:?} {:?}", eq, config.file1, config.file2);
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(
    name = "uxfcmp",
    version,
    about = "Compares two UXF files for equality or equivalence.

Equality comparisons ignore insignificant whitespace.

Equivalence comparisons ignore insignificant whitespace, comments, \
unused ttypes, and, in effect replaces any imports with the ttypes they \
define—if they are used."
)]
struct Config {
    /// Compare for equivalance rather than for strict equality
    #[clap(short, long, action)]
    equivalent: bool,

    /// Required UXF file1 to compare (can have any suffix, i.e., not just
    /// .uxf, and be gzip-compressed if it ends with .gz)
    #[clap(value_parser)]
    file1: PathBuf,

    /// Required UXF file2 to compare (can have any suffix, i.e., not just
    /// .uxf, and be gzip-compressed if it ends with .gz)
    #[clap(value_parser)]
    file2: PathBuf,
}
