use serde::{Serialize, Deserialize};
use tauri::State;
use rusqlite::{params};
use crate::DbConn;

#[derive(Serialize, Deserialize, Clone)]
pub struct SalesTransaction {
    pub id: i64,
    pub date: String,
    pub total_paid: f64,
    pub items: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductPerformance {
    pub id: i64,
    pub name: String,
    pub sales_count: u32,
    pub revenue: f64,
    pub stock: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RevenueOverview {
    pub daily: f64,
    pub weekly: f64,
    pub monthly: f64,
}

#[tauri::command]
pub fn get_sales_transactions(db: State<DbConn>) -> Result<Vec<SalesTransaction>, String> {
    let conn = db.conn().lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, sale_time, total_paid FROM sales ORDER BY sale_time DESC").map_err(|e| e.to_string())?;
    let sales_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, f64>(2)?))
    }).map_err(|e| e.to_string())?;

    let mut transactions = Vec::new();
    for sale in sales_iter {
        let (id, date, total_paid) = sale.map_err(|e| e.to_string())?;
        let mut items_stmt = conn.prepare(
            "SELECT p.name FROM sale_items si JOIN products p ON si.product_id = p.id WHERE si.sale_id = ?"
        ).map_err(|e| e.to_string())?;
        let items_iter = items_stmt.query_map(params![id], |row| row.get::<_, String>(0)).map_err(|e| e.to_string())?;
        let items: Vec<String> = items_iter.map(|x| x.unwrap_or_default()).collect();
        transactions.push(SalesTransaction { id, date, total_paid, items });
    }
    Ok(transactions)
}

#[tauri::command]
pub fn get_product_performance(db: State<DbConn>) -> Result<Vec<ProductPerformance>, String> {
    let conn = db.conn().lock().unwrap();
    let mut stmt = conn.prepare(
        "
        SELECT p.id, p.name, 
            COALESCE(SUM(si.quantity), 0) AS sales_count, 
            COALESCE(SUM(si.quantity * si.price), 0) AS revenue,
            p.stock
        FROM products p
        LEFT JOIN sale_items si ON si.product_id = p.id
        GROUP BY p.id
        "
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok(ProductPerformance {
            id: row.get(0)?,
            name: row.get(1)?,
            sales_count: row.get::<_, i64>(2)? as u32,
            revenue: row.get(3)?,
            stock: row.get::<_, i64>(4)? as u32,
        })
    }).map_err(|e| e.to_string())?;

    let items: Result<Vec<_>, _> = rows.collect();
    items.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_low_stock_products(db: State<DbConn>) -> Result<Vec<ProductPerformance>, String> {
    let conn = db.conn().lock().unwrap();
    let mut stmt = conn.prepare(
        "
        SELECT p.id, p.name, 
            COALESCE(SUM(si.quantity), 0) AS sales_count, 
            COALESCE(SUM(si.quantity * si.price), 0) AS revenue,
            p.stock
        FROM products p
        LEFT JOIN sale_items si ON si.product_id = p.id
        WHERE p.stock < 5
        GROUP BY p.id
        "
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([], |row| {
        Ok(ProductPerformance {
            id: row.get(0)?,
            name: row.get(1)?,
            sales_count: row.get::<_, i64>(2)? as u32,
            revenue: row.get(3)?,
            stock: row.get::<_, i64>(4)? as u32,
        })
    }).map_err(|e| e.to_string())?;

    let items: Result<Vec<_>, _> = rows.collect();
    items.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_revenue_overview(db: State<DbConn>) -> Result<RevenueOverview, String> {
    let conn = db.conn().lock().unwrap();
    let daily: f64 = conn.query_row(
        "SELECT COALESCE(SUM(total_paid),0) FROM sales WHERE date(sale_time) = date('now','localtime')",
        [],
        |row| row.get(0)
    ).unwrap_or(0.0);
    let weekly: f64 = conn.query_row(
        "SELECT COALESCE(SUM(total_paid),0) FROM sales WHERE date(sale_time) >= date('now','localtime','-6 days')",
        [],
        |row| row.get(0)
    ).unwrap_or(0.0);
    let monthly: f64 = conn.query_row(
        "SELECT COALESCE(SUM(total_paid),0) FROM sales WHERE date(sale_time) >= date('now','localtime','-29 days')",
        [],
        |row| row.get(0)
    ).unwrap_or(0.0);
    Ok(RevenueOverview { daily, weekly, monthly })
}