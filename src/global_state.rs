use tokio::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEMP: Mutex<u32> = Mutex::new(0);
}
