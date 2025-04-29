Its still on beta, stay tuned!

# Assistbot

> This is the NEXT GENERATION of assistbot

This features robust features of assistbot

- Be able to generate image/text from pollination.ai
- Supports enum
- Clean & more understandable module arrangement

It's already tested in [Shuttle](https://shuttle.dev), but since I used the free plan, I can't show the demo atm

# Installation
Go to assistbot folder (nested from root) and install the cargo packages

```shell
$ cd assistbot
```

Now actually i dont know how to add all packages from existing `Cargo.toml`. Since i use both NVim and VSCode, rust-analyzer automatically install it.. 

So if you use those, just wait until the analyzer finished

And then go back to the root folder, and run the packages

```shell
$ cd ..; cargo run .
```

To publish to [Shuttle](https://shuttle.dev), you need to install `cargo-shuttle`

```shell
$ cargo install
```