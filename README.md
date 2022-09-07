# Discord Webhook Agent
This is a little "script" I wrote for practice with Rust and asynchronous operations within Rust.

## Getting started
Clone this GitHub repository, then `cd` into the project.
```sh
git clone https://github.com/TheEgghead27/discord-webhook-rs.git
cd discord-webhook-rs
```

From here, you need to edit the `u` array in `src/main.rs` to contain the Discord webhook URLs that you wish to target.

After you've configured your target URLs, you can compile the program for general usage.
```
cargo build --release
```

## Invokation
Simply type the built executable's filename, optionally followed by a list of files you wish to attach to your webhook message.
```
./target/release/webhook [FILENAMES]
```

If you prefer, you can move the `webhook` binary executable to another location and execute it there.
