use std::{fs, mem};
use std::io::{Read, Error};
use nalgebra;
use std::path::{PathBuf, Path};
use gltf::accessor::Iter;
use std::borrow::Borrow;
use crate::CapriCore::cp_directx12::{CpID3D12CommandQueue, CpID3D12Device, CpID3DBlob, CpD3D12_RESOURCE_BARRIER, CpID3D12CommandDispacher, CpD3D12_CPU_DESCRIPTOR_HANDLE, CpID3D12RootSignature, CpID3D12PipelineState};
use winapi::um::d3dcompiler::{D3D_COMPILE_STANDARD_FILE_INCLUDE, D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION};
use winapi::um::d3d12::{D3D12_INPUT_ELEMENT_DESC, D3D12_APPEND_ALIGNED_ELEMENT, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, D3D_ROOT_SIGNATURE_VERSION_1_0, D3D12_VIEWPORT, D3D12_RECT, D3D12_VERTEX_BUFFER_VIEW, D3D12_INDEX_BUFFER_VIEW, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RESOURCE_STATE_PRESENT, D3D12_RESOURCE_STATE_RENDER_TARGET};
use std::ffi::CString;
use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT;
use crate::CapriCore::cp_default_value::{CpD3D12_ROOT_SIGNATURE_DESC, CpD3D12_GRAPHICS_PIPELINE_STATE_DESC};
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};
use crate::CapriCore::cp_directx12::CpD3d12ResourceBarrierDescType::CpD3d12ResourceTransitionBarrier;
use std::ptr::null_mut;
use winapi::um::d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST;

pub struct ShapelObject<'a> {
    pipelineState: CpID3D12PipelineState<'a>,
    rootsignature: CpID3D12RootSignature<'a>,
    cp_id3d12command_dispacher: Box<CpID3D12CommandDispacher<'a>>,
    pub d3d12_vertex_buffer_view: Vec<D3D12_VERTEX_BUFFER_VIEW>,
    pub d3d12_index_buffer_view: Box<D3D12_INDEX_BUFFER_VIEW>,
    pub indexcount: u32,
}

pub trait DrawObj<'a> {
    fn new(cp_id3d12device: &'a CpID3D12Device, cp_id3d12command_queue: &'a CpID3D12CommandQueue, model_path: &Path) -> ShapelObject<'a>;
    fn Update(&mut self, current_sw_heaps: CpD3D12_CPU_DESCRIPTOR_HANDLE, transition_barrier_desc: D3D12_RESOURCE_TRANSITION_BARRIER);
}

#[repr(C)]
#[derive(Clone, Debug, Copy)]
struct pointOnly(nalgebra::Point3<f32>);

impl<'a> DrawObj<'a> for ShapelObject<'a> {
    fn new(cp_id3d12device: &'a CpID3D12Device, cp_id3d12command_queue: &'a CpID3D12CommandQueue, model_path: &Path) -> ShapelObject<'a> {
        let mut _id3d12commanddispacher = Box::from(cp_id3d12device.cp_create_command_dispacher(0, cp_id3d12command_queue, 1, None).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) }));
        let (document, buffers, images) = gltf::import(model_path).unwrap_or_else(|x| { panic!("{}", x) });

        let mut vertexes_pos = Vec::<[f32; 3]>::new();
        let mut vertexes_id = Vec::<u32>::new();
        for mesh in document.meshes() {
            for prim in mesh.primitives() {
                let reader = prim.reader(|x| Some(&buffers[x.index()]));
                if let Some(iter) = reader.read_positions() {
                    vertexes_pos.append(&mut iter.collect::<Vec<[f32; 3]>>());
                }

                if let Some(iter) = reader.read_indices() {
                    vertexes_id.append(&mut iter.into_u32().collect());
                }
            }
        }
        let mut vertex_resource = Vec::<pointOnly>::new();
        for d in vertexes_pos {
            vertex_resource.push(pointOnly { 0: nalgebra::Point3::new(d[0], d[1], d[2]) });
        }

        let (mut CpVertResource, VbView) = cp_id3d12device.cp_create_buffer_resource(0, vertex_resource.into_boxed_slice()).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        let mut destdata = CpVertResource.cp_slice_map(0, None, CpVertResource.data.iter()).unwrap();
        destdata.copy_from_slice(&CpVertResource.data);


        let (mut CpIndexResource, idView) = cp_id3d12device.cp_create_index_resource(0, vertexes_id.into_boxed_slice()).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        let mut destdataId = CpIndexResource.cp_slice_map(0, None, CpIndexResource.data.iter()).unwrap();
        destdataId.copy_from_slice(&CpIndexResource.data);

        let vsBlob = CpID3DBlob::cp_d3dcompile_from_file("Asset\\TestShader.hlsl", None, D3D_COMPILE_STANDARD_FILE_INCLUDE, "vert", "vs_5_0", D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION, 0).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        let psBlob = CpID3DBlob::cp_d3dcompile_from_file("Asset\\TestShader.hlsl", None, D3D_COMPILE_STANDARD_FILE_INCLUDE, "frag", "ps_5_0", D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION, 0).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        let inputElementDesc = vec![
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: CString::new("POSITION").expect("CString::new failed").into_raw(),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            }
        ].into_boxed_slice();
        let cp_d3d12_root_signature_desc: CpD3D12_ROOT_SIGNATURE_DESC = Default::default();
        let rootSigBlob = cp_d3d12_root_signature_desc.cp_d3d12serialize_root_signature(D3D_ROOT_SIGNATURE_VERSION_1_0).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        let mut rootsignature = cp_id3d12device.cp_create_root_signature(0, &rootSigBlob).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
        let mut cpgraphicsPipelineStateDesc = CpD3D12_GRAPHICS_PIPELINE_STATE_DESC::create_d3d12_graphics_pipeline_state_desc(&vsBlob, &psBlob, &inputElementDesc, &mut rootsignature, None, None, None);
        let pipelineState = cp_id3d12device.cp_create_graphics_pipeline_state(&cpgraphicsPipelineStateDesc).unwrap_or_else(|v| {
            println!("last OS error: {:?}", Error::last_os_error());
            panic!("last OS error: {:?}", v)
        });
        ShapelObject {
            pipelineState: pipelineState,
            rootsignature: rootsignature,
            cp_id3d12command_dispacher: _id3d12commanddispacher,
            d3d12_vertex_buffer_view: vec![VbView],
            d3d12_index_buffer_view: Box::new(idView),
            indexcount: CpIndexResource.data.len() as u32,
        }
    }

    fn Update(&mut self, current_sw_heaps: CpD3D12_CPU_DESCRIPTOR_HANDLE, mut transition_barrier_desc: D3D12_RESOURCE_TRANSITION_BARRIER) {
        self.cp_id3d12command_dispacher.cp_reset(&mut None);
        let viewport = D3D12_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: WINDOW_WIDTH as f32,
            Height: WINDOW_HEIGHT as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        let scissorRect = D3D12_RECT {
            left: 0,
            top: 0,
            right: WINDOW_WIDTH as i32,
            bottom: WINDOW_HEIGHT as i32,
        };
        transition_barrier_desc.StateBefore = D3D12_RESOURCE_STATE_PRESENT;
        transition_barrier_desc.StateAfter = D3D12_RESOURCE_STATE_RENDER_TARGET;
        let barrier_desc = CpD3D12_RESOURCE_BARRIER::new(CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier: transition_barrier_desc, flags: 0 });
        self.cp_id3d12command_dispacher.command_lists[0].cp_resource_barrier(&vec![barrier_desc]);
        self.cp_id3d12command_dispacher.command_lists[0].cp_omset_render_targets(&vec![current_sw_heaps.value], true, None);
        self.cp_id3d12command_dispacher.command_lists[0].cp_rs_set_viewports(&vec![viewport]);
        self.cp_id3d12command_dispacher.command_lists[0].cp_rs_set_scissor_rects(&vec![scissorRect]);
        self.cp_id3d12command_dispacher.command_lists[0].cp_set_pipeline_states(&mut self.pipelineState);
        self.cp_id3d12command_dispacher.command_lists[0].cp_set_graphics_root_signature(&mut self.rootsignature);
        self.cp_id3d12command_dispacher.command_lists[0].cp_iaset_primitive_topology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        self.cp_id3d12command_dispacher.command_lists[0].cp_iaset_vertex_buffers(0, &self.d3d12_vertex_buffer_view);
        self.cp_id3d12command_dispacher.command_lists[0].cp_iaset_index_buffer(&self.d3d12_index_buffer_view);
        self.cp_id3d12command_dispacher.command_lists[0].cp_draw_indexed_instanced(self.indexcount, 1, 0, 0, 0);
        transition_barrier_desc.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
        transition_barrier_desc.StateAfter = D3D12_RESOURCE_STATE_PRESENT;

        let barrier_desc = CpD3D12_RESOURCE_BARRIER::new(CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier: transition_barrier_desc, flags: 0 });
        self.cp_id3d12command_dispacher.command_lists[0].cp_resource_barrier(&vec![barrier_desc]);

        self.cp_id3d12command_dispacher.command_lists[0].cp_close();
        self.cp_id3d12command_dispacher.cp_execute_command_lists();
    }
}
