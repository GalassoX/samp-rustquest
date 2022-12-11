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
