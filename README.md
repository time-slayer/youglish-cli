# youglish-cli

A fast, convenient cli for quickly searching English word or phrase pronunciations on [YouGlish](https://youglish.com).

## Installation

Prebuilt binary (Linux x86_64):

```bash
curl -sSfLo ~/.local/bin/yg https://github.com/time-slayer/youglish-cli/releases/latest/download/yg
chmod +x ~/.local/bin/yg
```

Make sure `~/.local/bin` is in `PATH`.

With cargo:

```bash
cargo install --git https://github.com/time-slayer/youglish-cli
```

## Usage

Simply provide a word or phrase to open it on YouGlish:

```bash
yg literally                # single word
yg a bottle of water        # phrase
```

Filter by accent:

```bash
yg -a us literally
yg a bottle of water -a uk  # can be specified anywhere
```

Or use a shorthand flag:

```bash
yg --us literally
```

Split a phrase into individual words:

```bash
yg -s literally engage      # opens 2 separate YouGlish tabs
```
