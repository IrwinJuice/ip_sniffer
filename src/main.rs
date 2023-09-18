use std::io;
use std::io::Write;
use std::net::IpAddr;

use clap::Parser;
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;

#[derive(Parser, Debug)]
struct Args {
    ipaddr: IpAddr,
    #[arg(short = 's', long = "start", value_name = "PORT", default_value = "1")]
    start_port: Option<u16>,
    #[arg(short = 'e', long = "end", value_name = "PORT", default_value = "65535")]
    end_port: Option<u16>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let (tx, rx) = channel::<u16>();

    if !start_port_guard(&args.start_port.unwrap()) {
        println!("Start port must be greater than 0");
        std::process::exit(1);
    }
    if !end_port_guard(&args.end_port.unwrap()) {
        println!("End port must be less than or equal to 65535");
        std::process::exit(1);
    }
    for port in args.start_port.unwrap()..=args.end_port.unwrap() {
        let tx = tx.clone();
        tokio::spawn(async move {
            scan(tx, port, args.ipaddr).await;
        });
    }
    // Drop the tx clones.
    drop(tx);

    let mut out = vec![];
    for p in rx {
        out.push(p);
    }

    println!();
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

async fn scan(tx: Sender<u16>, port: u16, ipaddr: IpAddr) {
    let connection = TcpStream::connect(format!("{}:{}", ipaddr, port)).await;
    if connection.is_ok() {
        print!(".");
        io::stdout().flush().unwrap();
        tx.send(port).unwrap();
    }
}
fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= u16::MAX
}