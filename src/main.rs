mod dbutils;
mod fsutils;
mod schema;

use dbutils::dbutils::*;
use dotenv::dotenv;
use fsutils::fsutils::*;

use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), sqlx::Error> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            env::var("DATABASE_URL")
                .expect("Couldn't connect to the database url provided")
                .as_str(),
        )
        .await?;

    let files_from_fs = read_files_from_fs("./")
        .await
        .expect("No files found in directory");

    write_files_to_db(&pool, &files_from_fs)
        .await
        .expect("Couldn't write files to db");

    let files_from_db = read_files_from_db(&pool)
        .await
        .expect("No files found in db");

    println!("Files from DB:");
    for file in &files_from_db {
        println!("{:?}", file);
    }

    println!();

    println!("Files from FS:");
    for file in &files_from_fs {
        println!("{:?}", file);
    }

    println!();

    Ok(())
}
