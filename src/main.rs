use futures::future::join_all;
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use std::env;
use std::path::PathBuf;
use std::process::Command;
use tokio::fs;
use csv::ReaderBuilder;

static BUF_NAME: &str = "/tmp/webhook.tmp";

struct Destination {
    url: String,
    name: String,
    prefix: String,
}

struct File {
    name: PathBuf,
    data: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let mut files: Vec<File> = Vec::new();
    for (i, arg) in env::args().enumerate() {
        if (i < 1) ||
            // don't read "dev" if we're in `cargo run dev`
            (cfg!(debug_assertions) && i == 1 && arg == "dev")
        {
            continue;
        }
        files.push(File {
            name: PathBuf::from(&arg),
            data: fs::read(arg).await.unwrap(),
        });
    }

    let u = get_dest();

    let editor: String = match env::var("EDITOR") {
        Ok(ed) => ed,
        Err(_) => String::from("vim"),
    };
    Command::new(editor)
        .arg(BUF_NAME)
        // even though the docs say spawn() and status() should do the same thing with stdio,
        // it seems to mess up stdin when using spawn() instead of status()
        .status()
        .expect("EDITOR failed to start - have you exported the environment variable?");

    let contents = fs::read_to_string(BUF_NAME)
        .await
        .unwrap()
        // .trim() returns a &str which we dont own, and disappears with this String after
        // this expression
        // so instead we use .to_owned() to take ownership of it as a String
        .trim()
        .to_owned();

    let client = Client::new();

    let mut v = Vec::new();
    for dest in u {
        v.push(req(&contents, &files, dest, &client));
    }

    // https://stackoverflow.com/questions/63326882/how-to-wait-for-a-list-of-async-function-calls-in-rust
    join_all(v).await;
}

fn get_dest() -> Vec<Destination> {
    let path: String = env::var("XDG_CONFIG_HOME")
        .unwrap_or(env::var("HOME").unwrap_or("~".to_string()) + "/.config/")
        + "webhooks.tsv";

    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .comment(Some(b'#'))
        .flexible(true)
        .from_path(path)
        .expect("~/.config/webhooks.tsv exists");

    let mut out: Vec<Destination> = Vec::new();
    for result in reader.records() {
        let record = result.unwrap();
        out.push(Destination {
            url: record[0].to_owned(),
            name: String::from(record.get(1).unwrap_or("")),
            prefix: String::from(record.get(2).unwrap_or("")),
        });
    }

    return out;
}

async fn req(msg: &String, files: &Vec<File>, dest: Destination, client: &reqwest::Client) {
    let mut form = Form::new().part(
        "payload_json",
        Part::text(format!("{{\"content\": \"{} {}\"}}", dest.prefix, msg))
            .mime_str("application/json")
            .expect("Failed to set mime"),
    );

    for (i, file) in files.iter().enumerate() {
        form = form.part(
            format!("files[{i}]"),
            Part::bytes(file.data.to_owned())
                .file_name(file.name.file_name().unwrap().to_str().unwrap().to_owned()),
        );
        println!("Attached {} as files[{}]", file.name.display(), i);
    }

    let res = client.post(dest.url).multipart(form).send().await;
    match res {
        Ok(resp) => {
            println!("{}: {:#?}", dest.name, resp.status());
            if resp.error_for_status_ref().is_err() {
                println!("{:#?}\n******", resp);
                println!("{:#?}", resp.text().await.unwrap_or("[null]".to_string()));
            }
        }
        Err(error) => panic!("uwupsies error: {:?}", error),
    }
}
