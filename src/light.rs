use gfx;
use gfx::{CommandBuffer, Resources};

use mesh::MeshData;
use program::{LightData, LightMeta, MAX_LIGHTS};
use color::Color;
use na::{Point3, Vector3};

#[derive(Clone, Copy)]
pub struct SpotLightInfo {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub falloff: f32,
}

#[derive(Clone, Copy)]
pub enum LightType {
    Directional(Vector3<f32>),
    Point(Point3<f32>),
    Spot(Point3<f32>, Vector3<f32>, SpotLightInfo),
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
        spot_info: SpotLightInfo,
        diffuse: Color,
        spec: Color,
        amb: Color,
    ) -> Self {
        Self::new(LightType::Spot(pos, dir, spot_info), diffuse, spec, amb)
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
        match self.light_type {
            LightType::Directional(ref dir) => LightData {
                position: [0.0, 0.0, 0.0, 1.0],
                direction: dir.to_homogeneous().into(),
                diffuse_color: self.diffuse_color.into(),
                specular_color: self.specular_color.into(),
                ambient_color: self.ambient_color.into(),
                light_type: 0,
                spotlight_outer: 0.0,
                spotlight_inner: 0.0,
                spotlight_falloff: 0.0,
            },
            LightType::Point(ref pos) => LightData {
                position: pos.to_homogeneous().into(),
                direction: [0.0, 0.0, 0.0, 0.0],
                diffuse_color: self.diffuse_color.into(),
                specular_color: self.specular_color.into(),
                ambient_color: self.ambient_color.into(),
                light_type: 1,
                spotlight_outer: 0.0,
                spotlight_inner: 0.0,
                spotlight_falloff: 0.0,
            },
            LightType::Spot(
                ref pos,
                ref dir,
                SpotLightInfo {
                    ref inner_radius,
                    ref outer_radius,
                    ref falloff,
                },
            ) => LightData {
                position: pos.to_homogeneous().into(),
                direction: dir.to_homogeneous().into(),
                diffuse_color: self.diffuse_color.into(),
                specular_color: self.specular_color.into(),
                ambient_color: self.ambient_color.into(),
                light_type: 2,
                spotlight_outer: *outer_radius,
                spotlight_inner: *inner_radius,
                spotlight_falloff: *falloff,
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
    let slice: Vec<LightData> = pre_slice.iter().map(|light| (*light).into()).collect();

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
