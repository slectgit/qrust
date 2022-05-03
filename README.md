# qrust
This crate QR Code Form arguments.

## Arguments
```
qrust <data_text> <output_path> <size_px> <forground> <background>  
```

## Examples
#### Encode any data to a QR Code
```
qrust "https://google.com/" 512 google.png 000000 ffffff
```
<img src="https://raw.githubusercontent.com/slectgit/qrust/master/example/google.png"/>

```
qrust "https://github.com/slectgit/" 256 slect.png ff0052 ffffff
```
<img src="https://raw.githubusercontent.com/slectgit/qrust/master/example/slect.png"/>

```
qrust "https://github.com/slectgit/deno-rest" 128 slect-alt.png ffffff ff0052
```
<img src="https://raw.githubusercontent.com/slectgit/qrust/master/example/slect-alt.png"/>
