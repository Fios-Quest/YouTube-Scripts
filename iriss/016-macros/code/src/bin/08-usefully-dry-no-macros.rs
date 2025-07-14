#[derive(Clone, Debug)]
pub struct Role {
    pub id: u128,
    pub company_id: u128,
    pub name: String,
}

pub trait HasCompany {
    fn get_company_id(&self) -> u128;
}

impl HasCompany for Role {
    fn get_company_id(&self) -> u128 {
        self.company_id
    }
}

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
