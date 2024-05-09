pub mod cli;
pub mod process;
pub mod utils;

pub use cli::*;
use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
