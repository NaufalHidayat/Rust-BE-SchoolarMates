use serde::{Serialize};

#[doc = "University struct"]
#[derive(Debug, Serialize)]
pub struct UniversityStruct {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub major: String,
  pub quantity: i32,
  pub image: String,
  pub link: String,
  pub alias: String,
}