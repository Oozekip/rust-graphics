use std::f32;
use std::u32;
use std::str::FromStr;
use std::io;

use na::{Vector2, Vector3};
use regex::{Captures, Regex};

use mesh::Mesh;
use program::Vertex;
use utility;

type UVMapFn = fn(&Vector3<f32>) -> Vector2<f32>;
type NormalComputeFn = fn(&[Vector3<f32>], &[(u32, u32, u32)])
    -> Vec<Vector3<f32>>;
type VertexIndex = (u32, u32, u32);
type Triangle = (VertexIndex, VertexIndex, VertexIndex);

fn find_or_insert<T: Eq + Clone>(val: &T, cont: &mut Vec<T>) -> usize {
    let index;

    if let Some(pos) = cont.iter().position(|check| check == val) {
        index = pos;
    } else {
        index = cont.len();
        cont.push(val.clone());
    }

    index
}

fn get_extents(verts: &[Vector3<f32>]) -> (Vector3<f32>, Vector3<f32>) {
    verts.iter().fold(
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

fn normalize_scale(verts: &[Vector3<f32>]) -> Vec<Vector3<f32>> {
    let (min, max) = get_extents(verts);
    let diff = max - min;
    let x = f32::abs(diff.x);
    let y = f32::abs(diff.y);
    let z = f32::abs(diff.z);

    let max_extent = f32::max(x, f32::max(y, z));

    verts
        .iter()
        .map(|vert| ((1.0 / max_extent) * vert))
        .collect()
}

fn center_verts(verts: &[Vector3<f32>]) -> Vec<Vector3<f32>> {
    let (min, max) = get_extents(verts);
    let center = 0.5 * (max - min);

    verts.iter().map(|vert| vert - center).collect()
}

fn get_tri_match(
    uv_fn: &Option<UVMapFn>,
    normal_fn: &Option<NormalComputeFn>,
) -> Box<Fn(&str) -> Option<Triangle>> {
    let normal_compute = normal_fn.is_some();
    let uv_compute = uv_fn.is_some();

    let reg = Regex::new(get_tri_regex(&uv_fn, &normal_fn)).unwrap();

    match (uv_compute, normal_compute) {
        (false, false) => Box::new(move |line| {
            let caps = reg.captures(line)?;
            let v1 = (
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[2]).unwrap() - 1,
                u32::from_str(&caps[3]).unwrap() - 1,
            );
            let v2 = (
                u32::from_str(&caps[4]).unwrap() - 1,
                u32::from_str(&caps[5]).unwrap() - 1,
                u32::from_str(&caps[6]).unwrap() - 1,
            );
            let v3 = (
                u32::from_str(&caps[7]).unwrap() - 1,
                u32::from_str(&caps[8]).unwrap() - 1,
                u32::from_str(&caps[9]).unwrap() - 1,
            );

            Some((v1, v2, v3))
        }),
        (true, false) => Box::new(move |line| {
            let caps = reg.captures(line)?;
            let v1 = (
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[2]).unwrap() - 1,
            );
            let v2 = (
                u32::from_str(&caps[3]).unwrap() - 1,
                u32::from_str(&caps[3]).unwrap() - 1,
                u32::from_str(&caps[4]).unwrap() - 1,
            );
            let v3 = (
                u32::from_str(&caps[5]).unwrap() - 1,
                u32::from_str(&caps[5]).unwrap() - 1,
                u32::from_str(&caps[6]).unwrap() - 1,
            );

            Some((v1, v2, v3))
        }),
        _ => Box::new(|_| None),
    }
}

fn get_tri_regex(uv_fn: &Option<UVMapFn>, normal_fn: &Option<NormalComputeFn>) -> &'static str {
    let tri_reg;

    let normal_compute = normal_fn.is_some();
    let uv_compute = uv_fn.is_some();

    if normal_compute && uv_compute {
        tri_reg = r"^(?:f) (\d+)(?:/\d*)* (\d+)(?:/\d*)* (\d+)(?:/\d*)*";
    } else if normal_compute {
        tri_reg = r"^(?:f) (\d+)/(\d+)(?:/\d*)* (\d+)/(\d+)(?:/\d*)* (\d+)/(\d+)(?:/\d*)*";
    } else if uv_compute {
        tri_reg = r"^(?:f) (\d+)/(?:\d+)?/(\d+) (\d+)/(?:\d+)?/(\d+) (\d+)/(?:\d+)?/(\d+)";
    } else {
        tri_reg = r"^(?:f) (\d+)/(\d+)/(\d+) (\d+)/(\d+)/(\d+) (\d+)/(\d+)/(\d+)";
    }

    tri_reg
}

fn get_tri_from_caps(
    caps: &Captures,
    uv_compute: &Option<UVMapFn>,
    normal_compute: &Option<NormalComputeFn>,
) -> Triangle {
    let (v1, v2, v3);

    match (uv_compute.is_some(), normal_compute.is_some()) {
        (true, true) => {
            v1 = (
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[1]).unwrap() - 1,
            );
            v2 = (
                u32::from_str(&caps[2]).unwrap() - 1,
                u32::from_str(&caps[2]).unwrap() - 1,
                u32::from_str(&caps[2]).unwrap() - 1,
            );
            v3 = (
                u32::from_str(&caps[3]).unwrap() - 1,
                u32::from_str(&caps[3]).unwrap() - 1,
                u32::from_str(&caps[3]).unwrap() - 1,
            );
        }
        (true, false) => {
            v1 = (
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[2]).unwrap() - 1,
            );
            v2 = (
                u32::from_str(&caps[3]).unwrap() - 1,
                u32::from_str(&caps[3]).unwrap() - 1,
                u32::from_str(&caps[4]).unwrap() - 1,
            );
            v3 = (
                u32::from_str(&caps[5]).unwrap() - 1,
                u32::from_str(&caps[5]).unwrap() - 1,
                u32::from_str(&caps[6]).unwrap() - 1,
            );
        }
        (false, true) => {
            v1 = (
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[2]).unwrap() - 1,
                u32::from_str(&caps[1]).unwrap() - 1,
            );
            v2 = (
                u32::from_str(&caps[3]).unwrap() - 1,
                u32::from_str(&caps[4]).unwrap() - 1,
                u32::from_str(&caps[3]).unwrap() - 1,
            );
            v3 = (
                u32::from_str(&caps[5]).unwrap() - 1,
                u32::from_str(&caps[6]).unwrap() - 1,
                u32::from_str(&caps[5]).unwrap() - 1,
            );
        }
        (false, false) => {
            v1 = (
                u32::from_str(&caps[1]).unwrap() - 1,
                u32::from_str(&caps[2]).unwrap() - 1,
                u32::from_str(&caps[3]).unwrap() - 1,
            );
            v2 = (
                u32::from_str(&caps[4]).unwrap() - 1,
                u32::from_str(&caps[5]).unwrap() - 1,
                u32::from_str(&caps[6]).unwrap() - 1,
            );
            v3 = (
                u32::from_str(&caps[7]).unwrap() - 1,
                u32::from_str(&caps[8]).unwrap() - 1,
                u32::from_str(&caps[9]).unwrap() - 1,
            );
        }
    }

    (v1, v2, v3)
}

pub fn load_file_with(
    file_path: &str,
    uv_fn: &Option<UVMapFn>,
    normal_fn: &Option<NormalComputeFn>,
) -> Result<Mesh, io::Error> {
    let content = utility::read_in_file(file_path)?;

    // Raw vectors for constructing final info
    // Verts, normals, UVs, faces
    let initial = (vec![], vec![], vec![], vec![]);

    // Final vectors after parsing is done
    let mut tri_enumerate = Vec::new();

    let mut mesh = Mesh::new();

    let tri_reg = get_tri_match(&uv_fn, &normal_fn);

    let vert_reg = Regex::new(
        r"^(?:v) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?)",
    ).unwrap();
    let normal_reg = Regex::new(
        r"^(?:vn) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?)",
    ).unwrap();
    let uv_reg = Regex::new(r"^(?:vt) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?)").unwrap();

    // Parse verts into raw components
    let (mut verts_raw, normals_raw, mut uvs_raw, tris_raw) = content.lines().fold(
        initial,
        |(mut verts, mut norms, mut uvs, mut tris), line| {
            // Verts
            if let Some(caps) = vert_reg.captures(line) {
                let x = f32::from_str(&caps[1]).unwrap();
                let y = f32::from_str(&caps[2]).unwrap();
                let z = f32::from_str(&caps[3]).unwrap();

                verts.push(Vector3::new(x, y, z));
            // Normals
            } else if let Some(caps) = normal_reg.captures(line) {
                if normal_fn.is_none() {
                    let x = f32::from_str(&caps[1]).unwrap();
                    let y = f32::from_str(&caps[2]).unwrap();
                    let z = f32::from_str(&caps[3]).unwrap();

                    norms.push(Vector3::new(x, y, z));
                }
            // UVs
            } else if let Some(caps) = uv_reg.captures(line) {
                if uv_fn.is_none() {
                    let x = f32::from_str(&caps[1]).unwrap();
                    let y = f32::from_str(&caps[2]).unwrap();

                    uvs.push(Vector2::new(x, y));
                }
            // tris
            } else if let Some(tri) = tri_reg(line) {
                tris.push(tri);
            }

            (verts, norms, uvs, tris)
        },
    );
    //verts_raw = center_verts(&verts_raw);
    verts_raw = normalize_scale(&verts_raw);
    //verts_raw = center_verts(&verts_raw);

    if let &Some(ref func) = uv_fn {
        uvs_raw = verts_raw.iter().map(func).collect();
    }

    // Enumerate unique verticies and index faces
    for (first, second, third) in tris_raw {
        let first_ind = find_or_insert(&first, &mut tri_enumerate) as u32;
        let second_ind = find_or_insert(&second, &mut tri_enumerate) as u32;
        let third_ind = find_or_insert(&third, &mut tri_enumerate) as u32;

        mesh.add_tri((first_ind, second_ind, third_ind));
    }

    // Stitch together verticies from raw data
    for (vert_ind, uv_ind, norm_ind) in tri_enumerate {
        let vert = verts_raw[vert_ind as usize];
        let norm = normals_raw[norm_ind as usize];
        let uv = uvs_raw[uv_ind as usize];

        mesh.add_vertex(&Vertex {
            pos: vert.into(),
            normal: norm.into(),
            uv: uv.into(),
        });
    }

    Ok(mesh)
}

pub fn load_file(file_path: &str) -> Result<Mesh, io::Error> {
    load_file_with(file_path, &None, &None)
}
