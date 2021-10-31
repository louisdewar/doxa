use std::time::Duration;

use doxa_core::tokio;

/// A way for other parts of the system to retrieve the stored agents.
/// This happens over the public download URL so it is possible for components to run on completely
/// different servers without access to the same file system.
pub struct AgentRetrieval {
    client: reqwest::Client,
    download_base: String,
}

impl AgentRetrieval {
    /// `download_base` is the download URL of the storage system such that appending
    /// `{competition_name}/{agent_id}` yields the correct download URL for an agent.
    pub fn new(download_base: String) -> AgentRetrieval {
        // TODO: in future maybe configure an agent id that could never be generated but always returns a
        // 200 okay result along with a recognisable string to test the download_base url.
        AgentRetrieval {
            client: reqwest::Client::new(),
            download_base,
        }
    }

    pub async fn download_agent(
        &self,
        agent_id: &str,
        competition: &str,
    ) -> Result<reqwest::Response, crate::RetrievalError> {
        let mut i = 0;
        loop {
            match self
                .client
                .get(format!(
                    "{}{}/{}?active=true",
                    self.download_base, competition, agent_id
                ))
                .send()
                .await
            {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if i == 5 {
                        return Err(e);
                    }

                    tokio::time::sleep(Duration::from_secs(i)).await;
                    i += 1;
                }
            };
        }
    }
}
