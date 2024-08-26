use crate::widget::BarWidget;
use std::time::Duration;
use std::time::Instant;
use sysinfo::Disks;

#[derive(Copy, Clone, Debug)]
pub struct StorageConfig {
    pub enable: bool,
}

impl From<StorageConfig> for Storage {
    fn from(value: StorageConfig) -> Self {
        Self {
            enable: value.enable,
            disks: Disks::new_with_refreshed_list(),
            last_updated: Instant::now(),
        }
    }
}

pub struct Storage {
    pub enable: bool,
    disks: Disks,
    last_updated: Instant,
}

impl BarWidget for Storage {
    fn output(&mut self) -> Vec<String> {
        let now = Instant::now();
        if now.duration_since(self.last_updated) > Duration::from_secs(10) {
            self.disks.refresh();
            self.last_updated = now;
        }

        let mut disks = vec![];

        for disk in &self.disks {
            let mount = disk.mount_point();
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;

            disks.push(format!(
                "{} {}%",
                mount.to_string_lossy(),
                (used * 100) / total
            ))
        }

        disks.sort();
        disks.reverse();

        disks
    }
}
