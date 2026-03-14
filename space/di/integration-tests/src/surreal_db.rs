use newtypes::*;

use crate::{
    stub::StubUserStore,
    user_store::{User, UserStore},
};

pub struct SurrealDbConfig {}

impl SurrealDbConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(SurrealDbConfig {})
    }
}

pub struct SurrealDb {}

impl SurrealDb {
    pub fn connect(config: SurrealDbConfig) -> anyhow::Result<Self> {
        let _ = config;
        Ok(Self {})
    }
}

pub struct SurrealDbUserStore {
    inner: StubUserStore,
}

impl SurrealDbUserStore {
    pub fn new(surreal_db: SurrealDb) -> Self {
        let _ = surreal_db;
        Self {
            inner: StubUserStore::new(),
        }
    }
}

impl UserStore for SurrealDbUserStore {
    fn store(&self, user: &User) -> anyhow::Result<()> {
        self.inner.store(user)
    }

    fn get_by_email(&self, email: &EmailAddress) -> anyhow::Result<User> {
        self.inner.get_by_email(email)
    }

    fn get_by_username(&self, username: &Username) -> anyhow::Result<crate::user_store::User> {
        self.inner.get_by_username(username)
    }
}
