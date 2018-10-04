
//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web

use gears::structure::model::ModelDocument;

use actix::prelude::*;
use actix_web::{
    middleware, server, App, AsyncResponder, FutureResponse, HttpRequest,
    HttpResponse, HttpMessage, Json, http::Method, http::StatusCode, pred,
};
use futures::future::Future;

use modelstore::filesystem::FileSystemModelStore;
use modelstore::model_executor::ModelStore;
use bytes::Bytes;
use std::str;

struct AppState {
    modelstore: FileSystemModelStore,
}

static CONTENT_TYPE_JSON: &'static str = "application/json; charset=utf-8";

pub fn serve(path: &str) {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let sys = actix::System::new("model-jsonapi");

    let modelstore = match FileSystemModelStore::new(&path) {
	Ok(res) => res,
	Err(_) => {
	    error!("Unable to initialize model. Is this a model direcory?");
	    return ()
	}
    };

    server::new(move || {

	let jsonapi_app = App::with_state(AppState{
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
		    |_req| HttpResponse::MethodNotAllowed());
	    });

	vec![
	    jsonapi_app
	]

    }).bind("0.0.0.0:8080")
    .unwrap()
	.start();

    println!("Started http server: 0.0.0.0:8080");
    let _ = sys.run();
}


fn jsonapi_index(_req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Found()
	.header("LOCATION", format!("api/model/1"))
	.finish()
}

fn p404(_req:&HttpRequest<AppState>) -> HttpResponse {
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
		    format!("[{}]", res.to_json_compact())
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
		    format!("{}", res.to_json_compact())
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
	.body(format!("{}", model.to_json_compact()))
}

fn update_model(req: &HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    // format!("{}", model.to_json());

    let req = req.clone();
    req
	.body()
	.from_err()
	.and_then(move |bytes: Bytes| {
	    match str::from_utf8(&bytes) {
		Ok(body) => {
		    // println!("==== BODY ==== {:?}", body);
		    match &req.state().modelstore.update(&body) {
			Ok(res) => {
			    Ok(HttpResponse::build(StatusCode::OK)
			       .content_type(CONTENT_TYPE_JSON)
			       .body( format!("{}", res.to_json_compact()))
			      )
			},
			Err(err) => {
			    Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
			       .content_type(CONTENT_TYPE_JSON)
			       .body( format!("{:?}", err))
			      )
			}
		    }
		},
		Err(_) => {
		    Ok(HttpResponse::build(StatusCode::BAD_REQUEST)
		       .content_type(CONTENT_TYPE_JSON)
		       .body(format!("Invalid JSON"))
		      )
		}
	    }
	})
    .responder()
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
