extern crate winapi;

use winapi::um::winnt::{HRESULT, LPCWSTR};
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM, HINSTANCE, FALSE, TRUE, BOOL};
use winapi::shared::windef::{HICON, HWND, RECT, HWND__, POINT};
use winapi::um::winuser::{MB_OK, MessageBoxW, WM_DESTROY, PostQuitMessage, WNDCLASSEXW, AdjustWindowRect, WS_OVERLAPPEDWINDOW, RegisterClassExW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, WS_VISIBLE, UnregisterClassW, LoadCursorW, IDC_ARROW, CS_OWNDC, AdjustWindowRectEx, ShowWindow, SW_SHOW, PeekMessageW, MSG, TranslateMessage, DispatchMessageW, WM_QUIT, PM_REMOVE, WS_OVERLAPPED};
use winapi::um::d3d12::{D3D12GetDebugInterface, ID3D12Device, D3D12CreateDevice, D3D12_COMMAND_LIST_TYPE_DIRECT, ID3D12CommandAllocator, ID3D12GraphicsCommandList, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_COMMAND_QUEUE_PRIORITY_NORMAL, ID3D12CommandQueue, ID3D12Pageable, ID3D12DeviceChild, ID3D12Object, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_DESCRIPTOR_HEAP_FLAG_NONE, ID3D12DescriptorHeap, ID3D12Resource, D3D12_CPU_DESCRIPTOR_HANDLE, ID3D12CommandList, D3D12_DESCRIPTOR_HEAP_TYPE, D3D12_RENDER_TARGET_VIEW_DESC, D3D12_COMMAND_LIST_TYPE, ID3D12PipelineState};
use winapi::um::d3d12sdklayers::{ID3D12Debug};
use winapi::shared::dxgi1_6::{IDXGIFactory6};
use winapi::shared::dxgi1_3::{CreateDXGIFactory2, DXGI_CREATE_FACTORY_DEBUG};
use winapi::shared::dxgi1_2::{DXGI_SWAP_CHAIN_DESC1, DXGI_SCALING_STRETCH, DXGI_ALPHA_MODE_UNSPECIFIED};
use winapi::shared::winerror::{S_OK};
use winapi::um::d3dcommon::{D3D_FEATURE_LEVEL_12_1};
use winapi::um::libloaderapi::{GetModuleHandleW};
use winapi::um::unknwnbase::{IUnknown};
use winapi::Interface;
use std::ptr::null_mut;
use winapi::shared::dxgi1_5::IDXGISwapChain4;
use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
use winapi::shared::dxgi::{DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_CHAIN_DESC};
use winapi::shared::dxgitype::{DXGI_USAGE_BACK_BUFFER, DXGI_SAMPLE_DESC};
use winapi::ctypes::c_void;
use std::io::Error;
use crate::Id3d12commandQueue;

pub struct CpID3D12Device<'a>(pub &'a ID3D12Device);

pub struct CpID3D12CommandQueue<'a> {
    pub(crate) value: &'a mut ID3D12CommandQueue,
    type_: D3D12_COMMAND_LIST_TYPE,
}

pub struct CpIDXGIFactory6<'a>(pub &'a IDXGIFactory6);

pub struct CpIDXGISwapChain4<'a> {
    pub value: &'a IDXGISwapChain4,
    pub desc: DXGI_SWAP_CHAIN_DESC1,
}

pub struct CpID3D12DescriptorHeap<'a> {
    value: &'a ID3D12DescriptorHeap,
    desc: D3D12_DESCRIPTOR_HEAP_DESC,
}

pub struct CpHWND<'a>(pub &'a mut HWND__, WNDCLASSEXW);

pub struct CpMSG {
    value: MSG,
    pub hasMessage: bool
}

pub struct CpID3D12Resource<'a>(pub &'a mut ID3D12Resource);

pub struct CpID3D12CommandAllocator<'a>(pub(crate) &'a mut ID3D12CommandAllocator);

pub struct CpID3D12GraphicsCommandList<'a>(pub &'a mut ID3D12GraphicsCommandList);

pub struct CpD3D12_CPU_DESCRIPTOR_HANDLE {
    pub(crate) value: D3D12_CPU_DESCRIPTOR_HANDLE,
    DescriptorHeapType: D3D12_DESCRIPTOR_HEAP_TYPE,
}

pub struct CpID3D12CommandDispacher<'a> {
    command_queue: &'a CpID3D12CommandQueue<'a>,
    pub command_allocator: CpID3D12CommandAllocator<'a>,
    pub command_lists: Vec<CpID3D12GraphicsCommandList<'a>>,
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

impl<'a> CpID3D12Device<'a> {
    pub fn new() -> CpID3D12Device<'a> {
        match CpID3D12Device::cp_d3d12create_device_result() {
            Ok(v) => return v,
            Err(v) => {
                println!("last OS error: {:?}", Error::last_os_error());
                panic!("{}", v);
            }
        }
    }
    fn cp_d3d12create_device_result() -> Result<CpID3D12Device<'a>, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match D3D12CreateDevice(null_mut(), D3D_FEATURE_LEVEL_12_1, &ID3D12Device::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    let mut _id3d12deviceOpt = (_unknownobj as *const ID3D12Device).as_ref();
                    match _id3d12deviceOpt {
                        Some(v) => return Ok(CpID3D12Device(v)),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            };
        }
    }
    pub fn cp_create_command_queue(&self, d3d12command_queue_desc_opt: Option<D3D12_COMMAND_QUEUE_DESC>) -> Result<CpID3D12CommandQueue<'a>, HRESULT> {
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
                        Some(v) => return Ok(CpID3D12CommandQueue { value: v, type_: d3d12command_queue_desc.Type }),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_descriptor_heap(&self, heap_desc_for_swapchain_opt: Option<D3D12_DESCRIPTOR_HEAP_DESC>) -> Result<CpID3D12DescriptorHeap<'a>, HRESULT> {
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
                        Some(_id3d12descripterheap_for_swapchain) => { return Ok(CpID3D12DescriptorHeap { value: _id3d12descripterheap_for_swapchain, desc: heap_desc_for_swapchain }); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_command_allocator(&self, type_: D3D12_COMMAND_LIST_TYPE) -> Result<CpID3D12CommandAllocator<'a>, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateCommandAllocator(type_, &ID3D12CommandAllocator::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12CommandAllocator).as_mut() {
                        Some(_id3d12command_allocator) => { return Ok(CpID3D12CommandAllocator(_id3d12command_allocator)); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            };
        }
    }
    pub fn cp_create_command_list(&self, node_mask: UINT, type_: D3D12_COMMAND_LIST_TYPE, p_command_allocator: &mut CpID3D12CommandAllocator, p_initial_state_opt: &mut Option<ID3D12PipelineState>) -> Result<CpID3D12GraphicsCommandList<'a>, HRESULT> {
        let p_initial_state: *mut ID3D12PipelineState = match p_initial_state_opt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            let mut _unknownobj = null_mut();
            match self.0.CreateCommandList(node_mask, type_, p_command_allocator.0, p_initial_state, &ID3D12GraphicsCommandList::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12GraphicsCommandList).as_mut() {
                        Some(_id3d12graphics_command_list) => { return Ok(CpID3D12GraphicsCommandList(_id3d12graphics_command_list)); }
                        None => { return Err(v); }
                    }
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_command_dispacher(&self, node_mask: UINT, cp_id3d12command_queue: &'a CpID3D12CommandQueue<'a>, listnum: u32, mut p_initial_state_opt: Option<ID3D12PipelineState>) -> Result<CpID3D12CommandDispacher<'a>, HRESULT> {
        let mut _id3d12command_allocator = self.cp_create_command_allocator(cp_id3d12command_queue.type_).unwrap_or_else(|v| { panic!("{}", v) });
        let mut command_lists = (0..listnum).map(|index| -> CpID3D12GraphicsCommandList {
            self.cp_create_command_list(node_mask, cp_id3d12command_queue.type_, &mut _id3d12command_allocator, &mut p_initial_state_opt).unwrap_or_else(|v| { panic!("{}", v) })
        }).collect();
        return Ok(CpID3D12CommandDispacher {
            command_queue: &cp_id3d12command_queue,
            command_allocator: _id3d12command_allocator,
            command_lists: command_lists,
        });
    }
    pub fn cp_get_descriptor_handle_increment_size(&self, DescriptorHeapType: D3D12_DESCRIPTOR_HEAP_TYPE) -> UINT {
        unsafe { self.0.GetDescriptorHandleIncrementSize(DescriptorHeapType) }
    }
    pub fn cp_create_render_target_view(&self, pResource: &mut CpID3D12Resource, pDesc_opt: Option<D3D12_RENDER_TARGET_VIEW_DESC>, DestDescriptor: CpD3D12_CPU_DESCRIPTOR_HANDLE) {
        let pDesc: *const D3D12_RENDER_TARGET_VIEW_DESC = match pDesc_opt {
            Some(v) => { &v }
            None => { null_mut() }
        };
        unsafe { self.0.CreateRenderTargetView(pResource.0, pDesc, DestDescriptor.value) }
    }
}

impl<'a> CpIDXGIFactory6<'a> {
    pub fn new() -> CpIDXGIFactory6<'a> {
        match CpIDXGIFactory6::cp_create_dxgifactory2_result() {
            Ok(v) => return v,
            Err(v) => panic!("{}", v)
        }
    }
    fn cp_create_dxgifactory2_result() -> Result<CpIDXGIFactory6<'a>, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG, &IDXGIFactory6::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *const IDXGIFactory6).as_ref() {
                        Some(_dxgi_factory) => return Ok(CpIDXGIFactory6(_dxgi_factory)),
                        None => return Err(v)
                    };
                }
                Err(v) => return Err(v)
            }
        }
    }
    pub fn cp_create_swap_chain_for_hwnd(&self, _que: &mut CpID3D12CommandQueue, hwnd: &mut CpHWND, dxgi_swap_chain_desc1_opt: Option<DXGI_SWAP_CHAIN_DESC1>) -> Result<CpIDXGISwapChain4<'a>, HRESULT> {
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
            match self.0.CreateSwapChainForHwnd(_que.value as *mut ID3D12CommandQueue as *mut IUnknown, hwnd.0, &dxgi_swap_chain_desc1, null_mut(), null_mut(), &mut _unknownobj).hresult_to_result() {
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
    pub fn cp_get_buffer(&self, buffer: UINT) -> Result<CpID3D12Resource, HRESULT> {
        unsafe {
            let mut _unknownobj = null_mut();
            match self.value.GetBuffer(buffer, &ID3D12Resource::uuidof(), &mut _unknownobj).hresult_to_result() {
                Ok(v) => {
                    match (_unknownobj as *mut ID3D12Resource).as_mut() {
                        Some(id3d12resource) => { return Ok(CpID3D12Resource(id3d12resource)); }
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

impl<'a> CpID3D12CommandAllocator<'a> {
    pub fn cp_reset(&self) -> Result<HRESULT, HRESULT> {
        unsafe {
            return self.0.Reset().hresult_to_result();
        }
    }
}

impl<'a> CpID3D12GraphicsCommandList<'a> {
    pub fn cp_reset(&self, cp_id3d12command_allocator: &mut CpID3D12CommandAllocator, p_initial_state_opt: &mut Option<ID3D12PipelineState>) -> Result<HRESULT, HRESULT> {
        let p_initial_state: *mut ID3D12PipelineState = match p_initial_state_opt {
            Some(v) => { v }
            None => { null_mut() }
        };
        unsafe {
            return self.0.Reset(cp_id3d12command_allocator.0, p_initial_state).hresult_to_result();
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

impl<'a> CpID3D12CommandQueue<'a> {
    pub fn cp_execute_command_lists(&self, cp_ID3D12CommandLists: &mut Vec<CpID3D12GraphicsCommandList>) {
        let NumCommandLists: u32 = cp_ID3D12CommandLists.len() as u32;
        let ppCommandLists = cp_ID3D12CommandLists.as_ptr() as *const *mut ID3D12CommandList;
        unsafe {
            self.value.ExecuteCommandLists(NumCommandLists, ppCommandLists);
        }
    }
}

impl<'a> CpID3D12Resource<'a> {}

impl<'a> CpID3D12DescriptorHeap<'a> {
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