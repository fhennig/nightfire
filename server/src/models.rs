use std::collections::HashMap;
use std::vec::Vec;

pub type Pin = i64;
pub type PinValue = f32;
pub type LightId = String;

pub struct Color {
    pub r: PinValue,
    pub g: PinValue,
    pub b: PinValue,
}

pub struct Light {
    pub r_pin: Pin,
    pub g_pin: Pin,
    pub b_pin: Pin,
}

pub trait PinHandler {
    fn pin_update(&mut self, pin: Pin, value: PinValue);
}

pub struct PinModel {
    pin_values: HashMap<Pin, PinValue>,
    handlers: Vec<Box<dyn PinHandler>>,
}

impl PinModel {
    pub fn new(pins: Vec<Pin>, handlers: Vec<Box<dyn PinHandler>>) -> PinModel {
        let map = HashMap::new();
        let mut model = PinModel {
            pin_values: map,
            handlers: handlers,
        };
        for pin in pins {
            let value = 0f32;
            model.set_pin(pin, value);
        }
        model
    }

    pub fn set_pin(&mut self, pin: Pin, value: PinValue) {
        self.pin_values.insert(pin, value);
        for listener in &mut self.handlers {
            listener.pin_update(pin, value);
        }
    }
}

pub struct LightModel {
    light_map: HashMap<LightId, Light>,
    pin_model: PinModel,
}

impl LightModel {
    pub fn new(pin_model: PinModel, lights: Vec<(LightId, Light)>) -> LightModel {
        let mut map = HashMap::new();
        for (light_id, light) in lights {
            map.insert(light_id, light);
        }
        let model = LightModel {
            light_map: map,
            pin_model: pin_model,
        };
        model
    }

    pub fn set_light(&mut self, light_id: LightId, color: Color) {
        let light = self.light_map.get(&light_id).unwrap();
        self.pin_model.set_pin(light.r_pin, color.r);
        self.pin_model.set_pin(light.g_pin, color.g);
        self.pin_model.set_pin(light.b_pin, color.b);
    }
}
