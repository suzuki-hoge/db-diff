use crate::domain::snapshot::SnapshotId;

#[derive(Eq, PartialEq, Debug)]
pub struct SnapshotResult {
    pub snapshot_id: SnapshotId,
    pub percent: usize,
    pub done: usize,
    pub total: usize,
    pub status: String,
}

impl SnapshotResult {
    pub fn init(snapshot_id: &SnapshotId, total: usize) -> Self {
        Self { snapshot_id: snapshot_id.clone(), percent: 0, done: 0, total, status: "processing".to_string() }
    }

    pub fn zero(snapshot_id: &SnapshotId) -> Self {
        Self { snapshot_id: snapshot_id.clone(), percent: 0, done: 0, total: 0, status: "queued".to_string() }
    }

    pub fn increment(&mut self) {
        self.done += 1;
        if self.total < 10 {
            self.percent = 0;
        } else {
            self.percent = self.done / (self.total / 10) * 10;
        }
    }

    pub fn complete(&mut self) {
        self.percent = 100;
        self.status = "complete".to_string();
    }

    pub fn failed(snapshot_id: &SnapshotId) -> Self {
        Self { snapshot_id: snapshot_id.clone(), percent: 0, done: 0, total: 0, status: "failed".to_string() }
    }
}
