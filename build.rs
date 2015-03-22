#![feature(core)]

use std::env;
use std::path::Path;
use std::process::{Stdio, Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(out_dir.as_slice());

    setup();
    configure();
    build();
    install(&dst);
    clean();
    println!("cargo:rustc-flags=-L {} -l termbox:static", dst.join("lib").display());
}

fn setup() {
    let mut cmd = Command::new("git");
    cmd.arg("clone");
    cmd.arg("https://github.com/nsf/termbox");
    cmd.arg(".termbox");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(manifest_dir.as_slice());
    cmd.current_dir(&cargo_dir);

    run(&mut cmd);
}

fn clean() {
    let mut cmd = Command::new("rm");
    cmd.arg("-rf");
    cmd.arg(".termbox");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(manifest_dir.as_slice());
    cmd.current_dir(&cargo_dir);
    run(&mut cmd);
}

fn configure() {
    let mut cmd = waf();
    cmd.arg("configure");
    cmd.arg("--prefix=/");

    let target = env::var("TARGET").unwrap();
    let mut cflags;
    if target.as_slice().contains("i686") {
        cflags = "-m32"
    } else if target.as_slice().contains("x86_64") {
        cflags = "-m64 -fPIC"
    } else {
        cflags = ""
    }
    println!("waf configure: setting CFLAGS to: `{}`", cflags);
    env::set_var("CFLAGS", cflags);

    run(&mut cmd);
    env::remove_var("CFLAGS");
}

fn build() {
    let mut cmd = waf();
    cmd.arg("build");
    cmd.arg("--targets=termbox_static");
    run(&mut cmd);
}

fn install(dst: &Path) {
    let mut cmd = waf();
    cmd.arg("install");
    cmd.arg("--targets=termbox_static");
    cmd.arg(format!("--destdir={}", dst.display()));
    run(&mut cmd);
}

fn waf() -> Command {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(manifest_dir.as_slice());
    let termbox_dir = cargo_dir.join(".termbox");
    let mut cmd = Command::new("./waf");
    cmd.current_dir(&termbox_dir);
    cmd
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success());
}
