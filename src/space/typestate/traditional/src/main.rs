struct InvalidStateError;

#[derive(Debug, PartialEq)]
enum Status {
    Open,
    Approved,
    Rejected,
    Merged,
}

struct PullRequest {
    status: Status,
    // ...other PR details ...
}

impl PullRequest {
    // Approve the request
    fn approve(&mut self) -> Result<*mut Self, InvalidStateError> {
        if self.status == Status::Open || self.status == Status::Approved {
            self.status = Status::Approved;
            Ok(self)
        } else {
            Err(InvalidStateError)
        }
    }

    // Reject the request
    fn reject(&mut self) -> Result<&mut Self, InvalidStateError> {
        if self.status == Status::Open
            || self.status == Status::Approved
            || self.status == Status::Rejected
        {
            self.status = Status::Rejected;
            Ok(self)
        } else {
            Err(InvalidStateError)
        }
    }

    // Merge the request
    fn merge(&mut self) -> Result<&mut Self, InvalidStateError> {
        if self.status == Status::Approved {
            self.status = Status::Merged;
            Ok(self)
        } else {
            Err(InvalidStateError)
        }
    }
}

fn main() {
    // You can approve a PR that's just been opened or already Approved
    let mut pr = PullRequest {
        status: Status::Open,
    };
    assert!(pr.approve().is_ok());
    assert_eq!(pr.status, Status::Approved);
    assert!(pr.approve().is_ok());

    // You can not approve or merge rejected PR
    let mut pr = PullRequest {
        status: Status::Rejected,
    };
    assert!(pr.approve().is_err());
    assert!(pr.merge().is_err());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_approve_open() {
        let mut pr = PullRequest { status: Status::Open };
        assert!(pr.approve().is_ok());
        assert_eq!(pr.status, Status::Approved);
    }
    #[test]
    fn test_can_approve_approved() {
        let mut pr = PullRequest { status: Status::Approved };
        assert!(pr.approve().is_ok());
        assert_eq!(pr.status, Status::Approved);
    }
    #[test]
    fn test_can_reject_open() {
        let mut pr = PullRequest { status: Status::Open };
        assert!(pr.reject().is_ok());
        assert_eq!(pr.status, Status::Rejected);
    }
    #[test]
    fn test_can_reject_approved() {
        let mut pr = PullRequest { status: Status::Approved };
        assert!(pr.reject().is_ok());
        assert_eq!(pr.status, Status::Rejected);
    }
    #[test]
    fn test_can_merge_approved() {
        let mut pr = PullRequest { status: Status::Approved };
        assert!(pr.merge().is_ok());
        assert_eq!(pr.status, Status::Merged);
    }
    #[test]
    fn test_can_not_approve_rejected() {
        let mut pr = PullRequest { status: Status::Rejected };
        assert!(pr.approve().is_err());
        assert_eq!(pr.status, Status::Rejected);
    }
    #[test]
    fn test_can_not_merge_reject() {
        let mut pr = PullRequest { status: Status::Rejected };
        assert!(pr.merge().is_err());
        assert_eq!(pr.status, Status::Rejected);
    }
    #[test]
    fn test_can_not_approve_merged() {
        let mut pr = PullRequest { status: Status::Merged };
        assert!(pr.approve().is_err());
        assert_eq!(pr.status, Status::Merged);
    }
    #[test]
    fn test_can_not_reject_merged() {
        let mut pr = PullRequest { status: Status::Merged };
        assert!(pr.reject().is_err());
        assert_eq!(pr.status, Status::Merged);
    }
}
