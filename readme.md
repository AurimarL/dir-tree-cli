# DTC dir-tree-cli 

`dir-tree-cli` is a command-line tool written in Rust that displays the folder structure of a specified directory in a tree-like format. It helps users visualize the organization of files and subdirectories, making it easier to navigate and understand the structure of their projects.

## Features

- Recursively lists all files and subdirectories.
- Ignores specified folders (e.g., `node_modules`, `.git`, etc.) to keep the output clean.
- Displays the folder structure in a visually appealing tree format.

## Installation

To install `dir-tree-cli`, you need to have Rust and Cargo installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can clone the repository and build the project:

```bash
git clone https://github.com/yourusername/dir-tree-cli.git
cd dir-tree-cli
cargo build --release
```

The compiled executable will be located in the `target/release` directory.

## Usage

To use `dir-tree-cli`, run the following command in your terminal:

```bash
./target/release/dtc <path>
```

Replace `<path>` with the path to the directory you want to visualize. For example:

```bash
./target/release/dtc /path/to/your/directory
```

### Example

```bash
./target/release/dtc /home/user/projects
```

Output:

```
projects
├── dir-tree-cli
│   ├── src
│   ├── Cargo.toml
│   └── README.md
├── another-project
│   ├── src
│   └── Cargo.toml
└── .git
```

### Ignored Folders

The following folders are ignored by default when displaying the folder structure:

- `node_modules`
- `target`
- `.next`
- `.ssh`
- `coverage`
- `.git`

You can modify the `IGNORED_FOLDERS` constant in the source code if you want to change which folders are ignored.

## Contributing

Contributions are welcome! If you have suggestions for improvements or new features, feel free to open an issue or submit a pull request.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes and commit them (`git commit -m 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

Aurimar Lopes - [aurimardev@gmail.com](mailto:aurimardev@gmail.com)

This README was writen using AI... it provides a clear overview of your project, making it easy for users to understand what it does and how to use it