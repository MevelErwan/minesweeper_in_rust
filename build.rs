fn main() {
    // Chemin vers les fichiers .lib
    println!(r"cargo:rustc-link-search=native=C:\Users\merde\rustlib");
    // Liaisons nécessaires
    println!(r"cargo:rustc-link-lib=static=SDL2");
    println!(r"cargo:rustc-link-lib=static=SDL2main");
}
