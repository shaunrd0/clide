//! Logging targets allow filtering of log messages by their source. By default, the log crate sets
//! the target to the module path where the log macro was invoked if no target is provided.
//!
//! These macros essentially disable using the default target and instead require the target to be
//! explicitly set. This is to avoid implicit pooling of log messages under the same default target,
//! which can make it difficult to filter log messages by their source.

#[macro_export]
macro_rules! info {
    // The target argument can be overridden using one of the following macros.
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::info!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::info!(target: $target, $($arg)+)
    });

    // The target argument will default to Self::ID if not provided.
    // Obviously, this is an error if Self::ID is not defined, forcing you to use the explicit form.
    (logger: $logger:expr, $($arg:tt)+) => ({
        log::info!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::info!(target: Self::ID, $($arg)+))
}

#[macro_export]
macro_rules! debug {
    // The target argument can be overridden using one of the following macros.
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::debug!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::debug!(target: $target, $($arg)+)
    });

    // The target argument will default to Self::ID if not provided.
    // Obviously, this is an error if Self::ID is not defined, forcing you to use the explicit form.
    (logger: $logger:expr, $($arg:tt)+) => ({
        log::debug!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::debug!(target: Self::ID, $($arg)+))
}

#[macro_export]
macro_rules! warn {
    // The target argument can be overridden using one of the following macros.
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::warn!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::warn!(target: $target, $($arg)+)
    });

    // The target argument will default to Self::ID if not provided.
    // Obviously, this is an error if Self::ID is not defined, forcing you to use the explicit form.
    (logger: $logger:expr, $($arg:tt)+) => ({
        log::warn!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::warn!(target: Self::ID, $($arg)+))
}

#[macro_export]
macro_rules! error {
    // The target argument can be overridden using one of the following macros.
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::error!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::error!(target: $target, $($arg)+)
    });

    // The target argument will default to Self::ID if not provided.
    // Obviously, this is an error if Self::ID is not defined, forcing you to use the explicit form.
    (logger: $logger:expr, $($arg:tt)+) => ({
        log::error!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::error!(target: Self::ID, $($arg)+))
}

#[macro_export]
macro_rules! trace {
    // The target argument can be overridden using one of the following macros.
    (logger: $logger:expr, target: $target:expr, $($arg:tt)+) => ({
        log::trace!(logger: $logger, target: $target, $($arg)+)
    });

    (target: $target:expr, $($arg:tt)+) => ({
        log::trace!(target: $target, $($arg)+)
    });

    // The target argument will default to Self::ID if not provided.
    // Obviously, this is an error if Self::ID is not defined, forcing you to use the explicit form.
    (logger: $logger:expr, $($arg:tt)+) => ({
        log::trace!(logger: $logger, target: Self::ID, $($arg)+)
    });

    ($($arg:tt)+) => (log::trace!(target: Self::ID, $($arg)+))
}
