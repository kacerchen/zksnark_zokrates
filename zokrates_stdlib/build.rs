use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn main() {
    // export stdlib folder to OUT_DIR
    export_stdlib();

    // generate tests
    write_tests();
}

fn export_stdlib() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    copy_items(&vec!["stdlib"], out_dir, &options).unwrap();
}

fn write_tests() {
    use glob::glob;

    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("tests.rs");
    let test_file = File::create(&destination).unwrap();
    let mut writer = BufWriter::new(test_file);

    for directory in glob("./tests/bench/**/*.json").unwrap() {
        write_test(&mut writer, &directory.unwrap());
    }
}

fn write_test<W: Write>(test_file: &mut W, test_path: &PathBuf) {
    let test_name = format!(
        "test_{}",
        test_path
            .strip_prefix("tests/bench")
            .unwrap()
            .display()
            .to_string()
            .replace("/", "_")
            .replace(".json", "")
            .replace(".", "")
    );

    write!(
        test_file,
        include_str!("./tests/test_template"),
        test_name = test_name,
        test_path = test_path.display()
    )
    .unwrap();
}
