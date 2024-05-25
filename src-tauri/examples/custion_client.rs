use hyper_util::rt::TokioExecutor;
// Minimal custom client example.
use k8s_openapi::api::core::v1::Pod;

use kube::{client::ConfigExt, Api, Client, Config, ResourceExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::infer().await?;

    let https = config.rustls_https_connector()?;
    let service = tower::ServiceBuilder::new()
        .layer(config.base_uri_layer())
        .option_layer(config.auth_layer()?)
        .service(hyper_util::client::legacy::Client::builder(TokioExecutor::new())
        .build(https));
    let client = Client::new(service, config.default_namespace);

    let pods: Api<Pod> = Api::default_namespaced(client);
    for p in pods.list(&Default::default()).await? {
        println!("{}", p.name_any());
    }

    Ok(())
}
