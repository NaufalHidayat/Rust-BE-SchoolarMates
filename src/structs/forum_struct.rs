use serde::{Serialize};

#[doc = "Forum struct"]
#[derive(Debug, Serialize)]
pub struct ForumStruct {
  pub alias: String,
  pub message: String,
  pub image: String,
  pub link: String,
}