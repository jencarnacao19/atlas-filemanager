// reason: a test/probe harness, not app code: panicking on setup failure and
// touching the disk directly are both intended here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::disallowed_methods)]
use std::path::Path;

fn main() {
    let p = Path::new("folder/");
    println!("p: {p:?}");
    println!("parent: {:?}", p.parent());
    let s = p.to_string_lossy();
    let key = if s.is_empty() {
        String::new()
    } else {
        format!("{s}/")
    };
    println!("key: {key:?}");
}
