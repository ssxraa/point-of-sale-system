use serde::{Serialize, Deserialize};
use tauri::State;
use rusqlite::{params};
use crate::DbConn;

#[derive(Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SalesTransaction {
    pub id: i64,
    pub date: String,
    pub total_paid: f64,
    pub items: Vec<CartItem>,
}

#[tauri::command]
pub fn checkout(
    db: State<DbConn>,
    items: Vec<CartItem>,
) -> Result<SalesTransaction, String> {
    let conn = db.conn().lock().unwrap();

    let total_paid: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();

    conn.execute(
        "INSERT INTO sales (total_paid) VALUES (?)",
        params![total_paid],
    ).map_err(|e| e.to_string())?;
    let sale_id = conn.last_insert_rowid();

    for item in &items {
        conn.execute(
            "INSERT INTO sale_items (sale_id, product_id, quantity, price) VALUES (?, ?, ?, ?)",
            params![sale_id, item.id, item.quantity, item.price],
        ).map_err(|e| e.to_string())?;
        // Update stock
        conn.execute(
            "UPDATE products SET stock = stock - ? WHERE id = ?",
            params![item.quantity, item.id],
        ).map_err(|e| e.to_string())?;
    }

    // Fetch sale date
    let mut stmt = conn.prepare("SELECT sale_time FROM sales WHERE id = ?").map_err(|e| e.to_string())?;
    let date: String = stmt.query_row(params![sale_id], |row| row.get(0)).map_err(|e| e.to_string())?;

    Ok(SalesTransaction {
        id: sale_id,
        date,
        total_paid,
        items,
    })
}