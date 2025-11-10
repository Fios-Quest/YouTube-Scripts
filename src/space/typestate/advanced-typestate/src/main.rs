use std::marker::PhantomData;

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
    phantom_state: PhantomData<S>,
    // ...other PR details ...
}

impl <S: PullRequestState> PullRequest<S> {
    fn new() -> Self {
        Self {
            phantom_state: PhantomData,
        }
    }
}

impl PullRequest<Open> {
    fn approve(self) -> PullRequest<Approved> {
        PullRequest::<Approved> {
            phantom_state: PhantomData,
            // ... move self into PullRequestApproved ...
        }
    }

    fn reject(self) -> PullRequest<Rejected> {
        PullRequest::<Rejected> {
            phantom_state: PhantomData,
            // ... move self into PullRequestApproved ...
        }
    }
}

impl PullRequest<Approved> {
    fn approve(self) -> PullRequest<Approved> {
        self
    }

    fn reject(self) -> PullRequest<Rejected> {
        PullRequest::<Rejected> {
            phantom_state: PhantomData,
            // ... move self into PullRequestApproved ...
        }
    }

    fn merge(self) -> PullRequest<Merged> {
        PullRequest::<Merged> {
            phantom_state: PhantomData,
            // ... move self into PullRequestApproved ...
        }
    }
}

fn main() {
    let open_pr = PullRequest::<Open>::new();

    // You can not merge an open PR, the next line won't compile
    // let merged_pr = open_pr.merge();

    // You can approve a PR with Ready for Review or already Approved
    let approved_pr = open_pr.approve();
    let still_approved = approved_pr.approve();

    // Then it can be merged
    let merged_pr = still_approved.merge();

    // The `.approve()` method doesn't exist for rejected PRs, commented line won't compile
    let rejected_pr = PullRequest::<Rejected>::new();
    // rejected_pr.approve();
}
