use actix_web::{web::{Json, Data}};

use crate::{models::{LoginData, AccessToken}, AppData};

async fn login(data: Json<LoginData>, appdata: Data<AppData>) -> Json<AccessToken> {
    if appdata.users.contains_key(&data.username) {
        Json(AccessToken::new(&data.username))
    } else {
        Json(AccessToken::blank())
    }
}
