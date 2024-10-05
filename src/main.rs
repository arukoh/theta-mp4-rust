use clap::Parser;
use std::{
    env,
    io::{self, Write},
};

use theta_mp4::parse;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    filename: String,

    #[arg(short, long, value_name = "Target Box")]
    target: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let target_boxes: Option<Vec<String>> = cli
        .target
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect());

    match parse(&cli.filename, target_boxes.as_deref()) {
        Some((_mp4, theta_meta)) => {
            if theta_meta.is_none() {
                writeln!(io::stderr(), "Metadata not found").unwrap();
                std::process::exit(0);
            }
            let meta = theta_meta.as_ref().unwrap();
            if let Some(rthu_box) = &meta.rthu {
                let _ = rthu_box.write_to_file(&cli.filename);
                if let Err(e) = rthu_box.write_to_file(&cli.filename) {
                    eprintln!("Failed to write the RTHU: {}", e);
                }
            }
            let json_result = serde_json::to_string_pretty(&meta.to_serializable()).unwrap();
            println!("{}", json_result);
        }
        None => {
            writeln!(io::stderr(), "Failed to parse the file").unwrap();
            std::process::exit(1);
        }
    }
}
