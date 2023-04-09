use std::borrow::BorrowMut;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::mpsc::channel;

use html_parser::{Dom, Node};
use image::GenericImageView;
use url::Url;
use urlencoding::decode;
use serde::Deserialize;
use rdev::{listen, Event, EventType, Key};
use tokio::sync::Mutex;

use crate::img::State;
use crate::ui::display;

pub struct Downloader {
    app: display::App,
    state: Arc<Mutex<State>>,
    client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
pub struct Proxy {
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
}

pub struct DownloaderOption {
    pub url: String,
    pub concurrent: u16,
    pub path: Option<String>,
}

impl Downloader {
    pub fn new(app: display::App) -> Self {
        let proxy = match envy::from_env::<Proxy>() {
            Ok(config) => config,
            Err(error) => panic!("{:#?}", error)
        };
        let mut client_builder = reqwest::Client::builder().gzip(true);
        if proxy.http_proxy.is_some() {
            client_builder = client_builder.proxy(reqwest::Proxy::http(proxy.http_proxy.unwrap()).unwrap())
        }

        if proxy.https_proxy.is_some() {
            client_builder = client_builder.proxy(reqwest::Proxy::https(proxy.https_proxy.unwrap()).unwrap())
        }

        let init_status = Arc::new(Mutex::new(State::RUNNING));
        let (tx, rx) = channel();

        tokio::spawn(async {
            let arc = init_status.clone();
            let result = rx.recv().unwrap();
            *arc.lock().await = State::STOP;
        });

        listen(|event| {
            match event.event_type {
                EventType::KeyRelease(key) => {
                    match key {
                        Key::KeyQ => {
                            tx.send(1).expect("TODO: panic message");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }).unwrap();

        Downloader { app, state: init_status, client: client_builder.build().unwrap() }
    }

    pub async fn download_page_images(&mut self, opt: DownloaderOption) -> Result<(), Box<dyn Error>> {
        let body = self.client.get(opt.url.as_str())
            .send().await?
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

        let parsed_url = Url::parse(opt.url.as_str())?;
        let base_url = parsed_url.scheme().to_owned() + "://" + parsed_url.host_str().unwrap();
        let mut download_urls: Vec<String> = Vec::new();

        for (_, href) in hrefs.enumerate() {
            let download_url = base_url.clone() + href.as_str();
            download_urls.push(download_url);
        }

        let app = self.app.borrow_mut();
        app.total = download_urls.len();
        app.finish = 0;

        let collected = parsed_url.path_segments().unwrap().collect::<Vec<&str>>();
        let mut save_path = collected[collected.len() - 1].to_string();
        if opt.path.is_some() {
            save_path = opt.path.unwrap() + "/" + collected[collected.len() - 1];
        }
        let decoded = decode(save_path.as_str()).expect("UTF-8").to_string() + "/";
        let save_path = Path::new(decoded.as_str());
        if !save_path.exists() {
            std::fs::create_dir(save_path).unwrap();
        }

        for item in download_urls {
            {
                if *self.state.lock().await == State::STOP {
                    return Ok(());
                }
            }

            app.append_log(format!("download img {}", item));
            let bytes = self.client.get(item.as_str()).send().await?.bytes().await?;

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
                    app.logs.next();
                    app.display()?;
                }
            };
        }

        Ok(())
    }
}


