use rusqlite::{params, Connection, Result};

/// Deletes a feeding record by its unique ID.
pub fn delete_record(conn: &Connection, id: i32) -> Result<()> {
    // Prepare the SQL statement to delete a record by ID
    let rows_affected = conn.execute("DELETE FROM feedings WHERE id = ?1", params![id])?;

    if rows_affected == 0 {
        println!("No feeding record found with ID: {}", id);
    } else {
        println!("Successfully deleted feeding record with ID: {}", id);
    }

    Ok(())
}
