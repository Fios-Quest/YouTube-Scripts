use newtypes::*;

use crate::{
    stub::StubUserStore,
    user_store::{User, UserStore},
};

pub struct RedisConfig {}

impl RedisConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(RedisConfig {})
    }
}

pub struct Redis {}

impl Redis {
    pub fn connect(config: RedisConfig) -> anyhow::Result<Self> {
        let _ = config;
        Ok(Self {})
    }
}

pub struct RedisUserStore {
    inner: StubUserStore,
}

impl RedisUserStore {
    pub fn new(redis: Redis) -> Self {
        let _ = redis;
        Self {
            inner: StubUserStore::new(),
        }
    }
}

impl UserStore for RedisUserStore {
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
