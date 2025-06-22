use tauri::State;
use rusqlite::params;
use crate::DbConn;
// NEW imports for argon2
use rand::rngs::OsRng;
use argon2::password_hash::{SaltString, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::Argon2;

#[tauri::command]
pub fn login(db: State<DbConn>, username: String, password: String) -> Result<bool, String> {
    let conn = db.conn().lock().unwrap();
    let mut stmt = conn.prepare("SELECT password_hash FROM users WHERE username = ?").map_err(|e| e.to_string())?;
    let pw_hash: String = match stmt.query_row(params![username], |row| row.get(0)) {
        Ok(hash) => hash,
        Err(_) => return Ok(false), // user not found
    };

    let parsed_hash = PasswordHash::new(&pw_hash).map_err(|e| e.to_string())?;
    let argon2 = Argon2::default();
    let is_valid = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();
    Ok(is_valid)
}

#[tauri::command]
pub fn set_password(db: State<DbConn>, username: String, new_password: String) -> Result<(), String> {
    let conn = db.conn().lock().unwrap();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(new_password.as_bytes(), &salt).map_err(|e| e.to_string())?.to_string();
    let updated = conn.execute("UPDATE users SET password_hash = ? WHERE username = ?", params![hash, username]).map_err(|e| e.to_string())?;
    if updated == 0 {
        Err("User not found".to_string())
    } else {
        Ok(())
    }
}