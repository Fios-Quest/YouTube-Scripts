pub trait HasCompany {
    fn get_company_id(&self) -> u128;
}

macro_rules! impl_has_company {
    ($name:ty) => {
        impl HasCompany for $name {
            fn get_company_id(&self) -> u128 {
                self.company_id
            }
        }
    };
}

#[derive(Clone, Debug)]
pub struct Role {
    pub id: u128,
    pub company_id: u128,
    pub name: String,
}

impl_has_company!(Role);

#[cfg(test)]
pub trait TestHelper: Sized {
    fn new_test() -> Result<Self, String>;
}

#[cfg(test)]
macro_rules! test_has_company_id {
    ($test_name:ident, $storable:ident) => {
        #[test]
        fn $test_name() {
            let storable = $storable::new_test().expect("Could not create storable");
            assert!(storable.get_company_id() > 0);
        }
    };
}

#[cfg(test)]
impl TestHelper for Role {
    fn new_test() -> Result<Role, String> {
        Ok(Role {
            id: 1234,
            company_id: 5678,
            name: "Test company".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_has_company_id!(test_role_get_company_by_id, Role);
}

fn main() {}
