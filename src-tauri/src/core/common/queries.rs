use modql::filter::ListOptions;
use serde_json::json;

use crate::core::entities::catalog::{item::ItemFilter, item_category::ItemCategoryFilter};

pub fn get_list_options(limit: Option<i64>, offset: Option<i64>) -> ListOptions {
    serde_json::from_value::<ListOptions>(json!({
        "limit": limit.unwrap_or(100),
        "offset": offset.unwrap_or(0)
    }))
    .unwrap()
}

// Item Category
pub fn get_item_cat_by_id(id: String) -> ItemCategoryFilter {
    serde_json::from_value::<ItemCategoryFilter>(json!({
        "id": id
    }))
    .unwrap()
}

pub fn get_item_cat_by_name(name: String) -> ItemCategoryFilter {
    serde_json::from_value::<ItemCategoryFilter>(json!({
        "name": name
    }))
    .unwrap()
}

// Item
pub fn get_item_by_id(id: String) -> ItemFilter {
    serde_json::from_value::<ItemFilter>(json!({
        "id": id
    }))
    .unwrap()
}

pub fn get_item_by_name(name: String) -> ItemFilter {
    serde_json::from_value::<ItemFilter>(json!({
        "name": name
    }))
    .unwrap()
}

pub fn get_item_by_cat_id(cat_id: String) -> ItemFilter {
    serde_json::from_value::<ItemFilter>(json!({
        "category_id": cat_id
    }))
    .unwrap()
}
