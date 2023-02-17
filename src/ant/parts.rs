use bevy::{prelude::*, render::mesh::VertexFormatSize};

pub struct Cost {
    carb: f32,
    prot: f32,
}

pub trait AntPart: Send + Sync + 'static {
    fn get_creation_cost(&self) -> Cost;
    fn get_maintenance_cost(&self) -> Cost;
}

pub struct ResourceSensor {
    range: f32,
}
impl AntPart for ResourceSensor {
    fn get_creation_cost(&self) -> Cost {
        return Cost {
            carb: self.range / 5.0,
            prot: self.range / 2.0,
        };
    }

    fn get_maintenance_cost(&self) -> Cost {
        return Cost {
            carb: self.range / 10.0,
            prot: 0.0,
        };
    }
}
