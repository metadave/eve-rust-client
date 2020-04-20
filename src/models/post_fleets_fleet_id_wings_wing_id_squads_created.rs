/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// PostFleetsFleetIdWingsWingIdSquadsCreated : 201 created object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostFleetsFleetIdWingsWingIdSquadsCreated {
  /// The squad_id of the newly created squad
  #[serde(rename = "squad_id")]
  squad_id: i64
}

impl PostFleetsFleetIdWingsWingIdSquadsCreated {
  /// 201 created object
  pub fn new(squad_id: i64) -> PostFleetsFleetIdWingsWingIdSquadsCreated {
    PostFleetsFleetIdWingsWingIdSquadsCreated {
      squad_id: squad_id
    }
  }

  pub fn set_squad_id(&mut self, squad_id: i64) {
    self.squad_id = squad_id;
  }

  pub fn with_squad_id(mut self, squad_id: i64) -> PostFleetsFleetIdWingsWingIdSquadsCreated {
    self.squad_id = squad_id;
    self
  }

  pub fn squad_id(&self) -> &i64 {
    &self.squad_id
  }


}


