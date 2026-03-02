# Drift Script Manager
**Drift Script Manager** is an all-in-one Orion Drift Spectator Script Manager. Powered by Dear ImGui and Rust the **Drift Script Manager** aims to standardize the way both users and developers interact with spectator scripts by providing a standard packaging implementation and an intuitive GUI for all your scripting needs.

## Features
- Intuitive UI for creating/editing/building scripts
- A way to standardize script zip packages layout
- Script templates (built-in and custom)
- In app template navigator/editor
- Fast and lightweight
- Compatible with any of your previous multi-file projects
- Dev Annotations

## Installation

### Windows
Download `Drift-Script-Manager-windows-x86_64.exe` from [Releases](https://github.com/dennssen/Drift-Script-Manager/releases) and run.

### Linux (AppImage - Recommended)
Download `Drift-Script-Manager-x86_64.AppImage` from [Releases](https://github.com/dennssen/Drift-Script-Manager/releases), make it executable, and run:
```bash
chmod +x Drift-Script-Manager-x86_64.AppImage
./Drift-Script-Manager-x86_64.AppImage
```
### Linux (Standalone Binary) 
Download `Drift-Script-Manager-linux-x86_64` from [Releases](https://github.com/dennssen/Drift-Script-Manager/releases), make it executable, and run:
```bash
chmod +x Drift-Script-Manager-linux-x86_64
./Drift-Script-Manager-linux-x86_64
```
Note: You may need to install X11/XCB dependencies on some minimal systems.

## Quick Start
1. Launch the application
2. Click "New Project" to create your first project
3. Fill in your project info
4. Click create

## Selecting a project (Editing/Building)
When selecting a project for either editing or building the file dialog expects your projects `package.json`

## How to use Dev Annotations
Dev annotations are comments that you can use to let the project manager know what you want to remember or don't want in the release code.

Here is an example of how to use Dev Notes:
```lua
function myCode()
    -- [Dev] This is for testing only and should be removed before release
    print("test")
end
```
With this example, if you were to try and build, the project manager will tell you what the note says and where it is and then give you the option to cancel the build process.

Here is an example of how to use Dev Blocks:
```lua
function myCode()
    -- [Begin Dev Block]
    print("test")
    -- [End Dev Block]
end
```
With this example, if you were to try and build, the project manager will remove all the code inside the Dev Blocks before placing the file in the build zip.

You can have multiple Dev annotations in the same file, and in every file. I recommend creating a vscode snippet for the Dev annotations.

## Building from source
```bash
git clone https://github.com/dennssen/Drift-Script-Manager.git
cd Drift-Script-Manager
cargo build --release
```
*Note: Master branch may contain unreleased code and features, it is advised to install a stable release from the release page*
