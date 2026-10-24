//! The `jespersen` command. `main.rs` only passes the process's arguments and streams to [`run`],
//! so the tests can run every command in memory.

mod run;

pub use run::run;
