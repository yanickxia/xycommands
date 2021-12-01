pub mod downloader;


#[derive(PartialEq)]
pub enum State {
    RUNNING,
    STOP,
}