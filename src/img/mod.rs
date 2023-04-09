pub mod downloader;
pub mod arg;


#[derive(PartialEq)]
pub enum State {
    RUNNING,
    STOP,
}