#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod inventory;
mod settings;
mod reports;
mod pos;
mod auth; 
use std::sync::Mutex;
use tauri::Manager;
use rusqlite::Connection;

pub struct DbConn(pub std::sync::Mutex<rusqlite::Connection>);

impl DbConn {
    pub fn conn(&self) -> &std::sync::Mutex<rusqlite::Connection> {
        &self.0
    }
}

fn main() {
    let conn = db::establish_connection().expect("Failed to open DB");
    db::init_db(&conn).expect("Failed to init DB");

    tauri::Builder::default()
        .manage(DbConn(Mutex::new(conn)))
        .invoke_handler(tauri::generate_handler![
            inventory::get_inventory,
            inventory::add_product,
            inventory::update_product,
            inventory::delete_product,
            settings::load_settings,
            settings::save_settings,
            settings::perform_backup,
            settings::restore_backup,
            settings::test_print,
            settings::find_printers,
            reports::get_sales_transactions,
            reports::get_product_performance,
            reports::get_low_stock_products,
            reports::get_revenue_overview,
            pos::checkout,
            auth::login,
            auth::set_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}