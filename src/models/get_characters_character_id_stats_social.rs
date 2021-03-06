/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdStatsSocial : social object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdStatsSocial {
  /// add_contact_bad integer
  #[serde(rename = "add_contact_bad")]
  add_contact_bad: Option<i64>,
  /// add_contact_good integer
  #[serde(rename = "add_contact_good")]
  add_contact_good: Option<i64>,
  /// add_contact_high integer
  #[serde(rename = "add_contact_high")]
  add_contact_high: Option<i64>,
  /// add_contact_horrible integer
  #[serde(rename = "add_contact_horrible")]
  add_contact_horrible: Option<i64>,
  /// add_contact_neutral integer
  #[serde(rename = "add_contact_neutral")]
  add_contact_neutral: Option<i64>,
  /// add_note integer
  #[serde(rename = "add_note")]
  add_note: Option<i64>,
  /// added_as_contact_bad integer
  #[serde(rename = "added_as_contact_bad")]
  added_as_contact_bad: Option<i64>,
  /// added_as_contact_good integer
  #[serde(rename = "added_as_contact_good")]
  added_as_contact_good: Option<i64>,
  /// added_as_contact_high integer
  #[serde(rename = "added_as_contact_high")]
  added_as_contact_high: Option<i64>,
  /// added_as_contact_horrible integer
  #[serde(rename = "added_as_contact_horrible")]
  added_as_contact_horrible: Option<i64>,
  /// added_as_contact_neutral integer
  #[serde(rename = "added_as_contact_neutral")]
  added_as_contact_neutral: Option<i64>,
  /// calendar_event_created integer
  #[serde(rename = "calendar_event_created")]
  calendar_event_created: Option<i64>,
  /// chat_messages_alliance integer
  #[serde(rename = "chat_messages_alliance")]
  chat_messages_alliance: Option<i64>,
  /// chat_messages_constellation integer
  #[serde(rename = "chat_messages_constellation")]
  chat_messages_constellation: Option<i64>,
  /// chat_messages_corporation integer
  #[serde(rename = "chat_messages_corporation")]
  chat_messages_corporation: Option<i64>,
  /// chat_messages_fleet integer
  #[serde(rename = "chat_messages_fleet")]
  chat_messages_fleet: Option<i64>,
  /// chat_messages_region integer
  #[serde(rename = "chat_messages_region")]
  chat_messages_region: Option<i64>,
  /// chat_messages_solarsystem integer
  #[serde(rename = "chat_messages_solarsystem")]
  chat_messages_solarsystem: Option<i64>,
  /// chat_messages_warfaction integer
  #[serde(rename = "chat_messages_warfaction")]
  chat_messages_warfaction: Option<i64>,
  /// chat_total_message_length integer
  #[serde(rename = "chat_total_message_length")]
  chat_total_message_length: Option<i64>,
  /// direct_trades integer
  #[serde(rename = "direct_trades")]
  direct_trades: Option<i64>,
  /// fleet_broadcasts integer
  #[serde(rename = "fleet_broadcasts")]
  fleet_broadcasts: Option<i64>,
  /// fleet_joins integer
  #[serde(rename = "fleet_joins")]
  fleet_joins: Option<i64>,
  /// mails_received integer
  #[serde(rename = "mails_received")]
  mails_received: Option<i64>,
  /// mails_sent integer
  #[serde(rename = "mails_sent")]
  mails_sent: Option<i64>
}

impl GetCharactersCharacterIdStatsSocial {
  /// social object
  pub fn new() -> GetCharactersCharacterIdStatsSocial {
    GetCharactersCharacterIdStatsSocial {
      add_contact_bad: None,
      add_contact_good: None,
      add_contact_high: None,
      add_contact_horrible: None,
      add_contact_neutral: None,
      add_note: None,
      added_as_contact_bad: None,
      added_as_contact_good: None,
      added_as_contact_high: None,
      added_as_contact_horrible: None,
      added_as_contact_neutral: None,
      calendar_event_created: None,
      chat_messages_alliance: None,
      chat_messages_constellation: None,
      chat_messages_corporation: None,
      chat_messages_fleet: None,
      chat_messages_region: None,
      chat_messages_solarsystem: None,
      chat_messages_warfaction: None,
      chat_total_message_length: None,
      direct_trades: None,
      fleet_broadcasts: None,
      fleet_joins: None,
      mails_received: None,
      mails_sent: None
    }
  }

  pub fn set_add_contact_bad(&mut self, add_contact_bad: i64) {
    self.add_contact_bad = Some(add_contact_bad);
  }

  pub fn with_add_contact_bad(mut self, add_contact_bad: i64) -> GetCharactersCharacterIdStatsSocial {
    self.add_contact_bad = Some(add_contact_bad);
    self
  }

  pub fn add_contact_bad(&self) -> Option<&i64> {
    self.add_contact_bad.as_ref()
  }

  pub fn reset_add_contact_bad(&mut self) {
    self.add_contact_bad = None;
  }

  pub fn set_add_contact_good(&mut self, add_contact_good: i64) {
    self.add_contact_good = Some(add_contact_good);
  }

  pub fn with_add_contact_good(mut self, add_contact_good: i64) -> GetCharactersCharacterIdStatsSocial {
    self.add_contact_good = Some(add_contact_good);
    self
  }

  pub fn add_contact_good(&self) -> Option<&i64> {
    self.add_contact_good.as_ref()
  }

  pub fn reset_add_contact_good(&mut self) {
    self.add_contact_good = None;
  }

  pub fn set_add_contact_high(&mut self, add_contact_high: i64) {
    self.add_contact_high = Some(add_contact_high);
  }

  pub fn with_add_contact_high(mut self, add_contact_high: i64) -> GetCharactersCharacterIdStatsSocial {
    self.add_contact_high = Some(add_contact_high);
    self
  }

  pub fn add_contact_high(&self) -> Option<&i64> {
    self.add_contact_high.as_ref()
  }

  pub fn reset_add_contact_high(&mut self) {
    self.add_contact_high = None;
  }

  pub fn set_add_contact_horrible(&mut self, add_contact_horrible: i64) {
    self.add_contact_horrible = Some(add_contact_horrible);
  }

  pub fn with_add_contact_horrible(mut self, add_contact_horrible: i64) -> GetCharactersCharacterIdStatsSocial {
    self.add_contact_horrible = Some(add_contact_horrible);
    self
  }

  pub fn add_contact_horrible(&self) -> Option<&i64> {
    self.add_contact_horrible.as_ref()
  }

  pub fn reset_add_contact_horrible(&mut self) {
    self.add_contact_horrible = None;
  }

  pub fn set_add_contact_neutral(&mut self, add_contact_neutral: i64) {
    self.add_contact_neutral = Some(add_contact_neutral);
  }

  pub fn with_add_contact_neutral(mut self, add_contact_neutral: i64) -> GetCharactersCharacterIdStatsSocial {
    self.add_contact_neutral = Some(add_contact_neutral);
    self
  }

  pub fn add_contact_neutral(&self) -> Option<&i64> {
    self.add_contact_neutral.as_ref()
  }

  pub fn reset_add_contact_neutral(&mut self) {
    self.add_contact_neutral = None;
  }

  pub fn set_add_note(&mut self, add_note: i64) {
    self.add_note = Some(add_note);
  }

  pub fn with_add_note(mut self, add_note: i64) -> GetCharactersCharacterIdStatsSocial {
    self.add_note = Some(add_note);
    self
  }

  pub fn add_note(&self) -> Option<&i64> {
    self.add_note.as_ref()
  }

  pub fn reset_add_note(&mut self) {
    self.add_note = None;
  }

  pub fn set_added_as_contact_bad(&mut self, added_as_contact_bad: i64) {
    self.added_as_contact_bad = Some(added_as_contact_bad);
  }

  pub fn with_added_as_contact_bad(mut self, added_as_contact_bad: i64) -> GetCharactersCharacterIdStatsSocial {
    self.added_as_contact_bad = Some(added_as_contact_bad);
    self
  }

  pub fn added_as_contact_bad(&self) -> Option<&i64> {
    self.added_as_contact_bad.as_ref()
  }

  pub fn reset_added_as_contact_bad(&mut self) {
    self.added_as_contact_bad = None;
  }

  pub fn set_added_as_contact_good(&mut self, added_as_contact_good: i64) {
    self.added_as_contact_good = Some(added_as_contact_good);
  }

  pub fn with_added_as_contact_good(mut self, added_as_contact_good: i64) -> GetCharactersCharacterIdStatsSocial {
    self.added_as_contact_good = Some(added_as_contact_good);
    self
  }

  pub fn added_as_contact_good(&self) -> Option<&i64> {
    self.added_as_contact_good.as_ref()
  }

  pub fn reset_added_as_contact_good(&mut self) {
    self.added_as_contact_good = None;
  }

  pub fn set_added_as_contact_high(&mut self, added_as_contact_high: i64) {
    self.added_as_contact_high = Some(added_as_contact_high);
  }

  pub fn with_added_as_contact_high(mut self, added_as_contact_high: i64) -> GetCharactersCharacterIdStatsSocial {
    self.added_as_contact_high = Some(added_as_contact_high);
    self
  }

  pub fn added_as_contact_high(&self) -> Option<&i64> {
    self.added_as_contact_high.as_ref()
  }

  pub fn reset_added_as_contact_high(&mut self) {
    self.added_as_contact_high = None;
  }

  pub fn set_added_as_contact_horrible(&mut self, added_as_contact_horrible: i64) {
    self.added_as_contact_horrible = Some(added_as_contact_horrible);
  }

  pub fn with_added_as_contact_horrible(mut self, added_as_contact_horrible: i64) -> GetCharactersCharacterIdStatsSocial {
    self.added_as_contact_horrible = Some(added_as_contact_horrible);
    self
  }

  pub fn added_as_contact_horrible(&self) -> Option<&i64> {
    self.added_as_contact_horrible.as_ref()
  }

  pub fn reset_added_as_contact_horrible(&mut self) {
    self.added_as_contact_horrible = None;
  }

  pub fn set_added_as_contact_neutral(&mut self, added_as_contact_neutral: i64) {
    self.added_as_contact_neutral = Some(added_as_contact_neutral);
  }

  pub fn with_added_as_contact_neutral(mut self, added_as_contact_neutral: i64) -> GetCharactersCharacterIdStatsSocial {
    self.added_as_contact_neutral = Some(added_as_contact_neutral);
    self
  }

  pub fn added_as_contact_neutral(&self) -> Option<&i64> {
    self.added_as_contact_neutral.as_ref()
  }

  pub fn reset_added_as_contact_neutral(&mut self) {
    self.added_as_contact_neutral = None;
  }

  pub fn set_calendar_event_created(&mut self, calendar_event_created: i64) {
    self.calendar_event_created = Some(calendar_event_created);
  }

  pub fn with_calendar_event_created(mut self, calendar_event_created: i64) -> GetCharactersCharacterIdStatsSocial {
    self.calendar_event_created = Some(calendar_event_created);
    self
  }

  pub fn calendar_event_created(&self) -> Option<&i64> {
    self.calendar_event_created.as_ref()
  }

  pub fn reset_calendar_event_created(&mut self) {
    self.calendar_event_created = None;
  }

  pub fn set_chat_messages_alliance(&mut self, chat_messages_alliance: i64) {
    self.chat_messages_alliance = Some(chat_messages_alliance);
  }

  pub fn with_chat_messages_alliance(mut self, chat_messages_alliance: i64) -> GetCharactersCharacterIdStatsSocial {
    self.chat_messages_alliance = Some(chat_messages_alliance);
    self
  }

  pub fn chat_messages_alliance(&self) -> Option<&i64> {
    self.chat_messages_alliance.as_ref()
  }

  pub fn reset_chat_messages_alliance(&mut self) {
    self.chat_messages_alliance = None;
  }

  pub fn set_chat_messages_constellation(&mut self, chat_messages_constellation: i64) {
    self.chat_messages_constellation = Some(chat_messages_constellation);
  }

  pub fn with_chat_messages_constellation(mut self, chat_messages_constellation: i64) -> GetCharactersCharacterIdStatsSocial {
    self.chat_messages_constellation = Some(chat_messages_constellation);
    self
  }

  pub fn chat_messages_constellation(&self) -> Option<&i64> {
    self.chat_messages_constellation.as_ref()
  }

  pub fn reset_chat_messages_constellation(&mut self) {
    self.chat_messages_constellation = None;
  }

  pub fn set_chat_messages_corporation(&mut self, chat_messages_corporation: i64) {
    self.chat_messages_corporation = Some(chat_messages_corporation);
  }

  pub fn with_chat_messages_corporation(mut self, chat_messages_corporation: i64) -> GetCharactersCharacterIdStatsSocial {
    self.chat_messages_corporation = Some(chat_messages_corporation);
    self
  }

  pub fn chat_messages_corporation(&self) -> Option<&i64> {
    self.chat_messages_corporation.as_ref()
  }

  pub fn reset_chat_messages_corporation(&mut self) {
    self.chat_messages_corporation = None;
  }

  pub fn set_chat_messages_fleet(&mut self, chat_messages_fleet: i64) {
    self.chat_messages_fleet = Some(chat_messages_fleet);
  }

  pub fn with_chat_messages_fleet(mut self, chat_messages_fleet: i64) -> GetCharactersCharacterIdStatsSocial {
    self.chat_messages_fleet = Some(chat_messages_fleet);
    self
  }

  pub fn chat_messages_fleet(&self) -> Option<&i64> {
    self.chat_messages_fleet.as_ref()
  }

  pub fn reset_chat_messages_fleet(&mut self) {
    self.chat_messages_fleet = None;
  }

  pub fn set_chat_messages_region(&mut self, chat_messages_region: i64) {
    self.chat_messages_region = Some(chat_messages_region);
  }

  pub fn with_chat_messages_region(mut self, chat_messages_region: i64) -> GetCharactersCharacterIdStatsSocial {
    self.chat_messages_region = Some(chat_messages_region);
    self
  }

  pub fn chat_messages_region(&self) -> Option<&i64> {
    self.chat_messages_region.as_ref()
  }

  pub fn reset_chat_messages_region(&mut self) {
    self.chat_messages_region = None;
  }

  pub fn set_chat_messages_solarsystem(&mut self, chat_messages_solarsystem: i64) {
    self.chat_messages_solarsystem = Some(chat_messages_solarsystem);
  }

  pub fn with_chat_messages_solarsystem(mut self, chat_messages_solarsystem: i64) -> GetCharactersCharacterIdStatsSocial {
    self.chat_messages_solarsystem = Some(chat_messages_solarsystem);
    self
  }

  pub fn chat_messages_solarsystem(&self) -> Option<&i64> {
    self.chat_messages_solarsystem.as_ref()
  }

  pub fn reset_chat_messages_solarsystem(&mut self) {
    self.chat_messages_solarsystem = None;
  }

  pub fn set_chat_messages_warfaction(&mut self, chat_messages_warfaction: i64) {
    self.chat_messages_warfaction = Some(chat_messages_warfaction);
  }

  pub fn with_chat_messages_warfaction(mut self, chat_messages_warfaction: i64) -> GetCharactersCharacterIdStatsSocial {
    self.chat_messages_warfaction = Some(chat_messages_warfaction);
    self
  }

  pub fn chat_messages_warfaction(&self) -> Option<&i64> {
    self.chat_messages_warfaction.as_ref()
  }

  pub fn reset_chat_messages_warfaction(&mut self) {
    self.chat_messages_warfaction = None;
  }

  pub fn set_chat_total_message_length(&mut self, chat_total_message_length: i64) {
    self.chat_total_message_length = Some(chat_total_message_length);
  }

  pub fn with_chat_total_message_length(mut self, chat_total_message_length: i64) -> GetCharactersCharacterIdStatsSocial {
    self.chat_total_message_length = Some(chat_total_message_length);
    self
  }

  pub fn chat_total_message_length(&self) -> Option<&i64> {
    self.chat_total_message_length.as_ref()
  }

  pub fn reset_chat_total_message_length(&mut self) {
    self.chat_total_message_length = None;
  }

  pub fn set_direct_trades(&mut self, direct_trades: i64) {
    self.direct_trades = Some(direct_trades);
  }

  pub fn with_direct_trades(mut self, direct_trades: i64) -> GetCharactersCharacterIdStatsSocial {
    self.direct_trades = Some(direct_trades);
    self
  }

  pub fn direct_trades(&self) -> Option<&i64> {
    self.direct_trades.as_ref()
  }

  pub fn reset_direct_trades(&mut self) {
    self.direct_trades = None;
  }

  pub fn set_fleet_broadcasts(&mut self, fleet_broadcasts: i64) {
    self.fleet_broadcasts = Some(fleet_broadcasts);
  }

  pub fn with_fleet_broadcasts(mut self, fleet_broadcasts: i64) -> GetCharactersCharacterIdStatsSocial {
    self.fleet_broadcasts = Some(fleet_broadcasts);
    self
  }

  pub fn fleet_broadcasts(&self) -> Option<&i64> {
    self.fleet_broadcasts.as_ref()
  }

  pub fn reset_fleet_broadcasts(&mut self) {
    self.fleet_broadcasts = None;
  }

  pub fn set_fleet_joins(&mut self, fleet_joins: i64) {
    self.fleet_joins = Some(fleet_joins);
  }

  pub fn with_fleet_joins(mut self, fleet_joins: i64) -> GetCharactersCharacterIdStatsSocial {
    self.fleet_joins = Some(fleet_joins);
    self
  }

  pub fn fleet_joins(&self) -> Option<&i64> {
    self.fleet_joins.as_ref()
  }

  pub fn reset_fleet_joins(&mut self) {
    self.fleet_joins = None;
  }

  pub fn set_mails_received(&mut self, mails_received: i64) {
    self.mails_received = Some(mails_received);
  }

  pub fn with_mails_received(mut self, mails_received: i64) -> GetCharactersCharacterIdStatsSocial {
    self.mails_received = Some(mails_received);
    self
  }

  pub fn mails_received(&self) -> Option<&i64> {
    self.mails_received.as_ref()
  }

  pub fn reset_mails_received(&mut self) {
    self.mails_received = None;
  }

  pub fn set_mails_sent(&mut self, mails_sent: i64) {
    self.mails_sent = Some(mails_sent);
  }

  pub fn with_mails_sent(mut self, mails_sent: i64) -> GetCharactersCharacterIdStatsSocial {
    self.mails_sent = Some(mails_sent);
    self
  }

  pub fn mails_sent(&self) -> Option<&i64> {
    self.mails_sent.as_ref()
  }

  pub fn reset_mails_sent(&mut self) {
    self.mails_sent = None;
  }

}



