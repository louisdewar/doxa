pub mod agent;
pub mod game;
pub mod leaderboard;
pub mod limits;
pub mod user;

pub mod response;

// TODO:
// - Check that a user is enrolled in this competition and return that as an error before returning
// the result for `_user` routes
// - Have default routes for `/_user/{username}` `/_game/{game_id}` etc..
