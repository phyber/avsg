# Axiom Verge Save Game Viewer

This tool allows you to view [Axiom Verge] save game files from the [Steam]
version of the game.

The Steam version of the game encrypts its save and settings data, so
decryption is needed to view them, which this tool provides.

This tool can also re-encrypt files after you're finished with them, allowing
you to edit the save game XML and resume playing the save game.

## Installation

  - Install Rust
  - Clone this repository
  - Within the repository execute `cargo build --release`
  - Place the binary at `target/release/avsg*` somewhere convenient

## Usage

This tool provides the user with three subcommands, `decrypt`, `encrypt`, and
`hacker`.

The `decrypt` and `encrypt` subcommands simpply decrypt and encrypt Axiom Verge
data for Steam.  The `hacker` subcommand will analyse a save data file and
inform the user which creatures they still need to hack to obtain the
[Hacker achievement].  The creature list for this achievement is based on the
[Bestiary].

### Decrypt

Decrypting a Steam save game and displaying the XML on stdout:

```
$ avsg decrypt Save0.sav
```

Decrypting a save game and writing the output directly to a file:

```
# Using the output argument
$ avsg decrypt Save0.sav Save0.xml

# Using output redirection in the shell
$ avsg decrypt Save0.sav > Save0.xml
```

### Encrypt

Encrypt a save game:

```
$ avsg encrypt Save0.xml Save0.sav
```

### Hacker

View creatures that required glitching for the Hacker achievement:

```
# Encrypted Steam save data
$ avsg hacker Save0.sav

# Unencrypted save data
$ avsg hacker --unencrypted Save0.xml
```

<!-- links -->
[Axiom Verge]: https://www.axiomverge.com/
[Bestiary]: https://axiom-verge.fandom.com/wiki/Category:Bestiary
[Hacker Achievement]: https://axiom-verge.fandom.com/wiki/Hacker_Trophy
[Steam]: https://store.steampowered.com/
