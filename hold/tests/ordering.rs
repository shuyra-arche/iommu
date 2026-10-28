//! Provider-owned safety cases for [`iommu_hold::Holds`].
//!
//! The six case bodies and the three support fixtures are the existing Source
//! `programs/tests/retention/{ordering,accessor,platform,references}.rs` files
//! copied byte-for-byte. This root only supplies the test-crate paths that the
//! Source `retention` test library used to supply.

#![allow(dead_code)]

extern crate alloc;

use iommu_hold as retention;

#[path = "support/accessor.rs"]
mod accessor;

#[path = "support/platform.rs"]
mod platform;

#[path = "support/references.rs"]
mod references;

#[path = "cases/ordering.rs"]
mod ordering;
