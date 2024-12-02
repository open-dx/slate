pub fn main() {
    if let Ok(profile) = std::env::var("PROFILE") {
        if profile == "release" {
            // Prevent the console window from appearing
            // when running the application.
            println!("cargo:rustc-cdylib-link-arg=-mwindows");
        }
    }
}