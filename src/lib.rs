pub mod cli;
pub mod process;
pub mod utils;

pub use cli::TextSignFormat;

#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
