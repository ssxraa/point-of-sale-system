use serde::{Serialize, Deserialize};
use tauri::State;
use rusqlite::params;
use crate::DbConn;

#[derive(Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub stock: u32,
}

#[tauri::command]
pub fn get_inventory(db: State<DbConn>) -> Result<Vec<InventoryItem>, String> {
    let conn = db.conn().lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, price, stock FROM products").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(InventoryItem {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            stock: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;
    let items: Result<Vec<_>, _> = rows.collect();
    items.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_product(
    db: State<DbConn>,
    name: String,
    price: f64,
    stock: u32,
) -> Result<InventoryItem, String> {
    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO products (name, price, stock) VALUES (?, ?, ?)",
        params![name, price, stock],
    ).map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();
    Ok(InventoryItem { id, name, price, stock })
}

#[tauri::command]
pub fn update_product(
    db: State<DbConn>,
    id: i64,
    name: String,
    price: f64,
    stock: u32,
) -> Result<InventoryItem, String> {
    let conn = db.conn().lock().unwrap();
    let updated = conn.execute(
        "UPDATE products SET name = ?, price = ?, stock = ? WHERE id = ?",
        params![name, price, stock, id],
    ).map_err(|e| e.to_string())?;
    if updated == 0 {
        Err("Product not found".to_string())
    } else {
        Ok(InventoryItem { id, name, price, stock })
    }
}

#[tauri::command]
pub fn delete_product(
    db: State<DbConn>,
    id: i64,
) -> Result<(), String> {
    let conn = db.conn().lock().unwrap();
    let deleted = conn.execute(
        "DELETE FROM products WHERE id = ?",
        params![id],
    ).map_err(|e| e.to_string())?;
    if deleted == 0 {
        Err("Product not found".to_string())
    } else {
        Ok(())
    }
}