pub use self::thing::*;
use crate::{
    utils::consts::{NAME, VERSION},
    CONFIG, LOGGING,
};
use anyhow::{anyhow, Result};
use log::error;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::atomic::Ordering,
};

pub mod thing;

impl Default for VConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Core config and global stats
/// The rua field is self state and
/// frontend global state.
/// When rua config changed, need to
/// notify frontend to update global state.
impl VConfig {
    pub fn new() -> Self {
        let r_config = RConfig::default();

        Self {
            core: None,
            rua: r_config,
            rua_path: PathBuf::new(),
            core_path: PathBuf::new(),
        }
    }

    /// Re-read config from file
    ///
    /// ## Arguments
    ///
    /// `resource_path`: the store path of `config.json` and `config.toml`
    pub fn init(&mut self, resource_path: &Path) -> Result<()> {
        let mut core_default = PathBuf::from(resource_path);
        core_default.push("config.json");

        let home = home::home_dir()
            .map(|path| {
                let mut path = path;
                path.push(format!(".config/{}", NAME));
                path
            })
            .unwrap_or_else(|| {
                error!("Cannot detect user home folder, use /usr/local instead");
                PathBuf::from(format!("/usr/local/{}", NAME))
            });
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

        if self.rua.logging {
            LOGGING.store(true, Ordering::Relaxed);
        }
        Ok(())
    }

    /// Reload core and rua config from file
    pub fn reload(&mut self) -> Result<()> {
        self.reload_core()?;
        self.reload_rua()?;
        Ok(())
    }

    pub fn reload_rua(&mut self) -> Result<()> {
        let mut config_file = File::open(&self.rua_path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;
        let mut rua_config = toml::from_str::<RConfig>(&buffer)?;
        // TODO upgrade from old version
        rua_config.version = VERSION.into();
        self.rua = rua_config;
        Ok(())
    }

    /// Reload core config file from VConfig
    pub fn reload_core(&mut self) -> Result<()> {
        let core_file = File::open(&self.core_path)?;
        let core_config: CoreConfig = serde_json::from_reader(core_file)?;
        self.core = Some(core_config);
        Ok(())
    }

    ///  Write core config to config file
    pub fn write_core(&mut self) -> Result<()> {
        let config = self.core.as_ref().ok_or(anyhow!("core config is empty"))?;
        let core_file = OpenOptions::new().write(true).open(&self.core_path)?;
        core_file.set_len(0)?;
        serde_json::to_writer_pretty(&core_file, &config)?;
        Ok(())
    }

    pub fn write_rua(&mut self) -> Result<()> {
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
///
/// ## Arguments
///
/// `target_path`: target config file path
/// `default_path`: if target path is not exist then use this default path
fn detect_and_create(target_path: &PathBuf, default_path: PathBuf) -> Result<()> {
    if !target_path.exists() {
        let parent = target_path
            .parent()
            .ok_or(anyhow!("config path parent is empty"))?;
        fs::create_dir_all(parent)?;
        fs::copy(default_path, target_path)?;
    }
    Ok(())
}

/// Find target node by node id.
///
/// ## Argments
///
/// `node_id`: target node's id
/// `rua`: RConfig reference
///
/// ## Return
///
/// Option, if target node is found return a reference, or None.
pub fn find_node<'a>(node_id: &String, rua: &'a RConfig) -> Result<&'a Node> {
    let mut node = None;
    rua.subscriptions.iter().for_each(|sub| {
        node = sub
            .nodes
            .iter()
            .find(|n| n.node_id.as_ref().unwrap_or(&"".to_string()) == node_id);
    });
    let node = node.ok_or(anyhow!("node {} not found", node_id))?;
    Ok(node)
}

/// Build core outbound item.
/// now only support vemss protocol
///
/// ## Arguments
///
/// `node`: node info from subscription
/// `tag`: outbound tag name
pub fn proxy_builder(node: &Node, tag: String) -> Result<Outbound> {
    let vmess = Vmess {
        address: node.add.clone(),
        port: node.port.parse()?,
        users: vec![CoreUser {
            id: node.id.clone(),
            alter_id: node.aid.parse()?,
            email: "rua@rua.rua".into(),
            security: "auto".into(),
        }],
    };

    let proxy = Outbound {
        tag,
        protocol: "vmess".into(),
        settings: OutboundSettings { vnext: vec![vmess] },
        stream_settings: Some(stream_settings_builder(node)?),
        proxy_setting: None,
        mux: None,
    };
    Ok(proxy)
}

/// Build outbound stream setting with node in subscription
pub fn stream_settings_builder(node: &Node) -> Result<StreamSettings> {
    let setting = StreamSettings {
        network: node.net.clone(),
        security: if !node.tls.is_empty() {
            node.tls.clone()
        } else {
            "none".into()
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

/// Change node's connectivity field in config
pub async fn change_connectivity(id: &str, connectivity: bool) -> Result<()> {
    let mut config = CONFIG.lock().await;
    let mut node = None;
    config.rua.subscriptions.iter_mut().for_each(|sub| {
        node = sub
            .nodes
            .iter_mut()
            .find(|n| n.node_id.as_ref().unwrap_or(&"".into()) == id);
    });
    let node = node.ok_or(anyhow!("node {} not found", &id))?;
    node.connectivity = Some(connectivity);
    Ok(())
}
