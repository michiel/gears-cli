
//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web

use gears::structure::model::ModelDocument;

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, AsyncResponder, Error, FutureResponse, HttpRequest,
    HttpResponse, Json, State, http::Method, http::StatusCode, pred,
};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

use model_schema::create_schema;
use model_schema::Schema;
use model_executor::FileSystemModelStore;
use model_executor::ModelStore;

use serde_json;

struct AppState {
    executor: Addr<GraphQLExecutor>,
    modelstore: FileSystemModelStore,
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
    let html = graphiql_source("/graphql/api");
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
	    Ok(obj) => Ok(HttpResponse::Ok()
			   .content_type("application/json")
			   .body(obj)),
	    Err(_) => Ok(HttpResponse::InternalServerError().into()),
	})
    .responder()
}

static CONTENT_TYPE_JSON: &'static str = "application/json; charset=utf-8";

pub fn serve(path: &str) {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let sys = actix::System::new("model-graphql");

    let schema = Arc::new(create_schema());
    let addr_graphql = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

    /*
    let path = Path::new(&"./");
    let safe_path = format!("{}", path.display());
    */
    let modelstore = FileSystemModelStore::new(&path);

    server::new(move || {
	let graphql_app = App::with_state(AppState{
	    executor: addr_graphql.clone(),
	    modelstore: modelstore.clone(),
	})
	.prefix("graphql")
	    .middleware(middleware::Logger::default())
	    .resource("/api", |r| r.method(http::Method::POST).with(graphql))
	    .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql))
	    .resource("/", |r| r.f(graphql_index))
	    .resource("", |r| r.f(graphql_index))
	    .default_resource(|r| {
		r.method(Method::GET).f(p404);
		r.route().filter(pred::Not(pred::Get())).f(
		    |_| HttpResponse::MethodNotAllowed());
	    });

	let jsonapi_app = App::with_state(AppState{
	    executor: addr_graphql.clone(),
	    modelstore: modelstore.clone(),
	})
	.prefix("jsonapi")
	    .middleware(middleware::Logger::default())
	    .scope("/model", |model_scope| {
		model_scope
		    .resource("", |r| {
			r.method(Method::GET).f(get_models);
			r.method(Method::POST).with(create_model);
		    })
		.resource("/{model_id}", |r| {
		    r.method(Method::GET).f(get_model);
		    r.method(Method::PUT).f(update_model);
		    // r.method(Method::DELETE).f(get_model)
		})
		/*
		.nested("/{model_id}/pages", |page_scope| {
		    page_scope
			.resource("", |r| {
			    r.method(Method::GET).f(get_pages);
			    // r.method(Method::POST).f(get_page);
			})
		    .resource("/{page_id}", |r| {
			r.method(Method::GET).f(get_page);
			// r.method(Method::PUT).with(get_page);
			// r.method(Method::DELETE).with(get_page)
		    })
		})
		.nested("/{model_id}/xflows", |xflow_scope| {
		    xflow_scope
			.resource("", |r| {
			    r.method(Method::GET).f(get_xflows);
			    // r.method(Method::POST).f(get_page);
			})
		    .resource("/{xflow_id}", |r| {
			r.method(Method::GET).f(get_xflow);
			// r.method(Method::PUT).with(get_page);
			// r.method(Method::DELETE).with(get_page)
		    })
		})
		*/
	    })

	.resource("/", |r| r.f(jsonapi_index))
	    .resource("", |r| r.f(jsonapi_index)
		     )

	    .default_resource(|r| {
		r.method(Method::GET).f(p404);
		r.route().filter(pred::Not(pred::Get())).f(
		    |req| HttpResponse::MethodNotAllowed());
	    });

	vec![
	    graphql_app,
	    jsonapi_app
	]

    }).bind("127.0.0.1:8080")
    .unwrap()
	.start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}


fn graphql_index(_req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Found()
	.header("LOCATION", "graphiql")
	.finish()
}

fn jsonapi_index(req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Found()
	.header("LOCATION", format!("api/model/1"))
	.finish()
}

fn p404(req:&HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND).finish()
}
//
// Models

fn get_models(req:&HttpRequest<AppState>) -> HttpResponse {
    match &req.state().modelstore.get(&"") {
	Ok(res) => {
	    HttpResponse::build(StatusCode::OK)
		.content_type(CONTENT_TYPE_JSON)
		.body(
		    format!("[{}]", res.to_json())
		    )
	},
	Err(_) => {
	    HttpResponse::build(StatusCode::NOT_FOUND).finish()
	}
    }
}

fn get_model(req:&HttpRequest<AppState>) -> HttpResponse {
    let model_id = &req.match_info()["model_id"];
    match &req.state().modelstore.get(&model_id) {
	Ok(res) => {
	    HttpResponse::build(StatusCode::OK)
		.content_type(CONTENT_TYPE_JSON)
		.body(
		    format!("{}", res.to_json())
		    )
	},
	Err(_) => {
	    HttpResponse::build(StatusCode::NOT_FOUND).finish()
	}
    }
}

fn create_model(model: Json<ModelDocument>) -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
	.content_type(CONTENT_TYPE_JSON)
	.body(format!("{}", model.to_json()))
}

fn update_model(req:&HttpRequest<AppState>) -> HttpResponse {
    // format!("{}", model.to_json());
    let model = ModelDocument::default();
    match &req.state().modelstore.update(&model.to_json()) {
	Ok(res) => {
	    HttpResponse::build(StatusCode::OK)
		.content_type(CONTENT_TYPE_JSON)
		.body(
		    format!("{}", res.to_json())
		    )
	},
	Err(_) => {
	    HttpResponse::build(StatusCode::NOT_FOUND).finish()
	}
    }

}

/*
//
// Pages

fn get_pages(req:&HttpRequest<AppState>) -> impl Responder {
    let model_id = &req.match_info()["model_id"];
    let model_uuid =  Uuid::parse_str(model_id);

    format!("{}",
	    serde_json::to_string_pretty(
		&req.state().model.body.pages
		).unwrap()
	   )
}

fn get_page(req:&HttpRequest<AppState>) -> impl Responder {
    let model_id = &req.match_info()["model_id"];
    let model_uuid =  Uuid::parse_str(model_id);

    let page_id = &req.match_info()["page_id"];
    let page_uuid =  Uuid::parse_str(page_id).unwrap();

    format!("{}",
	    serde_json::to_string_pretty(
		&req.state().model.get_page(&page_uuid).unwrap()
		).unwrap()
	   )
}

//
// XFlows

fn get_xflows(req:&HttpRequest<AppState>) -> impl Responder {
    let model_id = &req.match_info()["model_id"];
    let model_uuid =  Uuid::parse_str(model_id);

    format!("{}",
	    serde_json::to_string_pretty(
		&req.state().model.body.xflows
		).unwrap()
	   )
}

fn get_xflow(req:&HttpRequest<AppState>) -> impl Responder {
    let model_id = &req.match_info()["model_id"];
    let model_uuid =  Uuid::parse_str(model_id);

    let xflow_id = &req.match_info()["xflow_id"];
    let xflow_uuid =  Uuid::parse_str(xflow_id).unwrap();

    format!("{}",
	    serde_json::to_string_pretty(
		&req.state().model.get_xflow(&xflow_uuid).unwrap()
		).unwrap()
	   )
}

*/
