use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::error::Error;

// Import commands module
mod commands;
use commands::{add::add_feeding, delete::delete_record, totals::show_total};

/// CLI Arguments for the feeding tracker application
#[derive(Parser)]
#[command(name = "Feeding Tracker")]
#[command(about = "Track daily feeding amounts for your baby")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a feeding record with a specified amount in ounces
    Add {
        /// Amount of feeding in ounces
        amount: f32,
        /// Date and time of feeding (format: MM/DD/YYYY HH:MM)
        #[arg(short, long)]
        datetime: Option<String>,
    },
    /// Show feeding records and total amount for today or a specified date
    Total {
        /// Date to retrieve total feeding for (format: MM/DD/YYYY)
        #[arg(short, long)]
        date: Option<String>,
    },
    /// Delete a feeding record by its ID
    Delete {
        /// ID of the feeding record to delete
        id: i32,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Connect to SQLite database (creates a new database file if it doesn't exist)
    let conn = Connection::open("feeding_tracker.db")?;
    initialize_db(&conn)?;

    // Process commands
    match cli.command {
        Commands::Add { amount, datetime } => {
            add_feeding(&conn, amount, datetime)?;
        }
        Commands::Total { date } => {
            show_total(&conn, date)?;
        }
        Commands::Delete { id } => {
            delete_record(&conn, id)?;
        }
    }

    Ok(())
}

/// Initialize the database and create table if it doesn't exist
fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS feedings (
            id INTEGER PRIMARY KEY,
            datetime TEXT NOT NULL,
            amount REAL NOT NULL
        )",
        [],
    )?;
    Ok(())
}


// command that will list time between feeding and an average for the day
