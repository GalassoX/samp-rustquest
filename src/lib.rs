mod jsonmod;
mod request;
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
};

use json::*;
use log::error;
use samp::{
    amx::AmxIdent,
    exec_public, initialize_plugin,
    prelude::{AmxExt, SampPlugin},
};
use threadpool::ThreadPool;
pub struct Rustquest {
    jsons: HashMap<i32, JsonValue>,
    request_sender: Option<Sender<(i32, String, String)>>,
    request_receiver: Option<Receiver<(i32, String, String)>>,
    request_count: i32,
    threads: ThreadPool,
    amx_list: Vec<AmxIdent>,
}

impl SampPlugin for Rustquest {
    fn on_load(&mut self) {
        println!("Version: 0.2.0");

        let (request_sender, request_receiver) = channel();
        self.request_sender = Some(request_sender);
        self.request_receiver = Some(request_receiver);
        self.request_count = 0;
    }

    fn on_amx_load(&mut self, amx: &samp::prelude::Amx) {
        self.amx_list.push(amx.ident());
    }

    fn on_amx_unload(&mut self, amx: &samp::prelude::Amx) {
        self.amx_list.remove(
            self.amx_list
                .iter()
                .position(|a| *a == amx.ident())
                .unwrap(),
        );
    }

    fn process_tick(&mut self) {
        for (request_id, callback, data) in self.request_receiver.as_ref().unwrap().try_iter() {
            let mut executed = false;

            for amx in &self.amx_list {
                if let Some(amx) = samp::amx::get(*amx) {
                    if let Ok(_) = amx.find_public(&callback) {
                        let allocator = amx.allocator();
                        let amx_str = allocator.allot_string(data.as_str()).unwrap();
                        let _ = exec_public!(amx, &callback, request_id, amx_str);
                        executed = true;
                    }
                }
            }
            if !executed {
                error!("Cannot execute {:?} callback", callback);
            }
        }
    }
}

initialize_plugin!(
    natives: [
        Rustquest::create_json,
        Rustquest::json_get_int,
        Rustquest::json_set_int,
        Rustquest::json_get_str,
        Rustquest::json_set_str,
        Rustquest::request_get,
        Rustquest::json_create_arr,
        Rustquest::json_arr_to_str,
        Rustquest::json_arr_add_int,
        Rustquest::json_arr_add_float,
        Rustquest::json_arr_add_str,
        Rustquest::json_arr_is_int,
        Rustquest::json_arr_is_str,
        Rustquest::json_arr_get_int,
        Rustquest::json_arr_get_float,
        Rustquest::json_arr_get_str
    ], {
        samp::plugin::enable_process_tick();

        let samp_logger = samp::plugin::logger().level(log::LevelFilter::Info);
        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!(
                    "[SampRusquest - {}]: {}",
                    record.level().to_string().to_lowercase(),
                    message
                ))
            })
            .chain::<fern::Dispatch>(samp_logger)
            .apply();

        Rustquest {
            jsons: HashMap::new(),
            request_sender: None,
            request_receiver: None,
            request_count: 0,
            threads: ThreadPool::new(2),
            amx_list: Vec::new()
        }
    }
);
