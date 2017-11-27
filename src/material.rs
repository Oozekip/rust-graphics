use color::Color;
use program::MaterialData;
use texture::Texture;
use gfx::Resources;

#[derive(Clone)]
pub enum Material<R: Resources> {
    Untextured {
        diffuse_color: Color,
        ambient_color: Color,
        specular_color: Color,
        specular_power: f32,
    },
    Textured {
        ambient_color: Color,
        diffuse_texture: Texture<R>,
        specular_texture: Texture<R>,
    },
}

impl<R: Resources> Into<MaterialData> for Material<R> {
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
                use_diffuse_texture: 0,
                use_specular_texture: 0,
            },
            Material::Textured { ambient_color, .. } => MaterialData {
                diffuse_color: Color::black().into(),
                specular_color: Color::black().into(),
                ambient_color: ambient_color.into(),
                specular_power: 1.0,
                use_diffuse_texture: 1,
                use_specular_texture: 1,
            },
        }
    }
}
