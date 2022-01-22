use anyhow::Result;
use clap::Parser;
use futures::future::select_all;

mod actor;
mod network;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    #[clap(short, long, default_value_t = 10)]
    number: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let mut actor_threads = vec![];
    for id in 0..args.number {
        let child = tokio::spawn(async move { actor::run_actor(id).await });

        actor_threads.push(child);
    }

    let failed = select_all(actor_threads).await;
    let _ = dbg!(failed);

    Ok(())
}
