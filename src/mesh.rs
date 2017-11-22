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

    pub fn update_views(
        &mut self,
        color_view: RenderTargetView<R, ColorFormat>,
        depth_view: DepthStencilView<R, DepthFormat>,
    ) {
        self.data.out = color_view;
        self.data.out_depth = depth_view;
    }
}

pub struct Mesh {
    vertex_list: Vec<Vector3<f32>>,
    normal_list: Vec<Vector3<f32>>,
    tri_list: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Self {
        Mesh {
            vertex_list: Vec::new(),
            normal_list: Vec::new(),
            tri_list: Vec::new(),
        }
    }

    pub fn build<R: Resources, F: FactoryExt<R>>(
        &self,
        factory: &mut F,
        color_view: RenderTargetView<R, ColorFormat>,
        depth_view: DepthStencilView<R, DepthFormat>,
    ) -> Result<MeshData<R>, &'static str> {
        if self.normal_list.len() != self.vertex_list.len() {
            Err("Vertex and normal list do not match in length")
        } else {
            let mut vert_list = Vec::new();

            for i in 0..self.vertex_list.len() {
                let vert = self.vertex_list[i];
                let norm = self.normal_list[i];
                vert_list.push(Vertex {
                    pos: vert.into(),
                    normal: norm.into(),
                    color: Color::white().into(),
                });
            }

            let (vbo, slice) = factory
                .create_vertex_buffer_with_slice(vert_list.as_slice(), self.tri_list.as_slice());

            // Transform buffer
            let constant_buffer = factory.create_constant_buffer(1);

            // Material buffer
            let material_buffer = factory.create_constant_buffer(1);

            // Light buffers
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
                    material: material_buffer,
                },
            })
        }
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

    pub fn add_normal(&mut self, vert: &Vector3<f32>) -> &mut Self {
        self.normal_list.push(vert.clone());
        self
    }

    pub fn add_normals(&mut self, verts: &[Vector3<f32>]) -> &mut Self {
        for vert in verts {
            self.add_normal(vert);
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
        self.normalize_size();
        self.move_to_origin();
        self
    }

    pub fn preprocess_compute_normals(&mut self) -> &mut Self {
        self.preprocess();
        self.compute_normals();
        self
    }

    pub fn extents(&self) -> (Vector3<f32>, Vector3<f32>) {
        self.vertex_list.iter().fold(
            (
                Vector3::from_element(f32::INFINITY),
                Vector3::from_element(f32::NEG_INFINITY),
            ),
            |(min, max), vert| {
                let curr_min = Vector3::new(
                    f32::min(min.x, vert.x),
                    f32::min(min.y, vert.y),
                    f32::min(min.z, vert.z),
                );
                let curr_max = Vector3::new(
                    f32::max(max.x, vert.x),
                    f32::max(max.y, vert.y),
                    f32::max(max.z, vert.z),
                );

                (curr_min, curr_max)
            },
        )
    }

    pub fn centroid(&self) -> Vector3<f32> {
        let sum = self.vertex_list
            .iter()
            .fold(Vector3::new(0.0, 0.0, 0.0), |acc, &x| acc + x);

        sum / self.vertex_list.len() as f32
    }

    fn compute_normals(&mut self) {
        self.normal_list.clear();
        // Create list of normals for each vertex (Vec<Vec<Vector3>>)

        // For each triangle:
        //     Compute tri normal
        //     Add normal to list for each vertex (if it doesn't already exist)
        // Sum together and normalze vertex normals
        let mut vert_tri_normals = Vec::new();

        for _ in 0..self.vertex_list.len() {
            vert_tri_normals.push(Vec::new());
        }

        let mut i = 0;

        while i < self.tri_list.len() {
            let first = &self.vertex_list[self.tri_list[i] as usize];
            let second = &self.vertex_list[self.tri_list[i + 1] as usize];
            let third = &self.vertex_list[self.tri_list[i + 2] as usize];

            let a = second - first;
            let b = third - first;

            let norm = a.cross(&b).normalize();

            for j in 0..3 {
                let ind = self.tri_list[i + j] as usize;
                let in_vec = &mut vert_tri_normals[ind];

                // Insert normal into list iff it doesn't already exist
                if let None = in_vec.iter().position(|x: &Vector3<f32>| x.eq(&norm)) {
                    in_vec.push(norm);
                }
            }

            // Move forward three indicies (one tri)
            i += 3;
        }

        // Sum together normals and normalize them
        for vec_norms in &vert_tri_normals {
            let norm = vec_norms
                .iter()
                .fold(Vector3::new(0.0, 0.0, 0.0), |acc, x| acc + x)
                .normalize();
            self.add_normal(&norm);
        }
    }

    fn normalize_size(&mut self) {
        let (min, max) = self.extents();
        let diff = max - min;
        let x = f32::abs(diff.x);
        let y = f32::abs(diff.y);
        let z = f32::abs(diff.z);

        let max_extent = f32::max(x, f32::max(y, z));

        self.vertex_list = self.vertex_list
            .iter()
            .map(|vert| (1.0 / max_extent) * vert)
            .collect();
    }

    fn move_to_origin(&mut self) {
        let center = self.centroid();
        self.vertex_list = self.vertex_list.iter().map(|vert| vert - center).collect();
    }
}
