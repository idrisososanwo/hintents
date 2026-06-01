// Copyright 2026 Erst Users
// SPDX-License-Identifier: Apache-2.0

//
// You may obtain a copy of the License at
//
//
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

//
// You may obtain a copy of the License at
//
//
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

use crate::theme::ansi::apply;
use crate::theme::load_theme;
use std::path::PathBuf;

#[allow(dead_code)]
fn trace_viewer_temp_root() -> PathBuf {
    std::env::temp_dir().join("erst-trace-viewer")
}

#[allow(dead_code)]
pub fn trace_viewer_temp_path(file_name: &str) -> PathBuf {
    trace_viewer_temp_root().join(file_name)
}

#[allow(dead_code)]
pub fn render_trace() {
    let theme = load_theme();

    println!(
        "{} {}",
        apply(&theme.span, "SPAN"),
        apply(&theme.event, "User logged in")
    );

    println!(
        "{} {}",
        apply(&theme.error, "ERROR"),
        apply(&theme.error, "Connection failed")
    );
}
