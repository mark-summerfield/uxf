// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args = Args::parse();
    println!("lint           {:?}", args.lint);
    println!("dropunused     {:?}", args.dropunused);
    println!("replaceimports {:?}", args.replaceimports);
    println!("standalone     {:?}", args.standalone);
    println!("compact        {:?}", args.compact);
    println!("indent         {:?}", args.indent);
    println!("wrapwidth      {:?}", args.wrapwidth);
    println!("infile         {:?}", args.infile);
    println!("outfile        {:?}", args.outfile);
    let uxo = uxf::parse("uxf 1.0\n#<A Test>\n[]")?;
    println!("uxf cli:\n\n{}", uxo);
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(version = "0.9.0 (UXF 1.0)")]
#[clap(about = "Provides linting and uxf to uxf conversion (to produce \
standardized human-friendly formatting or compact formatting).

Converting uxf to uxf will alphabetically order any ttype definitions \
and will order map items by key (bytes < date < datetime < int < \
case-insensitive str). However, the order of imports is preserved (with \
any duplicates removed) to allow later imports to override earlier ones.")]
struct Args {
    /// Print lint warnings to stderr
    #[clap(short, long, action)]
    lint: bool,

    /// Drop unused imports and ttype definitions (best to use
    /// -s|--standalone)
    #[clap(short, long, action)]
    dropunused: bool,

    /// Replace imports with ttype definitions for ttypes that are actually
    /// used to make the outfile standalone (best to use -s|--standalone)
    #[clap(short, long, action)]
    replaceimports: bool,

    /// Same as -d|--dropunused and -r|--replaceimports together
    #[clap(short, long, action)]
    standalone: bool,

    /// Use compact output format (not human friendly; ignores indent and
    /// wrapwidth)
    #[clap(short, long, action)]
    compact: bool,

    /// Indent (0-8 spaces or 9 to use a tab; ignored if -c|--compact used)
    #[clap(short, long, default_value_t=2,
           value_parser=clap::value_parser!(u8).range(0..=9))]
    indent: u8,

    /// Wrapwidth (40-240; ignored if -c|--compact used)
    #[clap(short, long, default_value_t=96,
           value_parser=clap::value_parser!(u8).range(40..=240))]
    wrapwidth: u8,

    /// Required UXF infile (can have any suffix, i.e., not just .uxf, and
    /// be gzip-compressed if it ends with .gz)
    #[clap(value_parser)]
    infile: PathBuf,

    /// Optional UXF outfile; use - to write to stdout; not needed purely
    /// for linting; gzip-compressed if outfile ends .gz')
    #[clap(value_parser)]
    outfile: Option<PathBuf>,
}
