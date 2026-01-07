use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    init: Option<bool>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    todo!()
}
