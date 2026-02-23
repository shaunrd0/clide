// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

pub mod macros;

pub use libclide_macros::Loggable;
pub trait Loggable {
    const ID: &'static str;
}
