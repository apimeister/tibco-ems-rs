# tibco_ems
[![Latest Version](https://img.shields.io/crates/v/tibco_ems.svg)](https://crates.io/crates/tibco_ems)

Rust bindings for the Tibco EMS C library.

A low-level binding in provided by the module tibco_ems::c_binding.
A high-level binding is not yet part of this package.

# License
tibco_ems is licensed under Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0).

TIBCO Enterprise Messaging Service, and all related components therein are property of TIBCO Software, and are not provided with this software package. Refer to your own TIBCO License terms for details.

# Build

To build this crate, the TIBCO EMS C library must either be in the LD_LIBRARY_PATH or alternatively a EMS_HOME environment variable must be set.