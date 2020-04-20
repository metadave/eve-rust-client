/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// GetUniverseStationsStationIdOk : 200 ok object

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUniverseStationsStationIdOk {
  /// max_dockable_ship_volume number
  #[serde(rename = "max_dockable_ship_volume")]
  max_dockable_ship_volume: f32,
  /// name string
  #[serde(rename = "name")]
  name: String,
  /// office_rental_cost number
  #[serde(rename = "office_rental_cost")]
  office_rental_cost: f32,
  /// ID of the corporation that controls this station
  #[serde(rename = "owner")]
  owner: Option<i32>,
  #[serde(rename = "position")]
  position: ::models::GetUniverseStationsStationIdPosition,
  /// race_id integer
  #[serde(rename = "race_id")]
  race_id: Option<i32>,
  /// reprocessing_efficiency number
  #[serde(rename = "reprocessing_efficiency")]
  reprocessing_efficiency: f32,
  /// reprocessing_stations_take number
  #[serde(rename = "reprocessing_stations_take")]
  reprocessing_stations_take: f32,
  /// services array
  #[serde(rename = "services")]
  services: Vec<String>,
  /// station_id integer
  #[serde(rename = "station_id")]
  station_id: i32,
  /// The solar system this station is in
  #[serde(rename = "system_id")]
  system_id: i32,
  /// type_id integer
  #[serde(rename = "type_id")]
  type_id: i32
}

impl GetUniverseStationsStationIdOk {
  /// 200 ok object
  pub fn new(max_dockable_ship_volume: f32, name: String, office_rental_cost: f32, position: ::models::GetUniverseStationsStationIdPosition, reprocessing_efficiency: f32, reprocessing_stations_take: f32, services: Vec<String>, station_id: i32, system_id: i32, type_id: i32) -> GetUniverseStationsStationIdOk {
    GetUniverseStationsStationIdOk {
      max_dockable_ship_volume: max_dockable_ship_volume,
      name: name,
      office_rental_cost: office_rental_cost,
      owner: None,
      position: position,
      race_id: None,
      reprocessing_efficiency: reprocessing_efficiency,
      reprocessing_stations_take: reprocessing_stations_take,
      services: services,
      station_id: station_id,
      system_id: system_id,
      type_id: type_id
    }
  }

  pub fn set_max_dockable_ship_volume(&mut self, max_dockable_ship_volume: f32) {
    self.max_dockable_ship_volume = max_dockable_ship_volume;
  }

  pub fn with_max_dockable_ship_volume(mut self, max_dockable_ship_volume: f32) -> GetUniverseStationsStationIdOk {
    self.max_dockable_ship_volume = max_dockable_ship_volume;
    self
  }

  pub fn max_dockable_ship_volume(&self) -> &f32 {
    &self.max_dockable_ship_volume
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> GetUniverseStationsStationIdOk {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_office_rental_cost(&mut self, office_rental_cost: f32) {
    self.office_rental_cost = office_rental_cost;
  }

  pub fn with_office_rental_cost(mut self, office_rental_cost: f32) -> GetUniverseStationsStationIdOk {
    self.office_rental_cost = office_rental_cost;
    self
  }

  pub fn office_rental_cost(&self) -> &f32 {
    &self.office_rental_cost
  }


  pub fn set_owner(&mut self, owner: i32) {
    self.owner = Some(owner);
  }

  pub fn with_owner(mut self, owner: i32) -> GetUniverseStationsStationIdOk {
    self.owner = Some(owner);
    self
  }

  pub fn owner(&self) -> Option<&i32> {
    self.owner.as_ref()
  }

  pub fn reset_owner(&mut self) {
    self.owner = None;
  }

  pub fn set_position(&mut self, position: ::models::GetUniverseStationsStationIdPosition) {
    self.position = position;
  }

  pub fn with_position(mut self, position: ::models::GetUniverseStationsStationIdPosition) -> GetUniverseStationsStationIdOk {
    self.position = position;
    self
  }

  pub fn position(&self) -> &::models::GetUniverseStationsStationIdPosition {
    &self.position
  }


  pub fn set_race_id(&mut self, race_id: i32) {
    self.race_id = Some(race_id);
  }

  pub fn with_race_id(mut self, race_id: i32) -> GetUniverseStationsStationIdOk {
    self.race_id = Some(race_id);
    self
  }

  pub fn race_id(&self) -> Option<&i32> {
    self.race_id.as_ref()
  }

  pub fn reset_race_id(&mut self) {
    self.race_id = None;
  }

  pub fn set_reprocessing_efficiency(&mut self, reprocessing_efficiency: f32) {
    self.reprocessing_efficiency = reprocessing_efficiency;
  }

  pub fn with_reprocessing_efficiency(mut self, reprocessing_efficiency: f32) -> GetUniverseStationsStationIdOk {
    self.reprocessing_efficiency = reprocessing_efficiency;
    self
  }

  pub fn reprocessing_efficiency(&self) -> &f32 {
    &self.reprocessing_efficiency
  }


  pub fn set_reprocessing_stations_take(&mut self, reprocessing_stations_take: f32) {
    self.reprocessing_stations_take = reprocessing_stations_take;
  }

  pub fn with_reprocessing_stations_take(mut self, reprocessing_stations_take: f32) -> GetUniverseStationsStationIdOk {
    self.reprocessing_stations_take = reprocessing_stations_take;
    self
  }

  pub fn reprocessing_stations_take(&self) -> &f32 {
    &self.reprocessing_stations_take
  }


  pub fn set_services(&mut self, services: Vec<String>) {
    self.services = services;
  }

  pub fn with_services(mut self, services: Vec<String>) -> GetUniverseStationsStationIdOk {
    self.services = services;
    self
  }

  pub fn services(&self) -> &Vec<String> {
    &self.services
  }


  pub fn set_station_id(&mut self, station_id: i32) {
    self.station_id = station_id;
  }

  pub fn with_station_id(mut self, station_id: i32) -> GetUniverseStationsStationIdOk {
    self.station_id = station_id;
    self
  }

  pub fn station_id(&self) -> &i32 {
    &self.station_id
  }


  pub fn set_system_id(&mut self, system_id: i32) {
    self.system_id = system_id;
  }

  pub fn with_system_id(mut self, system_id: i32) -> GetUniverseStationsStationIdOk {
    self.system_id = system_id;
    self
  }

  pub fn system_id(&self) -> &i32 {
    &self.system_id
  }


  pub fn set_type_id(&mut self, type_id: i32) {
    self.type_id = type_id;
  }

  pub fn with_type_id(mut self, type_id: i32) -> GetUniverseStationsStationIdOk {
    self.type_id = type_id;
    self
  }

  pub fn type_id(&self) -> &i32 {
    &self.type_id
  }


}


