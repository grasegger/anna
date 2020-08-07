use crate::event_mapping::EventMapping;
use async_trait::async_trait;

#[async_trait]
pub trait Component {
    async fn get_event_mapping(&self) -> Result<Vec<EventMapping>, String>;
    async fn render(&self) -> Result<String, String>;
    async fn get_mount_point(&self) -> Result<String, String>;
}
