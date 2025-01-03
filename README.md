# open-project

`open-project` is a command-line utility written in Rust to streamline management of local clones of GitHub repos and opening them in Zellij.

Just type `open-project user repo` to automatically clone the repo if necessary and open it in Zellij.

## Install

Make sure you have **Zellij**, the terminal workspace manager, installed. [Install Zellij](https://zellij.dev/documentation/installation.html)

Build `open-project` with either `nix` or `cargo` and add the binary to your $PATH.

#### Nix

```bash
nix build github:cor/open-project
```

#### Cargo

```bash
git clone https://github.com/cor/open-project
cd open-project
cargo build --release
```

## Usage

```bash
open-project <username> <projectname>
```

For example:

```bash
open-project alice my-repo
```

This will:

1. Check for a directory `$HOME/dev/alice` and prompt to create it if missing.
2. Check for the repository `my-repo` under the user's GitHub and clone it if not present.
3. Attach to or create a Zellij session named `my-repo alice`.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve this tool.

