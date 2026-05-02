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
    Base64 {
        #[command(subcommand)]
        action: Base64Commands,
    },
    Caesar {
        #[command(subcommand)]
        action: CaesarCommands,
    },
}

#[derive(Subcommand, Clone)]
enum Base64Commands {
    Encode {
        text: String,
    },
    Decode {
        text: String,
    },
}

#[derive(Subcommand, Clone)]
enum CaesarCommands {
    Encode {
        text: String,
        #[arg(short, long, default_value_t = 13)]
        shift: u8,
    },
    Decode {
        text: String,
        #[arg(short, long, default_value_t = 13)]
        shift: u8,
    },
}

fn base64_encode(input: &str) {
    let encoded = general_purpose::STANDARD.encode(input);
    println!("{encoded}");
}

fn base64_decode(input: &str) {
    let decoded = general_purpose::STANDARD.decode(input).unwrap();
    println!("{}", String::from_utf8(decoded).unwrap());
}

fn caesar_shift(input: &str, shift: &u8) -> String {
    let shift = shift % 26;
    input.chars().map(|c| match c {
        'a'..='z' => (b'a' + (c as u8 - b'a' + shift) % 26) as char,
        'A'..='Z' => (b'A' + (c as u8 - b'A' + shift) % 26) as char,
        other => other,
    }).collect()
}

fn caesar_encode(input: &str, shift: &u8) {
    println!("{}", caesar_shift(input, shift));
}

fn caesar_decode(input: &str, shift: &u8) {
    println!("{}", caesar_shift(input, &(26 - (shift % 26))));
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
    Commands::Base64 { action } => match action 
    {
        Base64Commands::Encode { text } => {
            base64_encode(&text);
        }
        Base64Commands::Decode { text } => {
            base64_decode(&text);
        }
    }
        Commands::Caesar { action } => match action {
            CaesarCommands::Encode { text, shift } => {
                caesar_encode(&text, &shift);
            }
            CaesarCommands::Decode { text, shift } => {
                caesar_decode(&text, &shift);
            }
        }
    }
}
