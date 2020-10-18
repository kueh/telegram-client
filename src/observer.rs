
use std::sync::{RwLock};
use futures::channel::mpsc;
use std::collections::HashMap;
use crate::api::TdType;
use futures::SinkExt;

lazy_static! {
    static ref OBSERVER: Observer = {
      Observer::new()
    };
  }

struct Observer {
  channels: RwLock<HashMap<String, mpsc::Sender<TdType>>>,
}

impl Observer {
  fn new() -> Self {
    Self{
      channels: RwLock::new(HashMap::new())
    }
  }
  async fn notify(&self, extra: String, payload: TdType) {
      let mut map = self.channels.write().unwrap();
      let mut sender = map.get_mut(&extra).unwrap();
      sender.send(payload).await;
  }

  fn subscribe(&self, extra: String) -> mpsc::Receiver<TdType>{

    let (sender, mut receiver) = mpsc::channel::<TdType>(1);
        match self.channels.write() {
            Ok(mut map) => {
                map.insert(extra, sender);
            }
            _ => {}
        };
    receiver
  }

  fn unsubscribe(&self, extra: &str) {

    match self.channels.write() {
            Ok(mut map) => {
                map.remove(extra);
            }
            _ => {}
        };
  }
}


pub async fn notify(extra: String, payload: TdType) {
  OBSERVER.notify(extra, payload).await
}

pub fn subscribe(extra: String) -> mpsc::Receiver<TdType>{
  OBSERVER.subscribe(extra)
}

pub fn unsubscribe(extra: &str) {
  OBSERVER.unsubscribe(extra)
}
