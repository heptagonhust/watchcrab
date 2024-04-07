use core::time;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, process::exit, thread::sleep, time::SystemTime};

#[derive(Debug, Deserialize)]
struct Config {
    node_name: String,
    remote: String,
    watch: toml::Table,
}

#[derive(Debug, Serialize)]
struct Post {
    node_name: String,
    time: SystemTime,
    watch: HashMap<String, u64>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}: expect a config file!", args[0]);
        exit(1);
    }
    let contents = fs::read_to_string(&args[1]).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    println!("{:?}", config);
    let client = reqwest::Client::new();
    loop {
        let watch_result: HashMap<_, _> = config
            .watch
            .iter()
            .map(|w| {
                (
                    w.0.to_owned(),
                    fs::read_to_string(w.1.as_str().unwrap())
                        .unwrap_or("0".to_string())
                        .trim()
                        .parse::<u64>()
                        .unwrap_or(0),
                )
            })
            .collect();
        println!("{}: {:?}", config.node_name, watch_result);
        let _ = client
            .post(&config.remote)
            .json(&Post {
                node_name: config.node_name.to_string(),
                time: SystemTime::now(),
                watch: watch_result,
            })
            .send()
            .await;
        tokio::time::sleep(time::Duration::from_secs(1)).await;
    }
}
