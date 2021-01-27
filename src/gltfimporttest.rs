use std::{fs, mem};
use std::io::Read;
use nalgebra;
use std::path::PathBuf;
use gltf::accessor::Iter;
use std::borrow::Borrow;

fn as_raw_bytes<'a, T: ?Sized>(x: &'a T) -> &'a [u8] {
    unsafe {
        std::slice::from_raw_parts(
            x as *const T as *const u8,
            std::mem::size_of_val(x))
    }
}
fn size_of_raw_bytes<'a, T: ?Sized>(x: &'a T)->usize{
    as_raw_bytes(x).len()
}
pub fn gltfimport() {
    let srcdir = PathBuf::from("./");
    println!("{:?}", fs::canonicalize(&srcdir));
    //ルートディレクトリはrustDirectxです。
    let (document, buffers, images) = gltf::import("C:\\Users\\Desktop\\CLionProjects\\rustDirectx\\src\\Asset\\Box.glb").unwrap_or_else(|x| { panic!("{}", x) });
    println!("{:?}", document);
    println!("{:?}", buffers);
    println!("{:?}", images);
    struct pointUv(nalgebra::Point3<f32>, nalgebra::Point2<f32>);
    let vertices = vec![
        pointUv(nalgebra::Point3::new(-1.0, -1.0, 0.0),nalgebra::Point2::new(1.0,1.0)),
        pointUv(nalgebra::Point3::new(0.0, 1.0, 0.0) ,nalgebra::Point2::new(1.0,0.0)),
        pointUv(nalgebra::Point3::new(1.0, -1.0, 0.0),nalgebra::Point2::new(0.0,1.0)),
        pointUv(nalgebra::Point3::new(1.0, 1.0, 0.0) ,nalgebra::Point2::new(0.0,0.0))
    ];
    println!("vertices:{:?}", as_raw_bytes(vertices.as_slice()).len());
    println!("vertices:{:?}", std::mem::size_of_val(vertices.as_slice()));
    println!("vertices:{:?}", std::mem::size_of_val(&vertices));
    println!("vertices:{:?}", vertices.first().unwrap().0);
    let boxed = vertices.into_boxed_slice();
    println!("vertices:{:?}", std::mem::size_of_val(boxed.as_ref()));
    println!("vertices:{:?}", std::mem::size_of_val(&boxed[0]));
    println!("vertices:{:?}", std::mem::size_of_val(boxed.first().unwrap()));
    println!("vertices:{:?}", std::mem::size_of::<nalgebra::Point3<f32>>());

    let gltfbox = document;
    //let acc = gltfbox.accessors();
    // acc.map(|x|{
    //     println!("AccName:{}",x.name().unwrap_or("None"));
    // });
    // gltfbox.nodes().map(|x|{
    //     println!("{}",x.name().unwrap_or("Nieq"));
    // });
    //let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    for mesh in gltfbox.meshes() {
        for prim in mesh.primitives() {
            for buff in gltfbox.buffers() {
                let reader = prim.reader(|x| Some(&buffers[buff.index()]));
                if let Some(iter) = reader.read_positions() {
                    println!("attributes().dimensions:{:?}", iter);
                    for vertex_position in iter {
                        println!("{:?}", vertex_position);
                    }
                }
                if let Some(iter) = reader.read_positions() {
                    println!("attributes().dimensions:{:?}", iter.into_iter().collect::<Vec<[f32;3]>>());
                }

                if let Some(iter) = reader.read_indices() {
                    println!("attributes().dimensions:{:?}", iter);
                    for vertex_position in iter.into_u32() {
                        println!("{:?}", vertex_position);
                    }
                }
            }
        }
    }
    println!("mesh.index:{}", gltfbox.meshes().len());
    for scene in gltfbox.scenes() {
        println!("node.len:{}", scene.nodes().len());
        for node in scene.nodes() {
            println!("node.index:{}", node.index());
            for nc in node.children() {
                println!("nc.index:{}", nc.index());
                if let Some(mesh) = nc.mesh()
                {
                    println!("primitives().len:{}", mesh.primitives().len());
                    for prim in mesh.primitives() {
                        println!("attributes().len:{}", prim.attributes().len());
                        println!("attributes().data_type:{:?}", prim.indices().unwrap().data_type());
                        println!("attributes().dimensions:{:?}", prim.indices().unwrap().dimensions());
                        let a = prim.indices().unwrap().view().unwrap().buffer();
                        println!("attributes().dimensionsa:{:?}", a);
                        println!("attributes().dimensionsa:{:?}", a.index());
                        let reader = prim.reader(|x| Some(&buffers[a.index()]));
                        if let Some(iter) = reader.read_positions() {
                            println!("attributes().dimensions:{:?}", iter);
                            for vertex_position in iter {
                                println!("{:?}", vertex_position);
                            }
                        }
                        if let Some(iter) = reader.read_indices() {
                            println!("attributes().dimensions:{:?}", iter);
                            for vertex_position in iter.into_u32() {
                                println!("{:?}", vertex_position);
                            }
                        }

                        println!("attributes().dimensions:{:?}", prim.indices().unwrap().view().unwrap().buffer().source());
                        println!("attributes().dimensions:{:?}", prim.indices().unwrap().view().unwrap().target().unwrap());
                        for attr in prim.attributes()
                        {
                            println!("attributes0:{}", attr.0.to_string());
                            //component_type
                            println!("attributes1data_type:{:?}", attr.1.data_type());
                            println!("attributes1dimensions:{:?}", attr.1.dimensions());
                            println!("attributes1:{:?}", attr.1.view().unwrap().buffer().index());
                        }
                    }
                }
            }
        }
    }
}