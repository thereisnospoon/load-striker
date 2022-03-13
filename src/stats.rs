use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub enum MonitoringEvent {
    RequestFinished { target: String, response_code: u16 },
}

pub fn consume_stats(rx: Receiver<MonitoringEvent>) {
    thread::spawn(move || {
        let mut aggregated_stats: HashMap<String, HashMap<u16, u64>> = HashMap::new();
        loop {
            thread::sleep(Duration::from_secs(2));

            rx.try_iter().for_each(|event| {
                let MonitoringEvent::RequestFinished {
                    target,
                    response_code,
                } = event;
                let target_stats = aggregated_stats.entry(target).or_insert(HashMap::new());
                let counts_for_response_code = (*target_stats).entry(response_code).or_insert(0);
                *counts_for_response_code += 1;
            });

            for (target, stats) in &aggregated_stats {
                let output = stats.iter().fold("".to_string(), |line, (code, count)| {
                    format!("{}   {}: {}", line, code, count)
                });
                println!("Responses from {}    =>    {}", target, output);
            }
        }
    });
}
