mod Cp_directx12;

extern crate winapi;

use std::io::Error;

const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32 = 480;


use winapi::um::winnt::{HRESULT, LPCWSTR};
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM, HINSTANCE, FALSE};
use winapi::shared::windef::{HICON, HWND, RECT, HWND__, POINT};
use winapi::um::winuser::{MB_OK, MessageBoxW, WM_DESTROY, PostQuitMessage, WNDCLASSEXW, AdjustWindowRect, WS_OVERLAPPEDWINDOW, RegisterClassExW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, WS_VISIBLE, UnregisterClassW, LoadCursorW, IDC_ARROW, CS_OWNDC, AdjustWindowRectEx, ShowWindow, SW_SHOW, PeekMessageW, MSG, TranslateMessage, DispatchMessageW, WM_QUIT, PM_REMOVE, WS_OVERLAPPED};
use winapi::um::d3d12::{D3D12GetDebugInterface, ID3D12Device, D3D12CreateDevice, D3D12_COMMAND_LIST_TYPE_DIRECT, ID3D12CommandAllocator, ID3D12GraphicsCommandList, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_COMMAND_QUEUE_PRIORITY_NORMAL, ID3D12CommandQueue, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_DESCRIPTOR_HEAP_FLAG_NONE, ID3D12DescriptorHeap, ID3D12Resource, D3D12_CPU_DESCRIPTOR_HANDLE, ID3D12CommandList};
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
use crate::Cp_directx12::{to_wide_chars, CpID3D12Device, CpIDXGIFactory6};
use crate::Cp_directx12::CpHWND;

trait HRESULTChecker {
    fn hresult_to_result(self) -> Result<i32, i32>;
}

impl HRESULTChecker for HRESULT {
    fn hresult_to_result(self) -> Result<HRESULT, HRESULT> {
        match self {
            S_OK => Ok(self),
            _ => Err(self)
        }
    }
}

extern "system" fn window_procedure(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => unsafe { PostQuitMessage(0) },
        _ => return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    };
    return 0;
}

fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) } else { Ok(ret) }
}

struct Id3d12commandQueue(*mut c_void);

fn main() {
    let mut _id3d12debug = null_mut();
    let mut _id3d12command_allocator: Option<&mut ID3D12CommandAllocator> = None;
    let mut _id3d12graphics_command_list: Option<&mut ID3D12GraphicsCommandList> = None;
    let mut _id3d12_command_queue: Id3d12commandQueue = Id3d12commandQueue(null_mut());
    let mut _dxgi_swap_chain4: Option<&IDXGISwapChain4> = None;
    let mut _id3d12descripterheap_for_swapchain: Option<&ID3D12DescriptorHeap> = None;
    unsafe {
        if D3D12GetDebugInterface(&ID3D12Debug::uuidof(), &mut _id3d12debug) >= 0 {
            if let Some(deb) = (_id3d12debug as *mut ID3D12Debug).as_ref() {
                deb.EnableDebugLayer();
                deb.Release();
                println!("OKDebug!");
            }
        }
    }
    println!("last OS error: {:?}", Error::last_os_error());

    let mut hwnd = CpHWND::new(None, None);
    unsafe {
        let mut hwndtmp = &mut hwnd.0;
        ShowWindow(*hwndtmp, SW_SHOW);
    };
    let mut _id3d12device = CpID3D12Device::new();
    let mut _dxgi_factory = CpIDXGIFactory6::new();
    let mut _id3d12_command_queue = _id3d12device.cp_create_command_queue(None).unwrap_or_else(|v| { panic!("last OS error: {:?}", v) });
    let _dxgi_swap_chain4 = _dxgi_factory.cp_create_swap_chain_for_hwnd(&mut _id3d12_command_queue, &mut hwnd, None).unwrap_or_else(|v| { panic!("last OS error: {:?}", v) });
    let swapchain_view_number = _dxgi_swap_chain4.cp_get_desc1().unwrap_or_else(|v| { panic!("last OS error: {:?}", v) }).BufferCount;
    let heap_desc_for_swapchain = D3D12_DESCRIPTOR_HEAP_DESC {
        Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
        NumDescriptors: swapchain_view_number,
        Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
        NodeMask: 0,
    };
    let _id3d12descripterheap_for_swapchain = _id3d12device.cp_create_descriptor_heap(Some(heap_desc_for_swapchain)).unwrap_or_else(|v| { panic!("last OS error: {:?}", v) });
    //ID3D12ResourceはCPU と GPU の物理メモリまたはヒープへの一般的な読み書きの能力をカプセル化します。
    // シェーダサンプリング用に最適化された多次元データだけでなく、単純なデータの配列を整理して操作するための抽象化が含まれています。
    for index in (0..swapchain_view_number) {
        let mut _swap_res = _dxgi_swap_chain4.cp_get_buffer(index).unwrap_or_else(|v| { panic!("last OS error: {:?}", v) });
        let mut handle = _id3d12descripterheap_for_swapchain.cp_get_cpudescriptor_handle_for_heap_start();
        _id3d12device.cp_create_render_target_view(&mut _swap_res, None, handle.cp_descriptor_handle_increment_ptr(&_id3d12device, index));
    }
    let mut _id3d12command_allocator = _id3d12device.cp_create_command_allocator(D3D12_COMMAND_LIST_TYPE_DIRECT).unwrap_or_else(|v| { panic!("last OS error: {:?}", v) });
    let mut _id3d12graphics_command_list = _id3d12device.cp_create_command_list(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &mut _id3d12command_allocator, None).unwrap_or_else(|v| { panic!("last OS error: {:?}", v) });

    let lists = [_id3d12graphics_command_list.0];
    let _dev = &_id3d12device.0;
    loop {
        let mut msg = MSG {
            hwnd: null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        if unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessageW(&msg) };
        }
        if msg.message == WM_QUIT {
            break;
        }

        unsafe { _id3d12command_allocator.0.Reset() };
        unsafe { lists[0].Reset(_id3d12command_allocator.0, null_mut()) };

        let clearcolor: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
        let current_buff_index = unsafe { _dxgi_swap_chain4.0.GetCurrentBackBufferIndex() };
        let mut current_sw_heaps =  _id3d12descripterheap_for_swapchain.cp_get_cpudescriptor_handle_for_heap_start().cp_descriptor_handle_increment_ptr(&_id3d12device,current_buff_index);
        unsafe { lists[0].OMSetRenderTargets(1, &current_sw_heaps.value, i32::from(true), null_mut()) };
        unsafe { lists[0].ClearRenderTargetView(current_sw_heaps.value, &clearcolor, 0, null_mut()) }
        unsafe { lists[0].Close() };
        unsafe { _id3d12_command_queue.0.ExecuteCommandLists(1, lists.as_ptr() as *const *mut ID3D12CommandList) }

        unsafe { _dxgi_swap_chain4.0.Present(1, 0) };
    }
    println!("last OS error: {:?}", Error::last_os_error());
    if let Err(v) = print_message("Hello, world!") { println!("{}", v) }
    //unsafe { UnregisterClassW(_wndclassexw.lpszClassName, _wndclassexw.hInstance); }
    hwnd.cp_unregister_class_w();
}