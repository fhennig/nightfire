use crate::models::{Color as ColorModel, LightModel};
use juniper::FieldResult;
use std::cell::RefCell;

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

struct Context {
    light_model: RefCell<LightModel>,
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
        let light_model = context.light_model.borrow();
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
        let mut light_model = context.light_model.borrow_mut();
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

type Schema = juniper::RootNode<'static, Query, Mutation>;
