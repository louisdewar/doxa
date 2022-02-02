use std::{path::PathBuf, sync::Arc, time::Duration};

use doxa_competition::{
    client::{
        async_trait,
        limiter::{LimiterConfig, TokenBucket, ONE_DAY, ONE_HOUR},
        serde_json, Competition, Context, GameEvent,
    },
    error::ContextError,
};

use crate::{
    dataset::Datasets,
    game_client::{ClimateHackGameClient, ClimateHackGameEvent, ClimateHackMatchRequest},
};

const SCORE_MULTIPLIER: u32 = 10_000_000;

pub struct ClimateHackCompetition {
    // TODO: Maybe have a hashmap with a series of randomly generated human friendly names for the
    // different phases, then use that name for the leaderboard and match request
    pub datasets: Arc<Datasets>,
    pub python_bin: PathBuf,
    pub primary_dataset: String,
}

#[async_trait]
impl Competition for ClimateHackCompetition {
    type GameClient = ClimateHackGameClient;

    const COMPETITION_NAME: &'static str = "climatehack";

    async fn startup(&self, _context: &Context<Self>) -> Result<(), ContextError> {
        Ok(())
    }

    async fn on_agent_activated(
        &self,
        context: &Context<Self>,
        agent_id: String,
    ) -> Result<(), ContextError> {
        context
            .emit_match_request(
                vec![agent_id],
                ClimateHackMatchRequest {
                    dataset: self.primary_dataset.clone(),
                },
            )
            .await?;

        Ok(())
    }

    async fn on_agent_deactivated(
        &self,
        _context: &Context<Self>,
        _agent_id: String,
    ) -> Result<(), ContextError> {
        Ok(())
    }

    async fn on_game_event(
        &self,
        context: &Context<Self>,
        event: GameEvent<ClimateHackGameEvent>,
    ) -> Result<(), ContextError> {
        if let ClimateHackGameEvent::FinalScore { score, dataset } = event.payload {
            let agent = context.get_game_participants_ordered(event.game_id).await?[0]
                .0
                .clone();
            let score = (score * SCORE_MULTIPLIER as f64) as i32;
            // context
            //     .add_game_result_active(agent, event.game_id, score)
            //     .await?;
            // context.set_score_by_game_result_sum(key, agent)
            context
                .upsert_score(Some(format!("dataset_{}", dataset)), agent, score)
                .await?;
        }

        Ok(())
    }

    fn event_filter(
        game_event: ClimateHackGameEvent,
        _is_admin: bool,
        _agent: Option<usize>,
    ) -> Option<serde_json::Value> {
        Some(serde_json::to_value(game_event).expect("failed to serialize game event"))
    }

    fn build_game_client(&self) -> Self::GameClient {
        ClimateHackGameClient {
            datasets: self.datasets.clone(),
            python_bin: self.python_bin.clone(),
        }
    }

    fn upload_limiter(&self, key: String) -> LimiterConfig {
        let mut limiter = LimiterConfig::new(key);

        limiter
            // 1 per minute
            .add_limit(TokenBucket::new(Duration::from_secs(60), 1))
            // 2 per 5 minutes
            .add_limit(TokenBucket::new(Duration::from_secs(5 * 60), 2))
            // 4 per hour
            .add_limit(TokenBucket::new(ONE_HOUR, 4))
            // 8 per day
            .add_limit(TokenBucket::new(ONE_DAY, 8));

        limiter
    }
}
