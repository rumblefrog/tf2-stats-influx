use std::collections::BTreeMap;
use std::convert::From;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub general: General,

    /// Key map of servers to collect metrics from
    /// The key name will be used as a tag when submitting to influx
    pub servers: BTreeMap<String, Server>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct General {
    /// HTTP endpoint of the influxdb
    pub influx_endpoint: String,

    /// Database name within influxdb to use
    pub influx_database: String,

    /// Influx authentication user
    pub influx_user: String,

    /// Influx user password
    pub influx_password: String,

    /// Interval in minutes to query servers and collect metrics
    pub interval: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Server {
    pub host: String,

    #[serde(default)]
    /// Game server port, will default to 27015 if not provided
    pub port: Port,

    pub rcon_password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Port(u16);

impl Default for Port {
    fn default() -> Self {
        Port(27015)
    }
}

impl From<Port> for u16 {
    fn from(port: Port) -> Self {
        port.0
    }
}
