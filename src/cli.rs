use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stegano-glyph")]
#[command(about = "Steganographic encryption tool - hide encrypted data in plain sight")]
#[command(long_about = "Stegano-Glyph encrypts your secret messages using Age encryption \
    and hides them within innocent-looking cover text using zero-width Unicode characters. \
    The result looks like normal text but contains your encrypted payload.")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode a secret message into cover text
    Encode {
        /// Cover text file (the innocent-looking text to hide data in)
        #[arg(short, long)]
        cover: String,

        /// Secret message to hide (if not provided, reads from stdin)
        #[arg(short, long)]
        message: Option<String>,

        /// File containing the secret message
        #[arg(short = 'f', long)]
        message_file: Option<String>,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Decode a hidden message from an artifact
    Decode {
        /// Input file containing the artifact (default: stdin)
        #[arg(short, long)]
        input: Option<String>,
    },
}

