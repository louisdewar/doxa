use doxa_auth::guard::AuthGuard;
use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use serde_json::json;

use crate::{
    client::{Competition, Context},
    error::{AgentAlreadyActive, AgentNotActive, AgentNotFound, TooManyActivations, UserNotOwner},
};

use super::limits::CompetitionLimits;

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
                "queued_at": game.queued_at,
                "started_at": game.started_at,
                "completed_at": game.completed_at,
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "games": games })))
}

/// The default route for `_agent/{agent_id}/score`.
pub async fn agent_score_primary<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let agent_id = path.into_inner();

    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    let score = context.get_agent_score(agent.id.clone(), None).await?;

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "score": score })))
}

/// The default route for `_agent/{agent_id}/score/{leaderboard}`.
pub async fn agent_score<C: Competition + ?Sized>(
    path: web::Path<(String, String)>,
    context: web::Data<Context<C>>,
) -> EndpointResult {
    let (agent_id, leaderboard_key) = path.into_inner();

    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    let score = context
        .get_agent_score(agent.id.clone(), Some(leaderboard_key))
        .await?;

    Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "score": score })))
}

/// The default route for `_agent/{agent_id}/reactivate`.
pub async fn reactivate_agent<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
    user: AuthGuard,
    limits: web::Data<CompetitionLimits>,
) -> EndpointResult {
    let agent_id = path.into_inner();

    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    if !(agent.owner == user.id() || user.admin()) {
        return Err(UserNotOwner.into());
    }

    if !user.admin() {
        limits
            .activations
            .get_permit(format!("{}-{}", C::COMPETITION_NAME, agent.owner))
            .await?
            .map_err(TooManyActivations::from)?;
    }
    context.activate_agent(agent_id).await?;

    Ok(HttpResponse::Ok().json(json!({})))
}

/// The default route for `_agent/{agent_id}/activate`.
pub async fn activate_agent<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
    user: AuthGuard,
    limits: web::Data<CompetitionLimits>,
) -> EndpointResult {
    let agent_id = path.into_inner();

    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    if !(agent.owner == user.id() || user.admin()) {
        return Err(UserNotOwner.into());
    }

    if agent.active {
        return Err(AgentAlreadyActive.into());
    }

    if !user.admin() {
        limits
            .activations
            .get_permit(format!("{}-{}", C::COMPETITION_NAME, agent.owner))
            .await?
            .map_err(TooManyActivations::from)?;
    }

    context.activate_agent(agent_id).await?;

    Ok(HttpResponse::Ok().json(json!({})))
}

/// The default route for `_agent/{agent_id}/deactivate`.
pub async fn deactivate_agent<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
    user: AuthGuard,
) -> EndpointResult {
    let agent_id = path.into_inner();

    let agent = context
        .get_agent(agent_id.clone())
        .await?
        .ok_or(AgentNotFound)?;

    if !(agent.owner == user.id() || user.admin()) {
        return Err(UserNotOwner.into());
    }

    if !agent.active {
        return Err(AgentNotActive.into());
    }

    context.deactivate_agent(agent_id).await?;

    Ok(HttpResponse::Ok().json(json!({})))
}
