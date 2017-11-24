use std::f32;
use std::u32;
use std::str::FromStr;

use std::fs::File;
use std::io;
use std::io::Read;
use na::{Vector2, Vector3};
use regex::Regex;

use mesh::Mesh;
use program::Vertex;

fn read_in(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

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

// pub fn load_file_vertex_only(file_path: &str) -> Result<Mesh, io::Error> {
//     let content = read_in(file_path)?;

//     let mut mesh = Mesh::new();
//     let tri_reg = Regex::new(r"^(?:f) (\d+) (\d+) (\d+)").unwrap();
//     let vert_reg = Regex::new(
//         r"^(?:v) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?)",
//     ).unwrap();

//     let initial = (Vec::new(), Vec::new());
//     let (verts, tris) = content
//         .lines()
//         .fold(initial, |(mut verts, mut tris), line| {
//             if let Option::Some(caps) = vert_reg.captures(line) {
//                 let x = f32::from_str(&caps[1]).unwrap();
//                 let y = f32::from_str(&caps[2]).unwrap();
//                 let z = f32::from_str(&caps[3]).unwrap();

//                 verts.push(Vector3::new(x, y, z));
//             } else if let Option::Some(caps) = tri_reg.captures(line) {
//                 let x = u32::from_str(&caps[1]).unwrap() - 1;
//                 let y = u32::from_str(&caps[2]).unwrap() - 1;
//                 let z = u32::from_str(&caps[3]).unwrap() - 1;

//                 tris.push((x, y, z));
//             }

//             (verts, tris)
//         });

//     mesh.add_tris(tris.as_slice())
//         .add_verticies(verts.as_slice());

//     mesh.preprocess_compute_normals();
//     Ok(mesh)
// }

pub fn load_file(file_path: &str) -> Result<Mesh, io::Error> {
    let content = read_in(file_path)?;

    // Raw vectors for constructing final info
    // Verts, normals, UVs, faces
    let initial = (vec![], vec![], vec![], vec![]);

    // Final vectors after parsing is done
    let mut tri_enumerate = Vec::new();

    let mut mesh = Mesh::new();

    let tri_reg =
        Regex::new(r"^(?:f) (\d+)/(\d+)/(\d+) (\d+)/(\d+)/(\d+) (\d+)/(\d+)/(\d+)").unwrap();
    // let tri_reg_inner = Regex::new(r"(\d+)\/(\d+)\/(\d+)").unwrap();
    let vert_reg = Regex::new(
        r"^(?:v) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?)",
    ).unwrap();
    let normal_reg = Regex::new(
        r"^(?:vn) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?)",
    ).unwrap();
    let uv_reg = Regex::new(r"^(?:vt) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?)").unwrap();

    // Parse verts into raw components
    let (verts_raw, normals_raw, uvs_raw, tris_raw) = content.lines().fold(
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
                let x = f32::from_str(&caps[1]).unwrap();
                let y = f32::from_str(&caps[2]).unwrap();
                let z = f32::from_str(&caps[3]).unwrap();

                norms.push(Vector3::new(x, y, z));
            // UVs
            } else if let Some(caps) = uv_reg.captures(line) {
                let x = f32::from_str(&caps[1]).unwrap();
                let y = f32::from_str(&caps[2]).unwrap();

                uvs.push(Vector2::new(x, y));
            // tris
            } else if let Some(caps) = tri_reg.captures(line) {
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

                tris.push((v1, v2, v3));
            }

            (verts, norms, uvs, tris)
        },
    );

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
