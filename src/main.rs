extern crate clap;

use std::borrow::BorrowMut;
use std::error::Error;
use std::sync::{Arc, mpsc};

use clap::{App, Arg, SubCommand};
use env_logger::*;
use log::{error, info};
use termion::event::Key;

mod ui;
mod img;
mod state;


#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    match run().await {
        Ok(_) => {}
        Err(err) => {
            error!("catch error {}",err.to_string())
        }
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let matches = App::new("xycmd")
        .version("1.0")
        .author("Yanick Xia. <me.yan.xia@qq.com>")
        .about("Does awesome things")
        .subcommand(SubCommand::with_name("img")
            .about("img commands")
            .help("img commands")
            .subcommand(SubCommand::with_name("downloader")
                .arg(Arg::with_name("url")
                    .long("url")
                    .short("u")
                    .takes_value(true)
                    .required(true)
                    .help("image page url")
                )))
        .get_matches();

    let app = ui::display::App::new(100)?;
    let mut downloader = img::downloader::Downloader::new(app);


    let (tx, rx) = mpsc::channel();
    let ctx = tx.clone();
    tokio::spawn(async {
        let mut key = ui::key::KeyBoard::new();
        key.run_background(ctx, rx);
    });

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("img") {
        if let Some(matches) = matches.subcommand_matches("downloader") {
            match matches.value_of("url") {
                None => {}
                Some(url) => {
                    downloader.download_page_images(url, Option::None).await?
                }
            }
        }
    }

    tx.send(Key::Char('q'))?;
    Ok(())
}