# jsondiff: v3.0.0

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
Enter JSON path for ./file1.json: "shopping.cart"."items".0."_id"                   # Considers double quote (") encapsulation as a property. Array indices can be used directly as is. Dot (.) is used to separate properties and/or indices.
Enter JSON path for ./file2.json: "cart"."items".1."_id"."\"Cart \\ Description\""  # Use backslash (\) to escape (", \) characters inside property name.
```

```bash
jsondiff ./file1.json ./file2.json --input quickview                                # Recommended way to use the tool, prints a preview of the JSON node you're currently at
Enter JSON path for ./file1.json: "shopping.cart"."items".0."_id"
Enter JSON path for ./file2.json: "cart"."items".1."_id"."\"Cart \\ Description\""
```

```bash
jsondiff ./file1.json ./file2.json --input tui                                      # Linux-only mode, opens a TUI to show the preview, allows backspacing
```
