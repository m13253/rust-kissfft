fn main() {
    let out_dir = match std::os::getenv("OUT_DIR") {
        Some(out_dir) => out_dir,
        None          => String::from_str(".")
    };
    let cc = match std::os::getenv("CC") {
        Some(cc) => cc,
        None     => String::from_str("gcc")
    };
    let ar = match std::os::getenv("AR") {
        Some(ar) => ar,
        None     => String::from_str("ar")
    };
    let compile_object = |filename: &str|
        std::io::Command::new(&cc)
            .args(&[String::from_str("-c"), String::from_str("-o"), format!("{}/{}.o", out_dir, filename), format!("src/{}.c", filename)])
            .stdin(std::io::process::InheritFd(0)).stdout(std::io::process::InheritFd(1)).stderr(std::io::process::InheritFd(2))
            .status();
    let objects = ["kiss_fft", "kiss_fft_free"];
    for object in objects.iter() {
        assert!(compile_object(*object).unwrap().success());
    }
    let create_archive = |archive: &str, objects: &[&str]| {
        std::io::Command::new(&ar)
            .args(&["crs", format!("{}/lib{}.a", out_dir, archive).as_slice()])
            .args(objects.iter().map(|object: &&str| -> String format!("{}/{}.o", out_dir, object)).collect::<Vec<String>>().as_slice())
            .stdin(std::io::process::InheritFd(0)).stdout(std::io::process::InheritFd(1)).stderr(std::io::process::InheritFd(2))
            .status()
    };
    assert!(create_archive("kissfft", &objects).unwrap().success());
    println!("cargo:rustc-flags=-L {} -l kissfft:static", out_dir);
}
