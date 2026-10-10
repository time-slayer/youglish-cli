# youglish-cli

A fast, convenient cli for quickly searching English word or phrase pronunciations on [YouGlish](https://youglish.com).

## Installation

## Usage

Simply provide a word or phrase to instantly generate and open the YouGlish link in your browser:

```bash
yg literally            # single word
yg a bottle of water    # phrase
```

Filter by accent with `--accent` (`-a`):

```bash
yg -a us literally
```

or directly with the accent's own flag:

```bash
yg --us literally
```
