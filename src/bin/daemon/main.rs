use std::time::Duration;
use servus::execution::*;
use tokio::time::delay_for;

/// Number of milliseconds to sleep between every job scheduler check.
/// Perhaps should be configurable.
const REFRESH_RATE: u64 = 500;

#[tokio::main]
async fn main()
{        
    let mut job_scheduler = ServusJobScheduler::new();

    println!("Started daemon.");

    loop {
        
        job_scheduler.schedule_jobs();

        job_scheduler.tick();

        delay_for(Duration::from_millis(REFRESH_RATE)).await;
    }
}