To make your Rust CLI tool globally available on your system, you can follow these steps:

1. **Build the Release Version**: Compile your Rust project to generate an optimized binary.
2. **Move the Binary to a Directory in Your PATH**: Copy or move the compiled binary to a directory that is included in your system’s `PATH`.
3. **Create a Shell Alias or Symlink (Optional)**: Alternatively, you can create a shell alias or symlink to simplify access to the tool.

Here’s how to do each step in detail:

### Step 1: Build the Release Version

First, navigate to your project directory and build the release version of your project:

```sh
cd path/to/your/project/create-directus-app
cargo build --release
```

This command will compile your project and place the binary in the `target/release` directory. The binary will be named `create-directus-app`.

### Step 2: Move the Binary to a Directory in Your PATH

You need to move the compiled binary to a directory that is included in your system's `PATH`. Common directories for this purpose are `/usr/local/bin` or `$HOME/.cargo/bin` (which is used by Rust).

#### On Linux and macOS:

You can move the binary using the `mv` command. Replace `/usr/local/bin` with the desired directory if you prefer a different one.

```sh
sudo mv target/release/create-directus-app /usr/local/bin/
```

Or, if you prefer to use a directory in your home folder (this does not require `sudo`):

```sh
mv target/release/create-directus-app $HOME/.cargo/bin/
```

Ensure `$HOME/.cargo/bin` is in your `PATH` by adding this line to your `.bashrc`, `.zshrc`, or equivalent shell configuration file:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

Then, reload your shell configuration:

```sh
source ~/.bashrc  # or source ~/.zshrc, depending on your shell
```

#### On Windows:

You can move the binary to a directory that is already in your `PATH`, or add a new directory to your `PATH` environment variable.

Move the binary using the `move` command (replace `C:\Tools` with a directory in your `PATH`):

```cmd
move target\release\create-directus-app.exe C:\Tools\
```

Or add the `$HOME\.cargo\bin` directory to your `PATH` environment variable:

1. Open the Start Search, type in “env”, and select “Edit the system environment variables”.
2. Click the “Environment Variables…” button.
3. In the “System variables” section, find the `Path` variable and click “Edit…”.
4. Click “New” and add the path to your binary directory, e.g., `C:\Users\YourUsername\.cargo\bin`.
5. Click “OK” on all dialogs to apply the changes.

### Step 3: Verify Installation

To verify that the `create-directus-app` command is available globally, open a new terminal and type:

```sh
create-directus-app --help
```

You should see the help message for your CLI tool, indicating it is correctly installed and available globally.

### Step 4: Create a Shell Alias or Symlink (Optional)

If you moved the binary to a non-standard location, you can create a symlink in a directory that’s already in your `PATH`.

#### On Linux and macOS:

```sh
ln -s /path/to/binary/create-directus-app /usr/local/bin/create-directus-app
```

#### On Windows:

Windows does not have a native `ln` command for creating symlinks, but you can use the `mklink` command from a Command Prompt running as Administrator:

```cmd
mklink C:\Path\In\Path\create-directus-app.exe C:\Path\To\Your\Binary\create-directus-app.exe
```

### Conclusion

By following these steps, you will have installed your Rust CLI tool globally and made it available via the `create-directus-app` command. Now, you can create Directus project structures from anywhere on your system with ease.

## Extenstion

https://docs.directus.io/extensions/creating-extensions.html
