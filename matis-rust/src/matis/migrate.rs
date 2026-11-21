//! Apply outstanding migrations to the configured SQLite database.

fn main() {
    if let Err(error) = matis::bootstrap::migrate_database(None) {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
