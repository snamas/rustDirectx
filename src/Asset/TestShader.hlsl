float4 vert(float4 pos:POSITION) : SV_POSITION{
    return pos;
}
float4 frag(float4 possv:SV_POSITION):SV_TARGET{
    return float4(possv.x,possv.y,1.0,1.0);
}