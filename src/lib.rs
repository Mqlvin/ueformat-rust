pub mod ueformat;
pub mod geometry;

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;

    use crate::{geometry::build_stl_mesh, ueformat::{get_vertices_indices_normals, open_uefile}};

    #[test]
    fn test() {
let mut uemodel = open_uefile("./mapdata/LandscapeStreamingProxy_0AXZYDTCPDF5QKE8GYSIJCE8L_254_4_3_0.uemodel").unwrap();
let (vertices, indices, normals) = get_vertices_indices_normals(&mut uemodel).unwrap();

// we now have geometrical data about the file, which can be processed or saved using stl_io

let mesh = build_stl_mesh(&vertices, &normals, &indices);
let mut file = OpenOptions::new().write(true).create_new(true).open("mesh.stl").unwrap();
stl_io::write_stl(&mut file, mesh.iter()).unwrap();
    }
}
