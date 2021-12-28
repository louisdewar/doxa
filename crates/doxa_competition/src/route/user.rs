use doxa_auth::{error::UserNotFound, guard::AuthGuard};
use doxa_core::{actix_web::web, error::HttpResponse, EndpointResult};
use serde_json::json;

use crate::{
    client::{Competition, Context},
    error::{NoActiveAgent, TooManyActivations, UserNotOwner},
};

use super::{
    limits::CompetitionLimits,
    response::{ActiveAgentResponse, ActiveGamesResponse, GameResponse, UserScoreResponse},
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
    Ok(HttpResponse::Ok().json(ActiveAgentResponse {
        active_agent: agent.map(|agent| agent.id),
    }))
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

    //Ok(HttpResponse::Ok().json(json!({ "agent": agent.id, "score": score })))
    Ok(HttpResponse::Ok().json(UserScoreResponse {
        agent: agent.id,
        score,
    }))
}

// /// The default route for `_user/{username}/rank`.
// pub async fn user_rank<C: Competition + ?Sized>(
//     path: web::Path<String>,
//     context: web::Data<Context<C>>,
// ) -> EndpointResult {
//     let username = path.into_inner();
//
//     let user = context
//         .get_user_by_username(username)
//         .await?
//         .ok_or(UserNotFound)?;
//
//     todo!();
// }

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
        .map(|game| GameResponse {
            game_id: game.id,
            queued_at: game.queued_at,
            started_at: game.started_at,
            completed_at: game.completed_at,
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(ActiveGamesResponse { games }))
}

/// The default route for `_user/{username}/reactivate_active_agent`.
pub async fn reactivate_agent<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
    user_auth: AuthGuard,
    limits: web::Data<CompetitionLimits>,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    if !(user.id == user_auth.id() || user_auth.admin()) {
        return Err(UserNotOwner.into());
    }

    let agent = context
        .get_active_agent(user.id)
        .await?
        .ok_or(NoActiveAgent)?;

    if !user_auth.admin() {
        limits
            .activations
            .get_permit(format!("{}-{}", C::COMPETITION_NAME, agent.owner))
            .await?
            .map_err(TooManyActivations::from)?;
    }

    context.activate_agent(agent.id).await?;

    Ok(HttpResponse::Ok().json(json!({})))
}

/// The default route for `_user/{username}/deactivate_active_agent`.
pub async fn deactivate_agent<C: Competition + ?Sized>(
    path: web::Path<String>,
    context: web::Data<Context<C>>,
    user_auth: AuthGuard,
) -> EndpointResult {
    let username = path.into_inner();

    let user = context
        .get_user_by_username(username)
        .await?
        .ok_or(UserNotFound)?;

    if !(user.id == user_auth.id() || user_auth.admin()) {
        return Err(UserNotOwner.into());
    }

    let agent = context
        .get_active_agent(user.id)
        .await?
        .ok_or(NoActiveAgent)?;

    context.deactivate_agent(agent.id).await?;

    Ok(HttpResponse::Ok().json(json!({})))
}
