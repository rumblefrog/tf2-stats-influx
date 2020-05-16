mod config;
mod measure;

use std::fs::read_to_string;
use std::thread;
use std::time::Duration;

use anyhow::Result;

use futures::future::join_all;

use crossbeam::{select, unbounded};

use influxdb::{Client, InfluxDbWriteable, WriteQuery};

#[tokio::main]
async fn main() -> Result<()> {
    let content = read_to_string("config.toml")?;

    let config: config::Config = toml::from_str(&content)?;

    let client = Client::new(
        config.general.influx_endpoint,
        config.general.influx_database,
    )
    .with_auth(config.general.influx_user, config.general.influx_password);

    let idle_time = 60 * config.general.interval;

    let (query_tx, query_rx) = unbounded();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(idle_time));
        query_tx.send(true).unwrap();
    });

    loop {
        select! {
            recv(query_rx) -> _ => {
                let mut measure_futures = Vec::new();

                for (name, server) in &config.servers {
                    measure_futures.push(measure::measure_server(name, server));
                }

                let result: Vec<Result<measure::ServerReading>> = join_all(measure_futures).await;

                let mut queries: Vec<WriteQuery> = Vec::new();

                let mut record_futures = Vec::new();

                for query in result {
                    if let Ok(reading) = query {
                        queries.push(reading.into_query("server_query"));
                    }
                }

                for query in &queries {
                    record_futures.push(client.query(query));
                }

                join_all(record_futures).await;
            },
        }
    }
}
