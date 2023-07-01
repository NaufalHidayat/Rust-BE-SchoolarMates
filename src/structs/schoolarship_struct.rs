use serde::{Serialize};

#[doc = "Schoolarship struct"]
#[derive(Debug, Serialize)]
pub struct SchoolarshipStruct {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub quantity: i32,
  pub requirement: String,
  pub univ_id: i32,
}

#[doc = "Detail schoolarship struct"]
#[derive(Debug, Serialize)]
pub struct DetailSchoolarshipStruct {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub quantity: i32,
  pub requirement: String,
  pub univ_id: i32,
  pub univ_major: String,
  pub univ_name: String,
  pub univ_alias: String,
  pub univ_description: String,
}