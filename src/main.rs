#[macro_use]
extern crate rocket;

use rocket::{
    fairing::AdHoc,
    serde::{json::Json, Deserialize, Serialize},
    State,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct Config {
    name: String,
    age: u8,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/config")]
fn get_config(config: &State<Config>) -> Json<Config> {
    Json(config.inner().clone())
}

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    let blog_post = BlogPost {
        id: 1,
        title: "My first blog post".to_string(),
        body: "This is my first blog post. I hope you like it!".to_string(),
        published: true,
    };
    Json(blog_post)
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    let blog_post = BlogPost {
        id,
        title: "Some title".to_string(),
        body: "Some body".to_string(),
        published: true,
    };
    Json(blog_post)
}

#[get("/")]
fn get_all_blog_posts() -> Json<Vec<BlogPost>> {
    let v = vec![
        BlogPost {
            id: 1,
            title: "My first blog post".to_string(),
            body: "This is my first blog post. I hope you like it!".to_string(),
            published: true,
        },
        BlogPost {
            id: 2,
            title: "My second blog post".to_string(),
            body: "This is my second blog post. I hope you like it!".to_string(),
            published: true,
        },
    ];
    Json(v)
}

#[post("/", data = "<blog_post>")]
fn create_blog_post(blog_post: Json<BlogPost>) -> Json<BlogPost> {
    blog_post
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(AdHoc::config::<Config>())
        .mount("/", routes![index, get_config])
        .mount(
            "/blog-posts",
            routes![
                get_random_blog_post,
                get_blog_post,
                get_all_blog_posts,
                create_blog_post
            ],
        )
        .launch()
        .await;
}
