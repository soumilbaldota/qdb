use std::sync::Arc;
use crossbeam::queue::ArrayQueue;
use crate::types::{ParsedQuery, PlannedQuery};
use tokio::task;

pub async fn run(
    queue: Arc<ArrayQueue<ParsedQuery>>,
    executor_queue: Arc<ArrayQueue<PlannedQuery>>,
) {
    println!("Starting Query Planner!");
    loop {
        if let Some(parsed) = queue.pop() {
            let plan = plan(&parsed.ast);

            let mut job = Some(PlannedQuery {
                plan,
                respond_to: parsed.respond_to,
            });

            while let Some(j) = job {
                match executor_queue.push(j) {
                    Ok(_) => break,
                    Err(e) => {
                        job = Some(e);
                        task::yield_now().await;
                    }
                }
            }
        } else {
            task::yield_now().await;
        }
    }
}

fn plan(ast: &str) -> String {
    format!("Plan({})", ast)
}