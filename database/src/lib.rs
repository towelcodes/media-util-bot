pub mod models;
pub mod schema;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::env;
use std::error::Error;

use crate::models::{LlmContext, NewLlmContext};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

/// Establishes a connection pool and automatically runs pending migrations.
pub fn establish_connection_pool() -> DbPool {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment variables");

    // Create database if it doesn't exist
    if !database_exists(&database_url) {
        create_database(&database_url).expect("Failed to create database");
    }

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool");

    // Run migrations immediately upon pool creation
    run_migrations(
        &mut pool
            .get()
            .expect("Failed to get connection from pool for migrations"),
    )
    .expect("Failed to run database migrations");

    pool
}

/// Helper function to run embedded migrations.
fn run_migrations(
    connection: &mut MysqlConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

// Helper functions to create the database if it doesn't exist.
// This is a common pattern for Diesel applications.
fn database_exists(database_url: &str) -> bool {
    MysqlConnection::establish(database_url).is_ok()
}

fn create_database(database_url: &str) -> Result<(), Box<dyn Error>> {
    let (db_name, url_without_db) = {
        let mut url_parts: Vec<&str> = database_url.split('/').collect();
        let db_name = url_parts.pop().unwrap_or("");
        (db_name.to_string(), url_parts.join("/"))
    };

    let mut conn = MysqlConnection::establish(&url_without_db)?;
    diesel::sql_query(format!("CREATE DATABASE IF NOT EXISTS {}", db_name)).execute(&mut conn)?;

    Ok(())
}

pub fn create_context(conn: &mut MysqlConnection, uid: u64, context: String) -> LlmContext {
    use crate::schema::llm_context;
    let new_context = NewLlmContext { uid, context };

    conn.transaction(|conn| {
        diesel::insert_into(llm_context::table)
            .values(&new_context)
            .execute(conn)?;

        llm_context::table
            .order(llm_context::last_updated.desc())
            .first(conn)
    })
    .expect("Failed to create LLM context record")
}

pub fn get_context(conn: &mut MysqlConnection, uid: u64) -> LlmContext {
    use crate::schema::llm_context;
    let mut result: LlmContext = llm_context::table
        .find(uid)
        .first(conn)
        .unwrap_or_else(|_| create_context(conn, uid, String::from("[]")));

    let now = chrono::Utc::now().naive_utc();
    if now.signed_duration_since(result.last_updated).num_hours() >= 1 {
        diesel::update(llm_context::table.find(uid))
            .set(llm_context::context.eq(""))
            .execute(conn)
            .expect("Failed to update context");

        result = llm_context::table
            .find(uid)
            .first(conn)
            .expect("Failed to find context");
    }

    result
}

pub fn update_context(conn: &mut MysqlConnection, uid: u64, context: String) -> LlmContext {
    use crate::schema::llm_context;
    conn.transaction(|conn| {
        diesel::update(llm_context::table.find(uid))
            .set((
                llm_context::context.eq(context),
                llm_context::last_updated.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        llm_context::table.find(uid).first(conn)
    })
    .expect("Failed to update LLM context record")
}

// pub fn marry(
//     conn: &mut MysqlConnection,
//     proposed: u64,
//     accepted: u64,
//     propose_message: Option<String>,
// ) -> Marriage {
//     use crate::schema::marriage_list;
//     let new_marriage = NewMarraige {
//         proposed,
//         accepted,
//         propose_message,
//     };

//     conn.transaction(|conn| {
//         diesel::insert_into(marriage_list::table)
//             .values(&new_marriage)
//             .execute(conn)?;

//         marriage_list::table
//             .order(marriage_list::id.desc())
//             .first(conn)
//     })
//     .expect("Failed to create marriage record")
// }
