extern crate core;

use std::error::Error;

use clap::{Args, Parser, Subcommand};
use log::error;

mod ui;
mod img;

#[derive(Subcommand)]
enum Commands {
    Image(img::arg::ImageArg)
}


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {}
        Err(err) => {
            error!("catch error {}",err.to_string())
        }
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();



    match &cli.command {
        Commands::Image(args) => {
            let app = ui::display::App::new(100)?;
            let mut downloader = img::downloader::Downloader::new(app);
            let opt = img::downloader::DownloaderOption {
                url: args.url.clone(),
                concurrent: args.concurrent,
                path: Some(args.path.clone()),
            };
            downloader.download_page_images(opt).await?
        }
    }
    Ok(())
}