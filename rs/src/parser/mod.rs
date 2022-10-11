// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

/// The UXF parser; only the parse() and parse_options() are public.
mod lexer;
mod parse;
mod token;

pub(crate) use crate::parser::parse::parse;
