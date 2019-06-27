//! false - returns with a non-zero exit code (failure)

use std::process;

/// Just return 1. Don't bother with any help or version output
fn main() {
    process::exit(1);
}
