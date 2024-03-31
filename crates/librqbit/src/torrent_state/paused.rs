use std::{collections::HashSet, fs::File, path::PathBuf, sync::Arc};

use parking_lot::Mutex;

use crate::chunk_tracker::{ChunkTracker, HaveNeededSelected};

use super::ManagedTorrentInfo;

pub struct TorrentStatePaused {
    pub(crate) info: Arc<ManagedTorrentInfo>,
    pub(crate) files: Vec<Arc<Mutex<File>>>,
    pub(crate) filenames: Vec<PathBuf>,
    pub(crate) chunk_tracker: ChunkTracker,
}

impl TorrentStatePaused {
    pub(crate) fn update_only_files(&mut self, only_files: &HashSet<usize>) -> anyhow::Result<()> {
        self.chunk_tracker
            .update_only_files(self.info.info.iter_file_lengths()?, only_files)?;
        Ok(())
    }

    pub(crate) fn hns(&self) -> &HaveNeededSelected {
        self.chunk_tracker.get_hns()
    }
}
