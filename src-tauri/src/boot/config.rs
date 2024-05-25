use crate::error::MyError;
use anyhow::Result;
use kube::config::Kubeconfig;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug,Clone)]
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
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Cluster {
    pub name: String,
    pub cluster: ClusterInfo,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct ClusterInfo {
    pub server: String,
    #[serde(rename = "certificate-authority-data")]
    pub certificate_authority_data: String,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Context {
    pub name: String,
    pub context: ContextInfo,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct ContextInfo {
    pub cluster: String,
    pub user: String,
    pub namespace: Option<String>,
}
// static mut KUBERNETES_LIST: Option<KubernetesConfig> = None;
pub static KUBERNETES_LIST :OnceCell<Kubeconfig> = OnceCell::new();

pub fn load_k8s_config(file_name: &str) -> Result<Kubeconfig, MyError> {
    let home = dirs::home_dir();
    match home {
        Some(path) => {
            let file_path = path.join(file_name).display().to_string();
            println!("{}", file_path);
            let file = File::open(file_path)?;
            let read = BufReader::new(file);
            let  kubernetes_config: Kubeconfig = serde_yaml::from_reader(read)?;
            // if kubernetes_config.active_view_context.is_none() {
            //     kubernetes_config.active_view_context =
            //         Some(kubernetes_config.current_context.clone());
            // }
            KUBERNETES_LIST.get_or_init(|| kubernetes_config.clone());
            Ok(kubernetes_config)
        }
        None => return Err(MyError::IOError("Home directory not found".to_string())),
    }
}
