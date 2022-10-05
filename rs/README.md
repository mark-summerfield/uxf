# Rust UXF Library

Uniform eXchange Format (UXF) is a plain text human readable optionally
typed storage format. UXF is designed to make life easier for software
developers and data designers. It directly competes with csv, ini, json,
toml, and yaml formats. One key advantage of UXF is that it supports custom
(i.e., user-defined) types. This can result in more compact, more readable,
and easier to parse data. And in some contexts it may prove to be a
convenient alternative to sqlite or xml.

Every `Uxf` object contains one `value` of type ``Value``: this is always a
collection (a ``List``, ``Map``, or ``Table``), which may be empty or
contain any number of other values—any mix of scalars and collections.

For details of the Uniform eXchange Format (UXF) supported by this library,
see the [UXF Overview](../README.md). ([Alternative link to UXF
Overview](https://github.com/mark-summerfield/uxf/blob/main/README.md).)

[crates.io](https://crates.io/crates/uxf) •
[docs](https://docs.rs/uxf/latest/uxf/)

## Commmand Line tool

- _uxf_ this can read UXF files (optionally gzip compressed) and lint and
  output UXF files (optionally gzip compressed; optionally replacing imports
  and dropping unused ttypes). It can also compare two UXF files for
  equality or equivalence. (For a full diff, format the two files with the
  same format options, and use a standard diff tool.)

## Changes

- 1.0.0 Added various convenience methods. For a real-world example, see
  [TLM (Track List Manager)](https://crates.io/crates/tlm) a cross-platform
  GUI application for managing playlists and playing tracks.
- 0.26.0 Added `Uxf::value_mut()`.
- 0.25.0 Added `Uxf::add_tclass()`.
- 0.24.0 Now correctly writes gzipped to .gz files (before it wrote nothing
  to these).
- 0.23.0 Bug fixes. Improved error messages & testing.
- 0.22.0 Bug fixes. Improved error messages & testing.
- 0.21.0 Improved error messages & testing.
- 0.20.0 _uxf_ now does linting and comparing; pretty printing complete.
- 0.16.0 More progress towards pretty printing: but at the moment only
  compact output works (along with parsing of course).
- 0.15.0 Changed uxf CLI to use subcommands: can now do compare, format, and
  lint (so dropped redundant uxfcmp).
- 0.14.0 Implemented uxfcmp.
- 0.13.0 Cleanups.
- 0.12.0 Parser does everything including imports.
- 0.11.0 Parser does everything except imports; can output unfriendly
  output.
- 0.2.0 Now use `Value::Null` rather than `Option<Value>` since this better
  represents UXF data.
- 0.1.0 Started.

---
