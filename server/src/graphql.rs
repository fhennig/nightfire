use crate::models::Color as ColorModel;
use crate::state::State;
use iron::{Iron, IronResult, Request};
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
    id: String,
    r: Option<f64>,
    g: Option<f64>,
    b: Option<f64>,
}

struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn manualMode(
        context: &Context,
        settings: Option<Vec<ManualModeLightSetting>>,
    ) -> FieldResult<MyResult> {
        let mut state = context.state.lock().unwrap();
        state.activate_manual_mode();
        let result = match settings {
            Some(light_settings) => {
                for ls in light_settings {
                    state.manual_mode.set_color(&ls.id, ls.r, ls.g, ls.b);
                }
                Ok(MyResult::Ok)
            }
            None => Ok(MyResult::Ok)
        };
        result
    }

    fn offMode(context: &Context) -> FieldResult<MyResult> {
        let mut state = context.state.lock().unwrap();
        state.activate_off_mode();
        Ok(MyResult::Ok)
    }

    fn pinkPulse(context: &Context) -> FieldResult<MyResult> {
        let mut state = context.state.lock().unwrap();
        state.activate_pink_pulse();
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

pub fn serve(state: Arc<Mutex<State>>) {
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
    mount.mount("/", Static::new(Path::new("site")));

    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
