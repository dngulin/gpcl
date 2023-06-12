use semver::Version;

fn main() {
    let qt_include_path = std::env::var("DEP_QT_INCLUDE_PATH").unwrap();
    let qt_version = std::env::var("DEP_QT_VERSION")
        .unwrap()
        .parse::<Version>()
        .expect("Parsing Qt version failed");

    if qt_version <= Version::new(6, 5, 0) {
        println!("cargo:rustc-cfg=no_qt");
        return;
    }

    let mut config = cpp_build::Config::new();

    for f in std::env::var("DEP_QT_COMPILE_FLAGS")
        .unwrap()
        .split_terminator(';')
    {
        config.flag(f);
    }

    config.include(&qt_include_path).build("src/main.rs");
}
