use crate::core::Repository;
use crate::core::User;

#[derive(Default)]
pub struct State {
    pub repository: Repository,
    pub user: User,
}
