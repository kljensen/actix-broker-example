#[macro_use]
extern crate actix;
extern crate actix_broker;
extern crate actix_web;

use actix::prelude::{Actor, Context, Handler, System};
use actix_broker::{Broker, BrokerSubscribe};
use actix_web::{server, App, Error, HttpRequest, HttpResponse};
use std::sync::{Arc, Mutex};

// This struct represents state
struct AppState {
    counter: usize,
}

#[derive(Clone, Debug, Message)]
struct Hello;

struct TestActor;

impl Actor for TestActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_async::<Hello>(ctx);
    }
}

impl Handler<Hello> for TestActor {
    type Result = ();

    fn handle(&mut self, msg: Hello, _ctx: &mut Self::Context) {
        println!("TestActor: Received {:?}", msg);
    }
}

fn index(req: &HttpRequest<Arc<Mutex<AppState>>>) -> Result<HttpResponse, Error> {
    Broker::issue_async(Hello);
    // let foo = req.state().count;
    let mut state = req.state().lock().unwrap();
    state.counter += 1;
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Request number: {}", state.counter)))
}

fn main() {
    System::run(|| {
        TestActor.start();

        let state = Arc::new(Mutex::new(AppState { counter: 0 }));
        server::new(move || App::with_state(state.clone()).resource("/", |r| r.f(index)))
            .bind("127.0.0.1:8080")
            .unwrap()
            .start();
        println!("Hit up 127.0.0.1:8080");
    });
}
