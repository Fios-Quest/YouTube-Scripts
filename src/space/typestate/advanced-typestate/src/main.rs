trait PullRequestState {}

struct Open;
struct Approved;
struct Rejected;
struct Merged;

impl PullRequestState for Open {}
impl PullRequestState for Approved {}
impl PullRequestState for Rejected {}
impl PullRequestState for Merged {}

struct PullRequest<S: PullRequestState> {
    state: S,
    // ...other PR details ...
}

impl PullRequest<Open> {
    fn open() -> Self {
        Self {
            state: Open,
        }
    }
}

impl PullRequest<Open> {
    fn approve(self) -> PullRequest<Approved> {
        PullRequest::<Approved> {
            state: Approved,
            // ... move self into PullRequest<Approved> ...
        }
    }

    fn reject(self) -> PullRequest<Rejected> {
        PullRequest::<Rejected> {
            state: Rejected,
            // ... move self into PullRequest<Approved> ...
        }
    }
}

impl PullRequest<Approved> {
    fn approve(self) -> PullRequest<Approved> {
        self
    }

    fn reject(self) -> PullRequest<Rejected> {
        PullRequest::<Rejected> {
            state: Rejected,
            // ... move self into PullRequest<Rejected> ...
        }
    }

    fn merge(self) -> PullRequest<Merged> {
        PullRequest::<Merged> {
            state: Merged,
            // ... move self into PullRequest<Merged> ...
        }
    }
}

fn main() {
    let open_pr = PullRequest::open();

    // You can not merge an open PR, the next line won't compile
    // let merged_pr = open_pr.merge();

    // You can approve a PR with Ready for Review or already Approved
    let approved_pr = open_pr.approve();
    let still_approved = approved_pr.approve();

    // Then it can be merged
    let merged_pr = still_approved.merge();

    // The `.approve()` method doesn't exist for rejected PRs, commented line won't compile
    let open_pr = PullRequest::open();
    let rejected_pr = open_pr.reject();
    // rejected_pr.approve();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_approve_open() {
        let open_pr = PullRequest::open();
        let _approved_pr = open_pr.approve();
    }

    #[test]
    fn test_can_approve_approved() {
        let open_pr = PullRequest::open();
        let _approved_pr = open_pr.approve().approve();
    }

    #[test]
    fn test_can_reject_open() {
        let open_pr = PullRequest::open();
        let _rejected_pr = open_pr.reject();
    }

    #[test]
    fn test_can_reject_approved() {
        let open_pr = PullRequest::open();
        let _rejected_pr = open_pr.approve().reject();
    }

    #[test]
    fn test_can_merge_approved() {
        let open_pr = PullRequest::open();
        let _merged_pr = open_pr.approve().merge();
    }
}