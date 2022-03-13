use crate::stats::MonitoringEvent;
use futures::future::join_all;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::mpsc::Sender;

async fn do_requests(targets: Vec<&str>, stats_sink: Sender<MonitoringEvent>) {
    loop {
        for target in targets.iter() {
            let response = reqwest::get(*target).await;
            match response {
                Ok(response) => {
                    let response_code = response.status().as_u16();
                    let _ = response.text().await;
                    stats_sink
                        .send(MonitoringEvent::RequestFinished {
                            target: target.to_string(),
                            response_code,
                        })
                        .unwrap();
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            }
        }
    }
}

pub async fn run_concurrent_requests(
    num_users: u32,
    targets: Vec<String>,
    stats_sink: Sender<MonitoringEvent>,
) {
    let mut busy_users = vec![];
    for _ in 1..=num_users {
        let mut user_targets = vec![];
        targets
            .iter()
            .for_each(|target| user_targets.push(&target[..]));
        user_targets.shuffle(&mut thread_rng());
        busy_users.push(do_requests(user_targets, stats_sink.clone()));
    }
    join_all(busy_users).await;
}
