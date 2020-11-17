use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

pub struct JobRunner<R> {
    workers: Vec<Worker>,
    shared: SharedDataPtr<R>,
    ready: Arc<Condvar>,
}

impl<R> JobRunner<R>
where
    R: Send + 'static,
{
    pub fn new(num_workers: usize) -> Self {
        let shared = SharedData::new();
        let ready = Arc::new(Condvar::new());

        let mut workers = Vec::new();
        workers.resize_with(num_workers, || {
            Worker::new(Arc::clone(&shared), Arc::clone(&ready))
        });

        Self {
            workers,
            shared,
            ready,
        }
    }

    pub fn add_job<F>(&mut self, f: F)
    where
        F: FnOnce() -> R,
        F: Send + 'static,
    {
        let mut d = self.shared.lock().unwrap();
        d.pending_jobs.push_back(Box::new(f));
        d.expected_results += 1;

        self.ready.notify_all();
    }

    // wait for result or return None if no active jobs
    pub fn get_result(&mut self) -> Option<R> {
        let mut d = self.shared.lock().unwrap();

        while d.expected_results > 0 && d.results.is_empty() {
            d = self.ready.wait(d).unwrap();
        }

        if !d.results.is_empty() {
            d.expected_results -= 1;
        }

        d.results.pop_front()
    }
}

impl<R> Drop for JobRunner<R> {
    fn drop(&mut self) {
        {
            let mut d = self.shared.lock().unwrap();
            d.shutdown = true;
            self.ready.notify_all();
        }

        for p in self.workers.drain(..) {
            p.handle.join().expect("join thread");
        }
    }
}

type JobFunc<R> = Box<dyn FnOnce() -> R + Send>;
type SharedDataPtr<R> = Arc<Mutex<SharedData<R>>>;

struct SharedData<R> {
    pending_jobs: VecDeque<JobFunc<R>>,
    results: VecDeque<R>,
    expected_results: usize,
    shutdown: bool,
}

impl<R> SharedData<R> {
    fn new() -> SharedDataPtr<R> {
        Arc::new(Mutex::new(Self {
            pending_jobs: VecDeque::with_capacity(16),
            results: VecDeque::with_capacity(16),
            expected_results: 0,
            shutdown: false,
        }))
    }
}

struct Worker {
    handle: JoinHandle<()>,
}

impl Worker {
    fn new<R>(data: SharedDataPtr<R>, ready: Arc<Condvar>) -> Self
    where
        R: Send + 'static,
    {
        Self {
            handle: thread::spawn(move || Self::run(data, ready)),
        }
    }

    fn run<R>(data: SharedDataPtr<R>, ready: Arc<Condvar>)
    where
        R: Send + 'static,
    {
        loop {
            let job = {
                let mut d = data.lock().unwrap();
                while !d.shutdown && d.pending_jobs.is_empty() {
                    d = ready.wait(d).unwrap();
                }

                d.pending_jobs.pop_front()
            };

            let r = match job {
                Some(f) => f(),
                None => return, // shutdown worker
            };

            let mut d = data.lock().unwrap();
            d.results.push_back(r);
            ready.notify_all(); // notify runner that result is ready
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
                    let _ = cond.wait(lock).unwrap();
                    "first".to_owned()
                });
            }

            pool.add_job(move || {
                let _ = mtx.lock().unwrap();
                cond.notify_all();
                "second".to_owned()
            });

            assert_eq!(Some("second"), pool.get_result().as_deref());
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
}
