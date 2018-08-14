
//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web

use gears::structure::model::ModelDocument;

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, Error, FutureResponse, HttpRequest,
    HttpResponse, Json, State,
};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

use model_schema;

use model_schema::create_schema;
use model_schema::Schema;

use serde_json;

struct AppState {
    executor: Addr<GraphQLExecutor>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    schema: Arc<Schema>,
}

impl GraphQLExecutor {
    fn new(schema: Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor { schema: schema }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let res = msg.0.execute(&self.schema, &());
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

fn graphiql(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

fn graphql(
    (st, data): (State<AppState>, Json<GraphQLData>),
) -> FutureResponse<HttpResponse> {
    st.executor
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

pub fn serve(model: &ModelDocument) {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let sys = actix::System::new("model-graphql");

    let schema = Arc::new(create_schema());
    let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState{executor: addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/graphql", |r| r.method(http::Method::POST).with(graphql))
            .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}



