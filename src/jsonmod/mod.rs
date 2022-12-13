use json::*;
use samp::{
    native,
    prelude::{Amx, AmxResult, AmxString, Ref, UnsizedBuffer},
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
        key: AmxString,
        mut dest: Ref<u32>,
    ) -> AmxResult<bool> {
        match self.jsons.get(&(json_id as i32)) {
            Some(json) => match json[key.to_string().as_str()].as_u32() {
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
        key: AmxString,
        value: u32,
    ) -> AmxResult<bool> {
        match self.jsons.get(&(json_id as i32)) {
            Some(json) => {
                let mut object = json.to_owned();
                object[key.to_string()] = value.into();
                self.jsons.insert(json_id as i32, object);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    #[native(name = "JSON_GetString")]
    pub fn json_get_str(
        &mut self,
        _amx: &Amx,
        json_id: usize,
        key: AmxString,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        match self.jsons.get(&(json_id as i32)) {
            Some(json) => match json[key.to_string().as_str()].as_str() {
                Some(v) => {
                    let mut buffer = dest.into_sized_buffer(size);
                    let _ = samp::cell::string::put_in_buffer(&mut buffer, v);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => Ok(false),
        }
    }

    #[native(name = "JSON_SetString")]
    pub fn json_set_str(
        &mut self,
        _amx: &Amx,
        json_id: usize,
        key: AmxString,
        value: AmxString,
    ) -> AmxResult<bool> {
        match self.jsons.get(&(json_id as i32)) {
            Some(json) => {
                let mut object = json.to_owned();
                object[key.to_string()] = value.to_string().into();
                self.jsons.insert(json_id as i32, object);
                Ok(true)
            }
            None => Ok(false),
        }
    }

    #[native(name = "JSON_CreateArray")]
    pub fn json_create_arr(&mut self, _amx: &Amx, source: AmxString) -> AmxResult<i32> {
        let json = match json::parse(source.to_string().as_str()) {
            Ok(j) => j,
            Err(_) => JsonValue::new_array(),
        };
        if !json.is_array() {
            return Ok(0);
        }
        let key = self.jsons.len() as i32;
        self.jsons.insert(key, json);
        Ok(key)
    }
    #[native(name = "JSON_ArrayStringify")]
    pub fn json_arr_to_str(
        &mut self,
        _amx: &Amx,
        arr_id: i32,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => {
                if json.is_array() {
                    let mut buffer = dest.into_sized_buffer(size);
                    let str = json::stringify(json.to_owned());
                    let _ = samp::cell::string::put_in_buffer(&mut buffer, &str);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }

    #[native(name = "JSON_ArrayAddInt")]
    pub fn json_arr_add_int(&mut self, _amx: &Amx, arr_id: i32, value: u32) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => {
                if json.is_array() {
                    let mut object = json.to_owned();
                    let _ = object.push(value);
                    self.jsons.insert(arr_id, object);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }
    #[native(name = "JSON_ArrayAddFloat")]
    pub fn json_arr_add_float(&mut self, _amx: &Amx, arr_id: i32, value: f32) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => {
                if json.is_array() {
                    let mut object = json.to_owned();
                    let _ = object.push(value);
                    self.jsons.insert(arr_id, object);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }
    #[native(name = "JSON_ArrayAddString")]
    pub fn json_arr_add_str(
        &mut self,
        _amx: &Amx,
        arr_id: i32,
        value: AmxString,
    ) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => {
                if json.is_array() {
                    let mut object = json.to_owned();
                    let _ = object.push(value.to_string());
                    self.jsons.insert(arr_id, object);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }

    #[native(name = "JSON_ArrayIsNumber")]
    pub fn json_arr_is_int(&mut self, _amx: &Amx, arr_id: i32, index: i32) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => Ok(json.is_array() && json[index as usize].is_number()),
            None => Ok(false),
        }
    }
    #[native(name = "JSON_ArrayIsString")]
    pub fn json_arr_is_str(&mut self, _amx: &Amx, arr_id: i32, index: i32) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => Ok(json.is_array() && json[index as usize].is_string()),
            None => Ok(false),
        }
    }

    #[native(name = "JSON_ArrayGetInt")]
    pub fn json_arr_get_int(
        &mut self,
        _amx: &Amx,
        arr_id: i32,
        index: i32,
        mut dest: Ref<u32>,
    ) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => {
                if json.is_array() {
                    match json[index as usize].as_u32() {
                        Some(v) => {
                            *dest = v;
                            Ok(true)
                        }
                        None => Ok(false),
                    }
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }
    #[native(name = "JSON_ArrayGetFloat")]
    pub fn json_arr_get_float(
        &mut self,
        _amx: &Amx,
        arr_id: i32,
        index: i32,
        mut dest: Ref<f32>,
    ) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => {
                if json.is_array() {
                    match json[index as usize].as_f32() {
                        Some(v) => {
                            *dest = v;
                            Ok(true)
                        }
                        None => Ok(false),
                    }
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }
    #[native(name = "JSON_ArrayGetString")]
    pub fn json_arr_get_str(
        &mut self,
        _amx: &Amx,
        arr_id: i32,
        index: i32,
        dest: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        match self.jsons.get(&arr_id) {
            Some(json) => match json[index as usize].as_str() {
                Some(v) => {
                    let mut buffer = dest.into_sized_buffer(size);
                    let _ = samp::cell::string::put_in_buffer(&mut buffer, v);
                    Ok(true)
                }
                None => Ok(false),
            },
            None => Ok(false),
        }
    }
}
