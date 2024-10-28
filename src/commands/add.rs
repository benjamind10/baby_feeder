use chrono::{Local, NaiveDateTime};
use rusqlite::{params, types::Type, Connection, Result};

/// Adds a new feeding record with the specified amount and optional datetime.
pub fn add_feeding(conn: &Connection, amount: f32, datetime: Option<String>) -> Result<()> {
    let datetime = match datetime {
        Some(dt_str) => NaiveDateTime::parse_from_str(&dt_str, "%m/%d/%Y %H:%M")
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?,
        None => Local::now().naive_local(),
    };

    conn.execute(
        "INSERT INTO feedings (datetime, amount) VALUES (?1, ?2)",
        params![datetime.format("%Y-%m-%d %H:%M:%S").to_string(), amount],
    )?;
    println!("Added feeding record: {} oz at {}", amount, datetime);
    Ok(())
}
