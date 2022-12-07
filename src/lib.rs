// mod internal;
// mod plugin;
use std::collections::HashMap;

// use plugin::Rustquest;
use json::*;
use samp::{
    initialize_plugin, native,
    prelude::{Amx, AmxResult, AmxString, SampPlugin},
};

pub struct Rustquest {
    jsons: HashMap<i32, JsonValue>,
}

impl SampPlugin for Rustquest {
    fn on_load(&mut self) {
        // info!("Version: 0.1.0")
        println!("Version: 0.1.0");
    }
}
// use std::collections::HashMap;
// use std::collections::LinkedList;

// use samp::args;
// use samp::error::AmxError;
// use samp::native;
// use samp::prelude::*;

impl Rustquest {
    // #[native(name = "Request")]
    // fn request(&mut self, amx: &Amx, mut args: samp::args::Args) -> AmxResult<bool> {
    //     let url = args
    //         .next::<AmxString>()
    //         .ok_or(AmxError::Params)?
    //         .to_string();
    // }

    #[native(name = "JSON_Create")]
    fn create_json(&mut self, amx: &Amx) -> AmxResult<i32> {
        let json = JsonValue::new_object();
        let key = self.jsons.len() as i32;
        self.jsons.insert(key, json);
        Ok(key)
    }

    #[native(name = "JSON_GetInt")]
    fn get_json(
        &mut self,
        amx: &Amx,
        json_id: usize,
        prop_name: AmxString,
        mut dest: usize,
    ) -> AmxResult<bool> {
        match self.jsons.get(&(json_id as i32)) {
            Some(json) => match json[prop_name.to_string().as_str()].as_usize() {
                Some(v) => {
                    dest = v;
                    Ok(true)
                }
                None => Ok(false),
            },
            None => Ok(false),
        }
    }
}

initialize_plugin!(
    natives: [
        Rustquest::create_json,
        Rustquest::get_json
    ], {
        Rustquest {
            jsons: HashMap::new(),
        }
    }
);

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
