use crate::app::app;
use clap::Parser;
use clap_derive::Parser;

mod app;
mod unix;
mod iso8601;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short = 'b', long, env = "QTIME_BIND_ADDR", default_value = "[::]:8080")]
    bind: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let listener = tokio::net::TcpListener::bind(args.bind).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}
