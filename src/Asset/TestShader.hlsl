float4 vert(float4 pos:POSITION) : SV_POSITION{
    return pos;
}
float4 frag(float4 pos:SV_POSITION):SV_TARGET{
    return float4(pos.x,pos.y,0.0,1.0);
}