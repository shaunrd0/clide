// SPDX-FileCopyrightText: 2026, Shaun Reed <shaunrd0@gmail.com>
//
// SPDX-License-Identifier: GNU General Public License v3.0 or later

//! Logging targets allow filtering of log messages by their source. By default, the log crate sets
//! the target to the module path where the log macro was invoked if no target is provided.
//!
//! These macros essentially disable using the default target and instead require the target to be
//! explicitly set. This is to avoid implicit pooling of log messages under the same default target,
//! which can make it difficult to filter log messages by their source.
//!
//! The Loggable trait can be implemented to automatically associate log messages with a struct.
//! ```
//! use libclide_macros::Loggable;
//!
//! #[derive(Loggable)]
//! struct MyStruct;
//! impl MyStruct {
//!      fn my_method(&self) {
//!          libclide::info!("This log message will use target <Self as Loggable>::ID, which is 'MyStruct'");
//!      }
//!  }
//! ```
//!
//! If the struct does not derive or implement Loggable, the target variant of the log macros must
//! be used instead.
//! ```
//! libclide::info!(target: "CustomTarget", "This log message will have the target 'CustomTarget'");
//! ```
//!

#[macro_export]
macro_rules! info {
    (target: $target:expr, $($arg:tt)+) => ({
        log::info!(target: $target, $($arg)+)
    });

    ($($arg:tt)+) => (log::info!(target: <Self as libclide::log::Loggable>::ID, $($arg)+))
}

#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => ({
        log::debug!(target: $target, $($arg)+)
    });

    ($($arg:tt)+) => (log::debug!(target: <Self as libclide::log::Loggable>::ID, $($arg)+))
}

#[macro_export]
macro_rules! warn {
    (target: $target:expr, $($arg:tt)+) => ({
        log::warn!(target: $target, $($arg)+)
    });

    ($($arg:tt)+) => (log::warn!(target: <Self as libclide::log::Loggable>::ID, $($arg)+))
}

#[macro_export]
macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => ({
        log::error!(target: $target, $($arg)+)
    });

    ($($arg:tt)+) => (log::error!(target: <Self as libclide::log::Loggable>::ID, $($arg)+))
}

#[macro_export]
macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => ({
        log::trace!(target: $target, $($arg)+)
    });

    ($($arg:tt)+) => (log::trace!(target: <Self as libclide::log::Loggable>::ID, $($arg)+))
}
