
#[derive(Copy, Clone, Debug)]
pub struct MySql {}

impl MySql {
    pub fn connect<A, U, P>(address: A, port: u8, username: U, password: P) -> anyhow::Result<Self>
    where
        A: AsRef<str>,
        U: AsRef<str>,
        P: AsRef<str>,
    {
        let _ = username;
        let _ = password;
        let _ = address;
        let _ = port;

        Ok(MySql {})
    }

    pub fn query<Q, P>(&self, query: Q, parameters: &[P]) -> anyhow::Result<()>
    where
        Q: AsRef<str>,
        P: AsRef<str>,
    {
        let _ = query;
        let _ = parameters;
        Ok(())
    }
}
