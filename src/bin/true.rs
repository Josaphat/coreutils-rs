//! true - returns with an exit code of 0 (success)

use std::process;

/// Just return 0. Don't bother with any help or version output.
fn main() {
    process::exit(0);
}
