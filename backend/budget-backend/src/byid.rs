use axum::{extract::{Path, State}, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::Purchase;
use crate::Category;
use crate::util::{Purchase as PurchaseModel};
use crate::util::{Category as CategoryModel};

pub async fn cat_get(Path(cat_id): Path<String>, State(db): State<DatabaseConnection>) -> (StatusCode, Json<CategoryModel>) {
    
    let id = cat_id.parse::<i32>().unwrap();
    let cat = Category::find_by_id(id).one(&db).await.unwrap().unwrap();
    let response: CategoryModel = CategoryModel {
        id: cat.id,
        name: cat.name,
        budget: cat.budget,
    };

    (StatusCode::OK, Json(response))
}

pub async fn pur_get(Path(pur_id): Path<String>, State(db): State<DatabaseConnection>) -> (StatusCode, Json<PurchaseModel>) {
    
    let id = pur_id.parse::<i32>().unwrap();
    let pur = Purchase::find_by_id(id).one(&db).await.unwrap().unwrap();
    let cat = Category::find_by_id(pur.cat_id).one(&db).await.unwrap().unwrap();
    let response: PurchaseModel = PurchaseModel {
        id: pur.id,
        desc: pur.desc,
        amount: pur.amount,
        date: pur.date,
        cat_id: pur.cat_id,
        category: cat.name,
    };

    (StatusCode::OK, Json(response))
}


