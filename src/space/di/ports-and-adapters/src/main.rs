use std::str::FromStr;

use newtypes::*;

struct MySql {}

struct User {}

trait UserStore {
    fn store(&self, user: &User) -> anyhow::Result<()>;

    fn get_by_email(&self, email: &EmailAddress) -> anyhow::Result<User>;

    fn get_by_username(&self, username: &Username) -> anyhow::Result<User>;
}

struct MySqlUserStore {
    mysql: MySql,
}

impl UserStore for MySqlUserStore {
    fn store(&self, user: &User) -> anyhow::Result<()> {
        user;
        Ok(())
    }

    fn get_by_email(&self, email: &EmailAddress) -> anyhow::Result<User> {
        email;
        Ok(User {})
    }

    fn get_by_username(&self, username: &Username) -> anyhow::Result<User> {
        username;
        Ok(User {})
    }
}

fn main() -> anyhow::Result<()> {
    let user_store = MySqlUserStore { mysql: MySql {} };

    let _ = &user_store.mysql;
    trait_user(user_store);
    Ok(())
}

fn trait_user<U: UserStore>(u: U) -> anyhow::Result<()> {
    let user = User {};
    let username = Username::from_str("")?;
    let email = EmailAddress::from_str("")?;

    u.store(&user)?;
    u.get_by_username(&username)?;
    u.get_by_email(&email)?;

    Ok(())
}
