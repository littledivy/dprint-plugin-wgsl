use dprint_core::configuration::get_unknown_property_diagnostics;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::SyncPluginHandler;
use dprint_core::plugins::SyncPluginInfo;
use std::path::Path;

use wgsl_formatter::FormattingOptions;

pub struct WgslPlugin {}

impl WgslPlugin {
    const fn new() -> Self {
        WgslPlugin {}
    }
}

impl SyncPluginHandler<Configuration> for MyPluginHandler {
    fn plugin_info(&mut self) -> SyncPluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        SyncPluginInfo {
            info: PluginInfo {
                name: env!("CARGO_PKG_NAME").to_string(),
                version: version.clone(),
                config_key: "json".to_string(),
                help_url: "https://dprint.dev/plugins/wgsl".to_string(),
                config_schema_url: format!(
                    "https://plugins.dprint.dev/dprint/dprint-plugin-wgsl/{}/schema.json",
                    version
                ),
                update_url: Some(
                    "https://plugins.dprint.dev/dprint/dprint-plugin-wgsl/latest.json".to_string(),
                ),
            },
            file_matching: FileMatchingInfo {
                file_extensions: vec!["wgsl".to_string()],
                file_names: vec![],
            },
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        let mut diagnostics = Vec::new();
        ResolveConfigurationResult {
            config: Configuration {},
            diagnostics,
        }
    }

    fn format(
        &mut self,
        file_path: &Path,
        input: &str,
        config: &Configuration,
        _format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> FormatResult,
    ) -> FormatResult {
        crate::format_text(file_path, input, config)
    }
}

generate_plugin_code!(WgslPlugin, WgslPlugin::new());
