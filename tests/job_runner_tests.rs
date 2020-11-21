#[macro_use]
extern crate rust_ray;

use std::sync::{Arc, Condvar, Mutex};

use rust_ray::utils::JobRunner;

#[test]
fn execute_tasks() {
    let mut pool = JobRunner::new(1);
    pool.add_job(|| "hello world!".to_owned());

    let r = pool.get_result();
    assert_eq!(Some("hello world!"), r.as_deref());
}

#[test]
fn spawn_threads() {
    panic_after!(500, {
        let mut pool = JobRunner::new(2);
        let mtx = Arc::new(Mutex::new(()));
        let cond = Arc::new(Condvar::new());

        {
            let mtx = Arc::clone(&mtx);
            let cond = Arc::clone(&cond);

            pool.add_job(move || {
                let lock = mtx.lock().unwrap();
                let lock = cond.wait(lock).unwrap();

                // wait one more time to make sure that it is finished after "seconds"
                let _ = cond.wait(lock).unwrap();

                "first".to_owned()
            });
        }

        {
            let mtx = Arc::clone(&mtx);
            let cond = Arc::clone(&cond);

            pool.add_job(move || {
                let _ = mtx.lock().unwrap();
                cond.notify_all();
                "second".to_owned()
            });
        }

        assert_eq!(Some("second"), pool.get_result().as_deref());

        {
            let _ = mtx.lock().unwrap();
            cond.notify_all();
        }
        assert_eq!(Some("first"), pool.get_result().as_deref());
    })
}

#[test]
fn return_none_if_no_jobs() {
    panic_after!(500, {
        let mut pool = JobRunner::new(2);

        for k in 0..10 {
            pool.add_job(move || k);
        }

        let mut count = 0;
        while let Some(_) = pool.get_result() {
            count += 1;
        }

        assert_eq!(count, 10);
    });
}
