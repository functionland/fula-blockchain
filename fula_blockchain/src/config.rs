use envconfig::Envconfig;

/// The client configuration struct.
#[derive(Debug, Envconfig)]
pub struct ClientConfig {
    #[envconfig(from = "NODE_URL", default = "ws://127.0.0.1:9944")]
    pub node_url: String,
    #[envconfig(from = "NODE_MAINTAINER_PHRASE")]
    pub node_maintainer_phrase: String,
    #[envconfig(from = "NODE_MAINTAINER_PASSWORD")]
    pub node_maintainer_password: Option<String>,
}
