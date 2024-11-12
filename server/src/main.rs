use std::sync::Arc;

use axum::{
    extract::{FromRef, State},
    response::IntoResponse,
    Router,
};
use axum_extra::routing::{RouterExt, TypedPath};
use axum_inertia::{vite, Inertia, InertiaConfig};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use typeshare::typeshare;

#[derive(Clone)]
struct AppState {
    inertia: InertiaConfig,
    users: Arc<Mutex<Vec<User>>>,
}

impl FromRef<AppState> for InertiaConfig {
    fn from_ref(input: &AppState) -> Self {
        input.inertia.clone()
    }
}

#[tokio::main]
async fn main() {
    let inertia = vite::Development::default()
        .port(5173)
        .main("src/main.ts")
        .lang("en")
        .title("My inertia app")
        .into_config();

    let app_state = AppState {
        inertia,
        users: Arc::new(Mutex::new(vec![
            User {
                id: 1,
                name: "John Smith".into(),
                titles: vec!["Mr".into(), "Esq".into()],
            },
            User {
                id: 2,
                name: "Pokamon".into(),
                titles: vec![],
            },
        ])),
    };
    // build our application with a single route
    let app: Router = Router::new().typed_get(get_root).with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[typeshare]
#[derive(Serialize)]
struct UserShowProps {
    users: Vec<User>,
}

#[derive(Deserialize, TypedPath)]
#[typed_path("/")]
struct Root;

/// Add vector so we can use proper form dependency
#[typeshare]
#[derive(Serialize, Clone)]
struct User {
    id: u32,
    name: String,
    titles: Vec<String>,
}

async fn get_root(
    _: Root,
    i: Inertia,
    State(AppState { users, .. }): State<AppState>,
) -> impl IntoResponse {
    let users = users.lock_owned().await.clone();
    i.render("Home", UserShowProps { users })
}
