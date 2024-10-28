use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};

/// Retrieves the total feeding amount for a specific day.
pub fn get_daily_total(conn: &Connection, date: NaiveDate) -> Result<f32> {
    let date_str = date.to_string();
    let mut stmt = conn.prepare("SELECT SUM(amount) FROM feedings WHERE date = ?1")?;
    let total: f32 = stmt
        .query_row(params![date_str], |row| row.get(0))
        .unwrap_or(0.0);
    Ok(total)
}
