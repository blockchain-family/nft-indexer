use anyhow::{anyhow, Result};
use std::{future::Future, pin::Pin};

pub struct Retrier<F, T>
where
    F: Fn() -> Pin<Box<dyn Future<Output = Result<T>> + Send>>,
{
    routine: F,
    attempts: u64,
    backoff: u64,
    factor: u64,
}

impl<F, T> Retrier<F, T>
where
    F: Fn() -> Pin<Box<dyn Future<Output = Result<T>> + Send>>,
{
    pub fn new(routine: F) -> Self {
        Self {
            routine,
            attempts: 3,
            backoff: 1,
            factor: 2,
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

    // TODO: trace id

    pub async fn run(mut self) -> Result<T> {
        for attempt in 1..=self.attempts {
            match (self.routine)().await {
                Err(e) => {
                    log::error!(
                        "Error occured: {:#?}, attempts left: {}",
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
