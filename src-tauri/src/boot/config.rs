use anyhow::Ok;
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

pub fn load_k8s_config(file_name: &str) -> anyhow::Result<KubernetesConfig> {
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

            println!("{:?}", kubernetes_config);
            Ok(kubernetes_config)
        }
        None => return Err(anyhow::anyhow!("home dir not found")),
    }
}
