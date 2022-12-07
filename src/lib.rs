// mod internal;
// mod plugin;
mod jsonmod;
use std::collections::HashMap;

// use plugin::Rustquest;
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

impl Rustquest {
    // #[native(name = "Request")]
    // fn request(&mut self, amx: &Amx, mut args: samp::args::Args) -> AmxResult<bool> {
    //     let url = args
    //         .next::<AmxString>()
    //         .ok_or(AmxError::Params)?
    //         .to_string();
    // }
}

initialize_plugin!(
    natives: [
        Rustquest::create_json,
        Rustquest::json_get_int,
        Rustquest::json_set_int,
        // RustquestJson::create_json,
        // RustquestJson::json_get_int,
        // RustquestJson::json_set_int,
    ], {
        Rustquest {
            jsons: HashMap::new(),
        }
    }
);

// fn print_type<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
