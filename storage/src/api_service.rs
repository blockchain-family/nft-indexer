use sqlx::postgres::PgPool;
use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct ApiService {
    db: Arc<PgPool>,
}

impl ApiService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

