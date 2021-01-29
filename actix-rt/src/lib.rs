//! Tokio-based single-thread async runtime for the Actix ecosystem.

#![deny(rust_2018_idioms, nonstandard_style)]
#![allow(clippy::type_complexity)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://actix.rs/img/logo.png")]
#![doc(html_favicon_url = "https://actix.rs/favicon.ico")]

use std::future::Future;

use tokio::task::JoinHandle;

// Cannot define a main macro when compiled into test harness.
// Workaround for https://github.com/rust-lang/rust/issues/62127.
#[cfg(all(feature = "macros", not(test)))]
pub use actix_macros::{main, test};

mod arbiter;
mod builder;
mod runtime;
mod system;

pub use self::arbiter::Arbiter;
pub use self::builder::{Builder, SystemRunner};
pub use self::runtime::Runtime;
pub use self::system::System;

/// Spawns a future on the current arbiter.
///
/// # Panics
/// Panics if Actix system is not running.
#[inline]
pub fn spawn<Fut>(f: Fut) -> JoinHandle<()>
where
    Fut: Future<Output = ()> + 'static,
{
    tokio::task::spawn_local(f)
}

/// Asynchronous signal handling
pub mod signal {
    #[cfg(unix)]
    pub mod unix {
        //! Unix specific signals.
        pub use tokio::signal::unix::*;
    }
    pub use tokio::signal::ctrl_c;
}

pub mod net {
    //! TCP/UDP/Unix bindings

    pub use tokio::net::UdpSocket;
    pub use tokio::net::{TcpListener, TcpStream};

    #[cfg(unix)]
    mod unix {
        pub use tokio::net::{UnixDatagram, UnixListener, UnixStream};
    }

    #[cfg(unix)]
    pub use self::unix::*;
}

pub mod time {
    //! Utilities for tracking time.

    pub use tokio::time::Instant;
    pub use tokio::time::{interval, interval_at, Interval};
    pub use tokio::time::{sleep, sleep_until, Sleep};
    pub use tokio::time::{timeout, Timeout};
}

pub mod task {
    //! Task management.

    pub use tokio::task::{spawn_blocking, yield_now, JoinHandle};
}
