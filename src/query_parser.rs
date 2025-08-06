use std::sync::Arc;
use crossbeam::queue::ArrayQueue;
use crate::types::{QueryRequest, ParsedQuery};
use tokio::task;

pub async fn run(
    queue: Arc<ArrayQueue<QueryRequest>>,
    planner_queue: Arc<ArrayQueue<ParsedQuery>>,
) {
    println!("Starting Query Parser");
    loop {
        if let Some(req) = queue.pop() {
            let ast = parse(&req.raw_sql);
            let mut job = Some(ParsedQuery {
                ast,
                respond_to: req.respond_to,
            });

            while let Some(j) = job {
                match planner_queue.push(j) {
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

fn parse(sql: &str) -> String {
    format!("AST({})", sql)
}
