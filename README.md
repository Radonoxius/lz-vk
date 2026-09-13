# lz-vk
A generic memory decompressor for GPUs using Vulkan

## Building the Project
This project requires `rustc` version 1.85 or higher and `clang` (preferably version 21).
Additionally, you need to install Vulkan and its development tools & headers.

To compile this for Android, you will need the Android NDK (Latest, LTS). You can also
compile this on an Android device itself via Termux.

Use the following commands to get all the necessary tools:

Ubuntu/Debian:
```bash
sudo apt install -y clang cmake pkg-config \
    libvulkan-dev vulkan-tools vulkan-validationlayers-dev \
    spirv-tools glslang-tools
```

Fedora:
```bash
sudo dnf install -y clang clang-tools-extra cmake pkgconf-pkg-config \
    vulkan-loader-devel vulkan-headers vulkan-tools vulkan-validation-layers-devel \
    spirv-tools glslang
```

Termux:
```bash
pkg install -y rust clang cmake pkg-config \
    vulkan-loader-generic vulkan-headers vulkan-tools \
    spirv-tools glslang
```

**NOTE: The build tooling isnt tested on Windows & Mac yet. However it should work out of the box**

