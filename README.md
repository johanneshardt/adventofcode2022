# adventofcode2022
Solutions to the problems in Advent of Code 2022, in Rust.

## Template
The provided [template](./template) can be used with [cargo generate](https://cargo-generate.github.io/cargo-generate/index.html), to generate some scaffolding code for solving a day,
like so: `cargo generate --path template --destination ./src --init --name "empty"`.

## Infiles
The [build script](build.rs) will automatically download missing input files for each day and place them in the correct subdirectory. This requires that you have created a `.env` file with your advent of code session cookie, see [`.env.example`](.env.example) for how it should look.
