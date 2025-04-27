use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct NewCategory {
    name: String,   
    budget: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewPurchase {
    desc: String,
    amount: String,
    date: String,
    category: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: i32,
    pub name: String,   
    pub budget: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Purchase {
    pub id: i32,
    pub desc: String,
    pub amount: i32,
    pub date: String,
    pub cat_id: i32,
    pub category: String,
}

impl From<NewCategory> for Category {
    fn from(item: NewCategory) -> Self {
        Self {
            id: 0,
            name: item.name,
            budget: if item.budget.trim().is_empty() {
                None
            } else {
                item.budget.parse::<i32>().ok()
            },
        }
    }
}

impl From<NewPurchase> for Purchase {
    fn from(item: NewPurchase) -> Self {
        Self {
            id: 0,
            desc: item.desc,
            amount: item.amount.parse::<i32>().unwrap(),
            date: item.date,
            cat_id: 0,
            category: item.category,
        }
    }
}
