/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdStatsCharacter : character object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdStatsCharacter {
  /// days_of_activity integer
  #[serde(rename = "days_of_activity")]
  days_of_activity: Option<i64>,
  /// minutes integer
  #[serde(rename = "minutes")]
  minutes: Option<i64>,
  /// sessions_started integer
  #[serde(rename = "sessions_started")]
  sessions_started: Option<i64>
}

impl GetCharactersCharacterIdStatsCharacter {
  /// character object
  pub fn new() -> GetCharactersCharacterIdStatsCharacter {
    GetCharactersCharacterIdStatsCharacter {
      days_of_activity: None,
      minutes: None,
      sessions_started: None
    }
  }

  pub fn set_days_of_activity(&mut self, days_of_activity: i64) {
    self.days_of_activity = Some(days_of_activity);
  }

  pub fn with_days_of_activity(mut self, days_of_activity: i64) -> GetCharactersCharacterIdStatsCharacter {
    self.days_of_activity = Some(days_of_activity);
    self
  }

  pub fn days_of_activity(&self) -> Option<&i64> {
    self.days_of_activity.as_ref()
  }

  pub fn reset_days_of_activity(&mut self) {
    self.days_of_activity = None;
  }

  pub fn set_minutes(&mut self, minutes: i64) {
    self.minutes = Some(minutes);
  }

  pub fn with_minutes(mut self, minutes: i64) -> GetCharactersCharacterIdStatsCharacter {
    self.minutes = Some(minutes);
    self
  }

  pub fn minutes(&self) -> Option<&i64> {
    self.minutes.as_ref()
  }

  pub fn reset_minutes(&mut self) {
    self.minutes = None;
  }

  pub fn set_sessions_started(&mut self, sessions_started: i64) {
    self.sessions_started = Some(sessions_started);
  }

  pub fn with_sessions_started(mut self, sessions_started: i64) -> GetCharactersCharacterIdStatsCharacter {
    self.sessions_started = Some(sessions_started);
    self
  }

  pub fn sessions_started(&self) -> Option<&i64> {
    self.sessions_started.as_ref()
  }

  pub fn reset_sessions_started(&mut self) {
    self.sessions_started = None;
  }

}



