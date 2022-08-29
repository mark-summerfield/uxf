// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::{bail, Result};
use clap::{AppSettings, Parser};
use std::{
    env,
    path::{Path, PathBuf},
    rc::Rc,
};

fn main() -> Result<()> {
    let config = Config::parse();
    let inbuf = canonicalize_file(&config.infile)?;
    let infile = inbuf.to_string_lossy().to_string();
    let mut outfile = String::new();
    if let Some(outbuf) = &config.outfile {
        check_same_file(&inbuf, outbuf)?;
        let outpath = canonicalize_file(outbuf)?;
        outfile = outpath.to_string_lossy().to_string();
    }
    let options = parse_options(&config);
    let uxo = uxf::parse_options(
        &infile,
        options,
        Some(Rc::new(if config.lint {
            uxf::on_event
        } else {
            move |_event| {}
        })),
    )?;
    if !outfile.is_empty() {
        output(&outfile, &config, &uxo)?;
    }
    Ok(())
}

fn output(outfile: &str, config: &Config, uxo: &uxf::Uxf) -> Result<()> {
    let text = if config.compact {
        uxo.to_string()
    } else {
        uxo.to_string_options(
            &uxf::Format::new(config.indent, config.wrapwidth, None),
            Some(Rc::new(if config.lint {
                uxf::on_event
            } else {
                move |_event| {} // ignore lints
            })),
        )?
    };
    if outfile == "-" {
        println!("{}", text);
    } else {
        // TODO write text to (possibly gzipped) file
    }
    Ok(())
}

fn parse_options(config: &Config) -> uxf::ParseOptions {
    let mut options = uxf::ParseOptions::AS_IS;
    if config.standalone || config.dropunused {
        options |= uxf::ParseOptions::DROP_UNUSED_TTYPES;
    }
    if config.standalone || config.replaceimports {
        options |= uxf::ParseOptions::REPLACE_IMPORTS;
    }
    options
}

fn check_same_file(a: &Path, b: &Path) -> Result<()> {
    if b != PathBuf::from("-") {
        let a = canonicalize_file(a)?;
        let b = canonicalize_file(b)?;
        if a == b {
            bail!("won't overwrite {}", a.display());
        }
    }
    Ok(())
}

fn canonicalize_file(p: &Path) -> Result<PathBuf> {
    let mut p =
        if let Ok(p) = p.canonicalize() { p } else { p.to_path_buf() };
    if p.is_relative() {
        let mut cwd = env::current_dir()?;
        cwd.push(p);
        p = cwd;
    }
    Ok(p)
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
