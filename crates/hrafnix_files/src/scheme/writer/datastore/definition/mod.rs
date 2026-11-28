//! Definition writers, matching the corresponding reader schemas.
//!
//! Ordered items and columns retain their datastore iteration order. Nullable
//! defaults are always written, including `null` and explicitly empty collections.
//! Unit identifiers are a frozen snapshot independent of runtime catalog names.

/// Boolean expressions and state labels.
pub(crate) mod boolean;
/// Choice expressions and ordered choices.
pub(crate) mod choice;
/// Shared constraint, table, and unit representations.
pub(crate) mod common;
/// File expressions and dialog options.
pub(crate) mod file;
/// Folder expressions and archive options.
pub(crate) mod folder;
/// Integer expressions and bounds.
pub(crate) mod integer;
/// Tagged item definitions shared by parameter and variable arrays.
pub(crate) mod item;
/// Ordered map schemas and default entries.
pub(crate) mod map;
/// Numeric expressions and bounds.
pub(crate) mod number;
/// Numeric expressions with preferred units.
pub(crate) mod number_with_units;
/// Separator labels.
pub(crate) mod separator;
/// String expressions.
pub(crate) mod string;
/// Tab labels.
pub(crate) mod tab;
/// Ordered numeric tables.
pub(crate) mod table;
/// Ordered numeric tables with units.
pub(crate) mod table_with_units;
/// Unit selection expressions and families.
pub(crate) mod unit;
