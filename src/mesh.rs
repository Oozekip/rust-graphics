use std::f32;

use image;
use gfx::{Resources, Slice};
use gfx::handle::{DepthStencilView, RenderTargetView};
use gfx::traits::FactoryExt;
use gfx::format::Rgba8;

use na::Vector3;

use texture::Texture;

use program::{pipe, ColorFormat, DepthFormat, Vertex, MAX_LIGHTS};


pub struct MeshData<R: Resources> {
    slice: Slice<R>,
    data: pipe::Data<R>,
}

impl<R: Resources> MeshData<R> {
    pub fn slice_ref(&self) -> &Slice<R> {
        &self.slice
    }

    pub fn data_ref(&self) -> &pipe::Data<R> {
        &self.data
    }

    pub fn data_ref_mut(&mut self) -> &mut pipe::Data<R> {
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

    pub fn update_diffuse_texture(&mut self, tex: Texture<R>) {
        let (_, sampler) = self.data.diffuse_texture.clone();

        self.data.diffuse_texture = (tex, sampler);
    }

    pub fn update_specular_texture(&mut self, tex: Texture<R>) {
        let (_, sampler) = self.data.specular_texture.clone();

        self.data.specular_texture = (tex, sampler);
    }
}

#[derive(Default)]
pub struct Mesh {
    vertex_list: Vec<Vertex>,
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
        let (vbo, slice) = factory
            .create_vertex_buffer_with_slice(self.vertex_list.as_slice(), self.tri_list.as_slice());

        // Transform buffer
        let constant_buffer = factory.create_constant_buffer(1);

        // Material buffer
        let material_buffer = factory.create_constant_buffer(1);

        // Light buffers
        let light_buffer = factory.create_constant_buffer(MAX_LIGHTS);
        let light_meta = factory.create_constant_buffer(1);

        use gfx::texture as t;

        let sampler = factory.create_sampler_linear();

        // Width/height have to be power of 2, so blank texture must be minumum 1x1
        let img = image::DynamicImage::new_rgba8(1, 1).to_rgba();
        let kind = t::Kind::D2(
            img.width() as t::Size,
            img.height() as t::Size,
            t::AaMode::Single,
        );

        let (_, empty_tex) = factory
            .create_texture_immutable_u8::<Rgba8>(kind, &[&img])
            .unwrap();

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
                diffuse_texture: (empty_tex.clone(), sampler.clone()),
                specular_texture: (empty_tex, sampler),
            },
        })
    }

    pub fn add_vertex(&mut self, vert: &Vertex) -> &mut Self {
        self.vertex_list.push(*vert);
        self
    }

    pub fn add_verticies(&mut self, verts: &[Vertex]) -> &mut Self {
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
            self.add_tri(*tri);
        }

        self
    }

    pub fn preprocess(&mut self) -> &mut Self {
        self.normalize_size();
        self.move_to_origin();
        self
    }

    pub fn extents(&self) -> (Vector3<f32>, Vector3<f32>) {
        self.vertex_list.iter().fold(
            (
                Vector3::from_element(f32::INFINITY),
                Vector3::from_element(f32::NEG_INFINITY),
            ),
            |(min, max), &Vertex { pos, .. }| {
                let vert = Vector3::from(pos);

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
        let sum = self.vertex_list.iter().fold(
            Vector3::new(0.0, 0.0, 0.0),
            |acc, &Vertex { pos, .. }| {
                let x = Vector3::from(pos);
                acc + x
            },
        );

        sum / self.vertex_list.len() as f32
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
            .map(|&Vertex { pos, normal, uv }| {
                let vert = Vector3::from(pos);

                Vertex {
                    pos: ((1.0 / max_extent) * vert).into(),
                    normal,
                    uv,
                }
            })
            .collect();
    }

    fn move_to_origin(&mut self) {
        let center = self.centroid();
        self.vertex_list = self.vertex_list
            .iter()
            .map(|&Vertex { pos, normal, uv }| {
                let vert = Vector3::from(pos);
                Vertex {
                    pos: (vert - center).into(),
                    normal: normal,
                    uv: uv,
                }
            })
            .collect();
    }
}
