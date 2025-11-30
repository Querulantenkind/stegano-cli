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

fn keygen(output_path: &str) -> Result<()> {
    let keypair = crypto::generate_keypair();

    // Write identity (private key) to file
    let identity_content = format!(
        "# created: {}\n# public key: {}\n{}\n",
        chrono_lite_timestamp(),
        keypair.recipient,
        keypair.identity
    );
    std::fs::write(output_path, &identity_content)?;

    // Set restrictive permissions on the identity file (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(output_path, perms)?;
    }

    eprintln!("Public key: {}", keypair.recipient);
    eprintln!("Identity written to: {}", output_path);

    Ok(())
}

/// Simple timestamp without external crate
fn chrono_lite_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Convert to rough date (good enough for a comment)
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining_days = days % 365;
    let months = remaining_days / 30 + 1;
    let day = remaining_days % 30 + 1;

    format!("{:04}-{:02}-{:02}", years, months, day)
}

fn encode(
    cover_path: &str,
    message: Option<&str>,
    message_file: Option<&str>,
    output: Option<&str>,
    recipients: &[String],
    recipient_file: Option<&str>,
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

    // Encrypt the secret - use public key if recipients provided, otherwise passphrase
    let encrypted = if !recipients.is_empty() || recipient_file.is_some() {
        crypto::encrypt_with_recipients(secret.as_bytes(), recipients, recipient_file)?
    } else {
        let passphrase = read_passphrase("Passphrase: ")?;
        crypto::encrypt_with_passphrase(secret.as_bytes(), &passphrase)?
    };

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

fn decode(input: Option<&str>, identity: Option<&str>) -> Result<()> {
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

    // Decrypt - use identity file if provided, otherwise passphrase
    let decrypted = if let Some(identity_path) = identity {
        crypto::decrypt_with_identity(&encrypted, identity_path)?
    } else {
        let passphrase = read_passphrase("Passphrase: ")?;
        crypto::decrypt_with_passphrase(&encrypted, &passphrase)?
    };

    // Output
    io::stdout().write_all(&decrypted)?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Keygen { output } => keygen(output),
        Commands::Encode {
            cover,
            message,
            message_file,
            output,
            recipient,
            recipient_file,
        } => encode(
            cover,
            message.as_deref(),
            message_file.as_deref(),
            output.as_deref(),
            recipient,
            recipient_file.as_deref(),
        ),
        Commands::Decode { input, identity } => decode(input.as_deref(), identity.as_deref()),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
