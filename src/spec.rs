// SPDX-License-Identifier: GPL-3.0-only

pub mod file;
pub mod gconf;
pub mod package;

use self::file::FileAction;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "kebab-case")]
pub struct SyncInfo {
	pub pop_sync_version: f32,
	pub files: HashMap<PathBuf, FileAction>,
}
