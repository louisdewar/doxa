use doxa_auth::error::UserNotFound;
use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use serde_json::json;

use crate::{
    client::{Competition, Context},
    error::NoActiveAgent,
};

/// The default route for `_user/{username}/active_agent`.
pub async fn user_active_agent<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    // TODO: check user enrollment (also in other routes)

    let agent = context.get_active_agent(user.id).await?;

    // Either show the agent id or null if it is None:
    Ok(HttpResponse::Ok().json(json!({ "active_agent": agent.map(|agent| agent.id) })))
}

// TODO: this route may not be a good idea because historical scores are likely to be inaccurate
/// The default route for `_user/{username}/high_score`.
pub async fn user_high_score<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    let best = context.get_high_score(user.id).await?;

    let score = best.as_ref().map(|best| best.score);
    let agent = best.map(|best| best.agent);

    Ok(HttpResponse::Ok().json(json!({ "agent": agent, "score": score })))
}

/// The default route for `_user/{username}/score`.
pub async fn user_score<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    // TODO: check user enrollment (also in other routes)

    let agent = context
        .get_active_agent(user.id)
        .await?
        .ok_or(NoActiveAgent)?;

    let score = context.get_agent_score(agent.id.clone()).await?;

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "score": score })))
}

/// The default route for `_user/{username}/rank`.
pub async fn user_rank<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    todo!();
}

/// The default route for `_user/{username}/agents`.
pub async fn user_agents<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    // TODO: check user enrollment

    let agents = context
        .get_user_agents(user.id)
        .await?
        .into_iter()
        .map(|agent| json!({ "id": agent.id, "uploaded_at": agent.uploaded_at }))
        .collect::<Vec<_>>();

    // Either show the agent id or null if it is None:
    Ok(HttpResponse::Ok().json(json!({ "agents": agents })))
}

/// The default route for `_user/{username}/active_games`.
pub async fn user_active_games<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    // TODO: check user enrollment

    let games = context
        .get_user_active_games(user.id)
        .await?
        .into_iter()
        .map(|game| {
            json!({
                "id": game.id,
                "start_time": game.start_time,
                "end_time": game.complete_time
            })
        })
        .collect::<Vec<_>>();

    // Either show the agent id or null if it is None:
    Ok(HttpResponse::Ok().json(json!({ "games": games })))
}
