use actix_web::{web, Responder};

use crate::{AppState, helpers::{response::*, parse::*}, structs::auth_struct::*};

#[doc = "Get all user"]
pub async fn get_user(state: web::Data<AppState>) -> impl Responder {
  let data = sqlx::query_as!(UserStruct, "select id, email, role from users")
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved user".to_string(),
    result
  )
}