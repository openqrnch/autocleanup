//! Simple library for cleaning up non-RIAA resources using RIAA.
//!
//! A function that needs to clean up resources automatically can create an
//! AutoCleanup object that will clean up those resources automatically.
//!
//! ```
//! use std::path::Path;
//! use autocleanup::AutoCleanup;
//!
//! fn do_something() -> Result<(), std::io::Error> {
//!   let mut ac = AutoCleanup::new();
//!   ac.push_file("/tmp/foo.sock");
//!
//!   // .. do things ..
//!
//!   Ok(())
//!   // /tmp/foo.sock will automatically be removed as the function
//!   // returns.
//! }
//! ```
//!
//! Be mindful of the [`Drop`] trait caveats; for instance calling
//! [`std::process::exit()`] will cause Drop traits not to run.
//!
//! Because the cleanup occurs at Drop there's no error handling for failed
//! cleanups -- errors will be silently ignored.
//!
//! [`std::process::exit()`]: https://doc.rust-lang.org/std/process/fn.exit.html
//! [`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html

use std::path::{Path, PathBuf};

/// Representation of a cleanup node.
pub enum Item {
  File(PathBuf),
  Dir(PathBuf)
}

pub struct AutoCleanup {
  items: Vec<Item>
}

impl AutoCleanup {
  /// Create a new autocleanup object.
  pub fn new() -> Self {
    AutoCleanup{ items: Vec::new() }
  }

  pub fn push(&mut self, item: Item) {
    self.items.push(item);
  }

  /// Push a file on to the list of objects to automatically clean up when the
  /// AutoClean object goes out of scope.
  pub fn push_file<P: AsRef<Path>>(&mut self, fname: P) {
    self.items.push(Item::File(fname.as_ref().to_path_buf()));
  }

  /// Push a directory on to the list of objects to automatically clean up when
  /// the AutoClean object goes out of scope.
  /// The removal operation currently does not remove items contained with it.
  /// It may be in the future be changed to do so.
  pub fn push_dir<P: AsRef<Path>>(&mut self, dname: P) {
    self.items.push(Item::Dir(dname.as_ref().to_path_buf()));
  }
}

impl Drop for AutoCleanup {
  /// Drop implementations don't have a good way to handle errors, so any
  /// errors are silently ignored.
  fn drop(&mut self) {
    for n in self.items.iter().rev() {
      match n {
        Item::File(p) => {
          let _ = std::fs::remove_file(p);
        }
        Item::Dir(p) => {
          let _ = std::fs::remove_dir(p);
        }
      }
    }
  }
}

#[test]
fn test() {
  let mut ac = AutoCleanup::new();
  ac.push_file("/nonexistent");
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
