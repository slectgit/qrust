# qrust
This Crate QR Code Form Arguments.

## Arguments
```
qrust <data_text> <output_path> <size_px> <forground> <background>  
```

## Examples
#### Encode Any Data To A QR Code
```
qrust "https://google.com/" 512 google.png 000000 ffffff
```
<img src="https://raw.githubusercontent.com/slectgit/qrust/master/example/google.png"/>

```
qrust "https://github.com/slectgit/" 256 slect.png ff0052 ffffff
```
<img src="https://raw.githubusercontent.com/slectgit/qrust/master/example/slect.png"/>

```
qrust "https://github.com/slectgit/qrust/" 128 slect-alt.png ffffff ff0052
```
<img src="https://raw.githubusercontent.com/slectgit/qrust/master/example/qrust.png"/>

## Install
#### Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
#### Clone qrust Repository
```
git clone https://github.com/slectgit/qrust.git
```
#### Build
```
cd qrust
git submodule update --init
cargo build --release
```
#### Export Path Or Move Binary Execute File To /usr/bin Dir
Export Path
```
export PATH=$PATH:"<qrust_dir>/target/release/"
```
Or Move Binary Execute File To /usr/bin Dir
```
sudo mv ./target/release/qrust /usr/bin
```

## Licence
### <a href="https://github.com/slectgit/qrust/blob/master/LICENSE">MIT</a>
