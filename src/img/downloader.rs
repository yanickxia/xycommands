use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::Bytes;
use reqwest::get;

use tokio::io::AsyncReadExt;
use html_parser::{Dom, Node};
use log::{debug, info};
use url::{Url};
use urlencoding::decode;


pub async fn download_page_images(url: &str, path: Option<&str>) -> Result<(), Box<dyn Error>> {
    let body = reqwest::get(url).await?
        .text().await?;
    let dom = Dom::parse(body.as_str())?;

    let iter = dom.children.get(0).unwrap().into_iter();

    let hrefs = iter.filter_map(|item| match item {
        Node::Element(ref element) if element.name == "img" => element.attributes["src"].clone(),
        _ => None,
    });

    let parsed_url = Url::parse(url)?;
    let base_url = parsed_url.scheme().to_owned() + "://" + parsed_url.host_str().unwrap();
    let mut download_urls: Vec<String> = Vec::new();

    debug!("\nThe following links where found:");
    for (index, href) in hrefs.enumerate() {
        let download_url = (base_url.clone() + href.as_str());
        debug!("{}: {}", index + 1, download_url.as_str());
        download_urls.push(download_url);
    }

    let mut save_path = "";
    let collected = parsed_url.path_segments().unwrap().collect::<Vec<&str>>();
    if path.is_none() {
        save_path = collected[collected.len() - 1];
    }
    let decoded = decode(save_path).expect("UTF-8").to_string() + "/";
    debug!("save path: {}", decoded);
    let save_path = Path::new(decoded.as_str());
    if !save_path.exists() {
        std::fs::create_dir(save_path).unwrap();
    }

    for item in download_urls {
        info!("download img {}", item);
        let bytes = reqwest::get(url).await?.bytes().await?;

        // get filename
        let image_url = Url::parse(item.as_str())?;
        let collected = image_url.path_segments().unwrap().collect::<Vec<&str>>();
        let file_name = collected[collected.len() - 1];


        let save_path = (decoded.to_string() + "/" + file_name);
        let path = Path::new(save_path.as_str());
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(mut file) => {
                file.write_all(bytes.as_ref())?
            }
        };
    }

    Ok(())
}
