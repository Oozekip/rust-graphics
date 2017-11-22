
use gfx;
use gfx::{CommandBuffer, Resources};

use na::{Matrix4, Point3, Vector3};

use program::{pipe, Transform};
use mesh::MeshData;
use material::Material;

pub struct Object {
    pub position: Point3<f32>, // Position
    pub scale: Vector3<f32>, // Scale amount
    pub rotation: Vector3<f32>, // Euler angles
    pub material: Material,
}

impl Object {
    pub fn new(
        mat: Material,
        pos: Point3<f32>,
        size: Vector3<f32>,
        rotation: Vector3<f32>,
    ) -> Self {
        Object {
            position: pos,
            scale: size,
            rotation: rotation,
            material: mat,
        }
    }

    pub fn build_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&(self.position - Point3::new(0.0, 0.0, 0.0))) *
            Matrix4::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z) *
            Matrix4::new_nonuniform_scaling(&self.scale)
    }
}

pub fn create_transform(obj: &Object, view: Matrix4<f32>, projection: Matrix4<f32>) -> Transform {
    Transform {
        model: obj.build_matrix().into(),
        view: view.into(),
        projection: projection.into(),
    }
}

pub fn draw<R: Resources, C: CommandBuffer<R>>(
    encoder: &mut gfx::Encoder<R, C>,
    mesh_data: &mut MeshData<R>,
    program: &gfx::pso::PipelineState<R, pipe::Meta>,
    obj: &Object,
    view: &Matrix4<f32>,
    projection: &Matrix4<f32>,
) {
    let trans_data = create_transform(&obj, view.clone(), projection.clone());

    encoder
        .update_buffer(&mesh_data.data_ref_mut().transform, &[trans_data], 0)
        .unwrap(); //update buffers

    encoder
        .update_buffer(
            &mesh_data.data_ref_mut().material,
            &[obj.material.clone().into()],
            0,
        )
        .unwrap(); //update buffers

    // draw commands with buffer data and attached pso
    encoder.draw(&mesh_data.slice_ref(), &program, mesh_data.data_ref());
}
