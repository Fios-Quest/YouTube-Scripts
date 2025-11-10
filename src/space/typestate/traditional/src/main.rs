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
    fn approve(&mut self) -> Result<(), InvalidStateError> {
        if self.status == Status::Open || self.status == Status::Approved {
            self.status = Status::Approved;
            Ok(())
        } else {
            Err(InvalidStateError)
        }
    }

    // Reject the request
    fn reject(&mut self) -> Result<(), InvalidStateError> {
        if self.status == Status::Open
            || self.status == Status::Approved
            || self.status == Status::Rejected
        {
            self.status = Status::Rejected;
            Ok(())
        } else {
            Err(InvalidStateError)
        }
    }

    // Merge the request
    fn merge(&mut self) -> Result<(), InvalidStateError> {
        if self.status == Status::Approved {
            self.status = Status::Merged;
            Ok(())
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
