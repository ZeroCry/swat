# swat
*A helper for painless use of AT command interfaces.*

Ever got annoyed by having to re-type some AT commands for your LTE modem over
and over? Miss standard line editing and history capabilities? Well look no
further! This little tool right here gives you

* Line editing (correct those pesky +/!/^ mixups and put quotation marks in
  whatever order you want).
* Unlimited history (you're just two keystrokes away from the next
  `at!gstatus?`).
* History search (hit Ctrl+R to unlock this power user feature).
* It even inserts the `at` for you!

## Getting started (building from source)

1. [Get Rust](https://rustup.rs/)
2. In this directory, run `cargo build --release`
3. Copy the binary from *./target/release/* (e.g. to *~/.local/bin/*)
4. Run `swat help`

## Coming soon
* Auto binary builds
* Interface for running a single command
