use std::env;

include!(concat!(env!("OUT_DIR"), "/pkg_path.rs"));

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

pub fn wx_config(args: &[&str]) -> Vec<String> {
    let flags = FLAGS
        .iter()
        .map(|&f| f.replace("@ROOT@", &PKG_PATH).replace('\n', " "));
    let (ldflags, cflags): (Vec<_>, Vec<_>) =
        flags.partition(|f| f.starts_with("-l") || f.starts_with("-L"));
    if args.contains(&"--cflags") {
        cflags
    } else {
        ldflags
    }
}
