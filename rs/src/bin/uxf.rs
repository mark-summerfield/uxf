// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use anyhow::Result;

fn main() -> Result<()> {
    let uxo = uxf::parse("uxf 1.0\n#<A Test>\n[]")?;
    println!("uxf cli:\n\n{}", uxo);
    Ok(())
}
