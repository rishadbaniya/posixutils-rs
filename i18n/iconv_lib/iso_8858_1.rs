//
// Copyright (c) 2024 Jeff Garzik
//
// This file is part of the posixutils-rs project covered under
// the MIT License.  For the full license text, please see the LICENSE
// file in the root directory of this project.
// SPDX-License-Identifier: MIT
//

use std::process::exit;

/// From ISO-8858-1 to UCS-4
pub fn to_ucs4<I: Iterator<Item = u8> + 'static>(
    input: I,
    _omit_invalid: bool,
    _suppress_error: bool,
) -> Box<dyn Iterator<Item = u32>> {
    Box::new(input.map(|code_point| code_point as u32))
}

/// From UCS-4 to ISO-8858-1
pub fn from_ucs4<I: Iterator<Item = u32> + 'static>(
    input: I,
    omit_invalid: bool,
    suppress_error: bool,
) -> Box<dyn Iterator<Item = u8>> {
    Box::new(input.enumerate().filter_map(move |(position, code_point)| {
        if code_point <= 255 {
            Some(code_point as u8)
        } else if omit_invalid {
            None
        } else {
            if !suppress_error {
                eprintln!("Error: Invalid input at position {}", position);
                exit(1)
            }
            None
        }
    }))
}
