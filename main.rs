#[macro_use]
extern crate actix;
extern crate actix_broker;
extern crate actix_web;

use actix::prelude::{Actor, Context, Handler, System};
use actix_broker::{Broker, BrokerSubscribe};
use actix_web::{server, App, Error, HttpRequest, HttpResponse};
use std::cell::Cell;

// This struct represents state
struct AppState {
    counter: Cell<usize>,
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

fn index(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    Broker::issue_async(Hello);
    let count = req.state().counter.get() + 1; // <- get count
    req.state().counter.set(count); // <- store new count in state

    // <- response with count
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Request number: {}", count)))
}

fn main() {
    System::run(|| {
        TestActor.start();

        server::new(|| {
            App::with_state(AppState {
                counter: Cell::new(0),
            }).resource("/", |r| r.f(index))
        }).bind("127.0.0.1:8080")
        .unwrap()
        .start();
        println!("Hit up 127.0.0.1:8080");
    });
}
