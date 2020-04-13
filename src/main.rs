use {
    sqlx::{prelude::*, Connect, SqliteConnection},
    std::io::{self, prelude::*},
};

#[derive(Debug, sqlx::FromRow)]
struct Item {
    id: i32,
    value: i32,
    name: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = SqliteConnection::connect("sqlite::memory:").await?;
    sqlx::query(
        "CREATE TABLE test (id INTEGER PRIMARY KEY, value INTEGER NOT NULL DEFAULT 0, name TEXT)",
    )
    .execute(&mut conn)
    .await?;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;

        let args = line.split(' ').collect::<Vec<_>>();
        if args.is_empty() {
            continue;
        }

        match args[0].trim() {
            "insert" => {
                sqlx::query("INSERT INTO test DEFAULT VALUES")
                    .execute(&mut conn)
                    .await?;
            }
            "increment" => {
                if args.len() > 1 {
                    sqlx::query("UPDATE test SET value = value + 1 WHERE id = ?")
                        .bind(args[1].parse::<i32>()?)
                        .execute(&mut conn)
                        .await?;
                }
            }
            "rename" => {
                if args.len() > 1 {
                    sqlx::query("UPDATE test SET name = ? WHERE id = ?")
                        .bind(args.get(2))
                        .bind(args[1].parse::<i32>()?)
                        .execute(&mut conn)
                        .await?;
                }
            }
            "rename2" => {
                if args.len() > 1 {
                    sqlx::query("UPDATE test SET name = ? WHERE id = ?")
                        .bind(args[1].parse::<i32>()?)
                        .bind(args.get(2))
                        .execute(&mut conn)
                        .await?;
                }
            }
            "delete" => {
                if args.len() > 1 {
                    sqlx::query("DELETE FROM test WHERE id = ?")
                        .bind(args[1].parse::<i32>()?)
                        .execute(&mut conn)
                        .await?;
                }
            }
            _ => (),
        }

        println!("\tid\tvalue\tname");

        for Item { id, value, name } in sqlx::query_as("SELECT * FROM test")
            .fetch_all(&mut conn)
            .await?
        {
            println!("\t{}\t{}\t{}", id, value, name.unwrap_or_default());
        }
    }

    Ok(())
}
