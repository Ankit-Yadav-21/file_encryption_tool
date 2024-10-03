# File Encryption Tool

This command-line tool provides basic file encryption and decryption functionality using XOR operations. It's designed to demonstrate file encryption concepts and is not recommended for use in real-world applications due to its simple encryption scheme.

## Features

- **Encrypt and Decrypt Files**: Encrypt or decrypt files using a provided key.
- **Key Generation**: Automatically generate a 16-byte key for encryption/decryption.
- **Custom Key Parsing**: Use a custom key provided as a comma-separated string.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Ankit-Yadav-21/file-encryption-tool.git

   ```

2. Navigate to the project directory:

   ```bash
   cd file-encryption-tool

   ```

3. Build the project using Cargo:

   ```bash
   cargo build --release

   ```

## Usage

To use the File Encryption Tool, run the compiled binary with the appropriate command-line arguments.

    file-encryption-tool --mode <encrypt/decrypt> --input <input_file> --output <output_file> [--key <key>] [--generate_key]

## Options

--mode: Specify the operation mode, either encrypt or decrypt.
--input: The path to the input file to be encrypted or decrypted.
--output: The path where the output file will be saved.
--key (optional): A custom key provided as a comma-separated string of bytes (e.g., --key "10,20,30,40,50,60,70,80,90,100,110,120,130,140,150,160").
--generate_key: Generate a random 16-byte key. If not provided, a custom key must be specified using the --key option.

## Examples

Encrypt a File Using a Custom Key:

    ```bash
    file-encryption-tool --mode encrypt --input sample.txt --output encrypted.bin --key "10,20,30,40,50,60,70,80,90,100,110,120,130,140,150,160"
    ```

Decrypt a File Using the Same Custom Key:

    ```bash
    file-encryption-tool --mode decrypt --input encrypted.bin --output decrypted.txt --key "10,20,30,40,50,60,70,80,90,100,110,120,130,140,150,160"
    ```

Encrypt a File with Auto-Generated Key:

    ```bash
    file-encryption-tool --mode encrypt --input sample.txt --output encrypted.bin --generate_key
    ```
    The generated key will be displayed in the terminal output.

## Error Handling

If the --generate_key option is not specified and no custom key is provided, the tool will return an error:

    ```bash
    Key not provided, and --generate_key not set
    ```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve this tool.

## Disclaimer

This tool uses XOR encryption for demonstration purposes only. Do not use it for real-world encryption needs, as XOR encryption is not secure.
