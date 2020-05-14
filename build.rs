use std::env;
use std::fs::create_dir;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::{exit, Command};

fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    if host != target {
        if !target.contains("linux") {
            eprintln!(
                "Currently, cross compilation of libtcc doesn't support target:{}",
                target
            );
            exit(1);
        }
        let cross = format!(
            "--cross-prefix={}-",
            cross_prefix(&target).unwrap_or_else(|| {
                eprintln!(
                    "Currently, cross compilation of libtcc doesn't support target:{}",
                    target
                );
                exit(1);
            })
        );

        let cpu = format!("--cpu={}", resolve_cpu(&target));
        let config_args = [
            &cross[..],
            &cpu[..],
            "--enable-static",
            "--enable-cross",
            "--extra-cflags=-fPIC -O3 -g",
        ];
        let make_args = ["libtcc.a"];
        println!("Cross: configure {:?}, make {:?}", config_args, make_args);
        build_tcc(Some(&config_args), Some(&make_args));
    } else if !tcc_installed() {
        let config_args = ["--enable-static", "--extra-cflags=-fPIC -O3 -g"];
        println!("Building: configure {:?}, make", config_args);
        build_tcc(Some(&config_args), None);
    } else {
        println!("Using installed libtcc");
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }

    println!("cargo:rustc-link-lib=static=tcc");
    println!("cargo:rerun-if-changed=build.rs");
}

fn build_tcc(config_arg: Option<&[&str]>, make_arg: Option<&[&str]>) {
    let tcc_src = env::current_dir().unwrap().join("src/tcc");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("build");

    if let Err(e) = create_dir(&build_dir) {
        if let ErrorKind::AlreadyExists = e.kind() {
        } else {
            eprintln!("Fail to creat build dir:{}", build_dir.display());
            exit(1);
        }
    }

    let mut configure = Command::new(tcc_src.join("configure"));
    configure
        .arg(format!("--prefix={}", out_dir.display()))
        .current_dir(&build_dir);
    if let Some(args) = config_arg {
        configure.args(args);
    }
    let status = configure.status().unwrap();
    if !status.success() {
        eprintln!("Fail to configure: {:?}", status);
        exit(1);
    }

    let mut make = Command::new("make");
    make.current_dir(&build_dir).arg(format!(
        "-j{}",
        env::var("NUM_JOBS").unwrap_or_else(|_| String::from("1"))
    ));
    if let Some(args) = make_arg {
        make.args(args);
    }
    let status = make.status().unwrap();

    if !status.success() {
        eprintln!("Fail to make: {:?}", status);
        exit(1);
    }

    let install_status = Command::new("make")
        .current_dir(&build_dir)
        .arg("install")
        .status()
        .unwrap();
    if !install_status.success() {
        eprintln!("Fail to install: {:?}", install_status);
        exit(1);
    }
    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
    println!("cargo:rerun-if-changed={}", tcc_src.display());
}

fn tcc_installed() -> bool {
    let cfg = cc::Build::new();
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let tcc_tmp = out.join("tcc-tmp");
    if let Err(e) = create_dir(&tcc_tmp) {
        if let ErrorKind::AlreadyExists = e.kind() {
        } else {
            eprintln!("Fail to creat build dir:{}", tcc_tmp.display());
            exit(1);
        }
    }
    let compiler = cfg.get_compiler();
    let mut cmd = Command::new(compiler.path());
    cmd.arg("src/libtcc_test.c")
        .arg("-o")
        .arg(tcc_tmp.join("a.out"))
        .arg("-Isrc/tcc")
        .arg("-ltcc")
        .arg("-ldl");
    println!("running {:?}", cmd);
    if let Ok(status) = cmd.status() {
        if status.success() {
            return true;
        }
    }
    false
}

fn cross_prefix(target: &str) -> Option<&'static str> {
    match target {
        "aarch64-unknown-linux-gnu" => Some("aarch64-linux-gnu"),
        "aarch64-unknown-linux-musl" => Some("aarch64-linux-musl"),
        "aarch64-unknown-netbsd" => Some("aarch64--netbsd"),
        "arm-unknown-linux-gnueabi" => Some("arm-linux-gnueabi"),
        "armv4t-unknown-linux-gnueabi" => Some("arm-linux-gnueabi"),
        "armv5te-unknown-linux-gnueabi" => Some("arm-linux-gnueabi"),
        "armv5te-unknown-linux-musleabi" => Some("arm-linux-gnueabi"),
        "arm-frc-linux-gnueabi" => Some("arm-frc-linux-gnueabi"),
        "arm-unknown-linux-gnueabihf" => Some("arm-linux-gnueabihf"),
        "arm-unknown-linux-musleabi" => Some("arm-linux-musleabi"),
        "arm-unknown-linux-musleabihf" => Some("arm-linux-musleabihf"),
        "arm-unknown-netbsd-eabi" => Some("arm--netbsdelf-eabi"),
        "armv6-unknown-netbsd-eabihf" => Some("armv6--netbsdelf-eabihf"),
        "armv7-unknown-linux-gnueabihf" => Some("arm-linux-gnueabihf"),
        "armv7-unknown-linux-musleabihf" => Some("arm-linux-musleabihf"),
        "armv7neon-unknown-linux-gnueabihf" => Some("arm-linux-gnueabihf"),
        "armv7neon-unknown-linux-musleabihf" => Some("arm-linux-musleabihf"),
        "thumbv7-unknown-linux-gnueabihf" => Some("arm-linux-gnueabihf"),
        "thumbv7-unknown-linux-musleabihf" => Some("arm-linux-musleabihf"),
        "thumbv7neon-unknown-linux-gnueabihf" => Some("arm-linux-gnueabihf"),
        "thumbv7neon-unknown-linux-musleabihf" => Some("arm-linux-musleabihf"),
        "armv7-unknown-netbsd-eabihf" => Some("armv7--netbsdelf-eabihf"),
        "i586-unknown-linux-musl" => Some("musl"),
        "i686-pc-windows-gnu" => Some("i686-w64-mingw32"),
        "i686-uwp-windows-gnu" => Some("i686-w64-mingw32"),
        "i686-unknown-linux-musl" => Some("musl"),
        "i686-unknown-netbsd" => Some("i486--netbsdelf"),
        "mips-unknown-linux-gnu" => Some("mips-linux-gnu"),
        "mipsel-unknown-linux-gnu" => Some("mipsel-linux-gnu"),
        "mips64-unknown-linux-gnuabi64" => Some("mips64-linux-gnuabi64"),
        "mips64el-unknown-linux-gnuabi64" => Some("mips64el-linux-gnuabi64"),
        "mipsisa32r6-unknown-linux-gnu" => Some("mipsisa32r6-linux-gnu"),
        "mipsisa32r6el-unknown-linux-gnu" => Some("mipsisa32r6el-linux-gnu"),
        "mipsisa64r6-unknown-linux-gnuabi64" => Some("mipsisa64r6-linux-gnuabi64"),
        "mipsisa64r6el-unknown-linux-gnuabi64" => Some("mipsisa64r6el-linux-gnuabi64"),
        "powerpc-unknown-linux-gnu" => Some("powerpc-linux-gnu"),
        "powerpc-unknown-linux-gnuspe" => Some("powerpc-linux-gnuspe"),
        "powerpc-unknown-netbsd" => Some("powerpc--netbsd"),
        "powerpc64-unknown-linux-gnu" => Some("powerpc-linux-gnu"),
        "powerpc64le-unknown-linux-gnu" => Some("powerpc64le-linux-gnu"),
        "riscv32i-unknown-none-elf" => Some("riscv32-unknown-elf"),
        "riscv32imac-unknown-none-elf" => Some("riscv32-unknown-elf"),
        "riscv32imc-unknown-none-elf" => Some("riscv32-unknown-elf"),
        "riscv64gc-unknown-none-elf" => Some("riscv64-unknown-elf"),
        "riscv64imac-unknown-none-elf" => Some("riscv64-unknown-elf"),
        "riscv64gc-unknown-linux-gnu" => Some("riscv64-linux-gnu"),
        "s390x-unknown-linux-gnu" => Some("s390x-linux-gnu"),
        "sparc-unknown-linux-gnu" => Some("sparc-linux-gnu"),
        "sparc64-unknown-linux-gnu" => Some("sparc64-linux-gnu"),
        "sparc64-unknown-netbsd" => Some("sparc64--netbsd"),
        "sparcv9-sun-solaris" => Some("sparcv9-sun-solaris"),
        "armv7a-none-eabi" => Some("arm-none-eabi"),
        "armv7a-none-eabihf" => Some("arm-none-eabi"),
        "armebv7r-none-eabi" => Some("arm-none-eabi"),
        "armebv7r-none-eabihf" => Some("arm-none-eabi"),
        "armv7r-none-eabi" => Some("arm-none-eabi"),
        "armv7r-none-eabihf" => Some("arm-none-eabi"),
        "thumbv6m-none-eabi" => Some("arm-none-eabi"),
        "thumbv7em-none-eabi" => Some("arm-none-eabi"),
        "thumbv7em-none-eabihf" => Some("arm-none-eabi"),
        "thumbv7m-none-eabi" => Some("arm-none-eabi"),
        "thumbv8m.base-none-eabi" => Some("arm-none-eabi"),
        "thumbv8m.main-none-eabi" => Some("arm-none-eabi"),
        "thumbv8m.main-none-eabihf" => Some("arm-none-eabi"),
        "x86_64-pc-windows-gnu" => Some("x86_64-w64-mingw32"),
        "x86_64-uwp-windows-gnu" => Some("x86_64-w64-mingw32"),
        "x86_64-rumprun-netbsd" => Some("x86_64-rumprun-netbsd"),
        "x86_64-unknown-linux-musl" => Some("musl"),
        "x86_64-unknown-netbsd" => Some("x86_64--netbsd"),
        "x86_64-unknown-linux-gnu" => Some("x86_64-linux-gnu"),
        _ => None,
    }
}

fn resolve_cpu(target: &str) -> &str {
    target.split('-').next().unwrap()
}
