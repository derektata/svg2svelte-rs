# svg2svelte-rs

Easily turn an SVG file into a Svelte component, Rewritten in Rust.

## How it works

`svg2svelte` parses the svg file's ids for `bind:` in the name and converts them into variables we can manipulate later on with an animation library (e.g. [anime.js])

## Dependencies

+ `svgo` will need to be installed globally

```bash
npm i -g svgo           # npm
yarn global add svgo    # yarn
pnpm add -g svgo        # pnpm
```

## Installation

build from source
```bash
git clone https://github.com/derektata/svg2svelte-rs
cd svg2svelte-rs
make
```

using cargo
```bash
cargo install svg2svelte
```

## Usage
```bash
USAGE:
    svg2svelte [OPTIONS] [SVG_FILE]

ARGS:
    <SVG_FILE>    File to be processed

OPTIONS:
    -h, --help          Print help information
    -t, --typescript    Create a Typescript component
    -V, --version       Print version information

EXAMPLES:
    svg2svelte ball.svg
    svg2svelte -t ball.svg
```

## Why?

The previous implementation I had written up in shell had issues running cross-platform and only worked on Linux, so I decided to start learning more Rust and this project was born.

[svelte]:https://svelte.dev/
[anime.js]:https://animejs.com/
