use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use std::{fs::File, io::Write};

use log::warn;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::utils::error::VResult;

/// Subscription nodes
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub v: String,
    // Node name
    pub ps: String,
    // Address
    pub add: String,
    pub port: String,
    pub id: String,
    // AlertID
    pub aid: String,
    // Protocol type
    pub net: String,
    // Protocol type
    #[serde(rename = "type")]
    pub type_field: String,
    pub host: String,
    pub path: String,
    pub tls: String,
    pub sni: String,
    pub alpn: String,
    // Add by manually
    pub subs: Option<String>,
    pub delay: Option<String>,
    pub node_id: Option<String>,
}

/// Core config root
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
    pub access: Option<PathBuf>,
    pub error: Option<PathBuf>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inbound {
    pub port: i64,
    // Listen address
    pub listen: String,
    pub tag: String,
    pub protocol: String,
    pub settings: InboundSettings,
    // Traffic sniffing
    pub sniffing: Sniffing,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundSettings {
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
    pub settings: OutboundSettings,
    pub tag: String,
    pub proxy_setting: Option<ProxySetting>,
    pub mux: Option<Mux>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxySetting {
    pub tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mux {
    pub enabled: bool,
    pub concurrency: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboundSettings {
    pub vnext: Option<Vec<Vmess>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vmess {
    pub address: String,
    pub port: String,
    pub users: Vec<CoreUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreUser {
    pub id: String,
    pub alter_id: String,
    pub email: String,
    pub security: String,
}

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
pub struct Subscription {
    pub name: String,
    pub url: String,
    pub nodes: Option<Vec<Node>>,
}

/// V2rayR config and frontend global state
#[derive(Debug, Serialize, Deserialize)]
pub struct RConfig {
    pub core_status: CoreStatus,
    pub subscriptions: Option<Vec<Subscription>>,
}

/// All config field
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VConfig {
    pub core: Option<CoreConfig>,
    pub rua: RConfig,
    pub core_path: PathBuf,
    pub rua_path: PathBuf,
}

/// The core current status
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CoreStatus {
    Started,
    Restarting,
    Stopped,
}

// impl CoreStatus {
//     fn as_str(&self) -> &'static str {
//         match self {
//             CoreStatus::Started => "Started",
//             CoreStatus::Restarting => "Restarting",
//             CoreStatus::Stopped => "Stopped",
//         }
//     }
// }

pub type ConfigState = Arc<Mutex<VConfig>>;

/// Core config and global stats
/// The rua field is self state and
/// frontend global state.
/// When rua config changed, need to
/// notify frontend to update global state.
impl VConfig {
    pub fn new() -> Self {
        use CoreStatus::*;

        let r_config = RConfig {
            core_status: Stopped,
            subscriptions: Some(vec![]),
        };

        Self {
            core: None,
            rua: r_config,
            core_path: PathBuf::new(),
            rua_path: PathBuf::new(),
        }
    }

    /// Re-read config from file
    pub fn init(&mut self, core_path: PathBuf, rua_path: PathBuf) -> VResult<()> {
        self.core_path = core_path;
        self.rua_path = rua_path;
        self.reload()?;
        Ok(())
    }

    /// Reload core and rua config from file
    pub fn reload(&mut self) -> VResult<()> {
        self.reload_core()?;
        self.reload_rua()?;
        Ok(())
    }

    pub fn reload_rua(&mut self) -> VResult<()> {
        let mut config_file = File::open(&self.rua_path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;
        let rua_config = toml::from_str::<RConfig>(&buffer)?;
        self.rua = rua_config;
        Ok(())
    }

    /// Reload core config file to VConfig
    pub fn reload_core(&mut self) -> VResult<()> {
        let core_file = File::open(&self.core_path)?;
        let core_config: CoreConfig = serde_json::from_reader(core_file)?;
        self.core = Some(core_config);
        Ok(())
    }

    ///  Write core config to config file
    pub fn write_core(&mut self) -> VResult<()> {
        let config = if let Some(c) = &self.core {
            c
        } else {
            warn!("core config is empty");
            return Ok(());
        };
        let core_file = OpenOptions::new().write(true).open(&self.core_path)?;
        serde_json::to_writer(&core_file, config)?;
        Ok(())
    }

    pub fn write_rua(&mut self) -> VResult<()> {
        let mut rua_file = OpenOptions::new().write(true).open(&self.rua_path)?;
        let rua_string = toml::to_string(&self.rua)?;
        rua_file.write_all(rua_string.as_bytes())?;
        Ok(())
    }
}