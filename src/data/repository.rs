use std::error::Error;
use async_trait::async_trait;


#[async_trait]
pub trait Repository<T> {
  async fn create(&self, value: T) -> Result<T, Box<dyn Error + Send + Sync>>;
  async fn update(&self, value: T, id: String) -> Result<T, Box<dyn Error + Send + Sync>>;
  async fn delete(&self, id: String) -> Result<String, Box<dyn Error + Send + Sync>>;
  async fn find_all(&self) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;
  async fn find_by_id(&self, id: String) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;
  async fn find_by_query(&self, query: String) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;
}