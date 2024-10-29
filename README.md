# Baby Feeding Tracker

The **Baby Feeding Tracker** is a simple CLI application built in Rust to help parents track daily feeding amounts for their baby. The application stores feeding records with date, time, and amount, and provides commands to add, view, and summarize feeding data.

## Features

- **Add Feeding Record**: Add a feeding record with the specified amount in ounces and an optional date-time.
- **View Records**: View feeding records for a specific date or today’s date.
- **Show Daily Total**: Show the total amount fed on a specific day or for today.

## Prerequisites

- **Rust**: Make sure Rust is installed on your system.
- **SQLite**: The application uses an SQLite database to store records (handled internally by the app).

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/baby_feeding_tracker.git
cd baby_feeding_tracker
```

## Building the Application

To build the application, use:
```bash
cargo build --release
```
This will create an optimized binary in the target/release directory.

## Running the Application

You can run the application using `cargo run` or by executing the built binary.

### 1. Add a Feeding Record

To add a feeding record, use the `add` command. Provide the amount in ounces and an optional date-time. If no date-time is specified, the current date and time will be used.

**Example with current date and time:**

```bash
cargo run -- add --amount 5.5
```
**Example with a specific date-time:**

```bash
cargo run -- add --amount 4.0 --datetime "10/28/2024 15:30"
```

Date Format: The date-time format for `--datetime` is `MM/DD/YYYY HH:MM`.

### 2. Show Feeding Records

To view feeding records, use the `total` command. You can view today’s records or records for a specific date.

**Example for today’s records:**

```bash
cargo run -- total
```

**Example for records on a specific date:**

```bash
cargo run -- total --date "10/28/2024"
```
Date Format: The date-time format for `--datetime` is `MM/DD/YYYY`.

### 3. Delete Feeding Record

To delete a feedng record, use the `delete` command, You can view records for a today or a specific date with the `total` command.
- Find the ID of the record to delete.
- Run the delete command with the ID to delete.

```bash
cargo run -- delete <id>
```

## Database Structure

The application stores feeding records in an SQLite database (feeding_tracker.db) in the project directory. Each record includes:

- **id:** Auto-incrementing primary key
- **datetime:** Date and time of feeding in the format YYYY-MM-DD HH:MM:SS
- **amount:** Amount fed in ounces (oz)

## Project Structure

- `src/main.rs`: Main entry point, CLI setup, and database initialization.
- `src/commands`: Contains modules for each command (add, records, total), making the code modular and organize

## Example Usage

```bash
# Add a feeding record with 5.5 oz at the current date and time
cargo run -- add --amount 5.5

# Add a feeding record with 4 oz on October 28, 2024, at 3:30 PM
cargo run -- add --amount 4.0 --datetime "10/28/2024 15:30"

# Show all feeding records for today
cargo run -- records

# Show all feeding records for October 28, 2024
cargo run -- records --date "10/28/2024"

# Show the total amount fed today
cargo run -- total
```

## Contributing

Feel free to fork this project, submit issues, and make pull requests. Any contributions are welcome!