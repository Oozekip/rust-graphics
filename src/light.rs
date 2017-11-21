use gfx;
use gfx::{CommandBuffer, Resources};

use mesh::MeshData;
use program::{LightData, LightMeta, MAX_LIGHTS};
use color::Color;
use na::{Point3, Vector3};

#[derive(Clone, Copy)]
pub enum LightType {
    Directional(Vector3<f32>),
    Point(Point3<f32>),
    Spot(Point3<f32>, Vector3<f32>),
}

#[derive(Clone, Copy)]
pub struct Light {
    pub light_type: LightType,
    pub diffuse_color: Color,
    pub specular_color: Color,
    pub ambient_color: Color,
}

impl Light {}

impl Light {
    pub fn new(light_type: LightType, diffuse: Color, spec: Color, amb: Color) -> Self {
        Light {
            light_type: light_type,
            diffuse_color: diffuse,
            specular_color: spec,
            ambient_color: amb,
        }
    }

    pub fn new_spot(
        pos: Point3<f32>,
        dir: Vector3<f32>,
        diffuse: Color,
        spec: Color,
        amb: Color,
    ) -> Self {
        Self::new(LightType::Spot(pos, dir), diffuse, spec, amb)
    }

    pub fn new_directional(dir: Vector3<f32>, diffuse: Color, spec: Color, amb: Color) -> Self {
        Self::new(LightType::Directional(dir), diffuse, spec, amb)
    }

    pub fn new_point(pos: Point3<f32>, diffuse: Color, spec: Color, amb: Color) -> Self {
        Self::new(LightType::Point(pos), diffuse, spec, amb)
    }
}

impl Into<LightData> for Light {
    fn into(self) -> LightData {
        match &self.light_type {
            &LightType::Directional(dir) => LightData {
                position: [0.0, 0.0, 0.0, 1.0],
                direction: dir.to_homogeneous().into(),
                diffuse_color: self.diffuse_color.into(),
                specular_color: self.specular_color.into(),
                ambient_color: self.ambient_color.into(),
                light_type: 0,
                _padding: [0i32; 3],
            },
            &LightType::Point(pos) => LightData {
                position: pos.to_homogeneous().into(),
                direction: [0.0, 0.0, 0.0, 0.0],
                diffuse_color: self.diffuse_color.into(),
                specular_color: self.specular_color.into(),
                ambient_color: self.ambient_color.into(),
                light_type: 1,
                _padding: [0i32; 3],
            },
            &LightType::Spot(pos, dir) => LightData {
                position: pos.to_homogeneous().into(),
                direction: dir.to_homogeneous().into(),
                diffuse_color: self.diffuse_color.into(),
                specular_color: self.specular_color.into(),
                ambient_color: self.ambient_color.into(),
                light_type: 2,
                _padding: [0i32; 3],
            },
        }
    }
}

pub fn upload_lights<R: Resources, C: CommandBuffer<R>>(
    encoder: &mut gfx::Encoder<R, C>,
    mesh_data: &mut MeshData<R>,
    lights: &[Light],
) {
    // Number of lights to be sent to the shader
    let count = usize::min(MAX_LIGHTS, lights.len());

    // Cut slice to right size
    let (pre_slice, _) = lights.split_at(count);

    // Convert lights to propper format
    let slice: Vec<LightData> = pre_slice.iter().map(|light| light.clone().into()).collect();

    // Send light metadata
    encoder
        .update_buffer(
            &mesh_data.data_ref_mut().light_meta,
            &[
                LightMeta {
                    count: count as i32,
                },
            ],
            0,
        )
        .unwrap();

    // Send light data
    encoder
        .update_buffer(&mesh_data.data_ref_mut().lights, slice.as_slice(), 0)
        .unwrap()
}
