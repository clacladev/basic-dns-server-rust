use anyhow::Result;
use server::start_server;

mod server;

fn main() -> Result<()> {
    start_server()
}
