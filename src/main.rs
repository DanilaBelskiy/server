use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

use egui::{SliderOrientation, Ui, Vec2};
use egui::style::Spacing;

pub mod gui;
use gui::TemplateApp;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    //Create a connection pull

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:super_pass@localhost/test")
        .await?;

    //Collect data from database

    let select_query = sqlx::query("SELECT * FROM users");
    let dudes: Vec<User> = select_query
        .map(|row: PgRow| User {
            id: row.get("id"),
            name: row.get("name"),
            login: row.get("login"),
            password: row.get("password"),
        })
        .fetch_all(&pool)
        .await?;

    let mut connected_users:Vec<ConnectedUser> = Vec::new();

    for i in 0..dudes.len(){
        connected_users.push(ConnectedUser::from(&dudes[i]))
    }

    println!("{:?}", connected_users);

    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(cc, connected_users)))
    );

    Ok(())

}

#[derive(Debug, FromRow)]
struct User {
    id: i64,
    name: String,
    login: String,
    password: String,
}

#[derive(Debug)]
pub struct ConnectedUser {
    id: i64,
    name: String,
    login: String,
    password: String,
}

impl ConnectedUser {
    fn from(user: &User) -> ConnectedUser {
        let id = user.id.clone();
        let name = user.name.clone();
        let login = user.login.clone();
        let password = user.password.clone();
        ConnectedUser{
            id,
            name,
            login,
            password,
        }
    }

    pub fn clone(user: ConnectedUser) -> ConnectedUser {
        ConnectedUser{
            id: user.id,
            name: user.name,
            login: user.login,
            password: user.password,
        }
    }
}
