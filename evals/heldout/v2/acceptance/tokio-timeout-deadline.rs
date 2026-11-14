use std::future::pending;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::time::timeout;

#[test]
fn ready_future_wins_even_at_a_zero_duration_deadline() {
    let runtime = Builder::new_current_thread().enable_time().build().unwrap();
    runtime.block_on(async {
        let result = timeout(Duration::ZERO, async { 42u32 }).await;
        assert_eq!(result.expect("ready work must be polled first"), 42);
    });
}

#[test]
fn genuinely_pending_work_still_expires() {
    let runtime = Builder::new_current_thread().enable_time().build().unwrap();
    runtime.block_on(async {
        let result = timeout(Duration::from_millis(2), pending::<()>()).await;
        assert!(result.is_err(), "pending work must time out");
    });
}
