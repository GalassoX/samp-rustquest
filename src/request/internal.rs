use log::error;
use std::sync::mpsc::Sender;

pub fn request_get(
    sender: Option<Sender<(i32, String, String)>>,
    request_id: i32,
    url: String,
    callback: String,
) {
    match reqwest::blocking::get(url) {
        Ok(data) => {
            // let _ =
            match data.text() {
                Ok(txt) => {
                    let _ = sender.as_ref().unwrap().send((request_id, callback, txt));
                }
                Err(_) => {
                    error!("{} -> invalid response", callback);
                }
            }
        }
        Err(error) => {
            error!("{} -> {:?}", callback, error);
        }
    }
}

pub fn request_post(
    sender: Option<Sender<(i32, String, String)>>,
    request_id: i32,
    url: String,
    callback: String,
    body: String,
) {
    let client = reqwest::blocking::Client::new();
    match client.post(&url).body(body).send() {
        Ok(data) => match data.text() {
            Ok(txt) => {
                let _ = sender.as_ref().unwrap().send((request_id, callback, txt));
            }
            Err(_) => {
                error!("{} : {} -> invalid response", callback, url);
            }
        },
        Err(error) => {
            error!("{} -> {:?}", callback, error);
        }
    }
}
