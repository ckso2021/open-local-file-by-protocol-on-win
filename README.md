# Open local file by custom protocol on Windows 
This repository is for an example application which using a custom protocol for webpage to open a local file on Windows.

## How to use
1. edit and register openfile.reg
2. create an "\<a\>" tag with href="```openfile://(your encoded url of file path)```"  

## How to build
1. install Rust on Windows
2. open folder of this repository
3. run ```cargo install --path .```
3. run ```cargo build --release```

## For testing
3. run ```cargo run -- openfile://D%3A%2F%2Ftesting%20for%20openfile.xlsx``` ( D://testing for openfile.xlsx )