use std::f32;
use std::u32;
use std::str::FromStr;

use std::fs::File;
use std::io;
use std::io::{Read};
use na::Vector3;
use regex::Regex;

use mesh::Mesh;

pub fn load_file(file_path: &str) -> Result<Mesh, io::Error> {
    let res = File::open(file_path);

    match res {
        Result::Err(what) => Result::Err(what),
        Result::Ok(mut file) => {
            let mut mesh = Mesh::new();
            let mut content = String::new();
            let tri_reg = Regex::new(r"^(?:f) (\d+) (\d+) (\d+)").unwrap();
            let vert_reg = Regex::new(
                r"^(?:v) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?) (-?[\d]+(?:\.[\d]+)?)",
            ).unwrap();

            file.read_to_string(&mut content).unwrap();

            let initial = (Vec::new(), Vec::new());
            let (verts, tris) = content.lines().fold(
                initial,
                |(mut verts, mut tris), line| {
                    if let Option::Some(caps) = vert_reg.captures(&line) {
                        let x = f32::from_str(&caps[1]).unwrap();
                        let y = f32::from_str(&caps[2]).unwrap();
                        let z = f32::from_str(&caps[3]).unwrap();

                        verts.push(Vector3::new(x, y, z));
                    } else if let Option::Some(caps) = tri_reg.captures(&line) {
                        let x = u32::from_str(&caps[1]).unwrap() - 1;
                        let y = u32::from_str(&caps[2]).unwrap() - 1;
                        let z = u32::from_str(&caps[3]).unwrap() - 1;

                        tris.push((x, y, z));
                    }

                    (verts, tris)
                },
            );

            mesh.add_tris(tris.as_slice()).add_verticies(
                verts.as_slice(),
            );

            mesh.preprocess_compute_normals();
            Result::Ok(mesh)
        }
    }
}
