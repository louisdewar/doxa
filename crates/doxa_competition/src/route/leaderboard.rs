use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use doxa_db::model::{leaderboard::LeaderboardScore, storage::AgentUpload, user::User};
use doxa_user::PublicBasicUserInfo;
use serde_json::json;

use crate::client::{Competition, Context};

fn leaderboard_response(leaderboard: Vec<(User, LeaderboardScore, AgentUpload)>) -> HttpResponse {
    let mut output = Vec::with_capacity(leaderboard.len());

    for (user, entry, agent) in leaderboard {
        output
            .push(json!({ "user": PublicBasicUserInfo::from(user), "agent": entry.agent, "score": entry.score, "activated_at": agent.activated_at, "uploaded_at": agent.uploaded_at }));
    }

    HttpResponse::Ok().json(json!({ "leaderboard": output }))
}

/// The default route for `_leaderboard/active`.
pub async fn active_leaderboard_primary<C: Competition + ?Sized>(
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let leaderboard = context.get_leaderboard(None).await?;

    Ok(leaderboard_response(leaderboard))
}

/// The default route for `_leaderboard/active/{leaderboard}`.
pub async fn active_leaderboard<C: Competition + ?Sized>(
    context: web::Data<Context<C>>,
    path: web::Path<String>,
) -> EndpointResult {
    let key = path.into_inner();
    let leaderboard = context.get_leaderboard(Some(key)).await?;
    Ok(leaderboard_response(leaderboard))
}
