mod jsonmod;
use std::collections::HashMap;

use json::*;
use samp::{initialize_plugin, prelude::SampPlugin};
pub struct Rustquest {
    jsons: HashMap<i32, JsonValue>,
}

impl SampPlugin for Rustquest {
    fn on_load(&mut self) {
        println!("Version: 0.1.0");
    }
}

initialize_plugin!(
    natives: [
        Rustquest::create_json,
        Rustquest::json_get_int,
        Rustquest::json_set_int,
        Rustquest::json_get_str,
        Rustquest::json_set_str
    ], {
        Rustquest {
            jsons: HashMap::new(),
        }
    }
);
