// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

mod lexer;
mod parser;
mod token;

pub(crate) use crate::parser::parser::parse;
