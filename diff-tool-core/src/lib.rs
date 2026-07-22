//! Shared library for diff-tool: side-by-side diff computation and a file
//! browser model used by the TUI.

pub mod diff;
pub mod file_browser;
pub mod path_display;

pub use diff::{
    compact_diff, diff_lines, DiffRow, DiffSide, DiffStats, RowKind, SideBySide,
    DEFAULT_CONTEXT_LINES,
};
pub use file_browser::{
    existing_ancestor_dir, parent_dir, sibling_files, switcher_entries, Entry, FileBrowser,
    NavigateTarget, ResolveError,
};
pub use path_display::{abbreviate_paths, abbreviated_path_titles};
