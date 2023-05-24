use std::fs::File;

use log::debug;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use tauri::AppHandle;

use crate::utils::error::VResult;

/// Outbound nodes
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub v: String,
    // Node name
    pub ps: String,
    // address
    pub add: String,
    pub port: String,
    pub id: String,
    pub aid: String,
    pub net: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub host: String,
    pub path: String,
    pub tls: String,
    pub sni: String,
    pub alpn: String,
}

/// Config roots
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreConfig {
    pub log: Log,
    pub inbounds: Vec<Inbound>,
    pub outbounds: Vec<Outbound>,
    pub routing: Routing,
    pub dns: Dns,
    pub policy: Policy,
    pub other: Other,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub loglevel: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inbound {
    pub port: i64,
    pub listen: String,
    pub tag: String,
    pub protocol: String,
    pub settings: Settings,
    pub sniffing: Sniffing,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub auth: String,
    pub udp: bool,
    pub ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sniffing {
    pub enabled: bool,
    pub dest_override: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbound {
    pub protocol: String,
    pub settings: Settings2,
    pub tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Routing {
    pub domain_strategy: String,
    pub rules: Vec<Rule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub ip: Vec<String>,
    pub outbound_tag: String,
    #[serde(default)]
    pub domain: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dns {
    pub hosts: Hosts,
    pub servers: (String, Servers, String, String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hosts {
    #[serde(rename = "domain:v2fly.org")]
    pub domain_v2fly_org: String,
    #[serde(rename = "domain:github.io")]
    pub domain_github_io: String,
    #[serde(rename = "domain:wikipedia.org")]
    pub domain_wikipedia_org: String,
    #[serde(rename = "domain:shadowsocks.org")]
    pub domain_shadowsocks_org: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Servers {
    pub address: String,
    pub port: i64,
    pub domains: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    pub levels: Levels,
    pub system: System,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Levels {
    #[serde(rename = "0")]
    pub n0: N0,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N0 {
    pub uplink_only: i64,
    pub downlink_only: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub stats_inbound_uplink: bool,
    pub stats_inbound_downlink: bool,
    pub stats_outbound_uplink: bool,
    pub stats_outbound_downlink: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Other {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VConfig {
    pub core: Option<CoreConfig>,
}

impl VConfig {
    pub fn new() -> Self {
        Self { core: None }
    }

    pub fn init_core(&mut self, handle: AppHandle) -> VResult<()> {
        let resource_path = handle
            .path_resolver()
            .resolve_resource("resources/config.json")
            .expect("can not found config file");
        let core_file = File::open(resource_path)?;
        debug!("{core_file:?}");
        let core_config: CoreConfig = serde_json::from_reader(core_file)?;
        self.core = Some(core_config);
        Ok(())
    }
}
