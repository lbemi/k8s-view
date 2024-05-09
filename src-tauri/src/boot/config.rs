use crate::error::MyError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub struct KubernetesConfig {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub clusters: Vec<Cluster>,
    pub contexts: Vec<Context>,
    #[serde(rename = "current-context")]
    pub current_context: String,
    #[serde(rename = "active-view-context")]
    pub active_view_context: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Cluster {
    pub name: String,
    pub cluster: ClusterInfo,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClusterInfo {
    pub server: String,
    #[serde(rename = "certificate-authority-data")]
    pub certificate_authority_data: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Context {
    pub name: String,
    pub context: ContextInfo,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ContextInfo {
    pub cluster: String,
    pub user: String,
    pub namespace: Option<String>,
}

pub fn load_k8s_config(file_name: &str) -> Result<KubernetesConfig, MyError> {
    let home = dirs::home_dir();
    match home {
        Some(path) => {
            let file_path = path.join(file_name).display().to_string();
            println!("{}", file_path);
            let file = File::open(file_path)?;
            let read = BufReader::new(file);
            let mut kubernetes_config: KubernetesConfig = serde_yaml::from_reader(read)?;
            if kubernetes_config.active_view_context.is_none() {
                kubernetes_config.active_view_context =
                    Some(kubernetes_config.current_context.clone());
            }

            Ok(kubernetes_config)
        }
        None => return Err(MyError::IOError("Home directory not found".to_string())),
    }
}
