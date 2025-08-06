use std::sync::Arc;
use crossbeam::queue::ArrayQueue;
use crate::types::{PlannedQuery, QueryResult};
use tokio::task;

pub fn start_executor_pool(
    queue: Arc<ArrayQueue<PlannedQuery>>,
    parallelism: usize,
) {
    println!("Starting query executor pool with {} workers", parallelism);
    for _ in 0..parallelism {
        let queue = Arc::clone(&queue);
        task::spawn(async move {
            loop {
                if let Some(job) = queue.pop() {
                    let result = execute(&job.plan);
                    let _ = job.respond_to.send(QueryResult { output: result });
                } else {
                    tokio::task::yield_now().await;
                }
            }
        });
    }
}

fn execute(plan: &str) -> String {
    format!("Executed: {}", plan)
}
