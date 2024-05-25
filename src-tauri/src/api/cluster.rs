use kube::config::NamedCluster;
use crate::{boot::config::load_k8s_config, error::MyError};

const KUBERNTETS_CONFIG_PATH: &str = ".kube/config";

#[tauri::command]
pub fn list_clusters() -> Result<Vec<NamedCluster>, MyError> {
    let clusters = load_k8s_config(KUBERNTETS_CONFIG_PATH)?;
    Ok(clusters.clusters)
}
