use anyhow::{anyhow, Result};
use std::{future::Future, pin::Pin};

pub struct Retrier<F, T, E>
where
    F: Fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
    E: std::fmt::Debug,
{
    routine: F,
    attempts: u64,
    backoff: u64,
    factor: u64,
    trace_id: String,
}

impl<F, T, E> Retrier<F, T, E>
where
    F: Fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
    E: std::fmt::Debug,
{
    pub fn new(routine: F) -> Self {
        Self {
            routine,
            attempts: 3,
            backoff: 1,
            factor: 2,
            trace_id: String::new(),
        }
    }

    pub fn attempts(mut self, attempts: u64) -> Self {
        self.attempts = attempts;
        self
    }

    pub fn backoff(mut self, backoff: u64) -> Self {
        self.backoff = backoff;
        self
    }

    pub fn factor(mut self, factor: u64) -> Self {
        self.factor = factor;
        self
    }

    pub fn trace_id(mut self, trace: String) -> Self {
        self.trace_id = trace;
        self
    }

    pub async fn run(mut self) -> Result<T> {
        for attempt in 1..=self.attempts {
            match (self.routine)().await {
                Err(e) => {
                    log::error!(
                        "[{}] Error occured: {:#?}, attempts left: {}",
                        self.trace_id.as_str(),
                        e,
                        self.attempts - attempt
                    );

                    let duration = std::time::Duration::from_millis(self.backoff);
                    std::thread::sleep(duration);
                    self.backoff *= self.factor;
                }
                Ok(v) => return Ok(v),
            }
        }

        Err(anyhow!("Ran out of attempts"))
    }
}
