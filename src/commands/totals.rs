use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use rusqlite::{params, types::Type, Connection, Result};

/// Structure representing a feeding record, including the unique ID.
#[derive(Debug)]
pub struct FeedingRecord {
    pub id: i32,
    pub datetime: NaiveDateTime,
    pub amount: f32,
}

/// Shows feeding records for a specified date or today, along with the total amount fed.
pub fn show_total(conn: &Connection, date: Option<String>) -> Result<()> {
    // Determine the target date based on user input or default to today
    let target_date = match date {
        Some(date_str) => {
            // Check if the year is provided as two digits
            let formatted_date_str = if date_str.len() == 8 && date_str[6..8].parse::<i32>().is_ok() {
                // Convert "MM/DD/YY" to "MM/DD/20YY"
                let (month_day, year_suffix) = date_str.split_at(6);
                format!("{}20{}", month_day, year_suffix)
            } else {
                date_str
            };

            // Now parse the adjusted date string as a four-digit year
            NaiveDate::parse_from_str(&formatted_date_str, "%m/%d/%Y")
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?
        }
        None => Local::now().naive_local().date(), // Default to today's date
    };

    let date_str = target_date.to_string(); // Format: "YYYY-MM-DD"

    // Prepare the SQL statement to select id, datetime, and amount for the target date
    let mut stmt =
        conn.prepare("SELECT id, datetime, amount FROM feedings WHERE datetime LIKE ?1")?;

    // Execute the query and map the results to FeedingRecord structs
    let feedings_iter = stmt.query_map(params![format!("{}%", date_str)], |row| {
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

    let mut total_amount = 0.0; // Initialize total feeding amount
    println!("Feedings for {}:", target_date);

    // Iterate over each feeding record and display its details
    for feeding_result in feedings_iter {
        let feeding = feeding_result?; // Handle potential errors

        println!(
            "ID: {} | {} - {} oz",
            feeding.id,
            feeding.datetime.format("%Y-%m-%d %H:%M"),
            feeding.amount
        );

        total_amount += feeding.amount; // Accumulate the total amount
    }

    // Display the total feeding amount for the target date
    println!("Total fed on {}: {} oz", target_date, total_amount);

    Ok(())
}
