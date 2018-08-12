use actix;

use actix_web::{middleware, server, App, HttpResponse};
use actix_web::http::Method;

pub fn serve() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");

    let sys = actix::System::new("static_index");

    server::new(|| {
        App::new()
	        .middleware(middleware::Logger::default())
                .resource("/", |r| r.f(|req| {
                    match *req.method() {
                        Method::GET => HttpResponse::Ok(),
                        Method::POST => HttpResponse::MethodNotAllowed(),
                        _ => HttpResponse::NotFound(),
                    }
                }))
            }
    ).bind("127.0.0.1:8080")
        .expect("Can not start server on given IP/Port")
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}

