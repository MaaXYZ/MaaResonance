# Maa Resonance

[简体中文](/README_zh.md) | [English](/README.md)

This is a project for creating a Maa Resonance front end with Tauri.

## Project Goals

- Automated Business Running
  - Choose different routes according to the fatigue level consumed by different strategies
- clear daily affairs

## Getting Started

Note that the building process is only currently tested and supported on Windows. Support for Linux is planned.

### MaaAgentBinary

This repo contains the `MaaAgentBinary` as a submodule. You need to clone this repo with the `--recurse-submodules` flag or run the following commands after cloning:

```bash
git submodule init  
git submodule update
```

### MaaFramework

You need to have a latest version (expected v1.7.*) of MaaFramework installed in your system. If you already have, make sure that CMake can find it. The easiest way to do this is to add your MaaFramework installation directory to your `CMAKE_PREFIX_PATH` or `PATH` environment variable.

If you don't have it installed, the python script will do that anyway so there is generally no need for you to do it manually.

### Run Dependency Script

If you have an installed version of MaaFramework, you can skip this step and manually copy the MaaFramework dlls to `tauri/` directory.  

If not, you can run

```bash
python ./scripts/makedeps.py
```

 to download and extract the latest version of MaaFramework to the `tauri/` directory. Then, as said above, you might need to set your PATH environment variables to point to the MaaFramework installation directory (`$projectDir/scripts/deps/maafw`).

Cmake and clang ( you can choose LLVM to instead ) is also needed.

### Build and Run

Use of `pnpm` is encouraged, but you can probably use `npm` or `yarn` as well.

```bash
pnpm install  
pnpm tauri dev
```

Or a `mock` command is provided where we will not try to really scan the devices but show a mock device, this is useful for UI development.

```bash
pnpm mock
```

Then build the project with:

```bash
pnpm tauri build
```

To build the project for production, use:

```bash
pnpm build
```

This will build different targets.
