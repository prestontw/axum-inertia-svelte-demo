use std::sync::Arc;

use axum::{
    extract::{FromRef, State},
    response::{IntoResponse, Redirect},
    Json, Router,
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
    let app: Router = Router::new()
        .typed_get(get_root)
        .typed_post(add_user)
        .typed_put(update_user)
        .with_state(app_state);

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

#[derive(Deserialize, TypedPath)]
#[typed_path("/users")]
struct Users;

#[derive(Deserialize, TypedPath)]
#[typed_path("/users/:id")]
struct UserId {
    id: u32,
}

#[typeshare]
#[derive(Serialize, Clone, Deserialize)]
struct User {
    id: u32,
    name: String,
    titles: Vec<String>,
}

#[typeshare]
#[derive(Serialize, Clone, Deserialize)]
struct NewUser {
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

async fn add_user(
    _: Users,
    State(AppState { users, .. }): State<AppState>,
    Json(new_user): Json<NewUser>,
) -> impl IntoResponse {
    let mut users = users.lock_owned().await.clone();
    let max_id = users.iter().map(|user| user.id).max().unwrap_or(0);
    let new_user = User {
        id: max_id + 1,
        name: new_user.name,
        titles: new_user.titles,
    };
    users.push(new_user);
    Redirect::to(&Root.to_string())
}

async fn update_user(
    UserId { id }: UserId,
    State(AppState { users, .. }): State<AppState>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    if user.id != id {
        return Err("body did not match path");
    }
    let mut users = users.lock_owned().await.clone();
    let original_user = users
        .iter_mut()
        .find(|user| user.id == id)
        .ok_or("Could not find user with id")?;
    *original_user = user;
    Ok(Redirect::to(&Root.to_string()))
}
