use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if !cfg!(target_os = "android") && target_os == "android" {
        // This branch is taken if Host OS isnt android, but we target android
        // Uses the Android NDK, Latest LTS
        let mut android_ndk_home = env::var("ANDROID_NDK_HOME")
            .unwrap_or("".into())
            .replace('\\', "/");

        let host_tag = // Host systems supported by Android NDK
            if cfg!(target_os = "windows") {
                "windows-x86_64"
            } else if cfg!(target_os = "macos") {
                "darwin-x86_64"
            } else {
                "linux-x86_64"
            };

        if android_ndk_home.is_empty() {
            panic!("ANDROID_NDK_HOME env-variable is undefined!");
        } else if android_ndk_home.ends_with("/") {
            android_ndk_home.pop();
        }

        let sysroot = format!("{android_ndk_home}/toolchains/llvm/prebuilt/{host_tag}/sysroot");
        let clang_resources = format!("{android_ndk_home}/toolchains/llvm/prebuilt/{host_tag}/lib/clang/21");

        println!("cargo::rustc-link-arg=--sysroot={sysroot}");
        println!("cargo::rustc-link-arg=-resource-dir={clang_resources}");

        // Vulkan 1.1 was made mandatory in Android 10
        println!("cargo::rustc-link-arg=--target={target_arch}-linux-android29");

        println!("cargo::rustc-link-lib=vulkan");
        println!("cargo::rustc-link-lib=android");
    } else {
        // Add extra flags if compiling the library on Android, via Termux
        #[cfg(target_os = "android")]
        android_termux_extras();

        // Common flags
        println!("cargo::rustc-link-lib=vulkan");
    }
}

#[cfg(target_os = "android")]
fn android_termux_extras() {
    println!("cargo::rustc-link-lib=android");
}