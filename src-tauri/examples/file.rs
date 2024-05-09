use anyhow::Ok;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};
#[derive(Serialize, Deserialize, Debug)]
struct KubernetesConfig {
    #[serde(rename = "apiVersion")]
    api_version: String,
    clusters: Vec<Cluster>,
    contexts: Vec<Context>,
    #[serde(rename = "current-context")]
    current_context: String,
    // view_context: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Cluster {
    name: String,
    cluster: ClusterInfo,
}
#[derive(Serialize, Deserialize, Debug)]
struct ClusterInfo {
    server: String,
    #[serde(rename = "certificate-authority-data")]
    certificate_authority_data: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Context {
    name: String,
    context: ContextInfo,
}
#[derive(Serialize, Deserialize, Debug)]
struct ContextInfo {
    cluster: String,
    user: String,
    // namespace: String,
}

fn read_file_by_line(file_name: &str) -> anyhow::Result<()> {
    let home = dirs::home_dir();
    match home {
        Some(path) => {
            let file_path = path.join(file_name).display().to_string();
            println!("{}", file_path);
            let file = File::open(file_path)?;
            let read = BufReader::new(file);
            let kubernetes_config: KubernetesConfig = serde_yaml::from_reader(read)?;
            println!("{:?}", kubernetes_config.contexts)
        }
        None => return Err(anyhow::anyhow!("home dir not found")),
    }

    Ok(())
}

fn main() {
    read_file_by_line(".kube/config").unwrap();
}
