//! A Simple File Syncing Utility.
//! NOTE: Most planned features have not been implemented yet.
//! 
//! Provides a simple cli interface for syncing files between local and remote
//! directories.
//! 
//! To use simply:
//! - Setup a SinkServer on the remote directory
//! - Setup a SinkClient on the local directory

mod client;
mod server;

use clap::Parser;
use client::SinkClient;
use server::SinkServer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, num_args(2), value_names(["ADDRESS", "DIRECTORY"]))]
    client: Option<Vec<String>>,

    #[arg(short, long, num_args(2), value_names(["ADDRESS", "DIRECTORY"]))]
    server: Option<Vec<String>>, 
}

fn start_server(args : Args) {
    match args.server {
        Some(s_args) => {
            let mut server = SinkServer::new(
                s_args[0].clone(), 
                s_args[1].clone(),
            );
            server.start();
        },
        None => {},
    };
}

fn start_client(args: Args) {
    match args.client {
        Some(c_args) => {
            let mut client = SinkClient::new(
                c_args[0].clone(),
                c_args[1].clone(),
            );
            client.start();
        }
        None => {},
    };
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if !(args.client.is_some() ^ args.server.is_some()) {
        panic!("expected one of --client, --server")
    }

    if args.client.is_some() {
        start_client(args);
    }
    else {
        start_server(args);
    }

    Ok(())
}
