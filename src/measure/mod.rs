use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;
use rcon::Connection;

use crate::config::Server;

#[derive(InfluxDbWriteable)]
pub struct ServerReading {
    pub latency: u16,

    pub cpu: f32,

    pub network_in: f32,

    pub network_out: f32,

    pub uptime: u32,

    pub map_changes: u16,

    pub fps: f32,

    pub players: u8,

    pub connects: u16,

    #[tag]
    pub name: String,

    pub time: DateTime<Utc>,
}

pub async fn measure_server(name: &str, server: &Server) -> Option<ServerReading> {
    if let Ok(mut conn) = Connection::connect(
        format!("{}:{}", server.host, Into::<u16>::into(server.port.clone())),
        &server.rcon_password,
    )
    .await
    {
        if let Ok(stats) = conn.cmd("stats").await {

        }
    }

    unimplemented!()
}
