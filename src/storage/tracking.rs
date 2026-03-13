//! Commit tracking and analytics

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::config::get_config_dir;

const DB_FILE: &str = "history.db";

lazy_static::lazy_static! {
    static ref TRACKER: Arc<Mutex<Option<Arc<Tracker>>>> = Arc::new(Mutex::new(None));
}

/// Get or create the tracker singleton
pub fn get_tracker() -> Result<Arc<Tracker>> {
    let mut guard = TRACKER.lock()
        .map_err(|_| anyhow::anyhow!("Tracker lock poisoned"))?;

    if guard.is_none() {
        *guard = Some(Arc::new(Tracker::new()?));
    }

    // Get the Arc containing the tracker and clone it
    let tracker_arc = guard.as_ref().unwrap();
    Ok(Arc::clone(tracker_arc))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingEvent {
    pub timestamp: DateTime<Utc>,
    pub repo_path: String,
    pub commit_type: String,
    pub version: String,
    pub project: String,
    pub message: String,
}

pub struct Tracker {
    conn: Arc<Mutex<Connection>>,
}

impl Tracker {
    /// Create a new tracker
    pub fn new() -> Result<Self> {
        let db_path = get_config_dir()?.join(DB_FILE);
        let conn = Connection::open(&db_path)
            .context("Failed to open tracking database")?;

        let tracker = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        tracker.init_schema()?;

        Ok(tracker)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock()
            .map_err(|_| anyhow::anyhow!("Connection lock poisoned"))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS commits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                repo_path TEXT NOT NULL,
                commit_type TEXT NOT NULL,
                version TEXT NOT NULL,
                project TEXT NOT NULL,
                message TEXT NOT NULL,
                hash TEXT
            )",
            [],
        ).context("Failed to create commits table")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                event_type TEXT NOT NULL,
                details TEXT
            )",
            [],
        ).context("Failed to create events table")?;

        // Create indexes
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_commits_timestamp ON commits(timestamp)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_commits_repo ON commits(repo_path)",
            [],
        )?;

        Ok(())
    }

    /// Track a commit
    pub fn track_commit(
        &self,
        event: &TrackingEvent,
    ) -> Result<()> {
        let conn = self.conn.lock()
            .map_err(|_| anyhow::anyhow!("Connection lock poisoned"))?;

        conn.execute(
            "INSERT INTO commits (timestamp, repo_path, commit_type, version, project, message)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                event.timestamp.to_rfc3339(),
                event.repo_path.as_str(),
                event.commit_type.as_str(),
                event.version.as_str(),
                event.project.as_str(),
                event.message.as_str(),
            ],
        ).context("Failed to insert commit")?;

        // Cleanup old records (90 days)
        let cutoff = Utc::now() - chrono::Duration::days(90);
        conn.execute(
            "DELETE FROM commits WHERE timestamp < ?1",
            params![cutoff.to_rfc3339()],
        )?;

        Ok(())
    }

    /// Get commit statistics
    pub fn get_stats(&self, repo_path: Option<&str>) -> Result<CommitStats> {
        let conn = self.conn.lock()
            .map_err(|_| anyhow::anyhow!("Connection lock poisoned"))?;

        // Use dynamic parameters based on whether repo_path is provided
        if let Some(path) = repo_path {
            let query = "SELECT
                COUNT(*) as total,
                SUM(CASE WHEN commit_type = 'RELEASE' THEN 1 ELSE 0 END) as releases,
                SUM(CASE WHEN commit_type = 'UPDATE' THEN 1 ELSE 0 END) as updates,
                SUM(CASE WHEN commit_type = 'PATCH' THEN 1 ELSE 0 END) as patches
             FROM commits WHERE repo_path = ?1";

            let mut stmt = conn.prepare(query)?;

            let result = stmt.query_row(
                &[&path],
                |row| {
                    Ok(CommitStats {
                        total: row.get(0)?,
                        releases: row.get(1).unwrap_or(0),
                        updates: row.get(2).unwrap_or(0),
                        patches: row.get(3).unwrap_or(0),
                    })
                },
            )?;

            Ok(result)
        } else {
            let query = "SELECT
                COUNT(*) as total,
                SUM(CASE WHEN commit_type = 'RELEASE' THEN 1 ELSE 0 END) as releases,
                SUM(CASE WHEN commit_type = 'UPDATE' THEN 1 ELSE 0 END) as updates,
                SUM(CASE WHEN commit_type = 'PATCH' THEN 1 ELSE 0 END) as patches
             FROM commits";

            let mut stmt = conn.prepare(query)?;

            let result = stmt.query_row(
                [],
                |row| {
                    Ok(CommitStats {
                        total: row.get(0)?,
                        releases: row.get(1).unwrap_or(0),
                        updates: row.get(2).unwrap_or(0),
                        patches: row.get(3).unwrap_or(0),
                    })
                },
            )?;

            Ok(result)
        }
    }

    /// Get recent commits
    pub fn get_recent_commits(&self, limit: usize) -> Result<Vec<TrackingEvent>> {
        let conn = self.conn.lock()
            .map_err(|_| anyhow::anyhow!("Connection lock poisoned"))?;

        let mut stmt = conn.prepare(
            "SELECT timestamp, repo_path, commit_type, version, project, message
             FROM commits
             ORDER BY timestamp DESC
             LIMIT ?1"
        )?;

        let rows = stmt.query_map(
            [limit as i64],
            |row| {
                Ok(TrackingEvent {
                    timestamp: row.get(0)?,
                    repo_path: row.get(1)?,
                    commit_type: row.get(2)?,
                    version: row.get(3)?,
                    project: row.get(4)?,
                    message: row.get(5)?,
                })
            },
        )?;

        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }

        Ok(events)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitStats {
    pub total: i64,
    pub releases: i64,
    pub updates: i64,
    pub patches: i64,
}
