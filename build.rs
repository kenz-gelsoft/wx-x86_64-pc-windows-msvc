use std::env;
use std::fs;
use std::path::Path;

const FLAGS: &[&str] = &[
    "-I@ROOT@/lib/vc14x_x64_dll/mswu",
    "-I@ROOT@/include",
    "-I@ROOT@/include/msvc",
    "-D__WXMSW__",
    "-DWXUSINGDLL",
    "-DwxDEBUG_LEVEL=0",
    "-L@ROOT@/lib/vc14x_x64_dll",
    "-lwxbase32u",
    "-lwxmsw32u_core",
    "-lkernel32",
    "-luser32",
    "-lgdi32",
    "-lcomdlg32",
    "-lshell32",
    "-lshlwapi",
    "-lcomctl32",
    "-lole32",
    "-loleaut32",
    "-luuid",
    "-lrpcrt4",
    "-ladvapi32",
    "-lversion",
    "-lws2_32",
    "-loleacc",
    "-luxtheme",
    "-lwxregexu",
    "-lwxjpeg",
    "-lwxpng",
    "-lwxtiff",
    "-lwxzlib",
    "-lwxexpat",
];

fn save_flags(flags: &str) {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("flags.rs");
    fs::write(&dest_path, format!("static FLAGS: &str = r\"{}\";", flags)).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

fn main() {
    let pkg_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let flags = FLAGS
        .iter()
        .map(|&f| f.replace("@ROOT@", &pkg_path).replace('\n', " "))
        .collect::<Vec<_>>();

    save_flags(&flags.join(" "));
    println!("cargo:cflags={}", flags.join(" "));
}
