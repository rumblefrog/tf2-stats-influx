use crate::config::Server;

use rcon::Connection;

use regex::Regex;

use lazy_static::lazy_static;

use chrono::{DateTime, Utc};

use anyhow::{anyhow, Result};

use influxdb::{InfluxDbWriteable, Timestamp};

#[derive(InfluxDbWriteable)]
pub struct ServerReading {
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


pub async fn measure_server(name: &str, server: &Server) -> Result<ServerReading> {
    // format: CPU percent, Bandwidth in, Bandwidth out, uptime, changelevels, framerate, total players
    // _snprintf(buf, bufSize - 1, "%-6.2f %-10.2f %-11.2f %-7i %-12i %-8.2f %-8i %-8i",
    // 			sv.GetCPUUsage() * 100, 
    // 			avgIn / 1024.0f,
    // 			avgOut / 1024.0f,
    // 			(int)(Sys_FloatTime()) / 60,
    // 			sv.GetSpawnCount() - 1,
    // 			1.0/host_frametime, // frame rate
    // 			sv.GetNumClients() - sv.GetNumProxies(),
    // 			sv.GetNumConnections());

    // CPU    In_(KB/s)  Out_(KB/s)  Uptime  Map_changes  FPS      Players  Connects
    // 99.22  87.41      118.41      1038    18           199.64   7        249

    lazy_static! {
        static ref SPLITTER: Regex = Regex::new(r"\s+").unwrap();
    }

    let mut conn: Connection = Connection::connect(format!("{}:{}", server.host, Into::<u16>::into(server.port.clone())), &server.rcon_password).await?;

    let stats: String = conn.cmd("stats").await?;
    
    let rows: Vec<&str> = stats.split('\n').collect();

    if let Some(raw_fields) = rows.get(1) {
        let fields: Vec<&str> = SPLITTER.split(raw_fields).collect();

        // Ensure at least 8 fields is present
        if fields.len() != 8 {
            return Err(anyhow!("Fields length is not equal to 8"));
        }

        return Ok(ServerReading{
            cpu: fields[0].parse()?,
            network_in: fields[1].parse()?,
            network_out: fields[2].parse()?,
            uptime: fields[3].parse()?,
            map_changes: fields[4].parse()?, 
            fps: fields[5].parse()?,
            players: fields[6].parse()?,
            connects: fields[7].parse()?,
            name: name.into(),
            time: Timestamp::Now.into(),
        });
    }

    Err(anyhow!("Row 1 does not exist"))
}
