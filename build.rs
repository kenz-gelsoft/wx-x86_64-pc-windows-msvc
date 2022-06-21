use std::env;

const FLAGS: &[&str] = &[
    "-I@ROOT@/lib/vc14x_x64_dll/mswu",
    "-I@ROOT@/include",
    "-I@ROOT@/include/msvc",
    "-D__WXMSW__",
    "-DWXUSINGDLL",
    "-DwxDEBUG_LEVEL=0",
    "-L@ROOT@/lib/vc14x_x64_dll",
    "-lwxbase31u",
    "-lwxmsw31u_core",
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

fn main() {
    let pkg_path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let flags = FLAGS
        .iter()
        .map(|&f| f.replace("@ROOT@", &pkg_path).replace('\n', " "))
        .collect::<Vec<_>>();

    println!("cargo:cflags={}", flags.join(" "));
}
