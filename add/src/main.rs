use std::path::PathBuf;

use clap::Parser;

mod async_add;
mod state;
mod sync_add;

#[derive(Parser)]
#[clap(name = "add-host", version = env!("CARGO_PKG_VERSION"))]
struct AddApp {
    x: u32,
    y: u32,
    #[clap(value_name = "COMPONENT_PATH")]
    component: PathBuf,
}

impl AddApp {
    async fn run(self) -> anyhow::Result<()> {
        let sum1 = async_add::add(self.component.clone(), self.x, self.y).await?;
        let sum2 = sync_add::add(self.component.clone(), self.x, self.y)?;

        assert_eq!(sum1, sum2);
        println!("{} + {} = {}", self.x, self.y, sum1);
        Ok(())
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    AddApp::parse().run().await
}
