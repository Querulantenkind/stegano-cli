# STEGANO-GLYPH

**Status:** Prototype / Active Development
**License:** MIT / Apache-2.0

## Abstract

Stegano-Glyph is a command-line utility designed for the secure transport of information through visual obfuscation. It generates abstract ASCII imagery—ranging from noise patterns to structured industrial schematics—and injects encrypted payloads directly into the character stream using non-printing Unicode markers.

This project operates at the intersection of cryptography and digital aesthetics, providing a mechanism for "plausible deniability" in communication. To the naked eye and standard text editors, the output is merely a generative art piece or a corrupted log file. To the recipient holding the correct identity key, it is a secure container for sensitive data.

## The Philosophy

In an environment defined by deep packet inspection and metadata retention, standard encryption flags attention. An encrypted PGP block declares itself as a secret immediately. True anonymity requires that the existence of the secret itself be concealed.

Stegano-Glyph treats visual chaos as a hiding place. By masquerading as "aesthetic noise"—or the visual artifacts of the `museum-of-abstract-cyphers`—sensitive payloads can be transmitted in plain sight on public forums, pastebins, or chat logs without triggering entropy-based scanning heuristics that look for standard cryptographic headers.

## Technical Specifications

### 1. Cryptographic Layer
The application utilizes the **Age** encryption standard (Actually Good Encryption), replacing legacy GPG complexities with modern primitives.
- **Primitive:** X25519 for key exchange, ChaCha20-Poly1305 for symmetric encryption.
- **Keys:** Uses compact, string-based public/private key pairs designed for easy manual entry in terminal environments.
- **Padding:** Payloads are padded to uniform block sizes to prevent traffic analysis based on message length.

### 2. Steganographic Layer
Information is not hidden in the visual pixels, but in the text encoding itself.
- **Method:** Zero-Width Character Injection.
- **Mechanism:** The encrypted ciphertext is converted into a binary stream. This stream is mapped to invisible Unicode code points (e.g., U+200B Zero Width Space, U+200C Zero Width Non-Joiner).
- **Injection Strategy:** These invisible characters are interleaved within the visible ASCII characters of the cover art. This ensures that line-wrapping or formatting changes in some viewers do not destroy the contiguous payload.

### 3. Visual Layer (The Cover)
The "cover text" is procedurally generated based on themes defined in the `style-atlas`.
- **Matrix:** Falling binary streams and katakana-esque glyphs.
- **Redacted:** Simulates a declassified government document with heavy use of block elements (█) and censoring bars.
- **Noise:** High-entropy character scatter, mimicking data corruption or static.
- **Ghost:** Minimalist, whitespace-heavy structures for low-profile transmission.

## Installation

Stegano-Glyph is written in Rust for memory safety and binary portability.

```bash
git clone https://github.com/Querulantenkind/stegano-glyph
cd stegano-glyph
cargo install --path .
```

## Usage Documentation

### Key Generation
Before transmission, generate an age-compatible identity.

```bash
# Generate a new identity file
stegano-glyph keygen --output ~/.stegano/identity.key

# Extract public key for distribution
stegano-glyph keygen --show-public --identity ~/.stegano/identity.key
```

### Encoding (Encryption + Hiding)
Embed a message into a generated visual artifact.

```bash
# Syntax: stegano-glyph encode [OPTIONS] --recipient [PUBKEY]
echo "Operation Midnight is go." | stegano-glyph encode \
    --recipient "age1ql3z7hjy54pw3hyww5..." \
    --style redacted \
    --cover-width 80 \
    > transmission_001.txt
```

### Decoding (Extraction + Decryption)
The tool automatically detects the steganographic layer, extracts it, and attempts decryption.

```bash
# Syntax: stegano-glyph decode --identity [PRIVKEY] < [FILE]
cat transmission_001.txt | stegano-glyph decode --identity ~/.stegano/identity.key
```

## Security Notice

While the encryption layer uses industry-standard primitives, the steganographic layer is susceptible to "sanitization" attacks. If the ASCII artifact is pasted into a system that strips non-ASCII characters or normalizes Unicode (like some strict social media platforms), the hidden payload will be destroyed. Always verify integrity via a checksum if possible.

**Disclaimer:** This tool is for educational and research purposes regarding data privacy and censorship resistance.
