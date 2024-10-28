use chrono::Local;
use rusqlite::{params, Connection, Result};

/// Adds a feeding record with a specified amount to the database.
pub fn add_feeding(conn: &Connection, amount: f32) -> Result<()> {
    let today = Local::now().date_naive();
    let today_str = today.to_string();
    conn.execute(
        "INSERT INTO feedings (date, amount) VALUES (?1, ?2)",
        params![today_str, amount],
    )?;
    println!("Added feeding record: {} oz", amount);
    Ok(())
}
