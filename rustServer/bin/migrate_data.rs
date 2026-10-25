#[path = "../migration/mod.rs"]
mod migration;

use std::{io, println};
use migration::migrate_v1_v2::DataMigrator;

pub const BASE_DATA_DIR: &str = "data";



fn main() -> io::Result<()> {
    match DataMigrator::migrate_data() {
        Ok(_) => {
            println!("Data migration completed successfully.");
        }
        Err(e) => {
            match e.kind() {
                io::ErrorKind::Interrupted => {
                    println!("{}", e);
                }
                _ => {
                    eprintln!("Data migration failed: {}", e);
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}