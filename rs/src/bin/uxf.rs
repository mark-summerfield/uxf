// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::{bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use flate2::{write::GzEncoder, Compression};
use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
};
use uxf::util::PathBufExt;

fn main() {
    let config = Config::parse();
    if let Err(err) = match &config.command {
        Commands::Format(format) => handle_format(format),
        Commands::Lint(lint) => handle_lint(lint),
        Commands::Compare(compare) => handle_compare(compare),
    } {
        eprintln!("{err:#}");
    }
}

fn handle_format(format: &Format) -> Result<()> {
    let inbuf = canonicalize_file(&format.infile)?;
    let infile = inbuf.to_string_lossy().to_string();
    let outfile = get_outfile(&inbuf, &format.outfile)?;
    let options = parser_options(format);
    let uxo = uxf::parse_options(
        &infile,
        options,
        if format.lint { None } else { Some(Rc::new(uxf::ignore_event)) },
    )?;
    if !outfile.is_empty() {
        output(&outfile, format, &uxo)?;
    }
    Ok(())
}

fn handle_lint(lint: &Lint) -> Result<()> {
    for file in &lint.files {
        if let Err(err) = handle_format(&Format::new_lint(file)) {
            eprintln!("{err:#}");
        }
    }
    Ok(())
}

fn handle_compare(compare: &Compare) -> Result<()> {
    let uxo1 = uxf::parse_options(
        &compare.file1.to_string_lossy(),
        if compare.equivalent {
            uxf::ParserOptions::AS_STANDALONE
        } else {
            uxf::ParserOptions::DEFAULT
        },
        Some(Rc::new(uxf::ignore_event)), // ignore lints
    )?;
    let uxo2 = uxf::parse_options(
        &compare.file2.to_string_lossy(),
        if compare.equivalent {
            uxf::ParserOptions::AS_STANDALONE
        } else {
            uxf::ParserOptions::DEFAULT
        },
        Some(Rc::new(uxf::ignore_event)), // ignore lints
    )?;
    let eq = if compare.equivalent {
        if uxo1.is_equivalent(&uxo2, uxf::Compare::EQUIVALENT) {
            "EQUIV"
        } else {
            "UNEQUIV"
        }
    } else if uxo1 == uxo2 {
        "EQUAL"
    } else {
        "UNEQUAL"
    };
    println!("{eq} {:?} {:?}", compare.file1, compare.file2);
    Ok(())
}

fn get_outfile(inbuf: &Path, outfile: &PathBuf) -> Result<String> {
    Ok(if !outfile.is_empty() {
        if outfile == &PathBuf::from("-") {
            "-".to_string()
        } else if outfile == &PathBuf::from("=") {
            inbuf.to_string_lossy().to_string()
        } else {
            check_same_file(inbuf, outfile)?;
            let outpath = canonicalize_file(outfile)?;
            outpath.to_string_lossy().to_string()
        }
    } else {
        "".to_string()
    })
}

fn output(outfile: &str, format: &Format, uxo: &uxf::Uxf) -> Result<()> {
    let text = if format.compact {
        uxo.to_string()
    } else {
        uxo.to_text_format(&uxf::Format::new(
            format.indent,
            format.wrapwidth,
            format.decimals,
        ))
    };
    if outfile == "-" {
        println!("{text}");
    } else {
        let raw = text.as_bytes();
        let mut file = File::create(outfile).with_context(|| {
            format!("E913:{outfile}:0:failed to create")
        })?;
        if outfile.ends_with(".gz") {
            let mut out = GzEncoder::new(&file, Compression::best());
            out.write_all(raw).with_context(|| {
                format!("E910:{outfile}:0:failed to write gzipped")
            })?;
            out.finish().with_context(|| {
                format!("E911:{outfile}:0:failed to gzip")
            })?;
        } else {
            file.write_all(raw).with_context(|| {
                format!("E912:{outfile}:0:failed to write")
            })?;
        }
    }
    Ok(())
}

fn parser_options(format: &Format) -> uxf::ParserOptions {
    let mut options = uxf::ParserOptions::DEFAULT;
    if format.standalone || format.dropunused {
        options |= uxf::ParserOptions::DROP_UNUSED_TTYPES;
    }
    if format.standalone || format.replaceimports {
        options |= uxf::ParserOptions::REPLACE_IMPORTS;
    }
    options
}

fn check_same_file(a: &Path, b: &Path) -> Result<()> {
    if b != PathBuf::from("-") {
        let a = canonicalize_file(a)?;
        let b = canonicalize_file(b)?;
        if a == b {
            bail!("E955:{}:0:won't overwrite: use = to force", a.display());
        }
    }
    Ok(())
}

fn canonicalize_file(p: &Path) -> Result<PathBuf> {
    let mut p =
        if let Ok(p) = p.canonicalize() { p } else { p.to_path_buf() };
    if p.is_relative() {
        let mut cwd = env::current_dir().with_context(|| {
            format!("E954:{}:0:failed to find folder for", p.display())
        })?;
        cwd.push(p);
        p = cwd;
    }
    Ok(p)
}

#[derive(Parser, Debug)]
#[clap(version, about = "Compares, Formats, and Lints UXF files.")]
struct Config {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compare two UXF files for equality ignoring insignificant
    /// whitespace, or for equivalence (with -e or --equivalent) in which
    /// case the comparison ignores insignificant whitespace, comments,
    /// unused ttypes, and, in effect replaces any imports with the ttypes
    /// they define—if they are used.
    /// If a diff is required, format the two UXF files using the same
    /// formatting options (and maybe use the -s --standalone option), then
    /// use a standard diff tool. (Use c or cmp or compare)
    #[clap(alias("c"))]
    #[clap(alias("cmp"))]
    Compare(Compare),

    /// Copy the infile to the outfile using the canonical human-readable
    /// format, or with the specified formatting options.
    /// This will alphabetically order any ttype definitions and will order
    /// map items by key (bytes < date < datetime < int < case-insensitive
    /// str). However, the order of imports is preserved (with any
    /// duplicates removed) to allow later imports to override earlier ones.
    /// The conversion will also automatically perform type repairs, e.g.,
    /// converting strings to dates or ints or reals if that is the target
    /// type, and similar. (Use f or fmt or format)
    #[clap(alias("f"))]
    #[clap(alias("fmt"))]
    Format(Format),

    /// Print the repairs that formatting would apply and lint warnings (if
    /// any) to stderr for the given file(s). (Use l or lnt or lint)
    #[clap(alias("l"))]
    #[clap(alias("lnt"))]
    Lint(Lint),
}

#[derive(Args, Debug)]
struct Compare {
    /// Compare for equivalance rather than for equality
    #[clap(short, long, action)]
    equivalent: bool,

    /// The first required file to compare (can have any suffix, i.e.,
    /// not just .uxf, and be gzip-compressed if it ends with .gz)
    #[clap(value_parser)]
    file1: PathBuf,

    /// The second required file to compare (ditto)
    #[clap(value_parser)]
    file2: PathBuf,
}

#[derive(Args, Debug)]
struct Format {
    /// Print lints to stderr. If only lints are wanted use l or lnt or
    /// lint.
    #[clap(short, long, action)]
    lint: bool,

    /// Same as -d|--dropunused and -r|--replaceimports together
    #[clap(short, long, action)]
    standalone: bool,

    /// Drop unused imports and ttype definitions (best to use
    /// -s|--standalone)
    #[clap(short, long, action)]
    dropunused: bool,

    /// Replace imports with ttype definitions for ttypes that are
    /// actually used to make the outfile standalone
    /// (best to use -s|--standalone)
    #[clap(short, long, action)]
    replaceimports: bool,

    /// Indent (0-8 spaces or 9 to use a tab; ignored if
    /// -c|--compact used)
    #[clap(short, long, default_value_t=2,
        value_parser=clap::value_parser!(u8).range(0..=9))]
    indent: u8,

    // The 40 is from (MAX_IDENTIFIER_LEN + 8)
    /// Wrapwidth (40-240; ignored if -c|--compact used)
    #[clap(short, long, default_value_t=96,
        value_parser=clap::value_parser!(u8).range(40..=240))]
    wrapwidth: u8,

    /// Decimal digits (0-15; 0 means use at least one (even if .0) and as
    /// many as needed; 1-15 means used that fixed number of digits)
    #[clap(short='D', long, default_value_t=0,
        value_parser=clap::value_parser!(u8).range(0..=15))]
    decimals: u8,

    /// Use compact output format (not human friendly; ignores indent
    /// and wrapwidth)
    #[clap(short, long, action)]
    compact: bool,

    /// Required infile
    #[clap(value_parser)]
    infile: PathBuf,

    /// Required outfile; use - to write to stdout or = to overwrite
    /// infile
    #[clap(value_parser)]
    outfile: PathBuf,
}

impl Format {
    fn new_lint(file: &Path) -> Self {
        Format {
            lint: true,
            standalone: false,
            dropunused: false,
            replaceimports: false,
            indent: 2,
            wrapwidth: 96,
            decimals: 0,
            compact: false,
            infile: file.to_path_buf(),
            outfile: PathBuf::new(),
        }
    }
}

#[derive(Args, Debug)]
struct Lint {
    /// The file(s) to lint.
    #[clap(value_parser, required = true)]
    files: Vec<PathBuf>,
}
