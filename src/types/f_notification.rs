use rtdlib::types::RObject;

use crate::errors;
use crate::types::*;

impl TGScopeNotificationSettings {


  /// Time left before notifications will be unmuted, in seconds.
  pub fn mute_for(&self) -> i32 { self.td_origin().mute_for().expect(errors::TELEGRAM_DATA_FAIL) }

  /// The name of an audio file to be used for notification sounds; only applies to iOS applications.
  pub fn sound(&self) -> String { self.td_origin().sound().expect(errors::TELEGRAM_DATA_FAIL) }

  /// True, if message content should be displayed in notifications.
  pub fn show_preview(&self) -> bool { self.td_origin().show_preview().expect(errors::TELEGRAM_DATA_FAIL) }

  /// True, if notifications for incoming pinned messages will be created as for an ordinary unread message.
  pub fn disable_pinned_message_notifications(&self) -> bool { self.td_origin().disable_pinned_message_notifications().expect(errors::TELEGRAM_DATA_FAIL) }

  /// True, if notifications for messages with mentions will be created as for an ordinary unread message.
  pub fn disable_mention_notifications(&self) -> bool { self.td_origin().disable_mention_notifications().expect(errors::TELEGRAM_DATA_FAIL) }


}



impl TGUpdateScopeNotificationSettings {
  pub fn scope(&self) -> TGNotificationSettingsScope { TGNotificationSettingsScope::of(self.td_origin().scope().expect(errors::TELEGRAM_DATA_FAIL)) }

  pub fn notification_settings(&self) -> TGScopeNotificationSettings {
    TGScopeNotificationSettings::from_json(self.td_origin().notification_settings().expect(errors::TELEGRAM_DATA_FAIL).to_json())
      .expect(errors::TELEGRAM_DATA_FAIL)
  }
}

