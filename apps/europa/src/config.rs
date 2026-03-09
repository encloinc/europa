use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BitcoinNetwork {
    #[serde(alias = "bitcoin")]
    Mainnet,
    Testnet3,
    Testnet4,
    Signet,
    Regtest,
}

impl BitcoinNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet3 => "testnet3",
            Self::Testnet4 => "testnet4",
            Self::Signet => "signet",
            Self::Regtest => "regtest",
        }
    }

    pub fn bitcoin_symbol(self) -> &'static str {
        match self {
            Self::Mainnet => "BTC",
            Self::Testnet3 | Self::Testnet4 => "tBTC",
            Self::Signet => "sBTC",
            Self::Regtest => "rBTC",
        }
    }

    pub fn bitcoin_icon_src(self) -> &'static str {
        match self {
            Self::Mainnet => "/assets/svgs/bitcoin.svg",
            Self::Testnet3 | Self::Testnet4 | Self::Regtest => "/assets/svgs/bitcoin-green.svg",
            Self::Signet => "/assets/svgs/bitcoin-pink.svg",
        }
    }

    pub fn bitcoin_triple_icon_src(self) -> &'static str {
        match self {
            Self::Mainnet => "/assets/svgs/3-bitcoin.svg",
            Self::Testnet3 | Self::Testnet4 | Self::Regtest => "/assets/svgs/3-bitcoin-green.svg",
            Self::Signet => "/assets/svgs/3-bitcoin-pink.svg",
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub network: BitcoinNetwork,
    pub host: String,
    pub port: u16,
    pub required_confirmations: u32,
    pub tx_refresh_pages_max: u32,
    pub electrs_esplora_endpoint: String,
    pub explorer_endpoint: String,
    pub btc_mxn_endpoint: String,
    pub test_mode_address: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NetworkEndpointMap {
    #[serde(default, alias = "bitcoin")]
    mainnet: Option<String>,
    #[serde(default)]
    testnet3: Option<String>,
    #[serde(default)]
    testnet4: Option<String>,
    #[serde(default)]
    signet: Option<String>,
    #[serde(default)]
    regtest: Option<String>,
}

impl NetworkEndpointMap {
    fn resolve(&self, network: BitcoinNetwork, label: &str) -> Result<String> {
        let value = match network {
            BitcoinNetwork::Mainnet => self.mainnet.as_deref(),
            BitcoinNetwork::Testnet3 => self.testnet3.as_deref(),
            BitcoinNetwork::Testnet4 => self.testnet4.as_deref(),
            BitcoinNetwork::Signet => self.signet.as_deref(),
            BitcoinNetwork::Regtest => self.regtest.as_deref(),
        }
        .ok_or_else(|| anyhow!("{label} is missing for {}", network.as_str()))?;

        normalize_config_url(value, label)
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawConfig {
    network: BitcoinNetwork,
    host: String,
    port: u16,
    required_confirmations: u32,
    tx_refresh_pages_max: u32,
    electrs_esplora_endpoints: NetworkEndpointMap,
    explorer_endpoints: NetworkEndpointMap,
    btc_mxn_endpoint: String,
    #[serde(default)]
    test_mode_address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ClientConfig {
    network: &'static str,
    storage_key: String,
    required_confirmations: u32,
    tx_refresh_pages_max: u32,
    electrs_esplora_endpoint: String,
    explorer_endpoint: String,
    test_mode_address: Option<String>,
}

impl AppConfig {
    pub fn from_args() -> Result<Self> {
        let config_path = parse_config_path()?;
        Self::from_path(&config_path)
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let file = File::open(path)
            .with_context(|| format!("failed to open config file {}", path.display()))?;
        let reader = BufReader::new(file);
        let raw: RawConfig = serde_json::from_reader(reader)
            .with_context(|| format!("failed to parse config file {}", path.display()))?;

        if raw.host.trim().is_empty() {
            bail!("config host must not be empty");
        }
        if raw.required_confirmations == 0 {
            bail!("config required_confirmations must be greater than zero");
        }
        if raw.tx_refresh_pages_max == 0 {
            bail!("config tx_refresh_pages_max must be greater than zero");
        }

        let electrs_esplora_endpoint = raw
            .electrs_esplora_endpoints
            .resolve(raw.network, "electrs_esplora_endpoints")?;
        let explorer_endpoint = raw
            .explorer_endpoints
            .resolve(raw.network, "explorer_endpoints")?;
        let btc_mxn_endpoint = normalize_config_url(&raw.btc_mxn_endpoint, "btc_mxn_endpoint")?;
        let test_mode_address = normalize_optional_config_value(raw.test_mode_address);

        Ok(Self {
            network: raw.network,
            host: raw.host.trim().to_owned(),
            port: raw.port,
            required_confirmations: raw.required_confirmations,
            tx_refresh_pages_max: raw.tx_refresh_pages_max,
            electrs_esplora_endpoint,
            explorer_endpoint,
            btc_mxn_endpoint,
            test_mode_address,
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn client_config(&self) -> ClientConfig {
        ClientConfig {
            network: self.network.as_str(),
            storage_key: format!("mibilleterabitcoin.wallet.v1.{}", self.network.as_str()),
            required_confirmations: self.required_confirmations,
            tx_refresh_pages_max: self.tx_refresh_pages_max,
            electrs_esplora_endpoint: self.electrs_esplora_endpoint.clone(),
            explorer_endpoint: self.explorer_endpoint.clone(),
            test_mode_address: self.test_mode_address.clone(),
        }
    }
}

fn normalize_config_url(value: &str, label: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        bail!("{label} must not be empty");
    }

    let normalized = trimmed.trim_end_matches('/').to_owned();
    if normalized.is_empty() {
        bail!("{label} must not be empty");
    }

    Ok(normalized)
}

fn normalize_optional_config_value(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        }
    })
}

fn parse_config_path() -> Result<PathBuf> {
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        if arg == "--config" {
            let value = args
                .next()
                .ok_or_else(|| anyhow!("missing path after --config"))?;
            return Ok(PathBuf::from(value));
        }

        if let Some(value) = arg.strip_prefix("--config=") {
            return Ok(PathBuf::from(value));
        }
    }

    bail!("expected --config <path-to-config.json>");
}
