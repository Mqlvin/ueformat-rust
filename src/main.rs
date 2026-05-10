use std::fs::OpenOptions;
use crate::{geometry::build_stl_mesh, ueformat::{get_vertices_indices_normals, open_uefile}};

mod ueformat;
mod geometry;

fn main() {
    let mut uemodel = open_uefile("./path/to/mymodel.uemodel").unwrap();
    let (vertices, indices, normals) = get_vertices_indices_normals(&mut uemodel).unwrap();

    let mesh = build_stl_mesh(&vertices, &normals, &indices);

    let mut file = OpenOptions::new().write(true).create_new(true).open("mesh.stl").unwrap();
    stl_io::write_stl(&mut file, mesh.iter()).unwrap();
}

