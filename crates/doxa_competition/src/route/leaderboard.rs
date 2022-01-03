use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use serde_json::json;

use crate::client::{Competition, Context};

/// The default route for `_leaderboard/active`.
pub async fn active_leaderboard_primary<C: Competition + ?Sized>(
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let leaderboard = context.get_leaderboard(None).await?;

    let mut output = Vec::with_capacity(leaderboard.len());

    for (user, entry) in leaderboard {
        output
            .push(json!({ "username": user.username, "agent": entry.agent, "score": entry.score }));
    }

    // Either show the agent id or null if it is None:
    Ok(HttpResponse::Ok().json(json!({ "leaderboard": output })))
}

/// The default route for `_leaderboard/active/{leaderboard}`.
pub async fn active_leaderboard<C: Competition + ?Sized>(
    context: web::Data<Context<C>>,
    path: web::Path<String>,
) -> EndpointResult {
    let key = path.into_inner();
    let leaderboard = context.get_leaderboard(Some(key)).await?;

    let mut output = Vec::with_capacity(leaderboard.len());

    for (user, entry) in leaderboard {
        output
            .push(json!({ "username": user.username, "agent": entry.agent, "score": entry.score }));
    }

    // Either show the agent id or null if it is None:
    Ok(HttpResponse::Ok().json(json!({ "leaderboard": output })))
}
