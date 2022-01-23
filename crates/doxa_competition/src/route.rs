pub(crate) mod agent;
pub(crate) mod game;
pub(crate) mod leaderboard;
pub(crate) mod limits;
pub(crate) mod upload;
pub(crate) mod user;

pub(crate) mod response;

// TODO:
// - Check that a user is enrolled in this competition and return that as an error before returning
// the result for `_user` routes
