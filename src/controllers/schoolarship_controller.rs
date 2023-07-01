use serde_json::Value;
use actix_web::{web::{self}, Responder};

use crate::{AppState, helpers::{response::*, parse::*, validation::*}, structs::schoolarship_struct::*};

#[doc = "Get all schoolarship"]
pub async fn get_schoolarship(state: web::Data<AppState>) -> impl Responder {
  let data = sqlx::query_as!(DetailSchoolarshipStruct,
    "select s.id, s.name, s.description, s.quantity, s.requirement, u.id as univ_id,
      u.name as univ_name, u.alias as univ_alias, u.description as univ_description,
      u.major as univ_major from schoolarships s join universities u on s.univ_id = u.id")
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully retrieved schoolarship".to_string(),
    result
  )
}

#[doc = "Add new schoolarship"]
pub async fn add_schoolarship(state: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
  let name = to_str(map_get("name", body.to_owned()));
  let description = to_str(map_get("description", body.to_owned()));
  let major = to_str(map_get("major", body.to_owned()));
  let quantity = to_i32(map_get("quantity", body.to_owned()));
  let requirement = to_str(map_get("requirement", body.to_owned()));
  let univ_id = to_i32(map_get("univ_id", body.to_owned()));

  if check_if_empty(name.to_owned()) || check_if_empty(description.to_owned()) || check_if_empty(major.to_owned()) || check_if_empty(requirement.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let schoolarship_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from schoolarships where name = $1) as schoolarship_exists")
    .bind(name.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if schoolarship_exists {
    return response_json(
      "failed".to_string(),
      "Schoolarship already exists".to_string(),
      vec![]
    )
  }

  let data = sqlx::query_as!(SchoolarshipStruct,
    "insert into schoolarships (name, description, quantity, requirement, univ_id)
      values ($1, $2, $3, $4, $5) returning *",
    name, description, quantity, requirement, univ_id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully added schoolarship".to_string(),
    result
  )
}

#[doc = "Find schoolarship by id"]
pub async fn find_schoolarship(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();

  match sqlx::query_as!(SchoolarshipStruct, "select * from schoolarships where id = $1", id)
    .fetch_optional(&state.db)
    .await {
      Ok(Some(schoolarship_data)) => {
        let result = convert_vec_to_values(vec![schoolarship_data]);

        return response_json(
          "success".to_string(),
          "Successfully retrieved schoolarship".to_string(),
          result
        )
      },
      Ok(None) => {
        return response_json(
          "failed".to_string(),
          "Schoolarship not found".to_string(),
          vec![]
        )
      }
      Err(_) => return response_json(
        "error".to_string(),
        "Something went wrong".to_string(),
        vec![]
      )
    };
}

#[doc = "Update schoolarship by id"]
pub async fn update_schoolarship(state: web::Data<AppState>, body: web::Json<Value>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let name = to_str(map_get("name", body.to_owned()));
  let description = to_str(map_get("description", body.to_owned()));
  let major = to_str(map_get("major", body.to_owned()));
  let quantity = to_i32(map_get("quantity", body.to_owned()));
  let requirement = to_str(map_get("requirement", body.to_owned()));
  let univ_id = to_i32(map_get("univ_id", body.to_owned()));

  if check_if_empty(name.to_owned()) || check_if_empty(description.to_owned()) || check_if_empty(major.to_owned()) || check_if_empty(requirement.to_owned()) {
    return response_json(
      "failed".to_string(),
      "Please fill all fields".to_string(),
      vec![]
    )
  }

  let schoolarship_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from schoolarships where id = $1) as schoolarship_exists")
    .bind(id.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if !schoolarship_exists {
    return response_json(
      "failed".to_string(),
      "Schoolarship not found".to_string(),
      vec![]
    )
  }

  let data = sqlx::query_as!(SchoolarshipStruct,
    "update schoolarships set name = $1, description = $2, quantity = $3, requirement = $4, univ_id = $5 where id = $6 returning *",
    name, description, quantity, requirement, univ_id, id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully updated schoolarship".to_string(),
    result
  )
}

#[doc = "Delete schoolarship by id"]
pub async fn delete_schoolarship(state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
  let id = path.into_inner();
  let schoolarship_exists = sqlx::query_scalar::<_, bool>("select exists(select 1 from schoolarships where id = $1) as schoolarship_exists")
    .bind(id.clone())
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

  if !schoolarship_exists {
    return response_json(
      "failed".to_string(),
      "Schoolarship not found".to_string(),
      vec![]
    )
  }

  let data = sqlx::query_as!(SchoolarshipStruct, "delete from schoolarships where id = $1 returning *", id)
    .fetch_all(&state.db)
    .await
    .unwrap();

  let result = convert_vec_to_values(data);

  response_json(
    "success".to_string(),
    "Successfully deleted schoolarship".to_string(),
    result
  )
}