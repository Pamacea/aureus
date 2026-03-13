//! CLI command implementations

pub mod commit;
pub mod amend;
pub mod release;
pub mod suggest;
pub mod hooks;
pub mod config;
pub mod init;
pub mod stats;

pub use commit::execute as commit;
pub use amend::execute as amend;
pub use release::execute as release;
pub use suggest::execute as suggest;
pub use hooks::execute as hooks;
pub use config::execute as config;
pub use init::execute as init;
pub use stats::execute as stats;
