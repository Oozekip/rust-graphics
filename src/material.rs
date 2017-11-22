use color::Color;
use program::MaterialData;

#[derive(Clone, Copy)]
pub enum Material {
    Untextured {
        diffuse_color: Color,
        ambient_color: Color,
        specular_color: Color,
        specular_power: f32,
    },
}

impl Into<MaterialData> for Material {
    fn into(self) -> MaterialData {
        match self {
            Material::Untextured {
                diffuse_color,
                ambient_color,
                specular_color,
                specular_power,
            } => MaterialData {
                diffuse_color: diffuse_color.into(),
                specular_color: specular_color.into(),
                ambient_color: ambient_color.into(),
                specular_power: specular_power,
            },
        }
    }
}
