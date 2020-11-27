#[macro_export]
macro_rules! panic_after {
    ($millis:expr, $block:block) => {{
        let (tx, rx) = ::std::sync::mpsc::channel();
        let th = ::std::thread::spawn(move || {
            let r = ::std::panic::catch_unwind(|| $block);
            tx.send(r).unwrap();
        });

        let timeout = ::std::time::Duration::from_millis($millis);
        let thread_result = rx
            .recv_timeout(timeout)
            .expect(concat!($millis, " ms timeout"));

        if let Err(err) = thread_result {
            ::std::panic::resume_unwind(err);
        }

        th.join().expect("stop thread");
    }};
}
