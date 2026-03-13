//! Git diff operations

use anyhow::{Context, Result};
use git2::{Repository, Diff, DiffOptions};
use std::path::Path;

/// Get diff between working tree and index
pub fn get_diff(repo_path: &Path, cached: bool) -> Result<String> {
    let repo = Repository::discover(repo_path)?;

    let mut diff_options = DiffOptions::new();

    let tree = if cached {
        // Diff between index and HEAD
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        Some(commit.tree()?)
    } else {
        // Diff between working tree and index
        None
    };

    let diff = if let Some(tree) = tree {
        repo.diff_tree_to_index(Some(&tree), None, Some(&mut diff_options))?
    } else {
        repo.diff_index_to_workdir(None, Some(&mut diff_options))?
    };

    diff_to_string(&diff)
}

/// Get list of staged files
pub fn get_staged_files(repo_path: &Path) -> Result<Vec<String>> {
    let repo = Repository::discover(repo_path)?;
    let index = repo.index()?;
    let mut files = Vec::new();

    for entry in index.iter() {
        if !entry.path.is_empty() {
            let path = std::str::from_utf8(&entry.path)
                .unwrap_or("");
            files.push(path.to_string());
        }
    }

    Ok(files)
}

fn diff_to_string(diff: &Diff) -> Result<String> {
    

    let mut buf = Vec::new();
    diff.print(
        git2::DiffFormat::Patch,
        |_delta, _hunk, line| {
            let origin = line.origin();
            if origin != '\0' {
                buf.push(origin as u8);
            }
            buf.extend_from_slice(line.content());
            true
        },
    )?;

    String::from_utf8(buf)
        .context("Diff contains invalid UTF-8")
}
