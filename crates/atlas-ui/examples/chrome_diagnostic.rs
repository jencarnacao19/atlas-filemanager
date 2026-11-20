// reason: a test/probe harness, not app code: panicking on setup failure and
// touching the disk directly are both intended here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]
fn main() -> anyhow::Result<()> {
    let directory = std::env::args_os()
        .nth(1)
        .map(std::path::PathBuf::from)
        .expect("pass a diagnostic output directory");
    atlas_ui::ui::views::chrome_diagnostic::run(directory)
}
