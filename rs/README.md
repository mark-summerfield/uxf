# Rust UXF Library

UXF is a plain text human readable optionally typed storage format. UXF may
serve as a convenient alternative to csv, ini, json, sqlite, toml, xml, or
yaml.

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
