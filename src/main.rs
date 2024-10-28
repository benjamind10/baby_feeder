use chrono::Local;
use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

// Import commands from the commands module
mod commands;
use commands::{add::add_feeding, records::get_feedings, total::get_daily_total};

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
    /// Add a feeding record with a specified amount
    Add {
        /// Amount of feeding in oz
        amount: f32,
    },
    /// Show the total amount fed today
    Total,
    /// Show all feeding records for today
    Records,
}

impl Commands {
    /// Executes the logic for each command.
    fn run(&self, conn: &Connection) -> Result<()> {
        match self {
            Commands::Add { amount } => {
                add_feeding(conn, *amount)?;
            }
            Commands::Total => {
                let today = Local::now().date_naive();
                let total_today = get_daily_total(conn, today)?;
                println!("Total fed today: {} oz", total_today);
            }
            Commands::Records => {
                let today = Local::now().date_naive();
                let feedings_today = get_feedings(conn, today)?;
                println!("Feedings for today:");
                for feeding in feedings_today {
                    println!("{:?}", feeding);
                }
            }
        }
        Ok(())
    }
}

/// Initialize database and create table if it doesn't exist
fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS feedings (
            id INTEGER PRIMARY KEY,
            date TEXT NOT NULL,
            amount REAL NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Connect to SQLite database
    let conn = Connection::open("feeding_tracker.db")?;
    initialize_db(&conn)?;

    // Run the specified command
    cli.command.run(&conn)?;

    Ok(())
}
