# jsondiff: v3.0.5

This CLI tool allows comparing JSON data in two files from the terminal. It allows the user to input two files, navigate through them using a neat JSON path syntax and provides a line-based comparison.

## Requirements

- git
- cargo

## Installation

This tool is built using Rust, so it can be installed by compiling the source code with `cargo` from the [Rust page](https://www.rust-lang.org/tools/install).

### For Windows

You can download the [Windows installation script](./Install.ps1) and execute it in a machine using PowerShell as Administrator.

```powershell
Set-ExecutionPolicy -Scope Process Unrestricted
powershell.exe -File .\Install.ps1
```

### For Linux

You can download the [Linux installation script](./install.sh) and execute it.

```bash
chmod +x ./install.sh
./install.sh
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
