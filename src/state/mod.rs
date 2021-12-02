use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

lazy_static! {
    /// This is an example for using doc comment attributes
    pub static ref RUNNING: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
}