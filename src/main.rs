mod cli;
mod crypto;
mod error;
mod stego;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;
use std::io::{self, BufRead, Read, Write};

fn read_passphrase(prompt: &str) -> io::Result<String> {
    eprint!("{}", prompt);
    io::stderr().flush()?;

    // Try to read from /dev/tty first (works even when stdin is redirected)
    if let Ok(tty) = std::fs::File::open("/dev/tty") {
        let mut reader = io::BufReader::new(tty);
        let mut passphrase = String::new();
        reader.read_line(&mut passphrase)?;
        return Ok(passphrase.trim().to_string());
    }

    // Fall back to stdin
    let mut passphrase = String::new();
    io::stdin().read_line(&mut passphrase)?;
    Ok(passphrase.trim().to_string())
}

fn encode(
    cover_path: &str,
    message: Option<&str>,
    message_file: Option<&str>,
    output: Option<&str>,
) -> Result<()> {
    // Read cover text
    let cover = std::fs::read_to_string(cover_path)?;

    // Get secret message from argument, file, or stdin
    let secret = if let Some(msg) = message {
        msg.to_string()
    } else if let Some(path) = message_file {
        std::fs::read_to_string(path)?
    } else {
        eprintln!("Enter secret message (Ctrl+D when done):");
        let mut msg = String::new();
        io::stdin().read_to_string(&mut msg)?;
        msg
    };

    // Get passphrase
    let passphrase = read_passphrase("Passphrase: ")?;

    // Encrypt the secret
    let encrypted = crypto::encrypt_with_passphrase(secret.as_bytes(), &passphrase)?;

    // Embed into cover text
    let artifact = stego::embed(&cover, &encrypted)?;

    // Output
    match output {
        Some(path) => std::fs::write(path, &artifact)?,
        None => print!("{}", artifact),
    }

    eprintln!("\n[OK] Embedded {} bytes into cover text", encrypted.len());
    Ok(())
}

fn decode(input: Option<&str>) -> Result<()> {
    // Read artifact
    let artifact = match input {
        Some(path) => std::fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    // Extract hidden data
    let encrypted = stego::extract(&artifact)?;

    // Get passphrase
    let passphrase = read_passphrase("Passphrase: ")?;

    // Decrypt
    let decrypted = crypto::decrypt_with_passphrase(&encrypted, &passphrase)?;

    // Output
    io::stdout().write_all(&decrypted)?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Encode {
            cover,
            message,
            message_file,
            output,
        } => encode(
            cover,
            message.as_deref(),
            message_file.as_deref(),
            output.as_deref(),
        ),
        Commands::Decode { input } => decode(input.as_deref()),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

