mod internal;

use samp::{
    native,
    prelude::{Amx, AmxResult, AmxString},
};

use self::internal::request_get;

impl super::Rustquest {
    #[native(name = "Request_Get")]
    pub fn request_get(
        &mut self,
        _amx: &Amx,
        url: AmxString,
        callback: AmxString,
    ) -> AmxResult<i32> {
        let request_id = self.get_request_id();
        let url = url.to_string();
        let callback = callback.to_string();
        let sender = self.request_sender.clone();
        self.threads.execute(move || {
            request_get(sender, request_id, url, callback);
        });
        Ok(request_id)
    }

    fn get_request_id(&mut self) -> i32 {
        self.request_count += 1;
        self.request_count
    }
}
