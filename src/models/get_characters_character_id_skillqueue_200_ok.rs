/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetCharactersCharacterIdSkillqueue200Ok : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdSkillqueue200Ok {
  /// Date on which training of the skill will complete. Omitted if the skill queue is paused.
  #[serde(rename = "finish_date")]
  finish_date: Option<String>,
  /// finished_level integer
  #[serde(rename = "finished_level")]
  finished_level: i32,
  /// level_end_sp integer
  #[serde(rename = "level_end_sp")]
  level_end_sp: Option<i32>,
  /// Amount of SP that was in the skill when it started training it's current level. Used to calculate % of current level complete.
  #[serde(rename = "level_start_sp")]
  level_start_sp: Option<i32>,
  /// queue_position integer
  #[serde(rename = "queue_position")]
  queue_position: i32,
  /// skill_id integer
  #[serde(rename = "skill_id")]
  skill_id: i32,
  /// start_date string
  #[serde(rename = "start_date")]
  start_date: Option<String>,
  /// training_start_sp integer
  #[serde(rename = "training_start_sp")]
  training_start_sp: Option<i32>
}

impl GetCharactersCharacterIdSkillqueue200Ok {
  /// 200 ok object
  pub fn new(finished_level: i32, queue_position: i32, skill_id: i32) -> GetCharactersCharacterIdSkillqueue200Ok {
    GetCharactersCharacterIdSkillqueue200Ok {
      finish_date: None,
      finished_level: finished_level,
      level_end_sp: None,
      level_start_sp: None,
      queue_position: queue_position,
      skill_id: skill_id,
      start_date: None,
      training_start_sp: None
    }
  }

  pub fn set_finish_date(&mut self, finish_date: String) {
    self.finish_date = Some(finish_date);
  }

  pub fn with_finish_date(mut self, finish_date: String) -> GetCharactersCharacterIdSkillqueue200Ok {
    self.finish_date = Some(finish_date);
    self
  }

  pub fn finish_date(&self) -> Option<&String> {
    self.finish_date.as_ref()
  }

  pub fn reset_finish_date(&mut self) {
    self.finish_date = None;
  }

  pub fn set_finished_level(&mut self, finished_level: i32) {
    self.finished_level = finished_level;
  }

  pub fn with_finished_level(mut self, finished_level: i32) -> GetCharactersCharacterIdSkillqueue200Ok {
    self.finished_level = finished_level;
    self
  }

  pub fn finished_level(&self) -> &i32 {
    &self.finished_level
  }


  pub fn set_level_end_sp(&mut self, level_end_sp: i32) {
    self.level_end_sp = Some(level_end_sp);
  }

  pub fn with_level_end_sp(mut self, level_end_sp: i32) -> GetCharactersCharacterIdSkillqueue200Ok {
    self.level_end_sp = Some(level_end_sp);
    self
  }

  pub fn level_end_sp(&self) -> Option<&i32> {
    self.level_end_sp.as_ref()
  }

  pub fn reset_level_end_sp(&mut self) {
    self.level_end_sp = None;
  }

  pub fn set_level_start_sp(&mut self, level_start_sp: i32) {
    self.level_start_sp = Some(level_start_sp);
  }

  pub fn with_level_start_sp(mut self, level_start_sp: i32) -> GetCharactersCharacterIdSkillqueue200Ok {
    self.level_start_sp = Some(level_start_sp);
    self
  }

  pub fn level_start_sp(&self) -> Option<&i32> {
    self.level_start_sp.as_ref()
  }

  pub fn reset_level_start_sp(&mut self) {
    self.level_start_sp = None;
  }

  pub fn set_queue_position(&mut self, queue_position: i32) {
    self.queue_position = queue_position;
  }

  pub fn with_queue_position(mut self, queue_position: i32) -> GetCharactersCharacterIdSkillqueue200Ok {
    self.queue_position = queue_position;
    self
  }

  pub fn queue_position(&self) -> &i32 {
    &self.queue_position
  }


  pub fn set_skill_id(&mut self, skill_id: i32) {
    self.skill_id = skill_id;
  }

  pub fn with_skill_id(mut self, skill_id: i32) -> GetCharactersCharacterIdSkillqueue200Ok {
    self.skill_id = skill_id;
    self
  }

  pub fn skill_id(&self) -> &i32 {
    &self.skill_id
  }


  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> GetCharactersCharacterIdSkillqueue200Ok {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_training_start_sp(&mut self, training_start_sp: i32) {
    self.training_start_sp = Some(training_start_sp);
  }

  pub fn with_training_start_sp(mut self, training_start_sp: i32) -> GetCharactersCharacterIdSkillqueue200Ok {
    self.training_start_sp = Some(training_start_sp);
    self
  }

  pub fn training_start_sp(&self) -> Option<&i32> {
    self.training_start_sp.as_ref()
  }

  pub fn reset_training_start_sp(&mut self) {
    self.training_start_sp = None;
  }

}


