use hyper_util::rt::TokioExecutor;
use kube::{client::ConfigExt, config::KubeConfigOptions, Client, Config};
use tower::ServiceBuilder;

use crate::boot::config::KUBERNETES_LIST;

pub async fn get_kubernetes_config(cluster_name: &str) -> Option<Client> {
    let configs = KUBERNETES_LIST.get();
    match configs {
        Some(config) => {
            for context in config.contexts.iter() {
                if let Some(c) = context.context.as_ref() {
                    if c.cluster == cluster_name {
                        let kube_config_options = KubeConfigOptions {
                            context: Some(context.name.clone()),
                            cluster: Some(c.cluster.clone()),
                            user: Some(c.user.clone()),
                        };
                        let kube_config =
                            Config::from_custom_kubeconfig(config.clone(), &kube_config_options)
                                .await;
                        if let Ok(c) = kube_config {
                            let https_connector = c.rustls_https_connector();
                            if let Ok(https) = https_connector {
                                let service = ServiceBuilder::new()
                                    .layer(c.base_uri_layer())
                                    .option_layer(c.auth_layer().unwrap())
                                    .service(
                                        hyper_util::client::legacy::Client::builder(
                                            TokioExecutor::new(),
                                        )
                                        .build(https),
                                    );
                                let client = Client::new(service, c.default_namespace);
                                return Some(client);
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    }
                }
            }
            None
        }
        None => None,
    }
}
