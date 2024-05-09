
use k8s_openapi::api::core::v1::Pod;
use crate::error::MyError;
use crate::service::pod::PodStruct;


#[tauri::command]
pub async fn list_pods(namespace: &str) -> Result<Vec<Pod>, MyError> {
    println!("namespace: {}", namespace);
    PodStruct::namespace(namespace).await.list_pods().await
}

#[tauri::command]
pub async fn get_pod_by_name(name: String, namespace: &str) -> Result<Pod, MyError> {
    PodStruct::namespace(namespace).await.get_pod_by_name(name).await
}

#[tauri::command]
pub async fn delete_pod_by_name(name: String, namespace: &str) -> Result<String, MyError> {
    PodStruct::namespace(namespace).await.delete_pod_by_name(name).await
}

#[tauri::command]
pub async fn create_pod(pod: Pod, namespace: &str) -> Result<Pod, MyError> {
    PodStruct::namespace(namespace).await.create_pod(pod).await
}



