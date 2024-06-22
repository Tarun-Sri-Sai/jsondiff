# jsondiff: v1.0.0

This CLI tool allows comparing JSON data in two files from the terminal. It allows the user to input two files, navigate through them using a neat JSON path syntax and provides a line-based comparison.

## Installation

This tool is built using Rust, so it can be installed by compiling the source code with `cargo` from the Rust toolchain. The binary can be built using the following shell commands.

```bash
git clone https://github.com/Tarun-Sri-Sai/jsondiff.git
cd jsondiff
cargo build --release
```

### For Windows

```cmd
mkdir "C:\Program Files\jsondiff"
mv .\target\release\jsondiff.exe "C:\Program Files\jsondiff\"
```

_Note: Add `C:\Program Files\jsondiff` to your PATH environment variable._

### For Linux

```bash
mv ./target/release/jsondiff /usr/local/bin
```

## Usage

```bash
jsondiff ./file1.json ./file2.json
Enter JSON path for ./file1.json: "shopping\.cart"."users".0."_id"
Enter JSON path for ./file2.json: "cart"."users".1."_id"."\"description\""
```
