use chrono::{Local, NaiveDate, NaiveDateTime};
use rusqlite::{params, types::Type, Connection, Result};

/// Structure representing a feeding record.
#[derive(Debug)]
pub struct FeedingRecord {
    pub datetime: NaiveDateTime,
    pub amount: f32,
}

/// Shows feeding records for today or a specified date.
pub fn show_records(conn: &Connection, date: Option<String>) -> Result<()> {
    let target_date = match date {
        Some(date_str) => NaiveDate::parse_from_str(&date_str, "%m/%d/%Y")
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?,
        None => Local::now().naive_local().date(),
    };

    let date_str = target_date.to_string();
    let mut stmt = conn.prepare("SELECT datetime, amount FROM feedings WHERE datetime LIKE ?1")?;
    let feedings_iter = stmt.query_map(params![format!("{}%", date_str)], |row| {
        let datetime: String = row.get(0)?;
        let amount: f32 = row.get(1)?;
        let datetime = NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?;
        Ok(FeedingRecord { datetime, amount })
    })?;

    println!("Feedings for {}:", target_date);
    for feeding in feedings_iter {
        let feeding = feeding?;
        println!(
            "{} - {} oz",
            feeding.datetime.format("%Y-%m-%d %H:%M"),
            feeding.amount
        );
    }

    Ok(())
}
