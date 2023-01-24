#![allow(non_snake_case)]

use jloader::class_file::ClassFile;
use std::{
    error::Error,
    fs::{read_to_string, File},
    io::Read,
};

const TEST_PATH: &str = "test_verified_output/";

fn load_class(path: &str) -> Result<ClassFile, Box<dyn Error>> {
    let mut class_file: File = File::open(path).expect("Failed to open file");
    let mut contents = vec![00; class_file.metadata().unwrap().len() as usize];
    class_file
        .read_exact(&mut contents)
        .expect("Failed to read bytes");
    ClassFile::from_bytes(&contents)
}

#[test]
fn test_aiq() -> Result<(), Box<dyn Error>> {
    let output = read_to_string(TEST_PATH.to_string() + "aiq/aiq.class.txt")?;
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "aiq/aiq.class"))?.to_pretty_fmt(),
        output
    );
    Ok(())
}

#[test]
fn test_java_basic_main() -> Result<(), Box<dyn Error>> {
    let output = read_to_string(TEST_PATH.to_string() + "basic_main_java_test/test.class.txt")?;
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "basic_main_java_test/test.class"))?.to_pretty_fmt(),
        output
    );
    Ok(())
}

#[test]
fn test_kotlin_basic_main() -> Result<(), Box<dyn Error>> {
    let output = read_to_string(TEST_PATH.to_string() + "basic_main_kotlin_test/TestKt.class.txt")?;
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "basic_main_kotlin_test/TestKt.class"))?
            .to_pretty_fmt(),
        output
    );
    Ok(())
}

#[test]
fn test_scala_basic_main() -> Result<(), Box<dyn Error>> {
    let output = read_to_string(TEST_PATH.to_string() + "basic_main_scala_test/test$.class.txt")?;
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "basic_main_scala_test/test$.class"))?.to_pretty_fmt(),
        output
    );
    Ok(())
}

#[test]
fn test_java_annotations() -> Result<(), Box<dyn Error>> {
    let test_class_output =
        read_to_string(TEST_PATH.to_string() + "annotations_java_test/test.class.txt")?;
    let atRuntime_class_output =
        read_to_string(TEST_PATH.to_string() + "annotations_java_test/atRuntime.class.txt")?;
    let atCompile_class_output =
        read_to_string(TEST_PATH.to_string() + "annotations_java_test/atCompile.class.txt")?;
    let atRuntimeType_class_output =
        read_to_string(TEST_PATH.to_string() + "annotations_java_test/atRuntimeType.class.txt")?;
    let atCompileType_class_output =
        read_to_string(TEST_PATH.to_string() + "annotations_java_test/atCompileType.class.txt")?;
    let invisibleAnnotation_class_output = read_to_string(
        TEST_PATH.to_string() + "annotations_java_test/invisibleAnnotation.class.txt",
    )?;
    let visibleAnnotation_class_output = read_to_string(
        TEST_PATH.to_string() + "annotations_java_test/visibleAnnotation.class.txt",
    )?;
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "annotations_java_test/test.class"))?.to_pretty_fmt(),
        test_class_output
    );
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "annotations_java_test/atRuntime.class"))?
            .to_pretty_fmt(),
        atRuntime_class_output
    );
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "annotations_java_test/atCompile.class"))?
            .to_pretty_fmt(),
        atCompile_class_output
    );
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "annotations_java_test/atRuntimeType.class"))?
            .to_pretty_fmt(),
        atRuntimeType_class_output
    );
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "annotations_java_test/atCompileType.class"))?
            .to_pretty_fmt(),
        atCompileType_class_output
    );
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "annotations_java_test/invisibleAnnotation.class"))?
            .to_pretty_fmt(),
        invisibleAnnotation_class_output
    );
    assert_eq!(
        load_class(&(TEST_PATH.to_string() + "annotations_java_test/visibleAnnotation.class"))?
            .to_pretty_fmt(),
        visibleAnnotation_class_output
    );
    Ok(())
}
