use std::{fs::File, io::{BufReader, Read}};

use crate::ueformat::{error::ParseError, parser::UEFileParser};

mod error;
mod parser;

const MAGIC: &str = "UEFORMAT";
const COMPRESSION_METHOD: &str = "ZSTD";

const LOD_HEADER_NAME: &str = "LODS";
const VERTICES_HEADER_NAME: &str = "VERTICES";
const INDICES_HEADER_NAME: &str = "INDICES";
const NORMALS_HEADER_NAME: &str = "NORMALS";


pub fn open_uefile(path: &str) -> Result<UEFileParser, ParseError> {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(err) => return Err(ParseError::FileError(err))
    };

    let mut buf = BufReader::new(f);
    let mut bytes = Vec::new();
    match buf.read_to_end(&mut bytes) {
        Ok(_) => {},
        Err(err) => return Err(ParseError::FileError(err))
    }

    Ok(UEFileParser::new(bytes.clone()))
}

pub fn get_vertices_indices_normals(fp: &mut UEFileParser) -> Result<(
    Vec<[f32; 3]>,
    Vec<[u32; 3]>,
    Vec<[f32; 3]>
    ), ParseError> {

    match ensure_magic_bytes(fp) {
        Ok(_) => {}
        Err(err) => return Err(err)
    };

    let _identifier = fp.read_fstring()?;
    let _file_version = fp.read_byte()?;
    let _obj_name = fp.read_fstring()?;
    let is_compressed = fp.read_bool()?;

    let lod_level_bytes = match is_compressed {
        true => {
            let compression_type = fp.read_fstring()?;
            if compression_type != COMPRESSION_METHOD {
                return Err(ParseError::UnsupportedCompression(compression_type.to_string()));
            }
            let _uncompressed_size = fp.read_int()?;
            let _compressed_size = fp.read_int()?;
            fp.decompress_remaining_to_vec()?
        },
        false => {
            fp.read_to_end()?
}
    };

    let mut ld_fp = UEFileParser::new(lod_level_bytes);
    ensure_one_lod(&mut ld_fp)?;

    let (vertices, indices, normals) = extract_mesh_data(&mut ld_fp)?;
    Ok((vertices, indices, normals))
}


fn extract_mesh_data(fp: &mut UEFileParser) -> Result<(
    Vec<[f32; 3]>,
    Vec<[u32; 3]>,
    Vec<[f32; 3]>
    ), ParseError> {
    let mut vertices: Option<Vec<[f32; 3]>> = None;
    let mut indices: Option<Vec<[u32; 3]>> = None;
    let mut normals: Option<Vec<[f32; 3]>> = None;

    while !fp.eof() {
        let header_name = fp.read_fstring()?;
        let array_size = fp.read_int()?;
        let byte_size = fp.read_int()?;

        if header_name == VERTICES_HEADER_NAME {
            let flattened_vertices = fp.read_float_vector(array_size as usize * 3)?;
            let scale: f32 = 1.;
            let out: Vec<[f32; 3]> = flattened_vertices
                .chunks_exact(3)
                .map(|c| {
                    let x = c[0] * scale;
                    let y = c[1] * scale * -1.0; // apply (1, -1, 1) flip
                    let z = c[2] * scale;
                    [x, y, z]
                })
            .collect();
            vertices = Some(out);

        } else if header_name == INDICES_HEADER_NAME {
            let flattened_indices = fp.read_int_vector(array_size as usize)?;
            let out: Vec<[u32; 3]> = flattened_indices
                .chunks_exact(3)
                .map(|c| [c[0], c[1], c[2]])
                .collect();
            indices = Some(out);

        } else if header_name == NORMALS_HEADER_NAME {
            let flattened_normals = fp.read_float_vector(array_size as usize * 4)?;
            let out: Vec<[f32; 3]> = flattened_normals
                .chunks_exact(4)
                .map(|c| [c[1], c[2], c[3]]) // take columns 1..3
                .collect();
            normals = Some(out);

        } else {
            fp.skip(byte_size as i64)?;
        }
    }

    if let None = vertices {
        return Err(ParseError::MissingMeshData(VERTICES_HEADER_NAME.to_string()));
    };

    if let None = indices {
        return Err(ParseError::MissingMeshData(INDICES_HEADER_NAME.to_string()));
    };

    if let None = normals {
        return Err(ParseError::MissingMeshData(NORMALS_HEADER_NAME.to_string()));
    };

    Ok((vertices.unwrap(), indices.unwrap(), normals.unwrap()))
}

fn ensure_magic_bytes(fp: &mut UEFileParser) -> Result<(), ParseError> {
    match fp.read_string(MAGIC.len()) {
        Ok(str) => { 
            if str == MAGIC {
                Ok(())
            } else {
                Err(ParseError::NoMagicBytes)
            }
        },
        Err(_) => Err(ParseError::NoMagicBytes)
    }
}

fn ensure_one_lod(fp: &mut UEFileParser) -> Result<(), ParseError> {
    let mut header_name = String::new();
    let mut _array_size = 0;
    let mut byte_size = 0;

    while header_name != LOD_HEADER_NAME {
        fp.skip(byte_size as i64)?;
        header_name = fp.read_fstring()?;
        _array_size = fp.read_int()?;
        byte_size = fp.read_int()?;
    }

    // lod level, pos
    let mut found_lods: Vec<(String, u64)> = Vec::with_capacity(4);

    loop {
        if fp.eof() {
            break;
        }

        let lod_name = fp.read_fstring()?;
        let lod_size = fp.read_int()?;

        if !lod_name.starts_with("LOD") {
            break;
        }

        found_lods.push((lod_name, fp.get_pos()));

        fp.skip(lod_size as i64)?;
    }

    if found_lods.is_empty() {
        return Err(ParseError::MultipleLODs());
    }
    let lod = found_lods.last().unwrap();
    println!("Using lod: {} {}", lod.0, lod.1);
    fp.goto(lod.1)?;
    return Ok(());
}
