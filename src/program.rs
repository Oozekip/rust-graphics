
use gfx;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub const MAX_LIGHTS: usize = 8;

gfx_defines!{
    vertex Vertex{
        pos: [f32;3] = "vPos",
        color: [f32;4] = "vColor",
    }

    constant Light{
        pos: [f32;3] = "position",
        direction: [f32;3] = "direction",
        diffuse_color: [f32;4] = "diffuseColor",
    }

    constant LightMeta{
        count: i32 = "lightCount",
    }

    constant Transform{
        model: [[f32; 4]; 4] = "model",
        view: [[f32; 4]; 4] = "view",
        projection: [[f32; 4]; 4] = "projection",
    }

    pipeline pipe{
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        light_meta: gfx::ConstantBuffer<LightMeta> = "lightMeta",
        //lights: gfx::ConstantBuffer<Light> = "lights",
        out: gfx::BlendTarget<ColorFormat> =
        ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}
