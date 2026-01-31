use newtypes::*;

use crate::{
    stub::StubUserStore,
    user_store::{User, UserStore},
};

pub struct PostgresConfig {}

impl PostgresConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(PostgresConfig {})
    }
}

pub struct Postgres {}

impl Postgres {
    pub fn connect(config: PostgresConfig) -> anyhow::Result<Self> {
        let _ = config;
        Ok(Self {})
    }
}

pub struct PostgresUserStore {
    inner: StubUserStore,
}

impl PostgresUserStore {
    pub fn new(postgres: Postgres) -> Self {
        let _ = postgres;
        Self {
            inner: StubUserStore::new(),
        }
    }
}

impl UserStore for PostgresUserStore {
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
