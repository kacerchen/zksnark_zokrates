#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_and() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/and.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_ecc_edwardsAdd() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/ecc/edwardsAdd.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_ecc_edwardsCompress() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/ecc/edwardsCompress.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_ecc_edwardsOnCurve() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/ecc/edwardsOnCurve.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_ecc_edwardsOrderCheck() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/ecc/edwardsOrderCheck.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_ecc_edwardsScalarMult() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/ecc/edwardsScalarMult.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_ecc_proofOfOwnership() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/ecc/proofOfOwnership.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_hashes_pedersen_512bit() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/hashes/pedersen/512bit.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_hashes_pedersen_6bit() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/hashes/pedersen/6bit.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_hashes_sha256_512bit() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/hashes/sha256/512bit.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_hashes_sha256_512bitPacked() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/hashes/sha256/512bitPacked.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_hashes_sha256_512bitPacked2() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/hashes/sha256/512bitPacked2.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_hashes_sha256_512bitPadded() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/hashes/sha256/512bitPadded.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_hashes_utils_256bitsDirectionHelper() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/hashes/utils/256bitsDirectionHelper.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_or() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/or.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_signatures_verifyEddsa() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/signatures/verifyEddsa.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_utils_multiplexer_256bit() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/utils/multiplexer/256bit.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_utils_multiplexer_2bit() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/utils/multiplexer/2bit.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_utils_multiplexer_lookup1bit() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/utils/multiplexer/lookup1bit.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_utils_multiplexer_lookup2bit() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/utils/multiplexer/lookup2bit.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_utils_multiplexer_lookup3bitSigned() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/utils/multiplexer/lookup3bitSigned.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_utils_pack_nonStrictUnpack256() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/utils/pack/nonStrictUnpack256.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_utils_pack_pack128() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/utils/pack/pack128.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

#[test]
#[ignore]
#[allow(non_snake_case)]
fn test_utils_pack_unpack128() {
    use zokrates_field::field::{Field, FieldPrime};
    use std::path::PathBuf;
    use zokrates_fs_resolver::resolve;
    use zokrates_core::compile::compile;
    use std::fs::File;
    use std::io::{BufReader, Read};

    let t: utils::Tests = serde_json::from_reader(
        BufReader::new(
            File::open(
                &PathBuf::from("tests/bench/utils/pack/unpack128.json")
            ).unwrap()
        )
    ).unwrap();

    let mut code_reader = BufReader::new(File::open(&t.entry_point).unwrap());

    let bin = compile(
        &mut code_reader,
        Some(t.entry_point.parent().unwrap().to_str().unwrap().to_string()),
        Some(resolve)
    ).unwrap();

    for test in t.tests.into_iter() {
        let input = &test.input.values;
        let output = bin.execute(&input.iter().map(|v| FieldPrime::try_from_dec_str(&v.clone()).unwrap()).collect());

        match utils::compare(output, test.output) {
            Err(e) => {
                let mut code = File::open(&t.entry_point).unwrap();
                let mut s = String::new();
                code.read_to_string(&mut s).unwrap();
                let context = format!("\n{}\nCalled with input ({})\n", s, input.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "));
                panic!("{}{}", context, e)
            },
            Ok(..) => {}
        };
    }
}

