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
    /// Generate a new Age keypair for public key encryption
    Keygen {
        /// Output file for the identity (private key). Public key is printed to stderr.
        #[arg(short, long)]
        output: String,
    },

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

        /// Recipient public key (age1...). Can be specified multiple times.
        #[arg(short, long, action = clap::ArgAction::Append)]
        recipient: Vec<String>,

        /// File containing recipient public keys (one per line)
        #[arg(short = 'R', long)]
        recipient_file: Option<String>,
    },

    /// Decode a hidden message from an artifact
    Decode {
        /// Input file containing the artifact (default: stdin)
        #[arg(short, long)]
        input: Option<String>,

        /// Identity file (private key) for decryption. If not provided, uses passphrase.
        #[arg(short = 'I', long)]
        identity: Option<String>,
    },
}
