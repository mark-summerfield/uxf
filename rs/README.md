# Rust UXF Library

Uniform eXchange Format (UXF) is a plain text human readable optionally
typed storage format. UXF is designed to make life easier for software
developers and data designers. It directly competes with csv, ini, json,
toml, and yaml formats. One key advantage of UXF is that it supports custom
(i.e., user-defined) types. This can result in more compact, more readable,
and easier to parse data. And in some contexts it may prove to be a
convenient alternative to sqlite or xml.

For details of the Uniform eXchange Format (UXF) supported by this library,
see the [UXF Overview](../README.md). ([Alternative link to UXF
Overview](https://github.com/mark-summerfield/uxf/blob/main/README.md).)

**Note that this is WIP — currently the parser is complete (except for
imports), along with human __un__friendly output.**

[crates.io](https://crates.io/crates/uxf)
[docs](https://docs.rs/uxf/latest/uxf/)

## Commmand Line tools

- _uxf_ this can read UXF files (optionally gzip compressed) and lint and
  output UXF files (optionally gzip compressed; optionally replacing imports
  and dropping unused ttypes).
- _uxfcmp_ this compares two UXF files (optionally gzip compressed) for
  equality or equivalence.
- _uxflint_ this lints any number of UXF files. 

## Changes

- 0.11.0 Parser does everything except imports; can output __un__friendly
  output.
- 0.2.0 Now use `Value::Null` rather than `Option<Value>` since this better
  represents UXF data.
- 0.1.0 Started.

---
