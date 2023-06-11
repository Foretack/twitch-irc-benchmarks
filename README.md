# twitch-irc-benchmarks

The general outline of the benchmark:
- The first 1000 lines of `data.txt` are read and prepared during setup
- For each benchmark iteration, parse each line in a loop and discard the result

`data.txt` consists of roughly 200 thousand messages. The benchmarks only read the first 1000 lines from this file.

### Rust

Rust benchmarks use [criterion](https://github.com/bheisler/criterion.rs).

```
$ cd rust && cargo bench
```

`twitch-rs` uses the `simd` feature, which enables the parser implementation which uses SIMD instructions.
It can be disabled by removing the `simd` feature from the `Cargo.toml`.

`twitch-rs` also has support for a tag whitelist. Any tag which is not whitelisted will be ignored.
This can improve performance by up to 40%. In the results below, the version of the benchmark using
this feature is labelled `cheating`, which feels appropriate, because while it *is* fully parsing the tags
even if they're ignored, it isn't actually storing them in the parsed message.

### C# .NET

C# benchmarks use [BenchmarkDotNet](https://github.com/dotnet/BenchmarkDotNet).

```
$ cd dotnet && DOTNET_TieredPGO=1 dotnet run -c Release
```

Benchmarks run with both JIT and AOT where possible.

### Go

Go benchmarks use the built-in benchmarking tool.

```
$ cd go && go test -bench=.
```

### Node

Node benchmarks use benchmark.js
```
cd node && npm i && node bench.js
```

## Results

### x86

Benchmarks were run in WSL2 Ubuntu 22.04 on an AMD Ryzen 7950X

| library                                                                                                               | language                                    | time to parse 1000 lines |
| --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- | ------------------------ |
| [twitch](https://github.com/jprochazk/twitch-rs/tree/3f04961e70a2a4838af535540bb5cbb7b4319e44)                        | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 593.24 µs ± 0.91 µs      |
| [twitch](https://github.com/jprochazk/twitch-rs/tree/3f04961e70a2a4838af535540bb5cbb7b4319e44) (`-F simd`)            | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 357.75 µs ± 1.07 µs      |
| [twitch](https://github.com/jprochazk/twitch-rs/tree/3f04961e70a2a4838af535540bb5cbb7b4319e44) (`-F simd` + cheating) | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 195.51 µs ± 0.14 µs      |
| [twitch-irc](https://github.com/robotty/twitch-irc-rs/tree/v5.0.0)                                                    | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 2.2193 ms ± 2.58 µs      |
| [irc_rust](https://github.com/MoBlaa/irc_rust/tree/4ae66fb3176b1d46cec6764f1a76aa6e9673d08b)                          | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 969.86 µs ± 0.83 µs      |
| [justgrep](https://github.com/Mm2PL/justgrep/tree/v0.0.6)                                                             | Go 1.20                                     | 1.395126 ms              |
| [go-twitch-irc](https://github.com/jprochazk/go-twitch-irc/tree/v4.2.0)                                               | Go 1.20                                     | 3.75188 ms               |
| [minitwitch](https://github.com/jprochazk/minitwitch-bench/tree/a5d2c7b7f5717ff00e6a2f29fd1c0099ff02a59d)             | .NET 8.0.100-preview.4.23260.5              | 883.4 µs ± 4.78 µs       |
| [minitwitch](https://github.com/jprochazk/minitwitch-bench/tree/a5d2c7b7f5717ff00e6a2f29fd1c0099ff02a59d) (+ AOT)     | .NET 8.0.100-preview.4.23260.5              | 772.0 µs ± 5.27 µs       |

### arm64

Benchmarks were run with macOS 13.4 running on an M2 MacBook Air 16 GB

| library                                                                                                               | language                                    | time to parse 1000 lines |
| --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- | ------------------------ |
| [twitch](https://github.com/jprochazk/twitch-rs/tree/3f04961e70a2a4838af535540bb5cbb7b4319e44)                        | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 1.0960 ms ± 3.16 µs      |
| [twitch](https://github.com/jprochazk/twitch-rs/tree/3f04961e70a2a4838af535540bb5cbb7b4319e44) (`-F simd`)            | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 604.93 µs ± 0.50 µs      |
| [twitch](https://github.com/jprochazk/twitch-rs/tree/3f04961e70a2a4838af535540bb5cbb7b4319e44) (`-F simd` + cheating) | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 422.62 µs ± 1.00 µs      |
| [twitch-irc](https://github.com/robotty/twitch-irc-rs/tree/v5.0.0)                                                    | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 3.2780 ms ± 5.72 µs      |
| [irc_rust](https://github.com/MoBlaa/irc_rust/tree/4ae66fb3176b1d46cec6764f1a76aa6e9673d08b)                          | rustc 1.72.0-nightly (101fa903b 2023-06-04) | 1.6897 ms ± 9.81 µs      |
| [justgrep](https://github.com/Mm2PL/justgrep/tree/v0.0.6)                                                             | Go 1.20                                     | 1.707313 ms              |
| [go-twitch-irc](https://github.com/jprochazk/go-twitch-irc/tree/v4.2.0)                                               | Go 1.20                                     | 4.714300 ms              |
| *[minitwitch](https://github.com/jprochazk/minitwitch-bench/tree/a5d2c7b7f5717ff00e6a2f29fd1c0099ff02a59d)            | .NET 8.0.100-preview.4.23260.5              | 1.387 ms ± 0.0013 ms     |
| *[minitwitch](https://github.com/jprochazk/minitwitch-bench/tree/a5d2c7b7f5717ff00e6a2f29fd1c0099ff02a59d) (+ AOT)    | .NET 8.0.100-preview.4.23260.5              | 1.263 ms ± 0.0119 ms     |

NOTE: `minitwitch` was not run with `DOTNET_TieredPGO=1`, because the benchmark would segfault each time I attempted to run it.
