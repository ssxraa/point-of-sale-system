use serde::{Serialize, Deserialize};
use tauri::State;
use rusqlite::{params};
use crate::DbConn;

#[derive(Serialize, Deserialize, Clone)]
pub struct StoreSettings {
    pub store_name: String,
    pub store_address: String,
    pub store_phone: String,
    pub store_email: String,
    pub receipt_header: String,
    pub receipt_footer: String,
    pub show_logo: bool,
    pub backup_frequency: String,
    pub printer_model: String,
}

fn get_setting(conn: &rusqlite::Connection, key: &str, default: &str) -> String {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?",
        params![key],
        |row| row.get(0)
    ).unwrap_or(default.to_string())
}

fn set_setting(conn: &rusqlite::Connection, key: &str, value: &str) {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![key, value]
    ).ok();
}

#[tauri::command]
pub fn load_settings(db: State<DbConn>) -> Result<StoreSettings, String> {
    let conn = db.conn().lock().unwrap();
    Ok(StoreSettings {
        store_name: get_setting(&conn, "store_name", "Your Fabulous Store Name"),
        store_address: get_setting(&conn, "store_address", "123 Slay Street, City, Country"),
        store_phone: get_setting(&conn, "store_phone", "+123 456 7890"),
        store_email: get_setting(&conn, "store_email", "contact@yourstore.com"),
        receipt_header: get_setting(&conn, "receipt_header", "Thank you for your purchase!"),
        receipt_footer: get_setting(&conn, "receipt_footer", "Visit us again soon!"),
        show_logo: get_setting(&conn, "show_logo", "true") == "true",
        backup_frequency: get_setting(&conn, "backup_frequency", "daily"),
        printer_model: get_setting(&conn, "printer_model", "Epson TM-T88VI"),
    })
}

#[tauri::command]
pub fn save_settings(
    db: State<DbConn>,
    settings: StoreSettings,
) -> Result<(), String> {
    let conn = db.conn().lock().unwrap();
    set_setting(&conn, "store_name", &settings.store_name);
    set_setting(&conn, "store_address", &settings.store_address);
    set_setting(&conn, "store_phone", &settings.store_phone);
    set_setting(&conn, "store_email", &settings.store_email);
    set_setting(&conn, "receipt_header", &settings.receipt_header);
    set_setting(&conn, "receipt_footer", &settings.receipt_footer);
    set_setting(&conn, "show_logo", if settings.show_logo { "true" } else { "false" });
    set_setting(&conn, "backup_frequency", &settings.backup_frequency);
    set_setting(&conn, "printer_model", &settings.printer_model);
    Ok(())
}

#[tauri::command]
pub fn perform_backup(_db: State<DbConn>) -> Result<(), String> {
    // TODO: Implement real backup (copy pos.db)
    Ok(())
}

#[tauri::command]
pub fn restore_backup(_db: State<DbConn>) -> Result<(), String> {
    // TODO: Implement real restore (replace pos.db)
    Ok(())
}

#[tauri::command]
pub fn test_print(_db: State<DbConn>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn find_printers(_db: State<DbConn>) -> Result<Vec<String>, String> {
    Ok(vec![
        "Epson TM-T88VI".to_string(),
        "HP DeskJet 2700".to_string(),
        "Canon Pixma G2010".to_string(),
    ])
}