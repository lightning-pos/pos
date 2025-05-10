use libsql::Connection;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Custom migration function that properly handles SQL files with multiple statements
pub async fn run_migrations(conn: &Connection) -> Result<bool, Box<dyn std::error::Error>> {
    // Specify the path to the migrations folder
    let migrations_folder = PathBuf::from("./migrations");
    
    // Ensure the migrations table exists
    ensure_migrations_table(conn).await?;
    
    // Find and sort SQL files
    let files_to_run = find_sql_files(migrations_folder.clone())?;
    
    if files_to_run.is_empty() {
        return Ok(false);
    }
    
    let mut did_new_migration = false;
    
    // Process each migration file
    for file_path in files_to_run {
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        
        // Check if migration has already been applied
        if !is_migration_applied(conn, &file_name).await? {
            // Read the SQL file content
            let sql_content = fs::read_to_string(&file_path)?;
            
            // Begin transaction
            conn.execute_batch("BEGIN TRANSACTION").await?;
            
            match conn.execute_batch(&sql_content).await {
                Ok(_) => {
                    // Record the migration as applied
                    record_migration(conn, &file_name).await?;
                    eprintln!("Applied migration: {}", file_name);
                    did_new_migration = true;
                    
                    // Commit transaction
                    conn.execute_batch("COMMIT").await?;
                },
                Err(e) => {
                    // Rollback on error
                    let _ = conn.execute_batch("ROLLBACK").await;
                    return Err(Box::new(e));
                }
            }
        } else {
            eprintln!("Migration already applied: {}", file_name);
        }
    }
    
    Ok(did_new_migration)
}

/// Ensure the migrations table exists
async fn ensure_migrations_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let create_table_sql = r#"
    CREATE TABLE IF NOT EXISTS libsql_migrations (
        id TEXT PRIMARY KEY,
        status INTEGER DEFAULT 0,
        exec_time TIMESTAMP
    );
    "#;
    
    conn.execute_batch(create_table_sql).await?;
    Ok(())
}

/// Check if a migration has already been applied
async fn is_migration_applied(conn: &Connection, migration_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let mut stmt = conn
        .prepare("SELECT status FROM libsql_migrations WHERE id = ?")
        .await?;
    
    let mut rows = stmt.query([migration_id]).await?;
    
    if let Some(row) = rows.next().await? {
        let status: i64 = row.get(0)?;
        Ok(status == 1)
    } else {
        Ok(false)
    }
}

/// Record a migration as applied
async fn record_migration(conn: &Connection, migration_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT INTO libsql_migrations (id, status, exec_time) VALUES (?, 1, CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET status = 1, exec_time = CURRENT_TIMESTAMP",
        libsql::params![migration_id],
    )
    .await?;
    
    Ok(())
}

/// Find and sort SQL files in the migrations directory
fn find_sql_files(dir_path: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut file_paths: Vec<PathBuf> = vec![];
    
    if !dir_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Migrations directory not found: {:?}", dir_path),
        ));
    }
    
    for entry in fs::read_dir(dir_path)? {
        let entry_path = entry?.path();
        
        if entry_path.is_file() && 
           entry_path.extension().map_or(false, |ext| ext == "sql") {
            file_paths.push(entry_path);
        }
    }
    
    // Sort files by name to ensure correct order
    file_paths.sort_by(|a, b| {
        let a_name = a.file_name().unwrap_or_default();
        let b_name = b.file_name().unwrap_or_default();
        a_name.cmp(b_name)
    });
    
    Ok(file_paths)
}
