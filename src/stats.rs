use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub enum MonitoringEvent {
    RequestFinished {
        target: String,
        response_code: Option<u16>,
        error: Option<String>,
    },
}

pub fn consume_stats(rx: Receiver<MonitoringEvent>) {
    thread::spawn(move || {
        let mut aggregated_stats: HashMap<String, (HashMap<u16, u64>, HashMap<String, u64>)> =
            HashMap::new();
        loop {
            thread::sleep(Duration::from_secs(2));

            rx.try_iter().for_each(|event| {
                let MonitoringEvent::RequestFinished {
                    target,
                    response_code,
                    error,
                } = event;

                if let Some(response_code) = response_code {
                    let target_stats = &mut aggregated_stats
                        .entry(target.clone())
                        .or_insert((HashMap::new(), HashMap::new()))
                        .0;

                    let counts_for_response_code =
                        (*target_stats).entry(response_code).or_insert(0);
                    *counts_for_response_code += 1;
                };

                if let Some(error) = error {
                    let target_errors = &mut aggregated_stats
                        .entry(target)
                        .or_insert((HashMap::new(), HashMap::new()))
                        .1;

                    let counts_for_error = (*target_errors).entry(error).or_insert(0);
                    *counts_for_error += 1;
                };
            });

            for (target, (response_codes, errors)) in &aggregated_stats {
                let output = response_codes
                    .iter()
                    .fold("".to_string(), |line, (code, count)| {
                        format!("{}   {}: {}", line, code, count)
                    });
                println!("Responses from {}    =>    {}", target, output);
                if errors.len() > 0 {
                    println!("Errors:");
                    for (error, error_count) in errors {
                        println!("{} => {}", error, error_count);
                    }
                }
                println!("===========================================\n\n\n")
            }
        }
    });
}
