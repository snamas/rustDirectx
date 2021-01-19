
use Default;
use winapi::um::d3d12::{D3D12_GRAPHICS_PIPELINE_STATE_DESC, D3D12_SHADER_BYTECODE, D3D12_BLEND_DESC, D3D12_STREAM_OUTPUT_DESC, D3D12_RASTERIZER_DESC, D3D12_DEPTH_STENCIL_DESC, D3D12_DEPTH_STENCILOP_DESC, D3D12_INPUT_LAYOUT_DESC, D3D12_CACHED_PIPELINE_STATE, D3D12_RENDER_TARGET_BLEND_DESC, D3D12_COLOR_WRITE_ENABLE_ALL, D3D12_FILL_MODE_SOLID, D3D12_CULL_MODE_NONE, D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED, D3D12_DEFAULT_SAMPLE_MASK, D3D12_ROOT_SIGNATURE_DESC};
use winapi::shared::dxgitype::DXGI_SAMPLE_DESC;
use winapi::_core::ptr::{null_mut, null};
use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
use winapi::_core::ops::Deref;

pub struct CpD3D12_GRAPHICS_PIPELINE_STATE_DESC(pub(crate) D3D12_GRAPHICS_PIPELINE_STATE_DESC);
pub struct CpD3D12_ROOT_SIGNATURE_DESC(pub(crate) D3D12_ROOT_SIGNATURE_DESC);
impl Default for CpD3D12_GRAPHICS_PIPELINE_STATE_DESC{
    fn default() -> Self {
        let d3d12_graphics_pipeline_state_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC{
            pRootSignature: null_mut(),
            VS: D3D12_SHADER_BYTECODE { pShaderBytecode: null(), BytecodeLength: 0 },
            PS: D3D12_SHADER_BYTECODE { pShaderBytecode: null(), BytecodeLength: 0 },
            DS: D3D12_SHADER_BYTECODE { pShaderBytecode: null(), BytecodeLength: 0 },
            HS: D3D12_SHADER_BYTECODE { pShaderBytecode: null(), BytecodeLength: 0 },
            GS: D3D12_SHADER_BYTECODE { pShaderBytecode: null(), BytecodeLength: 0 },
            StreamOutput: D3D12_STREAM_OUTPUT_DESC {
                pSODeclaration: null(),
                NumEntries: 0,
                pBufferStrides: null(),
                NumStrides: 0,
                RasterizedStream: 0
            },
            BlendState: D3D12_BLEND_DESC {
                AlphaToCoverageEnable: FALSE,
                IndependentBlendEnable: FALSE,
                RenderTarget: [D3D12_RENDER_TARGET_BLEND_DESC{
                    BlendEnable: FALSE,
                    LogicOpEnable: FALSE,
                    SrcBlend: 0,
                    DestBlend: 0,
                    BlendOp: 0,
                    SrcBlendAlpha: 0,
                    DestBlendAlpha: 0,
                    BlendOpAlpha: 0,
                    LogicOp: 0,
                    RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL as u8
                };8]
            },
            SampleMask: D3D12_DEFAULT_SAMPLE_MASK,
            RasterizerState: D3D12_RASTERIZER_DESC {
                FillMode: D3D12_FILL_MODE_SOLID,
                CullMode: D3D12_CULL_MODE_NONE,
                FrontCounterClockwise: 0,
                DepthBias: 0,
                DepthBiasClamp: 0.0,
                SlopeScaledDepthBias: 0.0,
                DepthClipEnable: TRUE,
                MultisampleEnable: 0,
                AntialiasedLineEnable: 0,
                ForcedSampleCount: 0,
                ConservativeRaster: 0
            },
            DepthStencilState: D3D12_DEPTH_STENCIL_DESC {
                DepthEnable: 0,
                DepthWriteMask: 0,
                DepthFunc: 0,
                StencilEnable: 0,
                StencilReadMask: 0,
                StencilWriteMask: 0,
                FrontFace: D3D12_DEPTH_STENCILOP_DESC {
                    StencilFailOp: 0,
                    StencilDepthFailOp: 0,
                    StencilPassOp: 0,
                    StencilFunc: 0
                },
                BackFace: D3D12_DEPTH_STENCILOP_DESC {
                    StencilFailOp: 0,
                    StencilDepthFailOp: 0,
                    StencilPassOp: 0,
                    StencilFunc: 0
                }
            },
            InputLayout: D3D12_INPUT_LAYOUT_DESC { pInputElementDescs: null(), NumElements: 0 },
            IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED,
            PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            NumRenderTargets: 1,
            RTVFormats: [DXGI_FORMAT_R8G8B8A8_UNORM;8],
            DSVFormat: 0,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            NodeMask: 0,
            CachedPSO: D3D12_CACHED_PIPELINE_STATE { pCachedBlob: null(), CachedBlobSizeInBytes: 0 },
            Flags: 0
        };
        return CpD3D12_GRAPHICS_PIPELINE_STATE_DESC(d3d12_graphics_pipeline_state_desc);
    }
}
impl Default for CpD3D12_ROOT_SIGNATURE_DESC{
    fn default() -> Self {
        let d3d12_root_signature_desc = D3D12_ROOT_SIGNATURE_DESC{
            NumParameters: 0,
            pParameters: null(),
            NumStaticSamplers: 0,
            pStaticSamplers: null(),
            Flags: 0
        };
        CpD3D12_ROOT_SIGNATURE_DESC(d3d12_root_signature_desc)
    }
}