use anyhow::Result;
use cli_params::CliParam;
use server::{resolver::Resolver, start_server};
use std::env;

mod cli_params;
mod server;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let cli_params = CliParam::from(&args[1..]);
    let resolver = Resolver::from(cli_params.as_slice());

    start_server(resolver)
}
