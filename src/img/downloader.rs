use std::borrow::BorrowMut;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use html_parser::{Dom, Node};
use image::GenericImageView;
use url::Url;
use urlencoding::decode;

use crate::img::State;
use crate::ui::display;

pub struct Downloader {
    app: display::App,
    state: State,
}

impl Downloader {
    pub fn new(app: display::App) -> Self {
        Downloader { app, state: State::RUNNING }
    }

    pub async fn download_page_images(&mut self, url: &str, path: Option<&str>) -> Result<(), Box<dyn Error>> {
        let body = reqwest::get(url).await?
            .text().await?;
        let dom = Dom::parse(body.as_str())?;

        let iter = dom.children.get(0).unwrap().into_iter();

        let hrefs = iter.filter_map(|item| match item {
            Node::Element(ref element) if element.name == "img" => {
                if element.attributes.contains_key("src") {
                    return element.attributes["src"].clone();
                }
                None
            }
            _ => None,
        });

        let parsed_url = Url::parse(url)?;
        let base_url = parsed_url.scheme().to_owned() + "://" + parsed_url.host_str().unwrap();
        let mut download_urls: Vec<String> = Vec::new();

        for (_, href) in hrefs.enumerate() {
            let download_url = base_url.clone() + href.as_str();
            download_urls.push(download_url);
        }

        let app = self.app.borrow_mut();
        app.total = download_urls.len();
        app.finish = 0;

        let mut save_path = "";
        let collected = parsed_url.path_segments().unwrap().collect::<Vec<&str>>();
        if path.is_none() {
            save_path = collected[collected.len() - 1];
        }
        let decoded = decode(save_path).expect("UTF-8").to_string() + "/";
        let save_path = Path::new(decoded.as_str());
        if !save_path.exists() {
            std::fs::create_dir(save_path).unwrap();
        }

        let client = reqwest::ClientBuilder::new()
            .gzip(true)
            .build().unwrap();

        for item in download_urls {
            if self.state == State::STOP {
                return Ok(());
            }

            app.append_log(format!("download img {}", item));
            let bytes = client.get(item.as_str()).send().await?.bytes().await?;

            let item_image = image::load_from_memory(&bytes)?;
            let (x, y) = item_image.dimensions();
            if x < 100 || y < 100 {
                continue;
            }

            // get filename
            let image_url = Url::parse(item.as_str())?;
            let collected = image_url.path_segments().unwrap().collect::<Vec<&str>>();
            let file_name = collected[collected.len() - 1];


            let save_path = decoded.to_string() + "/" + file_name;
            let path = Path::new(save_path.as_str());
            let display = path.display();
            let _ = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(mut file) => {
                    file.write_all(bytes.as_ref())?;
                    app.finish += 1;
                    app.display()?;
                }
            };
        }

        Ok(())
    }
}


