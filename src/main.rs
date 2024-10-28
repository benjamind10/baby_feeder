use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use std::error::Error;

// Import commands module
mod commands;
use commands::{add::add_feeding, records::show_records, total::show_total_today};

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
    /// Show total feeding amount for today
    Total,
    /// Show all feeding records for today or a specified date
    Records {
        /// Date to retrieve records for (format: MM/DD/YYYY)
        #[arg(short, long)]
        date: Option<String>,
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
        Commands::Total => {
            show_total_today(&conn)?;
        }
        Commands::Records { date } => {
            show_records(&conn, date)?;
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
