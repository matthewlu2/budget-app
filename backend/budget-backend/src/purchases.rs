use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::purchase;
use crate::Purchase;
use crate::category;
use crate::Category;
use crate::util::{Purchase as PurchaseModel};
use crate::util::NewPurchase;
use sea_orm::*;

pub async fn get(State(db): State<DatabaseConnection>) -> (StatusCode, Json<Vec<PurchaseModel>>) {

    let purchases: Vec<purchase::Model> = Purchase::find().order_by_desc(purchase::Column::Date).all(&db).await.unwrap();

    let mut response = Vec::new();
    for pur in purchases {
        let cat = Category::find_by_id(pur.cat_id).one(&db).await.unwrap().unwrap();
        response.push(PurchaseModel {
            id: pur.id,
            desc: pur.desc,
            amount: pur.amount,
            date: pur.date,
            cat_id: pur.cat_id,
            category: cat.name,
        });
    }

    (StatusCode::OK, Json(response))
}

pub async fn post(State(db): State<DatabaseConnection>, Json(body): Json<NewPurchase>) -> (StatusCode, Json<PurchaseModel>) {

    let new_pur: PurchaseModel = body.clone().into();

    let cat: Option<category::Model> = Category::find()
    .filter(category::Column::Name.eq(new_pur.category))
    .one(&db)
    .await.unwrap();

    let category_id = cat.unwrap().id;

    let pur = purchase::ActiveModel {
        desc: ActiveValue::Set(new_pur.desc),
        amount: ActiveValue::Set(new_pur.amount),
        date: ActiveValue::Set(new_pur.date),
        cat_id: ActiveValue::Set(category_id),
        ..Default::default()
    };
    let _res = Purchase::insert(pur).exec(&db).await.expect("Failed to insert purchase");

    (StatusCode::CREATED, Json(body.clone().into()))
}   