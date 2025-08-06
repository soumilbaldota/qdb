use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use crossbeam::queue::ArrayQueue;
use tokio::sync::{oneshot};
use qdb::types::{QueryRequest};
use qdb::{query_parser, query_planner, query_executor};

const PARALLEL_EXECUTORS: usize = 8;
const QUEUE_SIZE: usize = 2048;

fn bench_full_pipeline(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let (parser_queue, _planner_queue, _executor_queue) = rt.block_on(async {
        let planner_queue = Arc::new(ArrayQueue::new(QUEUE_SIZE));
        let parser_queue = Arc::new(ArrayQueue::new(QUEUE_SIZE));
        let executor_queue = Arc::new(ArrayQueue::new(QUEUE_SIZE));

        tokio::spawn(query_parser::run(Arc::clone(&parser_queue), Arc::clone(&planner_queue)));
        tokio::spawn(query_planner::run(Arc::clone(&planner_queue), Arc::clone(&executor_queue)));
        query_executor::start_executor_pool(Arc::clone(&executor_queue), PARALLEL_EXECUTORS);

        (parser_queue, planner_queue, executor_queue)
    });

    for batch_size in [100, 500, 1000, 5000, 10_000, 20_000, 30_000, 40_000].iter() {
        c.bench_function(&format!("pipeline - {} queries", batch_size), |b| {
            b.to_async(&rt).iter(|| async {
                let mut handles = Vec::with_capacity(*batch_size);

                for _ in 0..*batch_size {
                    let parser_queue = Arc::clone(&parser_queue);
                    handles.push(tokio::spawn(async move {
                        let (tx, rx) = oneshot::channel();
                        let req = QueryRequest {
                            raw_sql: "SELECT 1".into(),
                            respond_to: tx,
                        };

                        let _ = parser_queue.push(req);
                        let _ = rx.await;
                    }));
                }

                for h in handles {
                    let _ = h.await;
                }
            }
            );
        });
    }
}

criterion_group!(benches, bench_full_pipeline);
criterion_main!(benches);