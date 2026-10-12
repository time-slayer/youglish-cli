# youglish-cli

A fast, convenient cli for quickly searching English word or phrase pronunciations on [YouGlish](https://youglish.com).

## Installation

```bash
# to ~/.local/bin
cargo install --git https://github.com/time-slayer/youglish-cli --root ~/.local
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

or directly with the accent's own flag:

```bash
yg --us literally
```
