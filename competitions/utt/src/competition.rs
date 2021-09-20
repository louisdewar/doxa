use doxa_competition::{
    client::{async_trait, serde_json, Competition, Context, GameEvent},
    error::ContextError,
};

use crate::game_client::{UTTTGameClient, UTTTGameEvent};

pub struct UTTTCompetition;

#[async_trait]
impl Competition for UTTTCompetition {
    type GameClient = UTTTGameClient;

    const COMPETITION_NAME: &'static str = "uttt";

    async fn startup(&self, _context: &Context<Self>) -> Result<(), ContextError> {
        Ok(())
    }

    async fn on_agent_activated(
        &self,
        context: &Context<Self>,
        agent_id: String,
    ) -> Result<(), ContextError> {
        context.pair_matching(agent_id, true, || ()).await?;

        Ok(())
    }

    async fn on_agent_deactivated(
        &self,
        context: &Context<Self>,
        agent_id: String,
    ) -> Result<(), ContextError> {
        context
            .remove_game_result_by_participant_and_update_scores_by_sum(agent_id)
            .await?;
        // context.pair_matching(agent_id, true, || ()).await?;

        Ok(())
    }

    async fn on_game_event(
        &self,
        context: &Context<Self>,
        event: GameEvent<UTTTGameEvent>,
    ) -> Result<(), ContextError> {
        match event.payload {
            UTTTGameEvent::Scores {
                draws,
                a_wins,
                b_wins,
            } => {
                let game = event.game_id;
                let agents = context.get_game_participants_ordered(game).await?;

                let draws = draws as i32;
                let a_score = a_wins as i32 * 2 + draws;
                let b_score = b_wins as i32 * 2 + draws;

                context
                    .add_game_results_active(
                        game,
                        agents.into_iter().zip(vec![a_score, b_score].into_iter()),
                        true,
                    )
                    .await?;

                // context
                //     .add_game_result(agents[0].clone(), game, a_score)
                //     .await?;
                // context
                //     .add_game_result(agents[1].clone(), game, b_score)
                //     .await?;

                // context
                //     .set_score_by_game_result_sum(agents[0].clone())
                //     .await?;
                // context
                //     .set_score_by_game_result_sum(agents[1].clone())
                //     .await?;
            }
            _ => {}
        }

        // context.upsert_score(agent, score);

        Ok(())
    }

    fn event_filter(
        game_event: UTTTGameEvent,
        _is_admin: bool,
        _agent: Option<usize>,
    ) -> Option<serde_json::Value> {
        Some(serde_json::to_value(game_event).expect("failed to serialize game event"))
    }
}
