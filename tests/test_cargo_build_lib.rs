use std::path;
use support::{basic_bin_manifest, execs, project, ProjectBuilder};
use support::{COMPILING, RUNNING};
use hamcrest::{assert_that};

fn setup() {
}

fn verbose_output_for_lib(p: &ProjectBuilder) -> String {
    format!("\
{compiling} {name} v{version} ({url})
{running} `rustc {dir}{sep}src{sep}lib.rs --crate-name {name} --crate-type lib -g \
        -C metadata=[..] \
        -C extra-filename=-[..] \
        --out-dir {dir}{sep}target \
        --dep-info [..] \
        -L {dir}{sep}target \
        -L {dir}{sep}target{sep}deps`
",
            running = RUNNING, compiling = COMPILING, sep = path::SEP,
            dir = p.root().display(), url = p.url(),
            name = "foo", version = "0.0.1")
}

test!(build_lib_only {
    let p = project("foo")
        .file("Cargo.toml", r#"
            [package]

            name = "foo"
            version = "0.0.1"
            authors = ["wycats@example.com"]
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .file("src/lib.rs", r#" "#);

    assert_that(p.cargo_process("build").arg("--lib").arg("-v"),
                execs()
                .with_status(0)
                .with_stdout(verbose_output_for_lib(&p)));
})


test!(build_with_no_lib {
    let p = project("foo")
        .file("Cargo.toml", basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            fn main() {}
        "#);

    assert_that(p.cargo_process("build").arg("--lib"),
                execs()
                .with_status(101)
                .with_stderr("There is no lib to build, remove `--lib` flag"));
})
