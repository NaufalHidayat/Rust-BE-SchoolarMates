use actix_web::{web::{self}, Responder};

use crate::{AppState, helpers::{response::*, parse::*}, structs::forum_struct::*};

#[doc = "Get all forum"]
pub async fn get_forum(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let data = sqlx::query_as!(ForumStruct,
    "select universities.alias, forums.message, universities.image, universities.link from forums
      join universities on forums.univ_id = universities.id where student_id = $1
      order by forums.id desc"
    , id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved forum".to_string(),
    result
  )
}