use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::category;
use crate::Category;
use crate::util::{Category as CategoryModel};
use crate::util::NewCategory;
use sea_orm::*;

pub async fn get(State(db): State<DatabaseConnection>) -> (StatusCode, Json<Vec<CategoryModel>>) {

    let categories: Vec<category::Model> = Category::find().all(&db).await.unwrap();
    let mut response = Vec::new();
    for c in categories {
        response.push(CategoryModel {
            id: c.id,
            name: c.name,
            budget: c.budget,
        });
    }

    (StatusCode::OK, Json(response))

}

pub async fn post(State(db): State<DatabaseConnection>, Json(body): Json<NewCategory>) -> (StatusCode, Json<CategoryModel>) {

    let new_cat: CategoryModel = body.clone().into();

    let cat = category::ActiveModel {
        name: ActiveValue::Set(new_cat.name),
        budget: ActiveValue::Set(new_cat.budget),
        ..Default::default()
    };
    let _res = Category::insert(cat).exec(&db).await.expect("Failed to insert category");


    (StatusCode::CREATED, Json(body.clone().into()))
}   