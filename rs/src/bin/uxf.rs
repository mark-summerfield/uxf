// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
}

fn main() -> Result<()> {
    let args = Args::parse();
    let uxo = uxf::parse("uxf 1.0\n#<A Test>\n[]")?;
    println!("uxf cli:\n\n{}", uxo);
    Ok(())
}
