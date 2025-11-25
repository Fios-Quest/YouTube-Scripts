struct PullRequestOpen {
    // ...other PR details ...
}

struct PullRequestApproved {
    // ...other PR details ...
}

struct PullRequestRejected {
    // ...other PR details ...
}

struct PullRequestMerged {
    // ...other PR details...
}

impl PullRequestOpen {
    fn approve(self) -> PullRequestApproved {
        PullRequestApproved {
            // ... move self into PullRequestApproved ...
        }
    }

    fn reject(self) -> PullRequestRejected {
        PullRequestRejected {
            // ... move self into PullRequestApproved ...
        }
    }
}

impl PullRequestApproved {
    fn approve(self) -> PullRequestApproved {
        self
    }

    fn reject(self) -> PullRequestRejected {
        PullRequestRejected {
            // ... move self into PullRequestApproved ...
        }
    }

    fn merge(self) -> PullRequestMerged {
        PullRequestMerged {
            // ... move self into PullRequestApproved ...
        }
    }
}

fn main() {
    let open_pr = PullRequestOpen {};

    // You can not merge an open PR, the next line won't compile
    // let merged_pr = open_pr.merge();

    // You can approve a PR with Ready for Review or already Approved
    let approved_pr = open_pr.approve();
    let still_approved = approved_pr.approve();

    // Then it can be merged
    let merged_pr = still_approved.merge();

    // The `.approve()` method doesn't exist for rejected PRs, commented line won't compile
    let rejected_pr = PullRequestRejected {};
    // rejected_pr.approve();
}
