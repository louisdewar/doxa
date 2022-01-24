use std::path::PathBuf;

use doxa_competition::{
    client::{async_trait, serde_json, Competition, Context, GameEvent},
    error::ContextError,
    tokio,
};

use crate::game_client::{ClimateHackGameClient, ClimateHackGameEvent, ClimateHackMatchRequest};

const SCORE_MULTIPLIER: u32 = 10_000_000;

#[derive(Clone)]
pub struct PhaseDataset {
    pub true_y_path: PathBuf,
    pub x_image_path: PathBuf,
    pub group_count: u32,
}

impl PhaseDataset {
    pub async fn new(true_y_path: PathBuf, x_image_path: PathBuf) -> PhaseDataset {
        let mut entries = tokio::fs::read_dir(&true_y_path)
            .await
            .expect("failed to read true y path");

        let mut count = 0;
        while let Some(entry) = entries
            .next_entry()
            .await
            .expect("failed to open dir entry")
        {
            if entry.file_name().to_string_lossy().ends_with(".npz") {
                count += 1;
            }
        }

        PhaseDataset {
            true_y_path,
            x_image_path,
            group_count: count,
        }
    }
}

pub struct ClimateHackCompetition {
    // TODO: Maybe have a hashmap with a series of randomly generated human friendly names for the
    // different phases, then use that name for the leaderboard and match request
    pub dataset: PhaseDataset,
    pub python_bin: PathBuf,
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
            .emit_match_request(vec![agent_id], ClimateHackMatchRequest::Phase1)
            .await?;

        Ok(())
    }

    async fn on_agent_deactivated(
        &self,
        context: &Context<Self>,
        agent_id: String,
    ) -> Result<(), ContextError> {
        context
            .remove_game_result_by_participant_and_update_scores_by_sum(None, agent_id)
            .await?;

        Ok(())
    }

    async fn on_game_event(
        &self,
        context: &Context<Self>,
        event: GameEvent<ClimateHackGameEvent>,
    ) -> Result<(), ContextError> {
        if let ClimateHackGameEvent::FinalScore { score } = event.payload {
            let agent = context.get_game_participants_ordered(event.game_id).await?[0]
                .0
                .clone();
            let score = (score * SCORE_MULTIPLIER as f64) as i32;
            context
                .add_game_result_active(agent, event.game_id, score)
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
            dataset: self.dataset.clone(),
            python_bin: self.python_bin.clone(),
        }
    }
}
