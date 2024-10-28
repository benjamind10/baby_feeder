use chrono::NaiveDate;
use rusqlite::{params, Connection, Result};

/// Struct representing a daily feeding record
#[derive(Debug)]
pub struct FeedingRecord {
    pub date: NaiveDate,
    pub amount: f32,
}

/// Retrieves all feeding records for a specific day.
pub fn get_feedings(conn: &Connection, date: NaiveDate) -> Result<Vec<FeedingRecord>> {
    let date_str = date.to_string();
    let mut stmt = conn.prepare("SELECT date, amount FROM feedings WHERE date = ?1")?;

    let feedings_iter = stmt.query_map(params![date_str], |row| {
        let date_str: String = row.get(0)?;
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;
        let amount = row.get(1)?;
        Ok(FeedingRecord { date, amount })
    })?;

    let feedings: Vec<FeedingRecord> = feedings_iter.collect::<Result<_, _>>()?;
    Ok(feedings)
}
