use stl_io::{Normal, Triangle, Vertex};

pub fn build_stl_mesh(
    vertices: &[[f32; 3]],
    normals: &[[f32; 3]],    // per-vertex normals
    indices: &[[u32; 3]],       // flat index list, 3 per triangle
) -> Vec<Triangle> {
    let mut mesh = Vec::with_capacity(indices.len());

    for tri in indices.iter() {
        // convert u32 indices to usize and bounds-check
        let ia = tri[0] as usize;
        let ib = tri[1] as usize;
        let ic = tri[2] as usize;
        if ia >= vertices.len() || ib >= vertices.len() || ic >= vertices.len() {
            continue;
        }

        let a = vertices[ia];
        let b = vertices[ib];
        let c = vertices[ic];

        // average the three vertex normals
        let na = normals.get(ia).copied().unwrap_or([0.0, 0.0, 0.0]);
        let nb = normals.get(ib).copied().unwrap_or([0.0, 0.0, 0.0]);
        let nc = normals.get(ic).copied().unwrap_or([0.0, 0.0, 0.0]);

        let mut nx = (na[0] + nb[0] + nc[0]) / 3.0;
        let mut ny = (na[1] + nb[1] + nc[1]) / 3.0;
        let mut nz = (na[2] + nb[2] + nc[2]) / 3.0;

        let len = (nx*nx + ny*ny + nz*nz).sqrt();
        if len > 1e-12 {
            nx /= len; ny /= len; nz /= len;
        }

        mesh.push(Triangle {
            normal: Normal::new([nx, ny, nz]),
            vertices: [
                Vertex::new(a),
                Vertex::new(b),
                Vertex::new(c),
            ],
        });
    }

    mesh
}
