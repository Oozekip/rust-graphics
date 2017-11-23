
use gfx;
use gfx::format::{Srgba8, DepthStencil};

pub type ColorFormat = Srgba8;
pub type DepthFormat = DepthStencil;
pub const MAX_LIGHTS: usize = 8;

gfx_defines!{
    vertex Vertex{
        pos: [f32;3] = "vPos",
        normal: [f32;3] = "vNormal",
    }

    constant MaterialData{
        diffuse_color: [f32;4] = "m_diffuse",
        ambient_color: [f32; 4] = "m_ambient",
        specular_color: [f32; 4] = "m_specular",
        specular_power: f32 = "m_specularPower",
        use_diffuse_texture: bool = "m_useDiffuseTexture",
        _udt_padding: [bool; 3] = "",
        use_specular_texture: bool = "m_useSpecularTexture",
    }

    constant LightData{
        diffuse_color: [f32;4] = "diffuse",
        ambient_color: [f32; 4] = "ambient",
        specular_color: [f32; 4] = "specular",
        position: [f32;4] = "position",
        direction: [f32;4] = "direction",
        // specular_texture: gfx::TextureSampler<[f32;4]> = "specularTexture",
        light_type: i32 = "type",
        spotlight_outer: f32 = "spotlightOuter",
        spotlight_inner: f32 = "spotlightInner",
        spotlight_falloff: f32 = "spotlightFalloff",
    }

    constant LightMeta{
        count: i32 = "lightCount",
        // c: [f32; 3] = "c",

    }

    constant Transform{
        model: [[f32; 4]; 4] = "model",
        view: [[f32; 4]; 4] = "view",
        projection: [[f32; 4]; 4] = "projection",
    }

    pipeline pipe{
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        material: gfx::ConstantBuffer<MaterialData> = "materialData",
        diffuse_texture: gfx::TextureSampler<[f32;4]> = "diffuseTexture",
        light_meta: gfx::ConstantBuffer<LightMeta> = "lightMeta",
        lights: gfx::ConstantBuffer<LightData> = "lightData",
        out: gfx::BlendTarget<ColorFormat> =
        ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}
