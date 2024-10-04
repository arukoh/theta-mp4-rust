use std::{env, io::{self, Write}};
use clap::Parser;

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
    let target_boxes: Option<Vec<String>> = cli.target.map(|t| {
        t.split(',')
         .map(|s| s.trim().to_string())
         .collect()
    });

    match parse(&cli.filename, target_boxes.as_deref()) {
        Some(theta_meta) => {
            if let Some(rthu_box) = &theta_meta.rthu {
                let _ = rthu_box.write_to_file(&cli.filename);
                if let Err(e) = rthu_box.write_to_file(&cli.filename) {
                    eprintln!("Failed to write the RTHU: {}", e);
                }
            }
            let json_result = serde_json::to_string_pretty(&theta_meta.to_serializable()).unwrap();
            println!("{}", json_result);
        }
        None => {
            writeln!(io::stderr(), "Failed to parse the file").unwrap();
            std::process::exit(1);
        }
    }
}
