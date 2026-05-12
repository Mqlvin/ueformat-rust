# UEFormat in Rust
UEFormat is an open-source, general-purpose 3D exchange format for extracted Unreal Engine assets.
This library allows you to convert (single-LOD) `.uemodel` files into STL files, which contain purely mesh data.

As mentioned, at the moment the app only exports mesh data. If any interest is taken in this project, it could be further extended to convert `.uemodel` files into more descriptive file-formats, such as glTF which supports material data.

However I only require mesh data at the moment, hence the small scope of this project.

Usage:
```rs
let mut uemodel = open_uefile("./path/to/mymodel.uemodel").unwrap();
let (vertices, indices, normals) = get_vertices_indices_normals(&mut uemodel).unwrap();

// we now have geometric data about the file, which can be processed independently, or saved as STL using stl_io

let mesh = build_stl_mesh(&vertices, &normals, &indices);
let mut file = OpenOptions::new().write(true).create_new(true).open("mesh.stl").unwrap();
stl_io::write_stl(&mut file, mesh.iter()).unwrap();
```
