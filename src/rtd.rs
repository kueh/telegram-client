use core::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::api::Api;
use crate::handler::Handler;
use crate::listener::Lout;
use futures::executor::block_on;

pub struct TdRecv {}

impl<'a> TdRecv{
    pub fn new() -> TdRecv {
        Self {}
    }

    pub fn start(
        &self,
        api: Arc<Api>,
        stop_flag: Arc<Mutex<bool>>,
        lout: Arc<Lout>,
    ) -> JoinHandle<()> {
        thread::spawn(move || block_on(async {
            let is_stop = stop_flag.lock().unwrap();
            while !*is_stop {
                if let Some(json) = api.receive(2.0) {
                    Handler::new(api.borrow(), lout.borrow()).handle(&json).await;
                }
            }
        }))
    }
}
