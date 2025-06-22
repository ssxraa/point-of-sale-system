use rusqlite::{Connection, Result};
// NEW imports for argon2
use rand::rngs::OsRng;
use argon2::password_hash::{SaltString, PasswordHasher};
use argon2::Argon2;

pub fn establish_connection() -> Result<Connection> {
    Connection::open("pos.db")
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            price REAL NOT NULL,
            stock INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS sales (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sale_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            total_paid REAL NOT NULL
        );
        CREATE TABLE IF NOT EXISTS sale_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sale_id INTEGER NOT NULL,
            product_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            price REAL NOT NULL,
            FOREIGN KEY(sale_id) REFERENCES sales(id),
            FOREIGN KEY(product_id) REFERENCES products(id)
        );
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT
        );
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        );
        "
    )?;

    // Check if admin exists; if not, insert default admin/admin
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE username = 'admin'")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    if count == 0 {
        let password = "admin";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
        conn.execute(
            "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
            ["admin", &hash],
        )?;
    }

    Ok(())
}