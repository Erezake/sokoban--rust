fn main() {
    println!("cargo:rustc-link-search=native=D:/MyLib/vcpkg/installed/x64-windows/lib");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
    println!("cargo:rustc-link-lib=SDL2_ttf");
}
