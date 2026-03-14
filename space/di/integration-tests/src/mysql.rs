use newtypes::*;

use crate::{
    stub::StubUserStore,
    user_store::{User, UserStore},
};

pub struct MySqlConfig {}

impl MySqlConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(MySqlConfig {})
    }
}

pub struct MySql {}

impl MySql {
    pub fn connect(config: MySqlConfig) -> anyhow::Result<Self> {
        let _ = config;
        Ok(Self {})
    }
}

pub struct MySqlUserStore {
    inner: StubUserStore,
}

impl MySqlUserStore {
    pub fn new(mysql: MySql) -> Self {
        let _ = mysql;
        Self {
            inner: StubUserStore::new(),
        }
    }
}

impl UserStore for MySqlUserStore {
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
