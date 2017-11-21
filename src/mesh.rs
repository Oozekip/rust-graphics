use std::f32;

use gfx::{Resources, Slice};
use gfx::handle::{DepthStencilView, RenderTargetView};
use gfx::traits::FactoryExt;

use na::Vector3;

use color::Color;

use program::{pipe, ColorFormat, DepthFormat, Vertex, MAX_LIGHTS};

pub struct MeshData<R: Resources> {
    slice: Slice<R>,
    data: pipe::Data<R>,
}

impl<R: Resources> MeshData<R> {
    pub fn slice_ref<'a>(&'a self) -> &'a Slice<R> {
        &self.slice
    }

    pub fn data_ref<'a>(&'a self) -> &'a pipe::Data<R> {
        &self.data
    }

    pub fn data_ref_mut<'a>(&'a mut self) -> &'a mut pipe::Data<R> {
        &mut self.data
    }
}

pub struct Mesh {
    vertex_list: Vec<Vector3<f32>>,
    tri_list: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertex_list: Vec::new(),
            tri_list: Vec::new(),
        }
    }

    pub fn build<R: Resources, F: FactoryExt<R>>(
        &self,
        factory: &mut F,
        color_view: RenderTargetView<R, ColorFormat>,
        depth_view: DepthStencilView<R, DepthFormat>,
    ) -> Result<MeshData<R>, &'static str> {
        let mut vert_list = Vec::new();

        for vert in &self.vertex_list {
            vert_list.push(Vertex {
                pos: vert.clone().into(),
                color: Color::white().into(),
            });
        }

        let (vbo, slice) =
            factory.create_vertex_buffer_with_slice(vert_list.as_slice(), self.tri_list.as_slice());

        let constant_buffer = factory.create_constant_buffer(1);
        let light_buffer = factory.create_constant_buffer(MAX_LIGHTS);

        let light_meta = factory.create_constant_buffer(1);

        Result::Ok(MeshData {
            slice: slice,
            data: pipe::Data {
                vbuf: vbo,
                transform: constant_buffer,
                out: color_view,
                out_depth: depth_view,
                light_meta: light_meta,
                lights: light_buffer,
            },
        })
    }

    pub fn add_vertex(&mut self, vert: &Vector3<f32>) -> &mut Self {
        self.vertex_list.push(vert.clone());
        self
    }

    pub fn add_verticies(&mut self, verts: &[Vector3<f32>]) -> &mut Self {
        for vert in verts {
            self.add_vertex(vert);
        }

        self
    }

    pub fn add_tri(&mut self, (first, second, third): (u32, u32, u32)) -> &mut Self {
        self.tri_list.push(first);
        self.tri_list.push(second);
        self.tri_list.push(third);
        self
    }

    pub fn add_tris(&mut self, tris: &[(u32, u32, u32)]) -> &mut Self {
        for tri in tris {
            self.add_tri(tri.clone());
        }

        self
    }

    pub fn preprocess(&mut self) -> &mut Self {
        self.move_to_origin();
        self
    }

    pub fn extents(&self) -> (Vector3<f32>, Vector3<f32>) {
        let mut curr_min = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut curr_max = Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for vert in &self.vertex_list {
            curr_min.x = f32::min(curr_min.x, vert.x);
            curr_min.y = f32::min(curr_min.y, vert.y);
            curr_min.z = f32::min(curr_min.z, vert.z);

            curr_max.x = f32::max(curr_max.x, vert.x);
            curr_max.y = f32::max(curr_max.y, vert.y);
            curr_max.z = f32::max(curr_max.z, vert.z);
        }

        (curr_min, curr_max)
    }

    pub fn centroid(&self) -> Vector3<f32> {
        let sum = self.vertex_list
            .iter()
            .fold(Vector3::new(0.0, 0.0, 0.0), |acc, &x| acc + x);

        sum / self.vertex_list.len() as f32
    }

    fn move_to_origin(&mut self) {
        let center = self.centroid();
        self.vertex_list = self.vertex_list.iter().map(|vert| vert - center).collect();
    }
}
