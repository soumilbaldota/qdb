mod types;
mod query_parser;
mod query_planner;
mod query_executor;
mod web_server;
use crossbeam::queue::ArrayQueue;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting QDB...");
    let parser_queue = Arc::new(ArrayQueue::new(2048));
    let planner_queue = Arc::new(ArrayQueue::new(2048));
    let executor_queue = Arc::new(ArrayQueue::new(2048));

    tokio::spawn(query_parser::run(Arc::clone(&parser_queue), Arc::clone(&planner_queue)));
    tokio::spawn(query_planner::run(Arc::clone(&planner_queue), Arc::clone(&executor_queue)));
    query_executor::start_executor_pool(Arc::clone(&executor_queue), num_cpus::get());

    web_server::start(Arc::clone(&parser_queue)).await
}
