use json::*;
use samp::{
    native,
    prelude::{Amx, AmxResult, AmxString, Ref},
};

impl super::Rustquest {
    #[native(name = "JSON_Create")]
    pub fn create_json(&mut self, _amx: &Amx, body: AmxString) -> AmxResult<i32> {
        let json = match json::parse(body.to_string().as_str()) {
            Ok(j) => j,
            Err(_) => JsonValue::new_object(),
        };
        let key = self.jsons.len() as i32;
        self.jsons.insert(key, json);
        Ok(key)
    }

    #[native(name = "JSON_GetInt")]
    pub fn json_get_int(
        &mut self,
        _amx: &Amx,
        json_id: usize,
        prop_name: AmxString,
        mut dest: Ref<u32>,
    ) -> AmxResult<bool> {
        match self.jsons.get(&(json_id as i32)) {
            Some(json) => match json[prop_name.to_string().as_str()].as_u32() {
                Some(v) => {
                    *dest = v;
                    Ok(true)
                }
                None => Ok(false),
            },
            None => Ok(false),
        }
    }
    #[native(name = "JSON_SetInt")]
    pub fn json_set_int(
        &mut self,
        _amx: &Amx,
        json_id: usize,
        prop_name: AmxString,
        value: u32,
    ) -> AmxResult<bool> {
        match self.jsons.get(&(json_id as i32)) {
            Some(json) => {
                let mut object = json.to_owned();
                object[prop_name.to_string()] = value.into();
                self.jsons.insert(json_id as i32, object);
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
