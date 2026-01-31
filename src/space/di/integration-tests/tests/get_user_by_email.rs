use std::str::FromStr;

use integration_tests::{
    mysql::{MySql, MySqlConfig, MySqlUserStore},
    postgres::{Postgres, PostgresConfig, PostgresUserStore},
    redis::{Redis, RedisConfig, RedisUserStore},
    stub::StubUserStore,
    surreal_db::{SurrealDb, SurrealDbConfig, SurrealDbUserStore},
    user_store::{User, UserStore},
};
use newtypes::*;

fn test_get_user_by_email<U: UserStore>(user_store: U) {
    let email_address = EmailAddress::from_str("daniel@example.com").unwrap();

    let user = User {
        username: Username::from_str("Daniel").unwrap(),
        email_address: email_address.clone(),
    };

    assert!(user_store.store(&user).is_ok());
    assert_eq!(user_store.get_by_email(&email_address).unwrap(), user);
}

#[test]
fn test_mysql_get_user_by_email() {
    let config = MySqlConfig::from_env().unwrap();
    let mysql = MySql::connect(config).unwrap();
    let mysql_user_store = MySqlUserStore::new(mysql);

    test_get_user_by_email(mysql_user_store);
}

#[test]
fn test_postgres_get_user_by_email() {
    let config = PostgresConfig::from_env().unwrap();
    let postgres = Postgres::connect(config).unwrap();
    let postgres_user_store = PostgresUserStore::new(postgres);

    test_get_user_by_email(postgres_user_store);
}

#[test]
fn test_redis_get_user_by_email() {
    let config = RedisConfig::from_env().unwrap();
    let redis = Redis::connect(config).unwrap();
    let redis_user_store = RedisUserStore::new(redis);

    test_get_user_by_email(redis_user_store);
}

#[test]
fn test_surreal_db_get_user_by_email() {
    let config = SurrealDbConfig::from_env().unwrap();
    let surreal_db = SurrealDb::connect(config).unwrap();
    let surreal_db_user_store = SurrealDbUserStore::new(surreal_db);

    test_get_user_by_email(surreal_db_user_store);
}

#[test]
fn test_stub_get_user_by_email() {
    let stub_user_store = StubUserStore::new();

    test_get_user_by_email(stub_user_store);
}
