use std::future::Future;
use std::task::{Context, Poll, Waker};
use tokio::sync::oneshot;

#[test]
fn sender_must_wait_until_receiver_closes() {
    let (mut sender, receiver) = oneshot::channel::<u8>();
    let mut closed = std::pin::pin!(sender.closed());
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(closed.as_mut().poll(&mut cx), Poll::Pending),
        "Sender::closed must remain pending while receiver is live");
    drop(receiver);
    assert!(matches!(closed.as_mut().poll(&mut cx), Poll::Ready(())),
        "Sender::closed must complete after receiver closes");
}

#[test]
fn dropping_sender_closes_receiver_without_a_value() {
    let (sender, mut receiver) = oneshot::channel::<u8>();
    drop(sender);
    assert!(receiver.try_recv().is_err());
}
