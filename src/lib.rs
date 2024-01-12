use std::path::Path;

use serde::Serialize;
use wgsl_formatter::FormattingOptions;

#[cfg(feature = "wasm")]
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm_plugin;

#[cfg(feature = "wasm")]
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use wasm_plugin::*;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {}

pub fn format_text(
    _path: &Path,
    source: &str,
    _config: &Configuration,
) -> anyhow::Result<Option<String>> {
    let formatting_options = FormattingOptions::default();
    let output = wgsl_formatter::format_str(source, &formatting_options);

    if source == output {
        Ok(None)
    } else {
        Ok(Some(output))
    }
}
