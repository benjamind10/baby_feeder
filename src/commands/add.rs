use chrono::{Local, NaiveDateTime, NaiveTime};
use rusqlite::{params, types::Type, Connection, Result};

/// Adds a new feeding record with the specified amount and optional datetime.
/// - If both date and time are provided, use them directly.
/// - If only time is provided, use today's date and the specified time.
/// - If neither is provided, use the current date and time.
pub fn add_feeding(conn: &Connection, amount: f32, datetime: Option<String>) -> Result<()> {
    // Determine the datetime for the feeding entry
    let datetime = match datetime {
        Some(dt_str) => {
            // Attempt to parse full date and time (MM/DD/YYYY HH:MM)
            if let Ok(parsed_dt) = NaiveDateTime::parse_from_str(&dt_str, "%m/%d/%Y %H:%M") {
                parsed_dt
            }
            // If parsing failed, attempt to parse only time (HH:MM) for today
            else if let Ok(parsed_time) = NaiveTime::parse_from_str(&dt_str, "%H:%M") {
                let today = Local::now().naive_local().date();
                NaiveDateTime::new(today, parsed_time)
            }
            // If both parsing attempts fail, return an error
            else {
                return Err(rusqlite::Error::FromSqlConversionFailure(
                    0,
                    Type::Text,
                    Box::new(std::fmt::Error),
                ));
            }
        }
        None => Local::now().naive_local(), // Use current date and time if none provided
    };

    // Insert the feeding record into the database
    conn.execute(
        "INSERT INTO feedings (datetime, amount) VALUES (?1, ?2)",
        params![datetime.format("%Y-%m-%d %H:%M:%S").to_string(), amount],
    )?;
    println!("Added feeding record: {} oz at {}", amount, datetime);
    Ok(())
}
