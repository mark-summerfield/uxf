// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod lexer;
mod parse;
mod token;

pub(crate) use crate::parser::parse::parse;
