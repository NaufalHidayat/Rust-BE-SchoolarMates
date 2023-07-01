use serde::{Serialize};

#[doc = "Application struct"]
#[derive(Debug, Serialize)]
pub struct ApplicationStruct {
  pub id: i32,
  pub student_id: i32,
  pub univ_id: i32,
  pub status: String,
  pub major: String,
}

#[doc = "Detail application struct"]
#[derive(Debug, Serialize)]
pub struct DetailApplicationStruct {
  pub id: i32,
  pub univ_id: i32,
  pub name: String,
  pub major: String,
  pub image: String,
  pub status: String,
}