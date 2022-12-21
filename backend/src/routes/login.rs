use actix_web::web::{Data, Json};

use crate::{
    models::{AccessToken, LoginData},
    AppData,
};

async fn login(data: Json<LoginData>, appdata: Data<AppData>) -> Json<AccessToken> {
    // if appdata.users.contains_key(&data.username) {
    //     Json(AccessToken::new(&data.username))
    // } else {
    // }
    Json(AccessToken::blank())
}
