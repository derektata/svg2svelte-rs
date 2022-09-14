
# svg2svelte-rs
![Logo](https://user-images.githubusercontent.com/63433989/190178451-42bdd0fe-7d0d-4303-92ad-724121769d44.svg)

Easily turn an SVG file into a Svelte component, Rewritten in Rust.
## How It Works

**svg2svelte** parses the svg file's ids for **`bind:`** in the name and converts them into variables we can manipulate later on with an animation library (e.g. [anime.js])

[anime.js]: https://animejs.com/
## Dependencies

+ `svgo` will need to be installed globally.

```bash
npm i -g svgo           # npm
yarn global add svgo    # yarn
pnpm add -g svgo        # pnpm
```
## Installation

### Build from source
```bash
git clone https://github.com/derektata/svg2svelte-rs
cd svg2svelte-rs
make
```

### Using Cargo
```bash
cargo install svg2svelte
```
## Usage/Examples

```plaintext
USAGE:
    svg2svelte [OPTIONS] [SVG_FILE]

ARGS:
    <SVG_FILE>    File to be processed

OPTIONS:
    -h, --help          Print help information
    -t, --typescript    Create a Typescript component
    -v, --verbose       Print the generated component to stdout
    -V, --version       Print version information

EXAMPLES:
    svg2svelte ball.svg
    svg2svelte -t ball.svg
```


## Screenshot

![screenshot](https://user-images.githubusercontent.com/63433989/190179524-4924ac11-d98f-49e7-aea1-49a41b53b60d.png)
## Why?

The previous implementation I had written up in shell had issues running cross-platform and only worked on Linux, so I decided to start learning more Rust and this project was born.
## License

[AGPL-3.0 License](https://github.com/derektata/svg2svelte-rs/blob/main/LICENSE)


## Author

[Derek Tata](https://www.github.com/derektata)

