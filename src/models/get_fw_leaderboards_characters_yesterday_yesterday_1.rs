/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetFwLeaderboardsCharactersYesterdayYesterday1 : yesterday object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCharactersYesterdayYesterday1 {
  /// Amount of victory points
  #[serde(rename = "amount")]
  amount: Option<i32>,
  /// character_id integer
  #[serde(rename = "character_id")]
  character_id: Option<i32>
}

impl GetFwLeaderboardsCharactersYesterdayYesterday1 {
  /// yesterday object
  pub fn new() -> GetFwLeaderboardsCharactersYesterdayYesterday1 {
    GetFwLeaderboardsCharactersYesterdayYesterday1 {
      amount: None,
      character_id: None
    }
  }

  pub fn set_amount(&mut self, amount: i32) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: i32) -> GetFwLeaderboardsCharactersYesterdayYesterday1 {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&i32> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_character_id(&mut self, character_id: i32) {
    self.character_id = Some(character_id);
  }

  pub fn with_character_id(mut self, character_id: i32) -> GetFwLeaderboardsCharactersYesterdayYesterday1 {
    self.character_id = Some(character_id);
    self
  }

  pub fn character_id(&self) -> Option<&i32> {
    self.character_id.as_ref()
  }

  pub fn reset_character_id(&mut self) {
    self.character_id = None;
  }

}


