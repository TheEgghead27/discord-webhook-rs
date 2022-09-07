use std::env;
use std::path::PathBuf;
use std::process::Command;
use tokio::fs;
use futures::future::join_all;
use reqwest::{Client, multipart::{Form, Part}};

static BUF_NAME: &str = "/tmp/webhook.tmp";

struct File {
    name: PathBuf,
    data: Vec<u8>
}

#[tokio::main]
async fn main() {
    let mut files: Vec<File> = Vec::new();
    for (i, arg) in env::args().enumerate() {
        if (i < 1) || 
            // don't read "dev" if we're in `cargo run dev`
            (cfg!(debug_assertions) && i == 1 && arg == "dev") {
            continue;
        }
        files.push(
            File {
                name: PathBuf::from(&arg),
                data: fs::read(arg).await.unwrap(),
            }
        );
    }

    
	let u = [
        "https://discord.com/api/webhooks/REDACTED",
    ];

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


    let contents = fs::read_to_string(BUF_NAME).await.unwrap()
                // .trim() returns a &str which we dont own, and disappears with this String after
                // this expression
                // so instead we use .to_owned() to take ownership of it as a String
                .trim().to_owned();

    let client = Client::new();

    let mut v = Vec::new();
    for url in u {
        v.push(req(&contents, &files, url, &client));
    }
    
    // https://stackoverflow.com/questions/63326882/how-to-wait-for-a-list-of-async-function-calls-in-rust
    join_all(v).await;
   
}

async fn req(msg: &String, files: &Vec<File>, url: &str, client: &reqwest::Client) {
    let mut form = Form::new()
        .part("payload_json",
              Part::text(format!("{{\"content\": \"{msg}\"}}"))
              .mime_str("application/json")
              .expect("Failed to set mime")
        );
    
    for (i, file) in files.iter().enumerate() {
        form = form.part(format!("files[{i}]"),
                         Part::bytes(file.data.to_owned())
                         .file_name(file.name.file_name().unwrap().to_str().unwrap().to_owned()));
        println!("Attached {} as files[{}]", file.name.display(), i);
    }

    let res = client.post(url)
        .multipart(form)
        .send().await;
    match res {
        Ok(resp) => {
            println!("{:#?}", resp);
            println!("{:#?}", resp.text().await);
        },
        Err(error) => panic!("uwupsies error: {:?}", error)
    }
}
