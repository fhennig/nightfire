use crate::models::{Color as ColorModel, State};
use iron::prelude::*;
use juniper::FieldResult;
use juniper_iron::GraphQLHandler;
use mount::Mount;
use std::sync::{Arc, Mutex};

#[derive(juniper::GraphQLObject)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
}

#[derive(juniper::GraphQLInputObject)]
struct NewColor {
    r: f64,
    g: f64,
    b: f64,
}

#[derive(juniper::GraphQLObject)]
struct Light {
    id: String,
    color: Color,
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
impl Query {
    fn lights(context: &Context) -> FieldResult<Vec<Light>> {
        let light_model = &mut context.state.lock().unwrap().light_model;
        let light_ids = light_model.all_light_ids();
        let mut lights = Vec::new();
        for light_id in &light_ids {
            let color = light_model.get_light(light_id);
            lights.push(Light {
                id: (*light_id).clone(),
                color: Color {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                },
            });
        }
        Ok(lights)
    }
}

struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn setLight(context: &Context, id: String, color: NewColor) -> FieldResult<Light> {
        let mut light_model = &mut context.state.lock().unwrap().light_model;
        light_model.set_light(
            &id,
            &ColorModel {
                r: color.r,
                g: color.g,
                b: color.b,
            },
        );
        Ok(Light {
            id: id,
            color: Color {
                r: color.r,
                g: color.g,
                b: color.b,
            },
        })
    }
}

struct ContextFactory {
    state: Arc<Mutex<State>>,
}

impl ContextFactory {
    fn new(state: Arc<Mutex<State>>) -> ContextFactory {
        ContextFactory {
            state: state,
        }
    }

    fn create_context(&self, _: &mut Request) -> IronResult<Context> {
        Ok(Context {
            state: Arc::clone(&self.state),
        })
    }
}

pub fn serve(state: Arc<Mutex<State>>) {
    let mut mount = Mount::new();

    let context_factory = ContextFactory::new(state);

    let graphql_endpoint =
        GraphQLHandler::new(move |x| context_factory.create_context(x), Query, Mutation);

    mount.mount("/graphql", graphql_endpoint);

    let chain = Chain::new(mount);

    Iron::new(chain).http("0.0.0.0:3000").unwrap();
}
