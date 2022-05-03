# qrust
This crate QR Code Form arguments.

## Arguments
```
qrust <data_text> <output_path> <size_px> <forground> <background>  
```

## Examples
#### Encode any data to a QR Code
```
qrust "https://google.com/" 256 google.png 000000 ffffff
```

```
qrust "https://github.com/slectgit/" 256 slect.png ff0052 ffffff
```

```
qrust "https://github.com/slectgit/deno-rest" 128 slect-alt.png ffffff ff0052
```