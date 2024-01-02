//! NOTE: Most planned features have not been implemented yet.
//! Sink: A Simple File Syncing Utility
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

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // TODO: Refactor
    let is_server: bool = match args.client {
        Some(_) => {
            match args.server {
                Some(_) => panic!("expected only one of --client, --server"),
                None => false,
            }
        },
        None => {
            match args.server {
                Some(_) => true,
                None => panic!("expected one of --client, --server"),
            }
        }
    };

    if is_server {
        let s_args: Vec<String> = match args.server {
            Some(s_args) => s_args,
            None => panic!("expected arguments for --server"),
        };

        let mut server = SinkServer::new(
            s_args[0].clone(), 
            s_args[1].clone(),
        );
        server.start();
    }
    else {
        let c_args: Vec<String> = match args.client {
            Some(c_args) => c_args,
            None => panic!("expected arguments for --client"),
        };

        let mut client = SinkClient::new(
            c_args[0].clone(),
            c_args[1].clone(),
        );
        client.start();
    }
    
    Ok(())
}
