mod Cp_directx12;
mod gltfimporttest;

use std::io::Error;
const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32 = 480;


use winapi::um::winnt::{HRESULT, LPCWSTR};
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM, HINSTANCE, FALSE, TRUE};
use winapi::shared::windef::{HICON, HWND, RECT, HWND__, POINT};
use winapi::um::winuser::{MB_OK, MessageBoxW, WM_DESTROY, PostQuitMessage, WNDCLASSEXW, AdjustWindowRect, WS_OVERLAPPEDWINDOW, RegisterClassExW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, WS_VISIBLE, UnregisterClassW, LoadCursorW, IDC_ARROW, CS_OWNDC, AdjustWindowRectEx, ShowWindow, SW_SHOW, PeekMessageW, MSG, TranslateMessage, DispatchMessageW, WM_QUIT, PM_REMOVE, WS_OVERLAPPED};
use winapi::um::d3d12::{D3D12GetDebugInterface, ID3D12Device, D3D12CreateDevice, D3D12_COMMAND_LIST_TYPE_DIRECT, ID3D12CommandAllocator, ID3D12GraphicsCommandList, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_COMMAND_QUEUE_PRIORITY_NORMAL, ID3D12CommandQueue, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_DESCRIPTOR_HEAP_FLAG_NONE, ID3D12DescriptorHeap, ID3D12Resource, D3D12_CPU_DESCRIPTOR_HANDLE, ID3D12CommandList, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_BARRIER_FLAG_NONE, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_STATE_PRESENT, D3D12_RESOURCE_ALIASING_BARRIER, D3D12_FENCE_FLAG_NONE};
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
use crate::Cp_directx12::{to_wide_chars, CpID3D12Device, CpIDXGIFactory6, CpMSG, CpD3D12_RESOURCE_BARRIER};
use crate::Cp_directx12::CpHWND;
use crate::gltfimporttest::gltfimport;
use crate::Cp_directx12::CpD3d12ResourceBarrierDescType::CpD3d12ResourceTransitionBarrier;
use winapi::um::synchapi::{CreateEventA, CreateEventExW, WaitForSingleObject};
use winapi::um::winbase::INFINITE;
use winapi::um::handleapi::CloseHandle;

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
    gltfimport();


    let mut _id3d12debug = null_mut();
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
    let swapchain_view_number = _dxgi_swap_chain4.desc.BufferCount;
    let heap_desc_for_swapchain = D3D12_DESCRIPTOR_HEAP_DESC {
        Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
        NumDescriptors: _dxgi_swap_chain4.desc.BufferCount,
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
    let mut _id3d12commanddispacher = _id3d12device.cp_create_command_dispacher(0, &_id3d12_command_queue, 1, None).unwrap_or_else(|v| { panic!("last OS error: {:?}", v) });
    let mut _id3d12fence = _id3d12device.cp_create_fence(1, D3D12_FENCE_FLAG_NONE).unwrap_or_else(|v| { panic!("last OS error: {:?}", Error::last_os_error()) });
    loop {
        let mut cpmsg = CpMSG::cp_peek_message_w(null_mut(),0,0,PM_REMOVE);
        if cpmsg.hasMessage{
            cpmsg.cp_translate_message();
            cpmsg.cp_dispatch_message_w();
        }
        if cpmsg.cp_has_wm_quit_message(){
            break;
        }
        let clearcolor: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
        let current_buff_index = _dxgi_swap_chain4.cp_get_current_back_buffer_index();
        let mut transition_barrier_desc = D3D12_RESOURCE_TRANSITION_BARRIER{
            pResource: _dxgi_swap_chain4.cp_get_buffer(current_buff_index).unwrap_or_else(|v| { panic!("last OS error: {:?}", v) }).value,
            Subresource: 0,
            StateBefore: D3D12_RESOURCE_STATE_PRESENT,
            StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET
        };
        let barrier_desc =CpD3D12_RESOURCE_BARRIER::new(CpD3d12ResourceTransitionBarrier{ d3d12_resource_transition_barrier: transition_barrier_desc, flags: 0 });
        unsafe { _id3d12commanddispacher.command_lists[0].cp_resource_barrier(vec![barrier_desc]) };

        let mut current_sw_heaps =  _id3d12descripterheap_for_swapchain.cp_get_cpudescriptor_handle_for_heap_start().cp_descriptor_handle_increment_ptr(&_id3d12device,current_buff_index);
        unsafe { _id3d12commanddispacher.command_lists[0].0.OMSetRenderTargets(1, &current_sw_heaps.value, i32::from(true), null_mut()) };
        unsafe { _id3d12commanddispacher.command_lists[0].0.ClearRenderTargetView(current_sw_heaps.value, &clearcolor, 0, null_mut()) }
        transition_barrier_desc.StateBefore = D3D12_RESOURCE_STATE_RENDER_TARGET;
        transition_barrier_desc.StateAfter = D3D12_RESOURCE_STATE_PRESENT;

        let barrier_desc =CpD3D12_RESOURCE_BARRIER::new(CpD3d12ResourceTransitionBarrier{ d3d12_resource_transition_barrier: transition_barrier_desc, flags: 0 });
        unsafe { _id3d12commanddispacher.command_lists[0].cp_resource_barrier(vec![barrier_desc]) };

        unsafe { _id3d12commanddispacher.command_lists[0].0.Close() };
        _id3d12commanddispacher.cp_execute_command_lists();

        _id3d12fence.cp_increment_counter(1);
        _id3d12_command_queue.cp_signal(&mut _id3d12fence);
        if (!_id3d12fence.cp_is_reach_fance_value()) {
            let event = unsafe{CreateEventA(null_mut(), FALSE, FALSE, null_mut())};
            _id3d12fence.cp_set_event_on_completion(event);
            unsafe{WaitForSingleObject(event, INFINITE)};
            unsafe{CloseHandle(event)};
        }

        _id3d12commanddispacher.cp_reset(&mut None);
        unsafe { _dxgi_swap_chain4.value.Present(1, 0) };
    }
    println!("last OS error: {:?}", Error::last_os_error());
    if let Err(v) = print_message("Hello, world!") { println!("{}", v) }
    //unsafe { UnregisterClassW(_wndclassexw.lpszClassName, _wndclassexw.hInstance); }
    hwnd.cp_unregister_class_w();
}