# STEGANO-GLYPH

**Version:** 0.2.0-alpha
**Status:** Active Development
**License:** MIT / Apache-2.0

## Abstract

Stegano-Glyph is a cryptographic obfuscation suite designed for the secure transport of information through hostile monitoring environments. Unlike traditional encryption tools which output easily identifiable high-entropy data blocks (such as PGP armor), Stegano-Glyph masks encrypted payloads within procedurally generated, low-entropy ASCII artifacts.

The tool operates on the principle of "Visual Indifference": creating data containers that appear to be innocuous digital refuse—system logs, compiler warnings, abstract art, or plain text documentation—thereby bypassing heuristic scanners designed to flag encrypted communications.

## Operational Doctrine

### The Grey Man Theory of Data
In physical surveillance, the "Grey Man" is an individual who avoids observation by blending seamlessly into the crowd. Stegano-Glyph applies this to digital signaling. By embedding data into structures that look like "noise" or "standard output," the transmission avoids metadata tagging.

### Threat Model
This tool is designed to counter:
1.  **Automated Entropy Scanners:** Systems that flag text blocks with high character randomness (typical of raw ciphertext).
2.  **Casual Observation:** Human monitors reviewing chat logs or pastebins who ignore "broken" text or "art" as irrelevant.
3.  **Association Attacks:** Preventing the linkage of a pseudonymous identity to PGP keys by never publicly displaying the key headers.

## Core Systems

### 1. The Cipher Engine
Stegano-Glyph utilizes the **Age** encryption standard (Actually Good Encryption) for the cryptographic layer.
- **Algorithm:** X25519 (Curve25519) for asymmetric key exchange; ChaCha20-Poly1305 for authenticated symmetric encryption.
- **Forward Secrecy:** Ephemeral session keys are generated for every payload.
- **Armor:** No PEM headers or footers are retained in the final artifact; only the raw binary stream is passed to the steganography layer.

### 2. The Steganography Engine
The engine maps the encrypted binary stream to invisible or unobtrusive Unicode code points.
- **Zero-Width Injection:** Utilizing `U+200B` (Zero Width Space), `U+200C` (Zero Width Non-Joiner), and `U+200D` (Zero Width Joiner) to encode binary data between visible characters.
- **Whitespace Modulation:** Encoding data by varying the use of space (0x20) and tab (0x09) characters at the end of lines (EOL), ideal for hiding data in source code files.
- **Homoglyph Substitution (Experimental):** Swapping Latin characters for visually identical Cyrillic or Greek counterparts to encode bitstreams (e.g., Latin 'a' vs Cyrillic 'а').

### 3. The Mimicry Engine (The Loom)
The "Loom" generates the cover text that houses the payload. It supports multiple camouflage patterns:

*   **Pattern A: Abstract (The Museum):** Generates aesthetic ASCII patterns, leveraging algorithms similar to those found in the `museum-of-abstract-cyphers`.
*   **Pattern B: Redacted (The Dossier):** Simulates declassified government documents with block elements (█), distinct timestamps, and "censored" text blocks.
*   **Pattern C: Source (The Developer):** Generates syntactically correct (but functionally useless) pseudo-code in Rust, Python, or C. The payload is hidden in the code's comments and whitespace.
*   **Pattern D: Logfile (The Sysadmin):** Mimics standard `syslog` or `dmesg` output, hiding data within timestamp variations and error codes.

## Advanced Capabilities

### Chaffing and Decoys
To counter coercion or deep forensic analysis, Stegano-Glyph supports "Chaffing."
- Users can embed *two* messages in a single artifact: a "Decoy" message and the "True" payload.
- The artifact contains two distinct encrypted streams intermingled.
- Decrypting with the *Duress Key* reveals the Decoy (e.g., "Meeting cancelled").
- Decrypting with the *Master Key* reveals the True payload (e.g., "Coordinates attached").

### Integrity Verification
Because steganography is fragile (susceptible to whitespace stripping by text editors), Stegano-Glyph includes a "Fragility Check."
- Before embedding, a CRC32 checksum of the payload is calculated.
- Upon decoding, if the checksum fails (indicating the cover text was formatted or stripped), the tool alerts the user that the message has been compromised or corrupted.

## Installation

```bash
git clone https://github.com/Querulantenkind/stegano-glyph
cd stegano-glyph
cargo build --release
cp target/release/stegano-glyph /usr/local/bin/
```

## Usage Guide

### 1. Identity Management
Generate an age-compatible key pair.

```bash
stegano-glyph keygen --out ~/.stegano/id_master
```

### 2. Artifact Generation (Encoding)

**Scenario:** Hiding a coordinate string inside a fake system log.

```bash
echo "48.8566 N, 2.3522 E" | stegano-glyph encode \
    --recipient-file public_keys.txt \
    --mimic logfile \
    --log-level error \
    --output error_dump_2024.log
```

**Scenario:** Hiding a manifesto inside a Markdown file (using whitespace modulation).

```bash
cat manifesto.txt | stegano-glyph encode \
    --recipient-pubkey "age1..." \
    --mimic markdown \
    --inject-strategy eol-whitespace \
    > README_DRAFT.md
```

### 3. Artifact Extraction (Decoding)

The tool automatically detects the presence of Stegano-Glyph artifacts in a file stream.

```bash
cat error_dump_2024.log | stegano-glyph decode --identity ~/.stegano/id_master
```

## Integration with TailsOS

For users of `tails-of-anonymity`, this tool is designed to live in the Persistent Storage partition (`/live/persistence/TailsData_unlocked/`).
- It requires no external network dependencies after compilation.
- It leaves no temp files; all processing is done in RAM.
- It pairs with the native `keepassxc` or `gnupg` flows for key management.

## Disclaimer

Stegano-Glyph provides **obscurity**, not just encryption. However, strict network monitoring may detect the statistical anomaly of the non-printing characters if deep file analysis is performed. This tool is intended for privacy research and censorship resistance.
