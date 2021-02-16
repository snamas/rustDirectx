pub(crate) mod cp_default_value;

use winapi::um::winnt::{HRESULT, LPCWSTR, HANDLE, LPCSTR};
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM, HINSTANCE, FALSE, TRUE, BOOL};
use winapi::shared::windef::{HICON, HWND, RECT, HWND__, POINT};
use winapi::um::winuser::{MB_OK, MessageBoxW, WM_DESTROY, PostQuitMessage, WNDCLASSEXW, AdjustWindowRect, WS_OVERLAPPEDWINDOW, RegisterClassExW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, WS_VISIBLE, UnregisterClassW, LoadCursorW, IDC_ARROW, CS_OWNDC, AdjustWindowRectEx, ShowWindow, SW_SHOW, PeekMessageW, MSG, TranslateMessage, DispatchMessageW, WM_QUIT, PM_REMOVE, WS_OVERLAPPED};
use winapi::um::d3d12::{D3D12GetDebugInterface, ID3D12Device, D3D12CreateDevice, D3D12_COMMAND_LIST_TYPE_DIRECT, ID3D12CommandAllocator, ID3D12GraphicsCommandList, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_COMMAND_QUEUE_PRIORITY_NORMAL, ID3D12CommandQueue, ID3D12Pageable, ID3D12DeviceChild, ID3D12Object, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_DESCRIPTOR_HEAP_FLAG_NONE, ID3D12DescriptorHeap, ID3D12Resource, D3D12_CPU_DESCRIPTOR_HANDLE, ID3D12CommandList, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_RENDER_TARGET_VIEW_DESC, D3D12_COMMAND_LIST_TYPE, ID3D12PipelineState, D3D12_RESOURCE_BARRIER, ID3D12Fence, D3D12_FENCE_FLAGS, D3D12_RESOURCE_BARRIER_TYPE, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RESOURCE_ALIASING_BARRIER, D3D12_RESOURCE_UAV_BARRIER, D3D12_RESOURCE_STATES, D3D12_RESOURCE_BARRIER_FLAGS, D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_BARRIER_u, D3D12_RESOURCE_BARRIER_TYPE_ALIASING, D3D12_RESOURCE_BARRIER_TYPE_UAV, D3D12_HEAP_PROPERTIES, D3D12_HEAP_FLAGS, D3D12_RESOURCE_DESC, D3D12_CLEAR_VALUE, D3D12_MEMORY_POOL_UNKNOWN, D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_HEAP_TYPE_UPLOAD, D3D12_RESOURCE_DIMENSION_BUFFER, D3D12_RESOURCE_FLAG_NONE, D3D12_TEXTURE_LAYOUT_ROW_MAJOR, D3D12_HEAP_FLAG_NONE, D3D12_RESOURCE_STATE_COPY_DEST, D3D12_RANGE, D3D12_VERTEX_BUFFER_VIEW, D3D12_RESOURCE_STATE_GENERIC_READ, D3D12_INDEX_BUFFER_VIEW, D3D12_GRAPHICS_PIPELINE_STATE_DESC, D3D12_SHADER_BYTECODE, D3D12_INPUT_ELEMENT_DESC, D3D12_INPUT_LAYOUT_DESC, D3D_ROOT_SIGNATURE_VERSION, D3D12SerializeRootSignature, ID3D12RootSignature, D3D12_APPEND_ALIGNED_ELEMENT, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA};
use winapi::um::d3d12sdklayers::{ID3D12Debug};
use winapi::shared::dxgi1_6::{IDXGIFactory6};
use winapi::shared::dxgi1_3::{CreateDXGIFactory2, DXGI_CREATE_FACTORY_DEBUG};
use winapi::shared::dxgi1_2::{DXGI_SWAP_CHAIN_DESC1, DXGI_SCALING_STRETCH, DXGI_ALPHA_MODE_UNSPECIFIED};
use winapi::shared::winerror::{S_OK};
use winapi::um::d3dcommon::{D3D_FEATURE_LEVEL_12_1, ID3DBlob, D3D_SHADER_MACRO, ID3DInclude, ID3D10Blob};
use winapi::um::libloaderapi::{GetModuleHandleW};
use winapi::um::unknwnbase::{IUnknown};
use winapi::Interface;
use std::ptr::null_mut;
use winapi::shared::dxgi1_5::IDXGISwapChain4;
use winapi::shared::dxgiformat::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN, DXGI_FORMAT_R16_UINT, DXGI_FORMAT_R32_UINT, DXGI_FORMAT_R32G32B32_FLOAT};
use winapi::shared::dxgi::{DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_CHAIN_DESC};
use winapi::shared::dxgitype::{DXGI_USAGE_BACK_BUFFER, DXGI_SAMPLE_DESC};
use winapi::ctypes::c_void;
use std::io::Error;
use winapi::um::d3dcompiler::D3DCompileFromFile;
use std::ffi::CString;
use crate::Cp_directx12::cp_default_value::{CpD3D12_GRAPHICS_PIPELINE_STATE_DESC, CpD3D12_ROOT_SIGNATURE_DESC};
use winapi::_core::ptr::null;
use std::fmt::Debug;

pub struct CpID3D12Device(pub Box<ID3D12Device>);

pub struct CpID3DBlob<'a>(pub &'a ID3DBlob);

pub struct CpID3D12CommandQueue {
    pub(crate) value: Box<ID3D12CommandQueue>,
    type_: D3D12_COMMAND_LIST_TYPE,
}

pub struct CpIDXGIFactory6(pub Box<IDXGIFactory6>);

pub struct CpIDXGISwapChain4<'a> {
    pub value: &'a IDXGISwapChain4,
    pub desc: DXGI_SWAP_CHAIN_DESC1,
}

pub struct CpID3D12DescriptorHeap {
    value: Box<ID3D12DescriptorHeap>,
    desc: D3D12_DESCRIPTOR_HEAP_DESC,
}

pub struct CpHWND<'a>(pub &'a mut HWND__, WNDCLASSEXW);

pub struct CpMSG {
    value: MSG,
    pub hasMessage: bool,
}

pub struct CpID3D12Resource<'a, T> {
    ///todo:ID3D12Resourceはヒープ領域に確保されているはず。Boxにしたらdrop時にエラーが発生する
    pub(crate) value: &'a mut ID3D12Resource,
    pub(crate) data: T,
    pub(crate) destdata: Option<&'a mut T>,
}

pub struct CpID3D12CommandAllocator(pub(crate) Box<ID3D12CommandAllocator>);

pub struct CpID3D12GraphicsCommandList(pub Box<ID3D12GraphicsCommandList>);

pub struct CpD3D12_RESOURCE_BARRIER<'a>(pub &'a mut D3D12_RESOURCE_BARRIER);

pub struct CpID3D12RootSignature<'a>(pub &'a mut ID3D12RootSignature);

pub struct CpID3D12PipelineState<'a>(pub &'a mut ID3D12PipelineState);

pub struct CpID3D12Fence<'a> {
    value: &'a mut ID3D12Fence,
    fenceval: u64,
}

pub struct CpD3D12_CPU_DESCRIPTOR_HANDLE {
    pub(crate) value: D3D12_CPU_DESCRIPTOR_HANDLE,
    DescriptorHeapType: D3D12_DESCRIPTOR_HEAP_TYPE,
}

pub struct CpID3D12CommandDispacher<'a> {
    command_queue: &'a CpID3D12CommandQueue,
    pub command_allocator: CpID3D12CommandAllocator,
    pub command_lists: Vec<CpID3D12GraphicsCommandList>,
}

impl CpD3D12_CPU_DESCRIPTOR_HANDLE {
    pub fn cp_descriptor_handle_increment_ptr(&self, cp_id3d12device: &CpID3D12Device, index: u32) -> CpD3D12_CPU_DESCRIPTOR_HANDLE {
        let mut newHandle: CpD3D12_CPU_DESCRIPTOR_HANDLE = CpD3D12_CPU_DESCRIPTOR_HANDLE {
            value: D3D12_CPU_DESCRIPTOR_HANDLE { ptr: self.value.ptr + (index * cp_id3d12device.cp_get_descriptor_handle_increment_size(self.DescriptorHeapType)) as usize },
            DescriptorHeapType: self.DescriptorHeapType,
        };
        return newHandle;
    }
}

// pub fn cp_descriptor_handle_increment_ptr (&self, handle:&mut D3D12_CPU_DESCRIPTOR_HANDLE, index:u32, DescriptorHeapType: D3D12_DESCRIPTOR_HEAP_TYPE) -> &mut D3D12_CPU_DESCRIPTOR_HANDLE {
//     handle.ptr += (index * self.cp_get_descriptor_handle_increment_size(DescriptorHeapType)) as usize;
//     return handle;
// }
pub fn to_wide_chars(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect()
}

trait HRESULTChecker {
    fn hresult_to_result(&self) -> Result<HRESULT, HRESULT>;
}

impl HRESULTChecker for HRESULT {
    fn hresult_to_result(&self) -> Result<HRESULT, HRESULT> {
        match *self {
            S_OK => Ok(*self),
            _ => Err(*self)
        }
    }
}

trait BOOLTranslater {
    fn BOOLtobool(&self) -> bool;
}

impl BOOLTranslater for BOOL {
    fn BOOLtobool(&self) -> bool {
        match *self {
            TRUE => true,
            FALSE => false,
            _ => false
        }
    }
}

impl CpID3D12Device {
    pub fn new() -> CpID3D12Device {
        match CpID3D12Device::cp_d3d12create_device_result() {
            Ok(v) => return v,
            Err(v) => {
                println!("last OS error: {:?}", Error::last_os_error());
                panic!("{}", v);
            }
        }
    }
    fn cp_d3d12create_device_result() -> Result<CpID3D12Device, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match D3D12CreateDevice(null_mut(), D3D_FEATURE_LEVEL_12_1, &ID3D12Device::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    let mut _id3d12deviceOpt = (_unknownobj as *const ID3D12Device).as_ref();
                    match _id3d12deviceOpt {
                        Some(v) => return Ok(CpID3D12Device(std::mem::transmute(v))),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            };
        }
    }
    pub fn cp_create_command_queue(&self, d3d12command_queue_desc_opt: Option<D3D12_COMMAND_QUEUE_DESC>) -> Result<CpID3D12CommandQueue, HRESULT> {
        let d3d12command_queue_desc = match d3d12command_queue_desc_opt {
            Some(v) => { v }
            None => {
                D3D12_COMMAND_QUEUE_DESC {
                    Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                    Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL as i32,
                    Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                    NodeMask: 0,
                }
            }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateCommandQueue(&d3d12command_queue_desc, &ID3D12CommandQueue::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    let mut _id3d12_command_queue = (_unknownobj as *mut ID3D12CommandQueue).as_mut();
                    match _id3d12_command_queue {
                        Some(v) => return Ok(CpID3D12CommandQueue { value: std::mem::transmute(v), type_: d3d12command_queue_desc.Type }),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_descriptor_heap(&self, heap_desc_for_swapchain_opt: Option<D3D12_DESCRIPTOR_HEAP_DESC>) -> Result<CpID3D12DescriptorHeap, HRESULT> {
        let heap_desc_for_swapchain = match heap_desc_for_swapchain_opt {
            Some(v) => { v }
            None => {
                D3D12_DESCRIPTOR_HEAP_DESC {
                    Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                    NumDescriptors: 1,
                    Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
                    NodeMask: 0,
                }
            }
        };
        let mut _unknownobj = null_mut();
        unsafe {
            match self.0.CreateDescriptorHeap(&heap_desc_for_swapchain, &ID3D12DescriptorHeap::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12DescriptorHeap).as_ref() {
                        Some(_id3d12descripterheap_for_swapchain) => { return Ok(CpID3D12DescriptorHeap { value: std::mem::transmute(_id3d12descripterheap_for_swapchain), desc: heap_desc_for_swapchain }); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_command_allocator(&self, type_: D3D12_COMMAND_LIST_TYPE) -> Result<CpID3D12CommandAllocator, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateCommandAllocator(type_, &ID3D12CommandAllocator::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12CommandAllocator).as_mut() {
                        Some(_id3d12command_allocator) => { return Ok(CpID3D12CommandAllocator(std::mem::transmute(_id3d12command_allocator))); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            };
        }
    }
    pub fn cp_create_command_list(&self, node_mask: UINT, type_: D3D12_COMMAND_LIST_TYPE, p_command_allocator: &mut CpID3D12CommandAllocator, p_initial_state_opt: &mut Option<ID3D12PipelineState>) -> Result<CpID3D12GraphicsCommandList, HRESULT> {
        let p_initial_state: *mut ID3D12PipelineState = match p_initial_state_opt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateCommandList(node_mask, type_, p_command_allocator.0.as_mut(), null_mut(), &ID3D12GraphicsCommandList::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12GraphicsCommandList).as_mut() {
                        Some(_id3d12graphics_command_list) => {
                            println!("cp_create_command_dispacher4: {:?}", Error::last_os_error());
                            return Ok(CpID3D12GraphicsCommandList( std::mem::transmute(_id3d12graphics_command_list) ));
                        }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_command_dispacher<'a>(&self, node_mask: UINT, cp_id3d12command_queue: &'a CpID3D12CommandQueue, listnum: u32, mut p_initial_state_opt: Option<ID3D12PipelineState>) -> Result<CpID3D12CommandDispacher<'a>, HRESULT> {
        let mut _id3d12command_allocator = self.cp_create_command_allocator(cp_id3d12command_queue.type_)?;
        let mut command_lists = (0..listnum).map(|index| -> CpID3D12GraphicsCommandList {
            self.cp_create_command_list(node_mask, cp_id3d12command_queue.type_, &mut _id3d12command_allocator, &mut p_initial_state_opt).unwrap_or_else(|v| { panic!("{}", v) })
        }).collect();
        return Ok(CpID3D12CommandDispacher {
            command_queue: cp_id3d12command_queue,
            command_allocator: _id3d12command_allocator,
            command_lists: command_lists,
        });
    }
    pub fn cp_get_descriptor_handle_increment_size(&self, DescriptorHeapType: D3D12_DESCRIPTOR_HEAP_TYPE) -> UINT {
        unsafe { self.0.GetDescriptorHandleIncrementSize(DescriptorHeapType) }
    }
    pub fn cp_create_render_target_view<T>(&self, pResource: &mut CpID3D12Resource<T>, pDesc_opt: Option<D3D12_RENDER_TARGET_VIEW_DESC>, DestDescriptor: CpD3D12_CPU_DESCRIPTOR_HANDLE) {
        let pDesc: *const D3D12_RENDER_TARGET_VIEW_DESC = match pDesc_opt {
            Some(v) => { &v }
            None => { null_mut() }
        };
        unsafe { self.0.CreateRenderTargetView(pResource.value, pDesc, DestDescriptor.value) }
    }
    pub fn cp_create_fence(&self, initialValue: u64, flags: D3D12_FENCE_FLAGS) -> Result<CpID3D12Fence, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateFence(initialValue, flags, &ID3D12Fence::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12Fence).as_mut() {
                        Some(_id3d12_fence) => { return Ok(CpID3D12Fence { value: _id3d12_fence, fenceval: initialValue }); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_committed_resource<T>(&self, pHeapProperties: &D3D12_HEAP_PROPERTIES, HeapFlags: D3D12_HEAP_FLAGS, pResourceDesc: &D3D12_RESOURCE_DESC, InitialResourceState: D3D12_RESOURCE_STATES, pOptimizedClearValueOpt: &Option<D3D12_CLEAR_VALUE>, data: T) -> Result<CpID3D12Resource<T>, HRESULT> {
        let pOptimizedClearValue: *const D3D12_CLEAR_VALUE = match pOptimizedClearValueOpt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateCommittedResource(pHeapProperties, HeapFlags, pResourceDesc, InitialResourceState, pOptimizedClearValue, &ID3D12Resource::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12Resource).as_mut() {
                        Some(_id3d12_resorce) => {
                            return Ok(CpID3D12Resource { value: std::mem::transmute(_id3d12_resorce), data, destdata: None }); }

                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_buffer_resource<T>(&self, nodemask: u32, vertices: Box<[T]>) -> Result<(CpID3D12Resource<Box<[T]>>, D3D12_VERTEX_BUFFER_VIEW), HRESULT> {
        let heapProperties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: nodemask,
            VisibleNodeMask: nodemask,
        };
        let resourceDesc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: std::mem::size_of_val(vertices.as_ref()) as u64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        let vertexRes = self.cp_create_committed_resource(
            &heapProperties,
            D3D12_HEAP_FLAG_NONE,
            &resourceDesc,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            &None, vertices)?;

        let vbView = D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: unsafe { vertexRes.value.GetGPUVirtualAddress() },
            SizeInBytes: std::mem::size_of_val(vertexRes.data.as_ref()) as u32,
            StrideInBytes: std::mem::size_of_val(vertexRes.data.first().ok_or(S_OK)?) as u32,
        };
        Ok((vertexRes, vbView))
    }
    pub fn cp_create_index_resource(&self, nodemask: u32, indices: Box<[u32]>) -> Result<(CpID3D12Resource<Box<[u32]>>, D3D12_INDEX_BUFFER_VIEW), HRESULT> {
        let heapProperties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: nodemask,
            VisibleNodeMask: nodemask,
        };
        let resourceDesc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: std::mem::size_of_val(indices.as_ref()) as u64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };
        let indexRes = self.cp_create_committed_resource(
            &heapProperties,
            D3D12_HEAP_FLAG_NONE,
            &resourceDesc,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            &None, indices)?;

        let idView = D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: unsafe { indexRes.value.GetGPUVirtualAddress() },
            SizeInBytes: std::mem::size_of_val(indexRes.data.as_ref()) as u32,
            Format: DXGI_FORMAT_R32_UINT,
        };
        Ok((indexRes, idView))
    }
    pub fn cp_create_root_signature(&self, nodeMask: u32, cpid3dblob: &CpID3DBlob) -> Result<CpID3D12RootSignature, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateRootSignature(nodeMask, cpid3dblob.cp_get_buffer_pointer(), cpid3dblob.cp_get_buffer_size(), &ID3D12RootSignature::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12RootSignature).as_mut() {
                        Some(_ID3D12RootSignature) => {
                            return Ok(CpID3D12RootSignature(_ID3D12RootSignature)); }

                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_graphics_pipeline_state(&self, d3d12_graphics_pipeline_state_desc: &mut D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> Result<CpID3D12PipelineState, HRESULT> {
        ///todo:inputElementDescのアドレスがおかしくなる。現在ここに書いているけど将来的には分ける。
        let inputElementDesc = [
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: CString::new("POSITION").expect("CString::new failed").into_raw(),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            }
        ];
        d3d12_graphics_pipeline_state_desc.InputLayout = D3D12_INPUT_LAYOUT_DESC { pInputElementDescs: inputElementDesc.as_ptr(), NumElements: inputElementDesc.len() as u32 };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateGraphicsPipelineState(d3d12_graphics_pipeline_state_desc, &ID3D12PipelineState::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12PipelineState).as_mut() {
                        Some(_ID3D12PipelineState) => {
                            return Ok(CpID3D12PipelineState(_ID3D12PipelineState));
                        }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
}

impl CpIDXGIFactory6 {
    pub fn new() -> CpIDXGIFactory6 {
        match CpIDXGIFactory6::cp_create_dxgifactory2_result() {
            Ok(v) => return v,
            Err(v) => panic!("{}", v)
        }
    }
    fn cp_create_dxgifactory2_result() -> Result<CpIDXGIFactory6, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG, &IDXGIFactory6::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *const IDXGIFactory6).as_ref() {
                        Some(_dxgi_factory) => return Ok(CpIDXGIFactory6(std::mem::transmute(_dxgi_factory))),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_swap_chain_for_hwnd(&self, _que: &mut CpID3D12CommandQueue, hwnd: &mut CpHWND, dxgi_swap_chain_desc1_opt: Option<DXGI_SWAP_CHAIN_DESC1>) -> Result<CpIDXGISwapChain4, HRESULT> {
        let dxgi_swap_chain_desc1 = match dxgi_swap_chain_desc1_opt {
            Some(v) => { v }
            None => {
                DXGI_SWAP_CHAIN_DESC1 {
                    Width: 0,
                    Height: 0,
                    Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    Stereo: i32::from(false),
                    SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                    BufferUsage: DXGI_USAGE_BACK_BUFFER,
                    BufferCount: 2,
                    Scaling: DXGI_SCALING_STRETCH,
                    SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                    AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
                    Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
                }
            }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateSwapChainForHwnd(_que.value.as_mut() as *mut ID3D12CommandQueue as *mut IUnknown, hwnd.0, &dxgi_swap_chain_desc1, null_mut(), null_mut(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut IDXGISwapChain4).as_ref() {
                        Some(_dxgi_swap_chain4) => { return Ok(CpIDXGISwapChain4 { value: _dxgi_swap_chain4, desc: dxgi_swap_chain_desc1 }); }
                        None => { return Err(v); }
                    };
                }
                Err(v) => return Err(v)
            }
        }
    }
}

impl CpMSG {
    ///ピークメッセージをを含んだMSG構造体を返す
    pub fn cp_peek_message_w(hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT) -> CpMSG {
        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        match unsafe { PeekMessageW(&mut msg, hWnd, wMsgFilterMin, wMsgFilterMax, wRemoveMsg).BOOLtobool() } {
            true => return CpMSG { value: msg, hasMessage: true },
            false => return CpMSG { value: msg, hasMessage: false }
        }
    }
    pub fn cp_translate_message(&self) -> bool {
        unsafe { return TranslateMessage(&self.value).BOOLtobool(); };
    }
    pub fn cp_dispatch_message_w(&self) {
        unsafe { DispatchMessageW(&self.value) };
    }
    pub fn cp_has_wm_quit_message(&self) -> bool {
        self.value.message == WM_QUIT
    }
}

impl CpD3D12_GRAPHICS_PIPELINE_STATE_DESC {
    /// D3D12_GRAPHICS_PIPELINE_STATE_DESCを作るための関数。初期化にいくらか追加しただけの関数なのでいろいろ後で設定しよう
    pub fn create_d3d12_graphics_pipeline_state_desc(vsBlob: &CpID3DBlob, psBlob: &CpID3DBlob, d3d12_input_element_descs: Box<[D3D12_INPUT_ELEMENT_DESC]>, cp_id3d12root_signature: &mut CpID3D12RootSignature, ds_blob_opt: Option<&CpID3DBlob>, hs_blob_opt: Option<&CpID3DBlob>, gs_blob_opt: Option<&CpID3DBlob>) -> D3D12_GRAPHICS_PIPELINE_STATE_DESC {
        let mut d3d12_graphics_pipeline_state_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            VS: D3D12_SHADER_BYTECODE { pShaderBytecode: vsBlob.cp_get_buffer_pointer(), BytecodeLength: vsBlob.cp_get_buffer_size() },
            PS: D3D12_SHADER_BYTECODE { pShaderBytecode: psBlob.cp_get_buffer_pointer(), BytecodeLength: psBlob.cp_get_buffer_size() },
            InputLayout: D3D12_INPUT_LAYOUT_DESC { pInputElementDescs: d3d12_input_element_descs.as_ptr(), NumElements: d3d12_input_element_descs.len() as u32 },
            pRootSignature: cp_id3d12root_signature.0,
            ..CpD3D12_GRAPHICS_PIPELINE_STATE_DESC::default().0
        };
        if let Some(ds_blob) = ds_blob_opt {
            d3d12_graphics_pipeline_state_desc.DS = D3D12_SHADER_BYTECODE { pShaderBytecode: ds_blob.cp_get_buffer_pointer(), BytecodeLength: ds_blob.cp_get_buffer_size() }
        }
        if let Some(hs_blob) = hs_blob_opt {
            d3d12_graphics_pipeline_state_desc.HS = D3D12_SHADER_BYTECODE { pShaderBytecode: hs_blob.cp_get_buffer_pointer(), BytecodeLength: hs_blob.cp_get_buffer_size() }
        }
        if let Some(gs_blob) = gs_blob_opt {
            d3d12_graphics_pipeline_state_desc.GS = D3D12_SHADER_BYTECODE { pShaderBytecode: gs_blob.cp_get_buffer_pointer(), BytecodeLength: gs_blob.cp_get_buffer_size() }
        }
        d3d12_graphics_pipeline_state_desc
    }
}

impl CpD3D12_ROOT_SIGNATURE_DESC {
    pub fn cp_d3d12serialize_root_signature<'a>(&self, version: D3D_ROOT_SIGNATURE_VERSION) -> Result<CpID3DBlob<'a>, (CpID3DBlob<'a>, HRESULT)> {
        let mut okBlob: *mut ID3D10Blob = null_mut();
        let mut errBlob: *mut ID3D10Blob = null_mut();
        unsafe {
            match D3D12SerializeRootSignature(&self.0, version, &mut okBlob, &mut errBlob).hresult_to_result() {
                Ok(_) => return Ok(CpID3DBlob(okBlob.as_mut().unwrap())),
                Err(v) => return Err((CpID3DBlob(errBlob.as_mut().unwrap()), v))
            }
        }
    }
}

pub enum CpD3d12ResourceBarrierDescType {
    CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier: D3D12_RESOURCE_TRANSITION_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS },
    CpD3d12ResourceAliasingBarrier { d3d12_resource_aliasing_barrier: D3D12_RESOURCE_ALIASING_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS },
    CpD3D12_RESOURCE_UAV_BARRIER { d3d12_resource_uav_barrier: D3D12_RESOURCE_UAV_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS },
}

impl<'a> CpD3D12_RESOURCE_BARRIER<'a> {
    ///リソースバリアのタイプで返すD3D12_RESOURCE_BARRIER構造体の共用体部分を決める
    pub fn new(desc_type: CpD3d12ResourceBarrierDescType) -> D3D12_RESOURCE_BARRIER {
        match desc_type {
            CpD3d12ResourceBarrierDescType::CpD3d12ResourceTransitionBarrier { d3d12_resource_transition_barrier, flags } => {
                CpD3D12_RESOURCE_BARRIER::cp_transition(&d3d12_resource_transition_barrier, flags)
            }
            CpD3d12ResourceBarrierDescType::CpD3d12ResourceAliasingBarrier { d3d12_resource_aliasing_barrier, flags } => {
                CpD3D12_RESOURCE_BARRIER::cp_aliasing(&d3d12_resource_aliasing_barrier, flags)
            }
            CpD3d12ResourceBarrierDescType::CpD3D12_RESOURCE_UAV_BARRIER { d3d12_resource_uav_barrier, flags } => {
                CpD3D12_RESOURCE_BARRIER::cp_uav(&d3d12_resource_uav_barrier, flags)
            }
        }
    }
    pub fn cp_transition(d3d12_resource_transition_barrier: &D3D12_RESOURCE_TRANSITION_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER
        {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: flags,
            u: unsafe {
                *std::mem::transmute::<&D3D12_RESOURCE_TRANSITION_BARRIER, &D3D12_RESOURCE_BARRIER_u>(d3d12_resource_transition_barrier)
            },
        }
    }
    pub fn cp_aliasing(d3d12_resource_aliasing_barrier: &D3D12_RESOURCE_ALIASING_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER
        {
            Type: D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
            Flags: flags,
            u: unsafe {
                *std::mem::transmute::<&D3D12_RESOURCE_ALIASING_BARRIER, &D3D12_RESOURCE_BARRIER_u>(d3d12_resource_aliasing_barrier)
            },
        }
    }
    pub fn cp_uav(d3d12_resource_uav_barrier: &D3D12_RESOURCE_UAV_BARRIER, flags: D3D12_RESOURCE_BARRIER_FLAGS) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER
        {
            Type: D3D12_RESOURCE_BARRIER_TYPE_UAV,
            Flags: flags,
            u: unsafe {
                *std::mem::transmute::<&D3D12_RESOURCE_UAV_BARRIER, &D3D12_RESOURCE_BARRIER_u>(d3d12_resource_uav_barrier)
            },
        }
    }
}

impl<'a> CpHWND<'a> {
    pub fn new(mut _wndclassexw_opt: Option<WNDCLASSEXW>, mut window_rc_opt: Option<RECT>) -> CpHWND<'a> {
        let mut _wndclassexw = match _wndclassexw_opt {
            Some(v) => { v }
            None => {
                extern "system" fn window_procedure(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
                    match msg {
                        WM_DESTROY => unsafe { PostQuitMessage(0) },
                        _ => return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
                    };
                    return 0;
                }
                WNDCLASSEXW {
                    cbSize: std::mem::size_of::<WNDCLASSEXW>() as UINT,
                    style: CS_OWNDC,
                    lpfnWndProc: Some(window_procedure),
                    cbClsExtra: 0,
                    cbWndExtra: 0,
                    hInstance: unsafe { GetModuleHandleW(null_mut()) as HINSTANCE },
                    hIcon: null_mut(),
                    hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
                    hbrBackground: null_mut(),
                    lpszMenuName: null_mut(),
                    lpszClassName: to_wide_chars("DX12Sample").as_ptr(),
                    hIconSm: 0 as HICON,
                }
            }
        };
        let mut window_rc = match window_rc_opt {
            Some(v) => { v }
            None => {
                const WINDOW_WIDTH: u32 = 720;
                const WINDOW_HEIGHT: u32 = 480;
                RECT { left: 0, top: 0, right: WINDOW_WIDTH as i32, bottom: WINDOW_HEIGHT as i32 }
            }
        };
        CpHWND::cp_adjust_window_rect_ex(window_rc);
        match CpHWND::cp_create_window_ex_w_result(_wndclassexw, window_rc) {
            Ok(v) => return v,
            Err(v) => panic!("{}", v)
        }
    }
    fn cp_adjust_window_rect_ex(mut window_rc: RECT) {
        unsafe { AdjustWindowRectEx(&mut window_rc, WS_OVERLAPPEDWINDOW, FALSE, 0); }
    }
    fn cp_create_window_ex_w_result(_wndclassexw: WNDCLASSEXW, window_rc: RECT) -> Result<CpHWND<'a>, Error> {
        match unsafe {
            //従来手法ではlpClassNameはRegisterClassExWと別だったが、RegisterClassExWの戻り値ATOMをLPCWSTRにキャストするとエラーコード1407での失敗が無くなる。
            CreateWindowExW(0, RegisterClassExW(&_wndclassexw) as LPCWSTR,
                            to_wide_chars("DX12inRust").as_ptr(),
                            //通常、WS_VISIBLEは子ウインドウを作成する際に指定します。
                            WS_OVERLAPPEDWINDOW,
                            CW_USEDEFAULT,
                            CW_USEDEFAULT,
                            window_rc.right - window_rc.left,
                            window_rc.bottom - window_rc.top,
                            null_mut(),
                            null_mut(),
                            _wndclassexw.hInstance,
                            null_mut()).as_mut()
        } {
            Some(v) => Ok(CpHWND(v, _wndclassexw)),
            None => Err(Error::last_os_error())
        }
    }
    pub fn cp_unregister_class_w(&self) {
        unsafe { UnregisterClassW(self.1.lpszClassName, self.1.hInstance); }
    }
}

///CpID3D12Resourceに渡す構造体はCloneトレイトを実装している必要がある
impl<'a, T: std::clone::Clone + Debug> CpID3D12Resource<'a, T> {
    pub fn cp_map<S>(&self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>) -> Result<&'a mut S, HRESULT> {
        let pReadRange: *const D3D12_RANGE = match pReadRangeOpt {
            Some(v) => { &v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.value.Map(subresource, pReadRange, &mut _unknownobj).hresult_to_result() {
                Ok(v) => match (_unknownobj as *mut S).as_mut() {
                    None => { Err(v) }
                    Some(_obj) => {
                        Ok(_obj)
                    }
                }
                Err(v) => Err(v)
            }
        }
    }
    ///内部でmapを呼び出すことで事前にmapをしなくても良くなった。CpID3D12Resourceを作る際に作ったデータをGPUにコピーする時はcopydataOptはNullを入れてね
    ///todo:ここcopyが正常にできないバグがある
    pub fn cp_copy(&mut self, copydataOpt: Option<&T>, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>) -> Result<HRESULT, HRESULT> {
        let copydata = match copydataOpt {
            Some(v) => { v }
            None => { &self.data }
        };
        match &mut self.destdata {
            Some(v) => {
                (**v).clone_from(copydata);
            }
            None => {
                let mut dest = self.cp_map::<T>(subresource, pReadRangeOpt)?;
                dest.clone_from(copydata);
                self.destdata = Some(dest);
            }
        };
        Ok(S_OK)
    }
    pub fn cp_unmap(&self, subresource: UINT, pReadRangeOpt: &Option<D3D12_RANGE>) {
        let pReadRange: *const D3D12_RANGE = match pReadRangeOpt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            self.value.Unmap(subresource, pReadRange)
        }
    }
}

impl<'a, T: std::clone::Clone + Debug> CpID3D12Resource<'a, T> {
    pub fn cp_slice_map<S>(&self, subresource: UINT, pReadRangeOpt: Option<D3D12_RANGE>, len: impl std::iter::ExactSizeIterator) -> Result<&mut [S], HRESULT> {
        let _arr_obj = self.cp_map::<S>(subresource, pReadRangeOpt)?;
        let _arr = unsafe { std::slice::from_raw_parts_mut(_arr_obj, len.len()) };
        Ok(_arr)
    }
}

impl<'a> CpIDXGISwapChain4<'a> {
    pub fn cp_get_desc1(&self) -> Result<DXGI_SWAP_CHAIN_DESC1, HRESULT> {
        unsafe {
            let mut dxgi_swap_chain_desc1: DXGI_SWAP_CHAIN_DESC1 = DXGI_SWAP_CHAIN_DESC1 {
                Width: 0,
                Height: 0,
                Format: 0,
                Stereo: 0,
                SampleDesc: DXGI_SAMPLE_DESC { Count: 0, Quality: 0 },
                BufferUsage: 0,
                BufferCount: 0,
                Scaling: 0,
                SwapEffect: 0,
                AlphaMode: 0,
                Flags: 0,
            };
            match self.value.GetDesc1(&mut dxgi_swap_chain_desc1).hresult_to_result() {
                Ok(_) => return Ok(dxgi_swap_chain_desc1),
                Err(v) => Err(v)
            }
        }
    }
    pub fn cp_get_buffer(&self, buffer: UINT) -> Result<CpID3D12Resource<UINT>, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.value.GetBuffer(buffer, &ID3D12Resource::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12Resource).as_mut() {
                        Some(id3d12resource) => { return Ok(CpID3D12Resource { value: id3d12resource, data: buffer, destdata: None }); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_get_current_back_buffer_index(&self) -> UINT {
        unsafe {
            return self.value.GetCurrentBackBufferIndex();
        }
    }
}

impl CpID3D12CommandAllocator {
    pub fn cp_reset(&self) -> Result<HRESULT, HRESULT> {
        unsafe {
            return self.0.Reset().hresult_to_result();
        }
    }
}

impl CpID3D12GraphicsCommandList {
    pub fn cp_reset(&self, cp_id3d12command_allocator: &mut CpID3D12CommandAllocator, p_initial_state_opt: &mut Option<ID3D12PipelineState>) -> Result<HRESULT, HRESULT> {
        let p_initial_state: *mut ID3D12PipelineState = match p_initial_state_opt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            return self.0.Reset(cp_id3d12command_allocator.0.as_mut(), p_initial_state).hresult_to_result();
        }
    }
    pub fn cp_resource_barrier(&self, _D3D12_RESOURCE_BARRIER: Vec<D3D12_RESOURCE_BARRIER>) {
        unsafe {
            self.0.ResourceBarrier(_D3D12_RESOURCE_BARRIER.len() as u32, _D3D12_RESOURCE_BARRIER.as_ptr())
        }
    }
}

impl<'a> CpID3D12CommandDispacher<'a> {
    pub fn cp_list_reset(&mut self, index: usize, p_initial_state_opt: &mut Option<ID3D12PipelineState>) -> Result<HRESULT, HRESULT> {
        self.command_lists[index].cp_reset(&mut self.command_allocator, p_initial_state_opt)
    }
    pub fn cp_list_allreset(&mut self, p_initial_state_opt: &mut Option<ID3D12PipelineState>) {
        for command_list in &self.command_lists {
            command_list.cp_reset(&mut self.command_allocator, p_initial_state_opt);
        }
    }
    pub fn cp_reset(&mut self, p_initial_state_opt: &mut Option<ID3D12PipelineState>) {
        self.command_allocator.cp_reset();
        self.cp_list_allreset(p_initial_state_opt);
    }
    pub fn cp_execute_command_lists(&mut self) {
        self.command_queue.cp_execute_command_lists(&mut self.command_lists)
    }
}

impl CpID3D12CommandQueue {
    pub fn cp_execute_command_lists(&self, cp_ID3D12CommandLists: &mut Vec<CpID3D12GraphicsCommandList>) {
        let NumCommandLists: u32 = cp_ID3D12CommandLists.len() as u32;
        let ppCommandLists = cp_ID3D12CommandLists.as_ptr() as *const *mut ID3D12CommandList;
        unsafe {
            self.value.ExecuteCommandLists(NumCommandLists, ppCommandLists);
        }
    }
    pub fn cp_signal(&self, cp_id3d12fence: &mut CpID3D12Fence) -> Result<HRESULT, HRESULT> {
        unsafe {
            self.value.Signal(cp_id3d12fence.value, cp_id3d12fence.fenceval).hresult_to_result()
        }
    }
}

impl<'a> CpID3DBlob<'a> {
    pub fn cp_d3dcompile_from_file(pFileName: &str, pDefinesOpt: Option<&D3D_SHADER_MACRO>, pInclude: *mut ID3DInclude, pEntrypoint: &str, pTarget: &str, Flags1: UINT, Flags2: UINT) -> Result<CpID3DBlob<'a>, (CpID3DBlob<'a>, HRESULT)> {
        let mut okBlob: *mut ID3D10Blob = null_mut();
        let mut errBlob: *mut ID3D10Blob = null_mut();
        let pDefines: *const D3D_SHADER_MACRO = match pDefinesOpt {
            Some(v) => { v }
            None => { null_mut() }
        };
        let CstrpEntrypoint = CString::new(pEntrypoint).expect("CString::new failed");
        let CstrpTarget = CString::new(pTarget).expect("CString::new failed");

        unsafe {
            match D3DCompileFromFile(to_wide_chars(pFileName).as_ptr(), pDefines, pInclude, CstrpEntrypoint.as_ptr(), CstrpTarget.as_ptr(), Flags1, Flags2, &mut okBlob, &mut errBlob).hresult_to_result() {
                Ok(_) => {
                    Ok(CpID3DBlob(okBlob.as_mut().unwrap()))
                }
                Err(v) => {
                    Err((CpID3DBlob(errBlob.as_mut().unwrap()), v))
                }
            }
        }
    }
    pub fn cp_get_buffer_pointer(&self) -> &'a mut c_void {
        unsafe {
            self.0.GetBufferPointer().as_mut().unwrap()
        }
    }
    pub fn cp_get_buffer_size(&self) -> usize {
        unsafe {
            self.0.GetBufferSize()
        }
    }
}

impl<'a> CpID3D12Fence<'a> {
    pub fn cp_get_completed_value(&self) -> u64 {
        unsafe {
            self.value.GetCompletedValue()
        }
    }
    pub fn cp_is_reach_fance_value(&self) -> bool {
        self.cp_get_completed_value() >= self.fenceval
    }
    pub fn cp_set_event_on_completion(&self, hEvent: HANDLE) -> Result<HRESULT, HRESULT> {
        unsafe {
            self.value.SetEventOnCompletion(self.fenceval, hEvent).hresult_to_result()
        }
    }
    pub fn cp_increment_counter(&mut self, incrementvalue: u64) {
        self.fenceval += incrementvalue;
    }
}

impl CpID3D12DescriptorHeap {
    pub fn cp_get_cpudescriptor_handle_for_heap_start(&self) -> CpD3D12_CPU_DESCRIPTOR_HANDLE {
        let descripter_heap = unsafe {
            self.value.GetCPUDescriptorHandleForHeapStart()
        };
        return CpD3D12_CPU_DESCRIPTOR_HANDLE {
            value: descripter_heap,
            DescriptorHeapType: self.desc.Type,
        };
    }
}

struct CpD12baseApp<'a> {
    _id3d12device: &'a ID3D12Device,
    _dxgi_factory: &'a IDXGIFactory6,
    hwnd: &'a mut HWND__,
}

struct CpD12baseAppOpt<'a> {
    _id3d12device: Option<&'a ID3D12Device>,
    _dxgi_factory: Option<&'a IDXGIFactory6>,
    hwnd: Option<&'a HWND__>,
}