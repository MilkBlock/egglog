# egglog: The Next-Generation Equality Saturation Engine

<a href="https://egraphs-good.github.io/egglog/">
    <img alt="Web Demo" src="https://img.shields.io/badge/-web demo-blue"></a>
<a href="https://egraphs-good.github.io/egglog/docs/egglog">
    <img alt="Main Branch Documentation" src="https://img.shields.io/badge/docs-main-blue"></a>
<a href="https://codspeed.io/egraphs-good/egglog">
    <img src="https://img.shields.io/endpoint?url=https://codspeed.io/badge.json" alt="CodSpeed Badge"/></a>
<a href="https://egraphs.zulipchat.com/#narrow/stream/375765-egglog">
    <img src="https://img.shields.io/badge/zulip-join%20chat-blue" alt="Zulip Chat"/></a>

This is the repo for the core of the `egglog` engine, which combines the power of equality saturation and Datalog.

For getting started, try out the [egglog tutorial](https://github.com/egraphs-good/egglog-tutorial)!

You can [try out egglog in your web browser](https://egraphs-good.github.io/egglog/).

For a "battery-included" experience, we recommend [egglog-experimental](https://github.com/egraphs-good/egglog-experimental). It provides more features through additional `egglog` plugins.

If you use `egglog`, please use [this citation](./CITATION.bib).

---

The following instructions are for using/developing the core directly.

## Prerequisites & compilation

```
apt-get install make cargo
cargo install --path=root_directory_of_cloned_repo
```

## Usage

The core can used in REPL mode with:

```
cargo run --release
```

The standard mode processes an input file:

```
cargo run --release [-f fact-directory] [-naive] [--to-json] [--to-dot] [--to-svg] <files.egg>
```

* The `--to-dot` command will save a graphviz dot file at the end of the program, replacing the `.egg` extension with `.dot`.
* The `--to-svg`, which requires [Graphviz to be installed](https://graphviz.org/download/), will save a graphviz svg file at the end of the program, replacing the `.egg` extension with `.svg`.

One can also use `egglog` as a Rust library by adding the following to your `Cargo.toml`:

```
[dependencies]
egglog = "1.0.0"
```

See also the [Python binding](https://github.com/egraphs-good/egglog-python).

## Developement

To view documentation in a browser, run `cargo doc --open`.

Run `cargo test` to run the core `egglog` tests.

<!---

## Community extensions

Move to experimental readme

* [@hatoo](https://github.com/hatoo) maintains an [egglog-language extension](https://marketplace.visualstudio.com/items?itemName=hatookov.egglog-language) in VS Code (just search for "egglog" in VS Code). (Outdated)
* [@segeljakt](https://github.com/segeljakt) maintains a [Neovim plugin](https://github.com/segeljakt/tree-sitter-egg) for egglog using tree-sitter. (Outdated)

--->

## Benchmarks

TODO: @Saul

<!---

## Benchmarks

We run all of our "examples" [as benchmarks in codspeed](https://codspeed.io/egraphs-good/egglog). These are in CI
for every commit in main and for all PRs. It will run the examples with extra instrumentation added so that it can
capture a single trace of the CPU interactions ([src](https://docs.codspeed.io/features/understanding-the-metrics/)):

> CodSpeed instruments your benchmarks to measure the performance of your code. A benchmark will be run only once and the CPU behavior will be simulated. This ensures that the measurement is as accurate as possible, taking into account not only the instructions executed but also the cache and memory access patterns. The simulation gives us an equivalent of the CPU cycles that includes cache and memory access.

Since many of the shorter running benchmarks have unstable timings due to non deterministic performance ([like in the memory allocator](https://github.com/oxc-project/backlog/issues/89)),
we ["ignore"](https://docs.codspeed.io/features/ignoring-benchmarks/) them in codspeed. That way, we still
capture their performance, but their timings don't show up in our reports by default.

We use 50ms as our cutoff currently, any benchmarks shorter than that are ignored. This number was selected to try to ignore
any benchmarks with have changes > 1% when they haven't been modified. Note that all the ignoring is done manually,
so if you add another example that's short, an admin on the codspeed project will need to manually ignore it.

## Profiling

One way to profile egglog is to use [samply](https://github.com/mstange/samply/). Here's how you can use it:

```bash
# install samply
cargo install --locked samply
# build a profile build which includes debug symbols
cargo build --profile profiling
# run the egglog file and profile
samply record ./target/profiling/egglog tests/extract-vec-bench.egg
# [optional] run the egglog file without logging or printing messages, which can help reduce the stdout
# when you are profiling extracting a large expression
env RUST_LOG=error samply record ./target/profiling/egglog --dont-print-messages tests/extract-vec-bench.egg
```

--->

