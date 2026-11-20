//! Read-only: lists what each cleanup category would offer. Never prepares or deletes.
// reason: a test/probe harness, not app code: panicking on setup failure and
// touching the disk directly are both intended here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]
use atlas_core::cleanup::{categories, ScanJob, ScanMode};
use std::time::{Duration, Instant};
fn main() {
    for cat in categories() {
        for root in cat.roots {
            let start = Instant::now();
            let job = ScanJob::start(root.clone(), 1, ScanMode::Category);
            let snap = loop {
                let s = job.receiver.recv_timeout(Duration::from_mins(2)).unwrap();
                if s.status.terminal() {
                    break s;
                }
            };
            let bytes: u64 = snap.reclaimable.iter().map(|e| e.bytes).sum();
            println!(
                "{:<22} {:>9.1} MB {:>7} files {:>6} ms  {:?} {}",
                cat.key,
                bytes as f64 / 1e6,
                snap.reclaimable.len(),
                start.elapsed().as_millis(),
                snap.status,
                root.display()
            );
        }
    }
}
