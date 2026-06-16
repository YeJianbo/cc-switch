use crate::codex_config::get_codex_config_dir;
use rusqlite::Connection;
use std::path::PathBuf;

#[tauri::command]
pub async fn clean_codex_database() -> Result<u64, String> {
    tokio::task::spawn_blocking(clean_codex_database_internal)
        .await
        .map_err(|e| format!("Task joined failed: {e}"))
}

pub fn clean_codex_database_internal() -> u64 {
    let codex_dir = get_codex_config_dir();
    let mut total_freed: u64 = 0;

    // 1. Clean state_5.sqlite
    let state_5_path = codex_dir.join("state_5.sqlite");
    if state_5_path.exists() {
        let backup_path = codex_dir.join("state_5.sqlite.bak");
        let _ = std::fs::copy(&state_5_path, &backup_path);

        let before_size = std::fs::metadata(&state_5_path).map(|m| m.len()).unwrap_or(0);
        if let Ok(conn) = Connection::open(&state_5_path) {
            let _ = conn.execute_batch("VACUUM;");
        }
        let after_size = std::fs::metadata(&state_5_path).map(|m| m.len()).unwrap_or(0);
        if before_size > after_size {
            total_freed += before_size - after_size;
        }
    }

    // 2. Clean globalStorage/state.vscdb
    let mut cursor_global_storage = None;

    #[cfg(target_os = "windows")]
    {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            cursor_global_storage = Some(
                PathBuf::from(appdata)
                    .join("Cursor")
                    .join("User")
                    .join("globalStorage"),
            );
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            cursor_global_storage = Some(
                PathBuf::from(home)
                    .join("Library/Application Support/Cursor/User/globalStorage"),
            );
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            cursor_global_storage =
                Some(PathBuf::from(home).join(".config/Cursor/User/globalStorage"));
        }
    }

    if let Some(global_storage) = cursor_global_storage {
        let vscdb_path = global_storage.join("state.vscdb");
        if vscdb_path.exists() {
            let backup_path = global_storage.join("state.vscdb.bak");
            let _ = std::fs::copy(&vscdb_path, &backup_path);

            let before_size = std::fs::metadata(&vscdb_path).map(|m| m.len()).unwrap_or(0);
            if let Ok(conn) = Connection::open(&vscdb_path) {
                let _ = conn.execute(
                    "DELETE FROM cursorDiskKV WHERE key LIKE 'agentKv:%' OR key LIKE 'bubbleId:%'",
                    [],
                );
                let _ = conn.execute_batch("VACUUM;");
            }
            let after_size = std::fs::metadata(&vscdb_path).map(|m| m.len()).unwrap_or(0);
            if before_size > after_size {
                total_freed += before_size - after_size;
            }
        }
    }

    total_freed
}
