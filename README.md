# svg2svelte-rs

easily turn a svg file into a [svelte] component, rewritten in rust

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

```bash
git clone https://github.com/derektata/svg2svelte-rs
cd svg2svelte-rs
make
```

## Why?

The previous implementation I had written up in shell had issues running cross-platform and only worked on Linux, so I decided to start learning more Rust and this project was born.

[svelte]:https://svelte.dev/
[anime.js]:https://animejs.com/
