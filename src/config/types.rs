use serde::{Deserialize, Serialize};
use std::{borrow::Cow, path::PathBuf};

use crate::consts::VERSION;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// Core subscriptions
pub struct Subscription {
    pub name: Cow<'static, str>,
    pub url: Cow<'static, str>,
    pub nodes: Vec<Node>,
}

/// RUA config and frontend global state
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RConfig {
    pub version: Cow<'static, str>,
    /// Subscriptions
    pub subscriptions: Vec<Subscription>,
    pub settings: RUABasicSetting,
}
impl Default for RConfig {
    fn default() -> Self {
        RConfig {
            version: VERSION.into(),
            subscriptions: vec![],
            settings: RUABasicSetting::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RUABasicSetting {
    /// speed test url
    pub speed_url: Cow<'static, str>,
    /// Current selected node id (node_id)
    pub current_id: Cow<'static, str>,
    pub logging: bool,
}
impl Default for RUABasicSetting {
    fn default() -> Self {
        Self {
            // TODO: default speed url
            speed_url: "".into(),
            current_id: "".into(),
            logging: false,
        }
    }
}

/// All config field
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VConfig {
    /// Core config from `config.json`
    pub core: Option<CoreConfig>,
    pub core_path: PathBuf,
    pub rua: RConfig,
    pub rua_path: PathBuf,
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
    // Current node speed, upload and download
    pub speed: Option<f64>,
    // Current node delay
    pub delay: Option<u64>,
    // Node connectivity
    pub connectivity: Option<bool>,
    // Node unique ID
    pub node_id: Option<String>,
    // Node raw link from subcription link
    pub raw_link: Option<String>,
    // Node net type
    pub node_type: Option<NodeType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Vmess,
    Vless,
    SS,
    Ssr,
    Trojan,
    Trojango,
    HttpProxy,
    HttpsProxy,
    SOCKS5,
    HTTP2,
    Unknown,
}
impl From<&str> for NodeType {
    fn from(value: &str) -> Self {
        use NodeType::*;
        match value.to_lowercase().as_str() {
            "vmess" => Vmess,
            "vless" => Vless,
            "ss" => SS,
            "ssr" => Ssr,
            "trojan" => Trojan,
            "trojan-go" => Trojango,
            "http-proxy" => HttpProxy,
            "https-proxy" => HttpsProxy,
            "socks5" => SOCKS5,
            "http2" => HTTP2,
            _ => Unknown,
        }
    }
}
impl NodeType {
    pub fn as_str(&self) -> &str {
        use NodeType::*;
        match self {
            Vmess => "vmess",
            Vless => "vless",
            SS => "ss",
            Ssr => "ssr",
            Trojan => "trojan",
            Trojango => "trojan-go",
            HttpProxy => "http-proxy",
            HttpsProxy => "https-proxy",
            SOCKS5 => "socks5",
            HTTP2 => "http2",
            Unknown => "unknown",
        }
    }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sniffing: Option<Sniffing>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboundSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_only: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outbound {
    pub protocol: String,
    pub settings: OutboundSettings,
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_settings: Option<StreamSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_setting: Option<ProxySetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mux: Option<Mux>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSettings {
    pub network: String,
    pub security: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_settings: Option<TlsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_settings: Option<TcpSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kcp_settings: Option<KcpSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws_settings: Option<WsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_settings: Option<HttpSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds_settings: Option<DsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quic_settings: Option<QuicSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

// https://www.v2ray.com/chapter_02/03_routing.html
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Routing {
    // "AsIs" | "IPIfNonMatch" | "IPOnDemand"
    pub domain_strategy: String,
    pub rules: Vec<Rule>,
    pub balancers: Vec<Balancers>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<Vec<String>>,
    #[serde(default)]
    pub domain: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Vec<String>>,
    pub inbound_tag: Option<Vec<String>>,
    pub protocol: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<String>,
    pub outbound_tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balancer_tag: Option<String>,
}
impl Rule {
    pub fn new(outbound_tag: String) -> Self {
        Self {
            type_field: "field".into(),
            ip: None,
            domain: None,
            port: None,
            network: None,
            source: None,
            user: None,
            inbound_tag: None,
            protocol: None,
            attrs: None,
            outbound_tag,
            balancer_tag: None,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balancers {
    pub tag: String,
    pub selector: Vec<String>,
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
