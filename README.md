# rustDirectx
RustでDirectX12を扱えるようにしたライブラリと、それを用いた描画サンプルが含まれています。
## CapriCore
CapriCoreライブラリは、DreictX12をRustで扱うためにRust仕様にラップしたライブラリです。
### cp_directx12.rs
cp_directx12.rsは、DirecctX12の基本的な部分をRustで扱いやするしたライブラリです。
## cp_default_value.rs
cp_default_value.rsはDirectX12で用いられている構造体の初期値を記載しているコードです。
## main.rs
main.rsはCapriCoreライブラリを用いて、実際に描画しているコードです。


## おまけ
### 独自解釈で生み出したDirectX12相関図
https://viewer.diagrams.net/?highlight=0000ff&edit=_blank&layers=1&nav=1&title=Directx12Overview.drawio#Uhttps%3A%2F%2Fraw.githubusercontent.com%2Fsnamas%2Fmypage%2Fmaster%2FDirectx12Overview.drawio

### ここまで出力できた
![DX12inRust 2021_02_28 1_53_04](https://user-images.githubusercontent.com/43674314/109393926-ba5a3b00-7967-11eb-9476-9e5855772695.png)
