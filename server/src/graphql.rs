use crate::lightid::LightId;
use crate::modes::Mode;
use crate::state::State;
use iron::middleware::Handler;
use iron::status;
use iron::{Iron, IronResult, Listening, Request, Response};
use juniper::FieldResult;
use juniper_iron::{GraphQLHandler, GraphiQLHandler};
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(juniper::GraphQLEnum)]
enum MyResult {
    Ok,
    NotOk,
}

pub struct Context {
    state: Arc<Mutex<State>>,
}

impl juniper::Context for Context {}

struct Query;

#[juniper::object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    Context = Context,
)]
impl Query {}

#[derive(juniper::GraphQLInputObject)]
struct ManualModeLightSetting {
    id: LightId,
    r: Option<f64>,
    g: Option<f64>,
    b: Option<f64>,
}

#[derive(juniper::GraphQLInputObject)]
struct Coordinate {
    x: f64,
    y: f64,
}

struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn manualMode(
        context: &Context,
        settings: Option<Vec<ManualModeLightSetting>>,
    ) -> FieldResult<MyResult> {
        let mut state = context.state.lock().unwrap();
        state.activate(Mode::ManualMode);
        let result = match settings {
            Some(light_settings) => {
                for ls in light_settings {
                    state.manual_mode.set_color(ls.id, ls.r, ls.g, ls.b);
                }
                Ok(MyResult::Ok)
            }
            None => Ok(MyResult::Ok),
        };
        result
    }

    fn offMode(context: &Context) -> FieldResult<MyResult> {
        let mut state = context.state.lock().unwrap();
        state.activate(Mode::OffMode);
        Ok(MyResult::Ok)
    }

    fn pinkPulse(context: &Context) -> FieldResult<MyResult> {
        let mut state = context.state.lock().unwrap();
        state.activate(Mode::PinkPulse);
        Ok(MyResult::Ok)
    }

    fn rainbow(context: &Context) -> FieldResult<MyResult> {
        let mut state = context.state.lock().unwrap();
        state.activate(Mode::Rainbow);
        Ok(MyResult::Ok)
    }

    fn controller(context: &Context, pos: Option<Coordinate>) -> FieldResult<MyResult> {
        let mut state = context.state.lock().unwrap();
        state.activate(Mode::Controller);
        // set position
        match pos {
            Some(pos) => state.controller_mode.set_pos(pos.x, pos.y),
            None => (),
        }
        // result
        Ok(MyResult::Ok)
    }
}

struct ContextFactory {
    state: Arc<Mutex<State>>,
}

impl ContextFactory {
    fn new(state: Arc<Mutex<State>>) -> ContextFactory {
        ContextFactory { state: state }
    }

    fn create_context(&self, _: &mut Request) -> IronResult<Context> {
        Ok(Context {
            state: Arc::clone(&self.state),
        })
    }
}

struct AppHandler {
    root_handler: Static,
}

impl AppHandler {
    fn new() -> AppHandler {
        AppHandler {
            root_handler: Static::new(Path::new("site")),
        }
    }
}

impl Handler for AppHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let response = self.root_handler.handle(req);
        match response {
            Err(_) => Ok(Response::with((status::Ok, Path::new("site/index.html")))),
            other => other,
        }
    }
}

pub fn serve(address: String, state: Arc<Mutex<State>>) -> Listening {
    let context_factory = ContextFactory::new(state);

    let graphql_endpoint = GraphQLHandler::new(
        move |req| context_factory.create_context(req),
        Query,
        Mutation,
    );

    let mut mount = Mount::new();

    mount.mount("/graphql", graphql_endpoint);
    let graphiql = GraphiQLHandler::new("/graphql");
    mount.mount("/graphiql", graphiql);
    mount.mount("/", AppHandler::new());
    println!("Starting GraphQL server");
    Iron::new(mount).http(address.as_str()).unwrap()
}
