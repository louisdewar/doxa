use crate::client::Competition;
use async_trait::async_trait;
use doxa_mq::model::UploadEvent;

/// A dummy competition for development/debugging
pub struct HelloWorldCompetiton;

#[async_trait]
impl Competition for HelloWorldCompetiton {
    async fn startup(&self, _context: &mut crate::client::Context) {
        println!("[hello_world] starting up");
    }

    fn configure_routes(&self, _service: &mut doxa_core::actix_web::web::ServiceConfig) {
        println!("[hello_world] configuring routes");
    }

    async fn on_upload(&self, _context: &mut crate::client::Context, upload_event: UploadEvent) {
        println!("[hello_world] on_upload - agent {}", upload_event.agent);
    }

    async fn on_execution_result(&self, _context: &mut crate::client::Context) {
        println!("[hello_world] on_execution_result");
    }

    fn name(&self) -> String {
        "helloworld".to_string()
    }
}
