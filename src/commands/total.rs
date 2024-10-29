use crate::commands::records::FeedingRecord;
use chrono::{Local, NaiveDateTime};
use rusqlite::{params, types::Type, Connection, Result};

/// Shows the total amount of feeding for today, along with each record's ID.
pub fn show_total_today(conn: &Connection) -> Result<()> {
    // Get today's date in YYYY-MM-DD format
    let today = Local::now().naive_local().date();
    let today_str = today.to_string(); // e.g., "2024-10-28"

    // Prepare the SQL statement to select id, datetime, and amount for today's feedings
    let mut stmt =
        conn.prepare("SELECT id, datetime, amount FROM feedings WHERE datetime LIKE ?1")?;

    // Execute the query and map the results to FeedingRecord structs
    let feedings_iter = stmt.query_map(params![format!("{}%", today_str)], |row| {
        let id: i32 = row.get(0)?; // Retrieve 'id' from the first column
        let datetime_str: String = row.get(1)?; // Retrieve 'datetime' from the second column
        let amount: f32 = row.get(2)?; // Retrieve 'amount' from the third column

        // Parse the datetime string into a NaiveDateTime object
        let datetime = NaiveDateTime::parse_from_str(&datetime_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, Type::Text, Box::new(e)))?;

        Ok(FeedingRecord {
            id,
            datetime,
            amount,
        })
    })?;

    let mut total_amount = 0.0;
    println!("Total feedings for today:");

    // Iterate over each feeding record and display its details
    for feeding_result in feedings_iter {
        let feeding = feeding_result?; // Handle potential errors

        println!(
            "ID: {} | {} - {} oz",
            feeding.id,
            feeding.datetime.format("%Y-%m-%d %H:%M"),
            feeding.amount
        );

        total_amount += feeding.amount;
    }

    println!("Total fed today: {} oz", total_amount);

    Ok(())
}
