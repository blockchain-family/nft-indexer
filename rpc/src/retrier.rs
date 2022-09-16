use anyhow::{anyhow, Result};
use std::future::Future;

pub struct Retrier<C, F, T>
where
    C: Fn() -> F,
    F: Future<Output = Result<T>>,
{
    routine: C,
    attempts: u64,
    backoff: u64,
    factor: u64,
}

impl<C, F, T> Retrier<C, F, T>
where
    C: Fn() -> F,
    F: Future<Output = Result<T>>,
{
    pub fn new(routine: C) -> Self {
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
