use log::error;
use std::sync::mpsc::Sender;

pub fn request_get(
    sender: Option<Sender<(i32, String, u16, String)>>,
    request_id: i32,
    url: String,
    callback: String,
) {
    match reqwest::blocking::get(url) {
        Ok(data) => {
            let status_code = data.status().as_u16();
            match data.text() {
                Ok(txt) => {
                    let _ = sender
                        .as_ref()
                        .unwrap()
                        .send((request_id, callback, status_code, txt));
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

pub fn request_ex(
    method: String,
    sender: Option<Sender<(i32, String, u16, String)>>,
    request_id: i32,
    url: String,
    callback: String,
    body: String,
) {
    let client = reqwest::blocking::Client::new();
    let response = match method.to_lowercase().as_str() {
        "post" => client.post(&url).body(body).send(),
        "put" => client.put(&url).body(body).send(),
        "patch" => client.patch(&url).body(body).send(),
        "delete" => client.delete(&url).body(body).send(),
        _ => panic!("Invalida HTTP method in plugin"),
    };
    match response {
        Ok(data) => {
            let status_code = data.status().as_u16();
            match &data.text() {
                Ok(txt) => {
                    let _ = sender.as_ref().unwrap().send((
                        request_id,
                        callback,
                        status_code,
                        txt.to_owned(),
                    ));
                }
                Err(_) => {
                    error!("{} : {} -> invalid response", callback, url);
                }
            }
        }
        Err(error) => {
            error!("{} -> {:?}", callback, error);
        }
    }
}
