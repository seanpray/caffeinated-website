mod models;
use actix_cors::Cors;
use std::{str::FromStr, sync::Arc, thread::sleep, time::Duration};

use models::*;
mod database;
use database::*;

mod routes;
use notion::{
    ids::DatabaseId,
    models::{
        properties::PropertyConfiguration,
        search::{self, DatabaseQuery, SearchRequest},
    },
    NotionApi,
};
use routes::*;

use actix_web::{get, web, App, HttpServer, Responder};
use dashmap::DashMap;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

pub(crate) struct AppData {
    pub tokens: Arc<DashMap<String, AccessToken>>,
    pub posts: Arc<DashMap<String, BlogPost>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // <user, token>
    let tokens: Arc<DashMap<String, AccessToken>> = Arc::new(DashMap::new());
    // <title, post>
    let blog_post: Arc<DashMap<String, BlogPost>> = Arc::new(DashMap::new());
    let post_loop = blog_post.clone();
    tokio::spawn(async move {
        let mut i = 0;
        let notion = loop {
            if let Ok(v) = NotionApi::new(env!("NOTION_SECRET").to_string()) {
                break v;
            }
            i += 1;
            sleep(Duration::from_secs(1));
            if i > 5 {
                panic!("failed to connect to notion");
            }
        };
        loop {
            if let Ok(v) = notion
                .query_database(
                    DatabaseId::from_str(env!("DB_ID")).unwrap(),
                    DatabaseQuery::default(),
                )
                .await
            {
                let mut new_post = DashMap::new();
                for p in v.results() {
                    let t = p.title();
                    let Some(title) = t else {
                        continue;
                    };
                    if title.is_empty() {
                        continue;
                    }
                    // for (k, v) in &p.properties.properties {
                    //     println!("{:?} : {:?}", k, v);
                    // }
                    let p = &p.properties.properties;
                    let Some(post) = BlogPost::from_property_map(p) else {
                        continue;
                    };
                    *new_post.entry(post.title().to_owned()).or_insert(post) = post.clone();
                }
                post_loop.clear();
                // absolutely terrible solution, but good enough for small scale
                (*post_loop).clone_into(&mut new_post);
            }
            sleep(Duration::from_secs(120));
        }
    });
    let blog_post = blog_post.clone();
    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:5173")
        //     .allowed_origin("https://team9293.com")
        //     .allow_any_method();
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(AppData {
                tokens: tokens.clone(),
                posts: blog_post.clone(),
            })
            .service(greet)
            .service(routes::posts::posts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
