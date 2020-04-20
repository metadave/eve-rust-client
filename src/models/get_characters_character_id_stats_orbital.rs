/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdStatsOrbital : orbital object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdStatsOrbital {
  /// strike_characters_killed integer
  #[serde(rename = "strike_characters_killed")]
  strike_characters_killed: Option<i64>,
  /// strike_damage_to_players_armor_amount integer
  #[serde(rename = "strike_damage_to_players_armor_amount")]
  strike_damage_to_players_armor_amount: Option<i64>,
  /// strike_damage_to_players_shield_amount integer
  #[serde(rename = "strike_damage_to_players_shield_amount")]
  strike_damage_to_players_shield_amount: Option<i64>
}

impl GetCharactersCharacterIdStatsOrbital {
  /// orbital object
  pub fn new() -> GetCharactersCharacterIdStatsOrbital {
    GetCharactersCharacterIdStatsOrbital {
      strike_characters_killed: None,
      strike_damage_to_players_armor_amount: None,
      strike_damage_to_players_shield_amount: None
    }
  }

  pub fn set_strike_characters_killed(&mut self, strike_characters_killed: i64) {
    self.strike_characters_killed = Some(strike_characters_killed);
  }

  pub fn with_strike_characters_killed(mut self, strike_characters_killed: i64) -> GetCharactersCharacterIdStatsOrbital {
    self.strike_characters_killed = Some(strike_characters_killed);
    self
  }

  pub fn strike_characters_killed(&self) -> Option<&i64> {
    self.strike_characters_killed.as_ref()
  }

  pub fn reset_strike_characters_killed(&mut self) {
    self.strike_characters_killed = None;
  }

  pub fn set_strike_damage_to_players_armor_amount(&mut self, strike_damage_to_players_armor_amount: i64) {
    self.strike_damage_to_players_armor_amount = Some(strike_damage_to_players_armor_amount);
  }

  pub fn with_strike_damage_to_players_armor_amount(mut self, strike_damage_to_players_armor_amount: i64) -> GetCharactersCharacterIdStatsOrbital {
    self.strike_damage_to_players_armor_amount = Some(strike_damage_to_players_armor_amount);
    self
  }

  pub fn strike_damage_to_players_armor_amount(&self) -> Option<&i64> {
    self.strike_damage_to_players_armor_amount.as_ref()
  }

  pub fn reset_strike_damage_to_players_armor_amount(&mut self) {
    self.strike_damage_to_players_armor_amount = None;
  }

  pub fn set_strike_damage_to_players_shield_amount(&mut self, strike_damage_to_players_shield_amount: i64) {
    self.strike_damage_to_players_shield_amount = Some(strike_damage_to_players_shield_amount);
  }

  pub fn with_strike_damage_to_players_shield_amount(mut self, strike_damage_to_players_shield_amount: i64) -> GetCharactersCharacterIdStatsOrbital {
    self.strike_damage_to_players_shield_amount = Some(strike_damage_to_players_shield_amount);
    self
  }

  pub fn strike_damage_to_players_shield_amount(&self) -> Option<&i64> {
    self.strike_damage_to_players_shield_amount.as_ref()
  }

  pub fn reset_strike_damage_to_players_shield_amount(&mut self) {
    self.strike_damage_to_players_shield_amount = None;
  }

}


