use axum::{response::IntoResponse, routing::get, Router};
use axum_inertia::{vite, Inertia};
use serde_json::json;

#[tokio::main]
async fn main() {
    let inertia = vite::Development::default()
        .port(5173)
        .main("src/main.ts")
        .lang("en")
        .title("My inertia app")
        .into_config();
    // build our application with a single route
    let app: Router = Router::new().route("/", get(get_root)).with_state(inertia);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_root(i: Inertia) -> impl IntoResponse {
    i.render(
        "Pages/Home",
        json!({ "posts": vec!["post one", "post two"] }),
    )
}
