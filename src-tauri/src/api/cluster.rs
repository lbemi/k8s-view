use once_cell::sync::Lazy;

use crate::{
    boot::config::{load_k8s_config, Cluster},
    error::MyError,
};

const KUBERNTETS_CONFIG_PATH: &str = ".kube/config";

#[tauri::command]
pub fn get_clusters() -> Result<Vec<Cluster>, MyError> {
    let clusters = load_k8s_config(KUBERNTETS_CONFIG_PATH)?;
    Ok(clusters.clusters)
}
