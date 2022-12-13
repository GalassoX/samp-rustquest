mod internal;

use log::error;
use samp::{
    native,
    prelude::{Amx, AmxResult, AmxString},
};

use self::internal::{request_get, request_post};

impl super::Rustquest {
    #[native(name = "Request")]
    pub fn request(
        &mut self,
        _amx: &Amx,
        url: AmxString,
        callback: AmxString,
        method: AmxString,
        data: AmxString,
    ) -> AmxResult<i32> {
        let request_id = self.get_request_id();
        let url = url.to_string();
        let callback = callback.to_string();
        let sender = self.request_sender.clone();
        let data = data.to_string();
        match method.to_string().to_lowercase().as_str() {
            "get" => {
                self.threads
                    .execute(move || request_get(sender, request_id, url, callback));
                Ok(request_id)
            }
            "post" => {
                self.threads
                    .execute(move || request_post(sender, request_id, url, callback, data));
                Ok(request_id)
            }
            _ => {
                error!("Method {} invalid", method.to_string());
                Ok(0)
            }
        }
    }

    fn get_request_id(&mut self) -> i32 {
        self.request_count += 1;
        self.request_count
    }
}
