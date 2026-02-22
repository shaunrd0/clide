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
//! The target argument can be overridden using one of the following macros.
//! ```
//! libclide::log!(target: "CustomTarget", "This log message will have the target 'CustomTarget'");
//! ```
//!
//! The target argument will default to Self::ID if not provided.
//! This is an error if Self::ID is not defined, forcing you to use the explicit form.
//! ```
//! libclide::log!("This log message will use target Self::ID, the name of the struct it was invoked in");
//! ```
//!
//! Self::ID can be defined using the `#[log_id]` attribute macro, which will automatically generate
//! a constant ID field with the name of the struct as its value.
//! ```
//! #[log_id]
//! struct MyStruct;
//! impl MyStruct {
//!      fn my_method(&self) {
//!          libclide::log!("This log message will use target Self::ID, which is 'MyStruct'");
//!      }
//!  }
//! ```
//!

#[macro_export]
macro_rules! info {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::info!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::info!(target: $target, $($arg)+)
    });

    (logger: $logger:expr, $($arg:tt)+) => ({
        log::info!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::info!(target: Self::ID, $($arg)+))
}

#[macro_export]
macro_rules! debug {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::debug!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::debug!(target: $target, $($arg)+)
    });

    (logger: $logger:expr, $($arg:tt)+) => ({
        log::debug!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::debug!(target: Self::ID, $($arg)+))
}

#[macro_export]
macro_rules! warn {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::warn!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::warn!(target: $target, $($arg)+)
    });

    (logger: $logger:expr, $($arg:tt)+) => ({
        log::warn!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::warn!(target: Self::ID, $($arg)+))
}

#[macro_export]
macro_rules! error {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::error!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::error!(target: $target, $($arg)+)
    });

    (logger: $logger:expr, $($arg:tt)+) => ({
        log::error!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::error!(target: Self::ID, $($arg)+))
}

#[macro_export]
macro_rules! trace {
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::trace!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::trace!(target: $target, $($arg)+)
    });

    (logger: $logger:expr, $($arg:tt)+) => ({
        log::trace!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::trace!(target: Self::ID, $($arg)+))
}
