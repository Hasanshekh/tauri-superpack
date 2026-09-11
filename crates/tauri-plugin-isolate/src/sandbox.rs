use crate::error::{Error, Result};
use boa_engine::{Context, Source};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvalResult {
    pub output: String,
    pub success: bool,
    pub error: Option<String>,
}

pub struct PluginInstance {
    pub manifest: PluginManifest,
    pub code: String,
    pub logs: Vec<String>,
}

pub struct SandboxManager {
    plugins: Mutex<HashMap<String, PluginInstance>>,
}

impl SandboxManager {
    pub fn new() -> Self {
        Self {
            plugins: Mutex::new(HashMap::new()),
        }
    }

    pub fn eval_code(&self, plugin_id: Option<&str>, code: &str) -> EvalResult {
        let mut context = Context::default();

        let source = Source::from_bytes(code.as_bytes());
        match context.eval(source) {
            Ok(val) => {
                let out_str = val.display().to_string();
                if let Some(id) = plugin_id {
                    let mut map = self.plugins.lock().unwrap();
                    if let Some(plugin) = map.get_mut(id) {
                        plugin.logs.push(format!("Output: {out_str}"));
                    }
                }
                EvalResult {
                    output: out_str,
                    success: true,
                    error: None,
                }
            }
            Err(err) => {
                let err_str = err.to_string();
                if let Some(id) = plugin_id {
                    let mut map = self.plugins.lock().unwrap();
                    if let Some(plugin) = map.get_mut(id) {
                        plugin.logs.push(format!("Error: {err_str}"));
                    }
                }
                EvalResult {
                    output: String::new(),
                    success: false,
                    error: Some(err_str),
                }
            }
        }
    }

    pub fn register_plugin(&self, manifest: PluginManifest, code: String) -> Result<EvalResult> {
        let mut map = self.plugins.lock().unwrap();
        if map.contains_key(&manifest.id) {
            return Err(Error::PluginAlreadyRegistered(manifest.id));
        }

        // Run initial execution / setup script in sandbox
        let id = manifest.id.clone();
        map.insert(
            id.clone(),
            PluginInstance {
                manifest,
                code: code.clone(),
                logs: Vec::new(),
            },
        );
        drop(map);

        let result = self.eval_code(Some(&id), &code);
        Ok(result)
    }

    pub fn unload_plugin(&self, id: &str) -> Result<()> {
        let mut map = self.plugins.lock().unwrap();
        if map.remove(id).is_none() {
            return Err(Error::PluginNotFound(id.to_string()));
        }
        Ok(())
    }

    pub fn list_plugins(&self) -> Vec<PluginManifest> {
        let map = self.plugins.lock().unwrap();
        map.values().map(|p| p.manifest.clone()).collect()
    }
}
