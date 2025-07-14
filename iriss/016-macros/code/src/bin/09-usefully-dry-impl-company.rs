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
mod tests {
    use super::*;

    #[test]
    fn test_role_get_company_by_id() {
        let role = Role {
            id: 1234,
            company_id: 5678,
            name: "Test company".into(),
        };
        assert_eq!(role.get_company_id(), 5678);
    }
}

fn main() {}
