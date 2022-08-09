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

- [Introduction](#introduction)
- [Rust UXF Types](#python-uxf-types)
- [API](#api)
    - [Reading and Writing UXF Data](#reading-and-writing-uxf-data)
    - [Command Line Usage](#command-line-usage)
- [Changes](#changes)

## Introduction

**This is a WIP which may or may not succeed! Not _yet_ usable!**


## Rust UXF Types

## API

The simplest part of the API loads and saves (dumps) UXF data from/to
strings or files.

See the rust docs for full API details.

### Reading and Writing UXF Data

### Uxf Type

### Value Type

### List Type

### Map Type

### Table Type

### TClass Type

### Field Type

Provides a definition of a field (`name` and `vtype`) for use in
``TClass``es.

### Command Line Usage

## Changes

- 0.2.0 Now use `Value::Null` rather than `Option<Value>` since this better
  represents UXF data.
- 0.1.0 Started.

---
