use crate::commands::records::FeedingRecord;
use chrono::{Local, NaiveDateTime};
use rusqlite::{params, types::Type, Connection, Result};

/// Shows the total amount of feeding for today, along with each record.
pub fn show_total_today(conn: &Connection) -> Result<()> {
    let today = Local::now().naive_local().date();
    let today_str = today.to_string();
    let mut stmt = conn.prepare("SELECT datetime, amount FROM feedings WHERE datetime LIKE ?1")?;
    let feedings_iter = stmt.query_map(params![format!("{}%", today_str)], |row| {
        let datetime: String = row.get(0)?;
        let amount: f32 = row.get(1)?;
        let datetime = NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?;
        Ok(FeedingRecord { datetime, amount })
    })?;

    let mut total_amount = 0.0;
    println!("Total feedings for today:");
    for feeding in feedings_iter {
        let feeding = feeding?;
        println!(
            "{} - {} oz",
            feeding.datetime.format("%Y-%m-%d %H:%M"),
            feeding.amount
        );
        total_amount += feeding.amount;
    }
    println!("Total fed today: {} oz", total_amount);

    Ok(())
}
