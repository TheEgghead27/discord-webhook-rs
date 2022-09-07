# Discord Webhook Agent
This is a little "script" I wrote for practice with Rust and asynchronous operations within Rust.

## Getting started
Clone this GitHub repository, then `cd` into the project and compile it (unlike other languages, cargo automatically installs dependencies on build/run).
```sh
git clone https://github.com/TheEgghead27/discord-webhook-rs.git
cd discord-webhook-rs
cargo build --release
```

## Configuration
The program expects a `webhooks.tsv` file at your `XDG_CONFIG_HOME` (normally `~/.config`), with a format as follows:
```tsv
WEBHOOK_URL	NAME(optional)	PREFIX(optional)
```
Note the use of `\t` tab-separated values instead of whitespaces.

If you are only specifying `WEBHOOK_URL`, you do not have to include a tab.

## Invokation
Simply type the built executable's filename, optionally followed by a list of files you wish to attach to your webhook message.
```
./target/release/webhook [FILENAMES]
```

If you prefer, you can move the `webhook` binary executable to another location and execute it there.
