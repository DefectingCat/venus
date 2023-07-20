use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs::File, io::Write};

use log::error;
use serde::Deserialize;
use serde_derive::Serialize;
use tokio::sync::Mutex;

use crate::commands::subs::NodeType;
use crate::utils::error::{VError, VResult};
use crate::{NAME, VERSION};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub name: String,
    pub url: String,
    pub nodes: Vec<Node>,
}

/// RUA config and frontend global state
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RConfig {
    pub logging: bool,
    pub version: String,
    /// Save state of all open windows to disk
    pub save_windows: bool,
    pub core_status: CoreStatus,
    pub subscriptions: Vec<Subscription>,
}

/// All config field
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VConfig {
    pub core: Option<CoreConfig>,
    pub core_path: PathBuf,
    pub rua: RConfig,
    pub rua_path: PathBuf,
}

/// The core current status
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CoreStatus {
    Started,
    Restarting,
    Stopped,
}
impl CoreStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            CoreStatus::Started => "Started",
            CoreStatus::Restarting => "Restarting",
            CoreStatus::Stopped => "Stopped",
        }
    }
}

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
            logging: false,
            version: VERSION.to_owned(),
            save_windows: true,
            core_status: Stopped,
            subscriptions: vec![],
        };

        Self {
            core: None,
            rua: r_config,
            rua_path: PathBuf::new(),
            core_path: PathBuf::new(),
        }
    }

    /// Re-read config from file
    pub fn init(&mut self, resource_path: &Path) -> VResult<()> {
        // let resource_path = resource_path.ok_or(VError::ResourceError("resource path is empty"))?;
        let mut core_default = PathBuf::from(resource_path);
        core_default.push("config.json");

        let home = match home::home_dir() {
            Some(path) => {
                let mut path = path;
                path.push(format!(".config/{}", NAME));
                path
            }
            None => {
                error!("Cannot detect user home folder, use /usr/local instead");
                PathBuf::from(format!("/usr/local/{}", NAME))
            }
        };
        let mut core_path = PathBuf::from(&home);
        core_path.push("config.json");
        let mut rua_path = PathBuf::from(&home);
        rua_path.push("config.toml");

        self.core_path = core_path.clone();
        self.rua_path = rua_path.clone();

        detect_and_create(&core_path, core_default)?;
        if !rua_path.exists() {
            self.write_rua()?;
        }

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
        let mut rua_config = toml::from_str::<RConfig>(&buffer)?;
        // Do not read core status from config file
        rua_config.core_status = self.rua.core_status;
        rua_config.version = VERSION.to_owned();
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
        let config = self
            .core
            .as_ref()
            .ok_or(VError::EmptyError("core config is empty"))?;
        let core_file = OpenOptions::new().write(true).open(&self.core_path)?;
        core_file.set_len(0)?;
        serde_json::to_writer(&core_file, &config)?;
        Ok(())
    }

    pub fn write_rua(&mut self) -> VResult<()> {
        let mut rua_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.rua_path)?;
        let rua_string = toml::to_string(&self.rua)?;
        rua_file.set_len(0)?;
        rua_file.write_all(rua_string.as_bytes())?;
        Ok(())
    }
}

/// Detect target config path exists
/// If not exists, create all parent folders
/// and copy default config file to target path.
fn detect_and_create(target_path: &PathBuf, default_path: PathBuf) -> VResult<()> {
    if !target_path.exists() {
        let parent = target_path
            .parent()
            .ok_or(VError::EmptyError("Core path parent is empty"))?;
        fs::create_dir_all(parent)?;
        fs::copy(default_path, target_path)?;
    }
    Ok(())
}

/// Build outbound stream setting with node in subscription
pub fn stream_settings_builder(node: &Node) -> VResult<StreamSettings> {
    let setting = StreamSettings {
        network: node.net.clone(),
        security: if !node.tls.is_empty() {
            node.tls.clone()
        } else {
            "none".to_owned()
        },
        tls_settings: if !node.tls.is_empty() {
            Some(TlsSettings {
                alpn: vec![],
                server_name: node.host.clone(),
                certificates: vec![],
                allow_insecure: false,
                disable_system_root: false,
            })
        } else {
            None
        },
        tcp_settings: None,
        kcp_settings: None,
        ws_settings: if node.net.as_str() == "ws" {
            Some(WsSettings {
                path: node.path.clone(),
                headers: WsHeaders {
                    host: node.host.clone(),
                },
            })
        } else {
            None
        },
        http_settings: None,
        ds_settings: None,
        quic_settings: None,
        sockopt: None,
    };

    Ok(setting)
}

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
    // Protocol type determine streamSettings network field
    pub net: String,
    // Protocol type
    #[serde(rename = "type")]
    pub type_field: String,
    pub host: String,
    // streamSettings
    pub path: String,
    // Determine streamSettings security field
    pub tls: String,
    // Determine streamSettings headers sni
    pub sni: String,
    pub alpn: String,
    // Add by manually
    // The subscription group
    pub subs: Option<String>,
    // Current node delay
    pub delay: Option<String>,
    // Node unique ID
    pub node_id: Option<String>,
    // Node raw link from subcription link
    pub raw_link: Option<String>,
    // Node net type
    pub node_type: Option<NodeType>,
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
    pub port: u16,
    // Listen address
    pub listen: String,
    pub tag: String,
    pub protocol: String,
    pub settings: InboundSettings,
    // Traffic sniffing
    pub sniffing: Option<Sniffing>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundSettings {
    pub auth: Option<String>,
    pub udp: bool,
    // pub ip: String,
    // for dokodemo-door
    // pub address: Option<String>,
    pub allow_transparent: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sniffing {
    pub enabled: bool,
    pub dest_override: Vec<String>,
    pub route_only: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbound {
    pub protocol: String,
    pub settings: OutboundSettings,
    pub tag: String,
    pub stream_settings: Option<StreamSettings>,
    pub proxy_setting: Option<ProxySetting>,
    pub mux: Option<Mux>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSettings {
    pub network: String,
    pub security: String,
    pub tls_settings: Option<TlsSettings>,
    pub tcp_settings: Option<TcpSettings>,
    pub kcp_settings: Option<KcpSettings>,
    pub ws_settings: Option<WsSettings>,
    pub http_settings: Option<HttpSettings>,
    pub ds_settings: Option<DsSettings>,
    pub quic_settings: Option<QuicSettings>,
    pub sockopt: Option<Sockopt>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TlsSettings {
    pub server_name: String,
    pub allow_insecure: bool,
    pub alpn: Vec<String>,
    pub certificates: Vec<String>,
    pub disable_system_root: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TcpSettings {
    pub header: KcpHeader,
    pub request: Option<Request>,
    pub response: Option<Response>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub version: String,
    pub method: String,
    pub path: Vec<String>,
    pub headers: Headers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers {
    #[serde(rename = "Host")]
    pub host: Vec<String>,
    #[serde(rename = "User-Agent")]
    pub user_agent: Vec<String>,
    #[serde(rename = "Accept-Encoding")]
    pub accept_encoding: Vec<String>,
    #[serde(rename = "Connection")]
    pub connection: Vec<String>,
    #[serde(rename = "Pragma")]
    pub pragma: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub version: String,
    pub status: String,
    pub reason: String,
    pub headers: Headers2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headers2 {
    #[serde(rename = "Content-Type")]
    pub content_type: Vec<String>,
    #[serde(rename = "Transfer-Encoding")]
    pub transfer_encoding: Vec<String>,
    #[serde(rename = "Connection")]
    pub connection: Vec<String>,
    #[serde(rename = "Pragma")]
    pub pragma: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KcpSettings {
    pub mtu: i64,
    pub tti: i64,
    pub uplink_capacity: i64,
    pub downlink_capacity: i64,
    pub congestion: bool,
    pub read_buffer_size: i64,
    pub write_buffer_size: i64,
    pub header: KcpHeader,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KcpHeader {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSettings {
    pub path: String,
    pub headers: WsHeaders,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsHeaders {
    #[serde(rename = "Host")]
    pub host: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpSettings {
    pub host: Vec<String>,
    pub path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DsSettings {
    path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuicSettings {
    pub security: String,
    pub key: String,
    pub header: KcpHeader,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sockopt {
    pub mark: i64,
    pub tcp_fast_open: bool,
    pub tproxy: String,
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
    #[serde(default)]
    pub vnext: Vec<Vmess>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vmess {
    pub address: String,
    pub port: u16,
    pub users: Vec<CoreUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreUser {
    pub id: String,
    pub alter_id: u16,
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
