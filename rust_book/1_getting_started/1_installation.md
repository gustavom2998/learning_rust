# Rust Installation

We can install rust by using the `rustup` tool, a CLI tool for managing Rust versions and associated tools. 

## Windows Installation

We're currently on Windows, so we'll install `rustup` from the Website:
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

We downloaded the 64 bit version. When running the exe we were prompted to install the Visual C++ Build Tools. We pressed the 1 key to perform this installation. Next we got prompted to confirm the Rustup installation. Some informations were passed:

```
Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  C:\Users\*\.rustup

The Cargo home directory is located at:

  C:\Users\*\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\*\.cargo\bin

This path will then be added to your PATH environment variable by
modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.
```

We confirmed the installation with these settings by hitting 1.

## Checking installation

```powershell
rustc --version
```

## Updating Rustup

```powershell
rustup update
```

## Unistalling Rustup

```powershell
rustup self uninstall
```