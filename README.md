# youglish-cli

A fast, convenient cli for quickly searching English word or phrase pronunciations on [YouGlish](https://youglish.com).

## Installation

### Linux

Latest binary

```bash
curl -sfo ~/.local/bin/yg https://github.com/time-slayer/youglish-cli/releases/latest/download/yg
chmod +x ~/.local/bin/yg
```

With cargo

```bash
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
