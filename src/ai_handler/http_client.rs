use lazy_static::lazy_static;
use reqwest::Client;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = Mutex::new(Client::new());
}
