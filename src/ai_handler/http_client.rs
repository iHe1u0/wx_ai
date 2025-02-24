use lazy_static::lazy_static;
use reqwest::Client;
use std::sync::Mutex;

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = Mutex::new(Client::new());
}
