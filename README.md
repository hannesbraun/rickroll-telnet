# rickroll-telnet

This is a rewrite of the ["Rickroll" implemented by David C. McNett](https://github.com/nugget/rickroll). I only copied the lyrics, everything else is rewritten from scratch. Still...

- Copyright (c) 2013-2017 David C. McNett <nugget@macnugget.org>
- Copyright (c) 2022 Hannes Braun

The usage is trivial. Build it (hint: `cargo build --release`).
The file `lyrics.dat` is required to exist in the current working directory at runtime.
The executable called `cli` outputs the lyrics out once to stdout.
The executable called `net` starts a TCP server on port 23.
On every connection... well, you know what's about to happen.
You can optionally specify the port by supplying it as the first argument to `net`.
