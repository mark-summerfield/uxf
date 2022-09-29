// Copyright © 2022 Mark Summerfield. All rights reserved.
// License: GPLv3

use crate::check::{check_name, check_vtype};
use anyhow::{bail, Result};
use std::{cmp::Ordering, collections::HashSet, fmt};

/// Returns a vector of fields which when unwrapped is suitable for
/// TClass::new(). Use an empty string for vtypes that should be None.
///
/// ```
/// let fields = uxf::field::make_fields(&[("Data", ""), ("Date", "date"),
///         ("Level", "real"), ("name", "str")]).unwrap();
/// assert_eq!(fields.len(), 4);
/// assert_eq!(format!("{}", fields[0]), "Data");
/// assert_eq!(format!("{}", fields[1]), "Date:date");
/// assert_eq!(format!("{}", fields[2]), "Level:real");
/// assert_eq!(format!("{}", fields[3]), "name:str");
/// ```
pub fn make_fields(
    name_vtype_pairs: &[(&str, &str)],
) -> Result<Vec<Field>> {
    make_fields_x(name_vtype_pairs, "", 0)
}

/// Returns a vector of fields which when unwrapped is suitable for
/// TClass::new(). Use an empty string for vtypes that should be None.
///
/// ```
/// let fields = uxf::field::make_fields(&[("Data", ""), ("Date", "date"),
///         ("Level", "real"), ("name", "str")]).unwrap();
/// assert_eq!(fields.len(), 4);
/// assert_eq!(format!("{}", fields[0]), "Data");
/// assert_eq!(format!("{}", fields[1]), "Date:date");
/// assert_eq!(format!("{}", fields[2]), "Level:real");
/// assert_eq!(format!("{}", fields[3]), "name:str");
/// ```
pub fn make_fields_x(
    name_vtype_pairs: &[(&str, &str)],
    filename: &str,
    lino: usize,
) -> Result<Vec<Field>> {
    let mut fields = vec![];
    for (name, vtype) in name_vtype_pairs {
        fields.push(Field::new(name, vtype)?);
    }
    check_fields_x(&fields, filename, lino)?;
    Ok(fields)
}

/// Returns `Ok(())` if there are no duplicate fields; otherwise `Err`.
pub(crate) fn check_fields(fields: &Vec<Field>) -> Result<()> {
    check_fields_x(fields, "-", 0)
}

/// Returns `Ok(())` if there are no duplicate fields; otherwise `Err`.
pub(crate) fn check_fields_x(
    fields: &Vec<Field>,
    filename: &str,
    lino: usize,
) -> Result<()> {
    let mut seen = HashSet::<&str>::new();
    for field in fields {
        let name = field.name();
        if seen.contains(&name) {
            bail!(
                "E336:{}:{}:can't have duplicate table tclass \
                field names, got {:?} twice",
                filename,
                lino,
                &name
            )
        } else {
            seen.insert(name);
        }
    }
    Ok(())
}

/// Provides a definition of a field (`name` and `vtype`) for use in
/// ``TClass``es.
///
/// ``Field``s are immutable.
#[derive(Clone, Debug, Eq)]
pub struct Field {
    name: String,
    vtype: Option<String>,
}

impl Field {
    /// Creates a new `Field` with the given `name` and `vtype` _or_
    /// returns an Err if either or both is or are invalid.
    /// A `vtype` of "" signifies `None`, i.e., that this field will accept
    /// an vtype
    pub fn new(name: &str, vtype: &str) -> Result<Self> {
        check_name(name)?;
        let vtype = if vtype.is_empty() {
            None
        } else {
            check_vtype(vtype)?;
            Some(vtype.to_string())
        };
        Ok(Field { name: name.to_string(), vtype })
    }

    /// Return's the ``Field``'s `name`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return's the ``Field``'s `vtype` (which may be `None`).
    pub fn vtype(&self) -> Option<&str> {
        match &self.vtype {
            None => None,
            Some(vtype) => Some(vtype),
        }
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        let aname = self.name.to_uppercase();
        let bname = other.name.to_uppercase();
        if aname != bname {
            // prefer case-insensitive ordering
            aname.cmp(&bname)
        } else if self.name != other.name {
            self.name.cmp(&other.name)
        } else {
            // identical names names so use vtype to tie-break
            self.vtype.cmp(&other.vtype)
        }
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.vtype == other.vtype
    }
}

impl fmt::Display for Field {
    /// Provides a .to_string() that returns a valid UXF fragment
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.vtype {
            Some(vtype) => {
                write!(f, "{}:{}", self.name, vtype)
            }
            None => write!(f, "{}", self.name),
        }
    }
}
