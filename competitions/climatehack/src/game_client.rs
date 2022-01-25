use std::{path::PathBuf, sync::Arc, time::Duration};

use doxa_competition::{
    client::{async_trait, GameClient, GameContext, GameError, Mount},
    tokio::{self, io::AsyncWriteExt},
    tracing::{debug, info},
};
use serde::{Deserialize, Serialize};

use crate::{dataset::Datasets, error::ClimateHackError, support::Scorer};

/// The maximum time for an agent to complete predictions of all the images in all the series of
/// a single group.
const MAX_SERIES_GROUP_TIME: Duration = Duration::from_secs(15 * 60);
const MAX_STARTUP_TIME: Duration = Duration::from_secs(60);

#[derive(Serialize, Deserialize)]
pub struct ClimateHackMatchRequest {
    pub dataset: String,
}

#[derive(Serialize, Deserialize)]
pub enum ClimateHackGameEvent {
    CheckpointScore {
        checkpoint: u32,
        score: f64,
        dataset: String,
    },
    FinalScore {
        score: f64,
        dataset: String,
    },
}

pub struct ClimateHackGameClient {
    pub(crate) datasets: Arc<Datasets>,
    pub(crate) python_bin: PathBuf,
}

impl ClimateHackGameClient {
    // Use inner async method for better diagnostics (avoid async_trait)
    async fn run_inner<'a>(
        &self,
        match_request: ClimateHackMatchRequest,
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), GameError<ClimateHackError>> {
        context.expect_n_agents(1)?;
        let dataset_name = match_request.dataset;
        info!(dataset=%dataset_name, "starting climate hack evaluation");
        let dataset = self
            .datasets
            .get_dataset(&dataset_name)
            .map_err(ClimateHackError::from)?;
        let group_count = dataset.group_count;
        let work_dir_path = context.work_dir_path().await?;
        let scorer = Scorer::new(self.python_bin.clone(), work_dir_path.join("scorer.py"))
            .await
            .map_err(ClimateHackError::Scorer)?;

        context
            .reboot_agent(
                0,
                vec!["/climatehack_test_x".to_string(), "/output".to_string()],
            )
            .await?;
        context.set_max_message_time(Some(MAX_STARTUP_TIME));
        let message = context.next_message(0).await.map_err(|e| {
            if e.is_message_receive_timeout() {
                GameError::Client(ClimateHackError::TimeoutStartup)
            } else {
                e.into()
            }
        })?;

        if message != b"STARTUP" {
            return Err(ClimateHackError::TimeoutGroup.into());
        }

        context.set_max_message_time(Some(MAX_SERIES_GROUP_TIME));

        let mut total_score = 0.0;
        for checkpoint in 0..group_count {
            debug!(%checkpoint, "started checkpoint");

            context
                .send_message_to_agent(0, format!("Process {}.npz\n", checkpoint).as_bytes())
                .await?;

            let message = context.next_message(0).await?;
            let expected = format!("Exported {}.npz", checkpoint);

            if message != expected.as_bytes() {
                return Err(GameError::Client(ClimateHackError::InvalidMessage {
                    message: String::from_utf8_lossy(message).to_string(),
                    expected,
                }));
            }

            let group_output_path = work_dir_path.join(format!("{}.npz", checkpoint));
            // TODO: create variant of take_file that returns a Stream and/or redirects to file
            // currently this loads the entire set of results into memory
            let group = context
                .take_file(0, format!("/output/{}.npz", checkpoint))
                .await?;

            let mut file = tokio::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&group_output_path)
                .await
                .map_err(ClimateHackError::WriteGroupError)?;

            file.write_all(&group)
                .await
                .map_err(ClimateHackError::WriteGroupError)?;

            let group_score = scorer
                .score(
                    &dataset.true_y_path.join(format!("{}.npz", checkpoint)),
                    group_output_path,
                )
                .await
                .map_err(ClimateHackError::Scorer)?;

            total_score += group_score;
            context
                .emit_game_event(
                    ClimateHackGameEvent::CheckpointScore {
                        checkpoint,
                        score: group_score,
                        dataset: dataset_name.clone(),
                    },
                    format!("checkpoint_{}", checkpoint),
                )
                .await?;

            info!(checkpoint=%checkpoint, "completed scoring checkpoint");
        }

        let final_score = total_score / (group_count as f64);

        context
            .emit_game_event(
                ClimateHackGameEvent::FinalScore {
                    score: final_score,
                    dataset: dataset_name.clone(),
                },
                "final",
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl GameClient for ClimateHackGameClient {
    type Error = ClimateHackError;

    type MatchRequest = ClimateHackMatchRequest;

    type GameEvent = ClimateHackGameEvent;

    const AGENT_RAM_MB: u64 = 1024;
    const AGENT_SCRATCH_MB: u64 = 2 * 1024;

    async fn run<'a>(
        &self,
        match_request: ClimateHackMatchRequest,
        context: &mut GameContext<'a, Self>,
    ) -> Result<(), GameError<Self::Error>> {
        self.run_inner(match_request, context).await
    }

    fn additional_mounts(&self, match_request: &Self::MatchRequest) -> Vec<Mount> {
        vec![Mount {
            path_on_host: self
                .datasets
                .get_dataset(&match_request.dataset)
                .expect("TODO: allow additionl_mounts to return an error")
                .x_image_path
                .clone(),
            path_on_guest: "/climatehack_test_x".to_string(),
            read_only: true,
        }]
    }
}
