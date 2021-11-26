extern crate clap;

use std::error::Error;
use clap::{App, Arg, SubCommand};
use env_logger::*;
use log::{error};

mod img;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

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
        // .arg(Arg::with_name("config")
        //     .short("c")
        //     .long("config")
        //     .value_name("FILE")
        //     .help("Sets a custom config file")
        //     .takes_value(true))
        // .arg(Arg::with_name("output")
        //     .help("Sets an optional output file")
        //     .index(1))
        // .arg(Arg::with_name("debug")
        //     .short("d")
        //     .multiple(true)
        //     .help("Turn debugging information on"))
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

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("img") {
        if let Some(matches) = matches.subcommand_matches("downloader") {
            match matches.value_of("url") {
                None => {}
                Some(url) => {
                    img::downloader::download_page_images(url, Option::None).await?
                }
            }
        }
    }
    Ok(())
}