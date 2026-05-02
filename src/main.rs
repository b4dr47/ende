use clap::{Parser, Subcommand};
use base64::{Engine as _, engine::general_purpose};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        text: String,    
    },
    Decode {
        text: String,
    },
}

fn encode(input: &str) {
    let encoded = general_purpose::STANDARD.encode(input);
    println!("{encoded}");
}

fn decode(input: &str) {
    let decoded = general_purpose::STANDARD.decode(input).unwrap();
    println!("{}", String::from_utf8(decoded).unwrap());
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { text } => {
            encode(&text);
        }
        Commands::Decode { text } => {
            decode(&text);
        }
    }
}
