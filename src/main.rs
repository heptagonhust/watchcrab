use core::time;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs, process::exit, thread::sleep, time::SystemTime};

#[derive(Debug, Deserialize)]
struct Config {
    remote: String,
    watch: toml::Table,
}

#[derive(Debug, Serialize)]
struct Post {
    time: SystemTime,
    watch: HashMap<String, u64>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}: expect a config file!", args[0]);
        exit(1);
    }
    let contents = fs::read_to_string(&args[1]).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    println!("{:?}", config);
    let client = reqwest::Client::new();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
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
        println!("{:?}", watch_result);
        runtime.block_on(async {
            let _ = client
                .post(&config.remote)
                .json(&Post {
                    time: SystemTime::now(),
                    watch: watch_result,
                })
                .send()
                .await;
        });
        sleep(time::Duration::from_secs(1));
    }
}
