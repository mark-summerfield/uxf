// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::Result;
use clap::{AppSettings, Parser};
use std::path::PathBuf;

fn main() -> Result<()> {
    let config = Config::parse();
    println!("lint           {:?}", config.lint);
    println!("dropunused     {:?}", config.dropunused);
    println!("replaceimports {:?}", config.replaceimports);
    println!("standalone     {:?}", config.standalone);
    println!("compact        {:?}", config.compact);
    println!("indent         {:?}", config.indent);
    println!("wrapwidth      {:?}", config.wrapwidth);
    println!("infile         {:?}", config.infile);
    println!("outfile        {:?}", config.outfile);

    // TODO
    let uxo = uxf::parse("uxf 1.0\n#<A Test>\n[]")?;
    println!("uxf cli:\n\n{}", uxo);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(
    version,
    about = "Provides linting and uxf to uxf conversion \
(to produce standardized human-friendly formatting or compact formatting).

Converting uxf to uxf will alphabetically order any ttype definitions \
and will order map items by key (bytes < date < datetime < int < \
case-insensitive str). However, the order of imports is preserved (with \
any duplicates removed) to allow later imports to override earlier ones."
)]
struct Config {
    /// Print lint warnings to stderr
    #[clap(short, long, action)]
    lint: bool,

    /// Same as -d|--dropunused and -r|--replaceimports together
    #[clap(short, long, action)]
    standalone: bool,

    /// Drop unused imports and ttype definitions (best to use
    /// -s|--standalone)
    #[clap(short, long, action)]
    dropunused: bool,

    /// Replace imports with ttype definitions for ttypes that are actually
    /// used to make the outfile standalone (best to use -s|--standalone)
    #[clap(short, long, action)]
    replaceimports: bool,

    /// Indent (0-8 spaces or 9 to use a tab; ignored if -c|--compact used)
    #[clap(short, long, default_value_t=2,
           value_parser=clap::value_parser!(u8).range(0..=9))]
    indent: u8,

    /// Wrapwidth (40-240; ignored if -c|--compact used)
    #[clap(short, long, default_value_t=96,
           value_parser=clap::value_parser!(u8).range(40..=240))]
    wrapwidth: u8,

    /// Use compact output format (not human friendly; ignores indent and
    /// wrapwidth)
    #[clap(short, long, action)]
    compact: bool,

    /// Required UXF infile (can have any suffix, i.e., not just .uxf, and
    /// be gzip-compressed if it ends with .gz)
    #[clap(value_parser)]
    infile: PathBuf,

    /// Optional UXF outfile; use - to write to stdout; not needed purely
    /// for linting; gzip-compressed if outfile ends .gz')
    #[clap(value_parser)]
    outfile: Option<PathBuf>,
}
