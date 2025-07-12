use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::{post, routes, launch};

#[derive(Deserialize)]
struct Query {
    query: String,
}

#[derive(Serialize)]
struct Response {
    result: String,
}

#[post("/search", data = "<query>")]
fn search(query: Json<Query>) -> Json<Response> {
    // Print the query to console/CLI
    println!("Received query: {}", query.query);

    // Return "ok" response
    Json(Response {
        result: "ok".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![search])
}