use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, 
    response::IntoResponse, routing,
    Extension, Router, Json, Server
};

use tower_http::trace::TraceLayer;

use mediator::DefaultAsyncMediator;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use crate::data::Repository;
use crate::logger::Writer;
 

#[derive(Clone)]
#[allow(unused)]
pub struct WebHost {
    app: Router,
}

#[derive(Clone)]
#[allow(unused)]
pub struct WebHostBuilder {
    app: Router,
}

#[allow(unused)]
impl WebHost {
    pub fn new(app: Router) -> Self {
        Self { app }
    }

    pub fn add_cors(mut self, cors: CorsLayer) -> Self {
        self.app = self.app.layer(cors);
        self
    }

    pub fn add_compression(mut self) -> Self {
        self.app = self.app.layer(CompressionLayer::new());
        self
    }

    pub fn add_logger<W>(mut self, logger: Arc<Mutex<W>>) -> Self 
    where W: Writer + Send + Sync + 'static
    {
        self.app = self.app.layer(Extension(logger.clone()));
        self
    }

    pub fn add_mediator(mut self, mediator: Arc<Mutex<DefaultAsyncMediator>>) -> Self {
        self.app = self.app.layer(Extension(mediator.clone()));
        self
    }

    pub fn add_repository<Entity, R>(mut self, repository: R) -> Self 
    where 
    Entity: Serialize + DeserializeOwned + Clone + 'static + Send + Sync,
    R: Repository<Entity> + 'static + Send + Sync
    {
      let repo = Arc::new(Mutex::new(repository));
      self.app = self.app.layer(Extension(repo.clone()));
      self
    }

    pub fn add_settings<S: 'static + std::marker::Send>(mut self, setting: Arc<Mutex<S>>) -> Self {
      self.app = self.app.layer(Extension(setting.clone()));
      self
    }

    pub fn add_apidocs(mut self, apidoc: utoipa::openapi::OpenApi) -> Self {
        let cfg = Arc::new(utoipa_swagger_ui::Config::from("/swagger/swagger.json"));
        self.app = self
            .app
            .route(
                "/swagger/swagger.json",
                routing::get({
                    let doc = apidoc.clone();
                    move || async { Json(doc) }
                }),
            )
            .route(
                "/swagger-ui/*tail",
                routing::get(serve_swagger_ui).layer(Extension(cfg)),
            );
        self
    }
///Start the server
    pub async fn start(mut self)  {
        self.app = self.app.layer(TraceLayer::new_for_http());
        let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
        let server = Server::bind(&address)
            .serve(self.app.into_make_service());

        if let Err(e) = server.await {
          eprintln!("Server Error: {}", e);
        }
    }
}

#[allow(unused)]
impl WebHostBuilder {
    pub fn new(app: Router) -> Self {
        Self { app }
    }

    pub fn build(self) -> WebHost {
        WebHost {
            app: self.app,
        }
    }
}

async fn serve_swagger_ui(
    Path(tail): Path<String>,
    Extension(state): Extension<Arc<utoipa_swagger_ui::Config<'static>>>,
) -> impl IntoResponse {
    match utoipa_swagger_ui::serve(&tail[1..], state) {
        Ok(file) => file
            .map(|file| {
                (
                    StatusCode::OK,
                    [("Content-Type", file.content_type)],
                    file.bytes,
                )
                    .into_response()
            })
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response()),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
