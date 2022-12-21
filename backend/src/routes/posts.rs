use crate::AppData;
use actix_web::{get, HttpRequest, Responder};
use dashmap::DashMap;
use std::sync::Arc;

use crate::models::BlogPost;

#[get("/posts")]
pub async fn posts(req: HttpRequest) -> impl Responder {
    let Some(data) = req.app_data::<AppData>() else {
        println!("invalid");
        return "".to_string();
    };
    // no .values() from DashMap :(
    let mut posts = Vec::with_capacity(data.posts.len());
    for post in data.posts.iter() {
        posts.push(post.value().clone());
    }
    serde_json::to_string(&posts).unwrap()
}
