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
        match &self {
            &Material::Untextured {
                diffuse_color: diffuse,
                ambient_color: ambient,
                specular_color: specular,
                specular_power: power,
            } => MaterialData {
                diffuse_color: diffuse.into(),
                specular_color: specular.into(),
                ambient_color: ambient.into(),
                specular_power: power,
            },
        }
    }
}
