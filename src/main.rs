#[macro_use]
extern crate rocket;

use diesel::{AsChangeset, Insertable, QueryDsl, Queryable, RunQueryDsl};
use rocket::{
    fairing::AdHoc,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use rocket_sync_db_pools::database;

mod schema;

#[database("postgres_database")]
pub struct DbConn(diesel::PgConnection);

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = schema::blog_posts)]
struct BlogPost {
    #[serde(skip_deserializing)]
    id: i32,
    title: String,
    body: String,
    published: Option<bool>,
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
async fn get_random_blog_post(conn: DbConn) -> Json<BlogPost> {
    conn.run(|c| {
        let id_coll = schema::blog_posts::table
            .select(schema::blog_posts::id)
            .load::<i32>(c)
            .expect("Failed to get ids");
        let count = id_coll.len();
        let idx = rand::random::<usize>();
        let id = if count > 0 { id_coll[idx % count] } else { 0 };
        schema::blog_posts::table.find(id).get_result::<BlogPost>(c)
    })
    .await
    .map(Json)
    .expect("Failed to get random blog post")
}

#[get("/<id>")]
async fn get_blog_post(conn: DbConn, id: i32) -> Json<BlogPost> {
    conn.run(move |c| {
        schema::blog_posts::table
            .find(id)
            .get_result::<BlogPost>(c)
            .map(Json)
    })
    .await
    .expect("Failed to get blog post")
}

#[get("/")]
async fn get_all_blog_posts(conn: DbConn) -> Json<Vec<BlogPost>> {
    conn.run(|c| schema::blog_posts::table.load::<BlogPost>(c).map(Json))
        .await
        .expect("Failed to get all blog posts")
}

#[post("/", data = "<blog_post>")]
async fn create_blog_post(conn: DbConn, blog_post: Json<BlogPost>) -> Json<BlogPost> {
    #[derive(Insertable)]
    #[diesel(table_name = schema::blog_posts)]
    struct NewBlogPost {
        title: String,
        body: String,
    }

    conn.run(|c| {
        let v = blog_post.into_inner();
        let v = NewBlogPost {
            title: v.title,
            body: v.body,
        };
        diesel::insert_into(schema::blog_posts::table)
            .values(&v)
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to create blog post")
}

#[put("/<id>", data = "<blog>")]
async fn update_blog_post(conn: DbConn, id: i32, blog: Json<BlogPost>) -> Json<BlogPost> {
    conn.run(move |c| {
        let mut v = blog.into_inner();
        v.id = id;
        diesel::update(schema::blog_posts::table.find(id))
            .set(&v)
            .get_result(c)
    })
    .await
    .map(Json)
    .expect("Failed to update blog post")
}

#[delete("/<id>")]
async fn delete_blog_post(conn: DbConn, id: i32) -> Json<BlogPost> {
    conn.run(move |c| diesel::delete(schema::blog_posts::table.find(id)).get_result(c))
        .await
        .map(Json)
        .expect("Failed to delete blog post")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(AdHoc::config::<Config>())
        .attach(DbConn::fairing())
        .mount("/", routes![index, get_config])
        .mount(
            "/blog-posts",
            routes![
                get_random_blog_post,
                get_blog_post,
                get_all_blog_posts,
                create_blog_post,
                update_blog_post,
                delete_blog_post,
            ],
        )
        .launch()
        .await;
}
