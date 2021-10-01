use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use serde_json::json;

use crate::{
    client::{Competition, Context},
    error::AgentNotFound,
};

/// The default route for `_agent/{agent_id}/games`.
pub async fn agent_games<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let agent_id = path.into_inner();

    // Used for ensuring that the agent exists otherwise the `get_agent_games` will return an
    // empty vec with no way to differentiate a non existant agent vs one with no matches
    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    let games: Vec<serde_json::Value> = context
        .get_agent_games(agent_id)
        .await?
        .into_iter()
        .map(|game| {
            json!({
                "id": game.id,
                "start_time": game.start_time,
                "end_time": game.complete_time
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "games": games })))
}

/// The default route for `_agent/{agent_id}/score`.
pub async fn agent_score<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let agent_id = path.into_inner();

    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    let score = context.get_agent_score(agent.id.clone()).await?;

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "score": score })))
}
