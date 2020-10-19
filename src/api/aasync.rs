use crate::api::{Api, TdType};
use crate::observer;
use rtdlib::types::*;
use rtdlib::errors::{RTDResult, RTDError};
use futures::StreamExt;

pub struct AsyncApi {
  api: Api,
}

impl AsyncApi {
  pub fn new(api: Api) -> Self {
    Self { api }
  }

  #[doc(hidden)]
  pub fn api(&self) -> &Api {
    &self.api
  }


  pub async fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> RTDResult<Chat> {
    let mut rec = observer::subscribe(get_chat.as_ref().extra().to_string());
    self.api.send(get_chat.as_ref());
    let chat = rec.next().await.unwrap();
    observer::unsubscribe(get_chat.as_ref().extra());
    match chat {
      TdType::Chat(chat) => {Ok(chat)}
      _ => {Err(RTDError::Custom("invalid type"))}
    }
  }

}

