# Daily Programmer Challenges

[![Build Status](https://travis-ci.com/Michael-F-Bryan/daily-programmer.svg?branch=master)](https://travis-ci.com/Michael-F-Bryan/daily-programmer)

([Challenges] | [API Docs])

Solution to random challenges taken from the [/r/dailyprogrammer/][reddit]
subreddit.

## Getting Started

First you'll want to grab the project's source code,

```console
$ git clone git@github.com:Michael-F-Bryan/daily-programmer.git
```

Now you should be able to build everything and start the runner program,

```console
$ cargo build --all --exclude dashboard
    Blocking waiting for file lock on build directory
   Compiling runner v0.1.0 (/home/michael/Documents/daily-programmer/runner)
    Finished dev [unoptimized + debuginfo] target(s) in 1.64s
$ cargo run --bin runner -- -h
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/runner -h`
runner 0.1.0
Michael Bryan <michaelfbryan@gmail.com>
Solution to random challenges taken from the /r/dailyprogrammer/ subreddit

USAGE:
    runner [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Generate more verbose output

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    list       List all known challenges
    run        Execute a challenge
    run-all    Run all the challenges
$ cargo run --bin runner -- run -n 375 -d easy
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/runner run -n 375 -d easy`
title: Print a new number by adding one to each of its digit
 Apr 26 23:33:04.711 INFO Starting the challenge
 Apr 26 23:33:04.711 INFO Running the example, expected: 10109, input: 998
 Apr 26 23:33:04.711 INFO Finished running the challenge, duration: 4.637µs
```

> **Note:** You need to `exclude` the `dashboard` project when compiling with
> `--all`. The `dashboard` crate is only designed to compile as WASM.

You'll need `cargo-web` To view the dashboard,

```console
$ cargo install cargo-web
```

And now you can run the dev server,

```console
$ cd dashboard
$ cargo web start --open
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
    Processing "main.wasm"...
    Finished processing of "main.wasm"!

If you need to serve any extra files put them in the 'static' directory
in the root of your crate; they will be served alongside your application.
You can also put a 'static' directory in your 'src' directory.

Your application is being served at '/main.js'. It will be automatically
rebuilt if you make any changes in your code.

You can access the web server at `http://[::1]:8000`.
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[reddit]: https://www.reddit.com/r/dailyprogrammer/
[API Docs]: https://michael-f-bryan.github.io/daily-programmer/crate-docs/
[Challenges]: https://michael-f-bryan.github.io/daily-programmer/dashboard/
