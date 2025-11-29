# Stegano-Glyph

> *Visible chaos, hidden order.*

**Stegano-Glyph** is a steganography tool that generates abstract ASCII art structures and embeds encrypted payloads within them using zero-width character injection. The result looks like aesthetic terminal art but contains a secure, hidden message.

## 📜 Manifesto

In an era of deep packet inspection and ubiquitous surveillance, the safest message is one that appears to be noise. Stegano-Glyph treats visual aesthetics as a cryptographic envelope.

## ⚡ Features

- **Generative Arts**: Creates "Matrix", "Noise", and "Block" style ASCII patterns on the fly.
- **Zero-Width Steganography**: Embeds payloads using non-printing Unicode characters (ZWSP, ZWNJ, etc.).
- **Age Encryption**: Native Rust implementation of the `age` encryption format for robust security.
- **Terminal Native**: Designed for piping, colored output, and TUI environments.

## 🛠 Usage

### Encrypt & Hide
Generate a "cyberpunk" style artifact containing a hidden message:

```bash
$ stegano-glyph encode --style matrix --pubkey "age1..." --message "The rendezvous is at midnight." > artifact.txt
```

### Reveal & Decrypt
Read an artifact and extract the hidden message:

```bash
$ cat artifact.txt | stegano-glyph decode --identity key.txt
> "The rendezvous is at midnight."
```

## 🏗 Architecture

- `src/art.rs`: Generative algorithms for ASCII textures.
- `src/stego.rs`: Bit-manipulation for zero-width character encoding.
- `src/crypto.rs`: Wrapper for Age encryption keys.
