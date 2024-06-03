//! NOTE: This file has been auto generated using build.rs, don't edit by hand!
//! You can regenerate the tests (which are based on the fixtures in `fixtures/integration_tests/`)
//! using the following command:
//! `cargo run -p m4-test-manager update-snapshots`
use m4::error::GetExitCode;
use m4_test_manager::TestSnapshot;
use similar_asserts::assert_eq;
use std::fs::read_to_string;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

fn init() {
    let _ = env_logger::builder()
        .is_test(true)
        // No timestamp to make it easier to diff output
        .format_timestamp(None)
        .try_init();
}

fn read_test(path: impl AsRef<std::path::Path>) -> TestSnapshot {
    let mut f = std::fs::File::open(path).unwrap();
    let snapshot = TestSnapshot::deserialize(&mut f);
    log::info!("Expecting stdout:\n\x1b[34m{}\x1b[0m", snapshot.stdout,);
    log::info!("Expecting stderr:\n\x1b[34m{}\x1b[0m", snapshot.stderr,);
    log::info!("Expecting status:\n\x1b[34m{}\x1b[0m", snapshot.status,);
    snapshot
}

fn run_command(input: &str) -> std::process::Output {
    // std::process::Command::new("cargo")
    //     .arg("run")
    //     .arg("--")
    //     .arg(input)
    //     .output()
    //     .unwrap()

    // The reason why we run the command using this as a library is so we can run with it built in
    // test configuration, with all the associated conditionally compiled test log instrumentation.
    log::info!(
        "Running command with input {input:?}:\n\x1b[34m{}\x1b[0m",
        read_to_string(input).unwrap()
    );
    let mut stdout: Vec<u8> = Vec::new();
    let mut stderr: Vec<u8> = Vec::new();
    let args = m4::Args {
        file: Some(input.into()),
        ..m4::Args::default()
    };
    let result = m4::run(&mut stdout, &mut stderr, args);
    let status = ExitStatus::from_raw(result.get_exit_code() as i32);
    log::info!("Received status: {status}");
    log::info!(
        "Received stdout:\n\x1b[34m{}\x1b[0m",
        String::from_utf8_lossy(&stdout)
    );
    log::info!(
        "Received stderr:\n\x1b[34m{}\x1b[0m",
        String::from_utf8_lossy(&stderr)
    );
    std::process::Output {
        stdout,
        stderr,
        status,
    }
}

#[test]
fn test_changecom() {
    init();
    let output = run_command("fixtures/integration_tests/changecom.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/changecom.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_changequote() {
    init();
    let output = run_command("fixtures/integration_tests/changequote.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/changequote.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define() {
    init();
    let output = run_command("fixtures/integration_tests/define.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[ignore]
#[test]
fn test_define_eval_order_unquoted() {
    init();
    let output = run_command("fixtures/integration_tests/define_eval_order_unquoted.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_eval_order_unquoted.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[ignore]
#[test]
fn test_define_eval_syntax_order_quoted_evaluated() {
    init();
    let output =
        run_command("fixtures/integration_tests/define_eval_syntax_order_quoted_evaluated.m4");

    let test: TestSnapshot =
        read_test("fixtures/integration_tests/define_eval_syntax_order_quoted_evaluated.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_eval_syntax_order_quoted_unevaluated() {
    init();
    let output =
        run_command("fixtures/integration_tests/define_eval_syntax_order_quoted_unevaluated.m4");

    let test: TestSnapshot =
        read_test("fixtures/integration_tests/define_eval_syntax_order_quoted_unevaluated.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[ignore]
#[test]
fn test_define_eval_syntax_order_unquoted() {
    init();
    let output = run_command("fixtures/integration_tests/define_eval_syntax_order_unquoted.m4");

    let test: TestSnapshot =
        read_test("fixtures/integration_tests/define_eval_syntax_order_unquoted.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_invalid_macro_name() {
    init();
    let output = run_command("fixtures/integration_tests/define_invalid_macro_name.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_invalid_macro_name.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_nested() {
    init();
    let output = run_command("fixtures/integration_tests/define_nested.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_nested.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_number_parsing() {
    init();
    let output = run_command("fixtures/integration_tests/define_number_parsing.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_number_parsing.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_order_defined() {
    init();
    let output = run_command("fixtures/integration_tests/define_order_defined.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_order_defined.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_order_undefined() {
    init();
    let output = run_command("fixtures/integration_tests/define_order_undefined.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_order_undefined.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_pushpopdef_undefine() {
    init();
    let output = run_command("fixtures/integration_tests/define_pushpopdef_undefine.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_pushpopdef_undefine.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_quoted_number_stacked() {
    init();
    let output = run_command("fixtures/integration_tests/define_quoted_number_stacked.m4");

    let test: TestSnapshot =
        read_test("fixtures/integration_tests/define_quoted_number_stacked.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_stacked() {
    init();
    let output = run_command("fixtures/integration_tests/define_stacked.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_stacked.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_undefine_order() {
    init();
    let output = run_command("fixtures/integration_tests/define_undefine_order.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_undefine_order.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_define_unquoted_number_arg() {
    init();
    let output = run_command("fixtures/integration_tests/define_unquoted_number_arg.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/define_unquoted_number_arg.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_dnl() {
    init();
    let output = run_command("fixtures/integration_tests/dnl.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/dnl.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_eval() {
    init();
    let output = run_command("fixtures/integration_tests/eval.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/eval.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_evaluation_order() {
    init();
    let output = run_command("fixtures/integration_tests/evaluation_order.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/evaluation_order.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_forloop_nested() {
    init();
    let output = run_command("fixtures/integration_tests/forloop_nested.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/forloop_nested.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_forloop_simple() {
    init();
    let output = run_command("fixtures/integration_tests/forloop_simple.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/forloop_simple.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_ifelse() {
    init();
    let output = run_command("fixtures/integration_tests/ifelse.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/ifelse.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_include() {
    init();
    let output = run_command("fixtures/integration_tests/include.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/include.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_incr() {
    init();
    let output = run_command("fixtures/integration_tests/incr.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/incr.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_macro_errprint_evaluation() {
    init();
    let output = run_command("fixtures/integration_tests/macro_errprint_evaluation.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/macro_errprint_evaluation.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_macro_errprint_no_evaluation() {
    init();
    let output = run_command("fixtures/integration_tests/macro_errprint_no_evaluation.m4");

    let test: TestSnapshot =
        read_test("fixtures/integration_tests/macro_errprint_no_evaluation.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_macro_errprint_no_evaluation_quoted() {
    init();
    let output = run_command("fixtures/integration_tests/macro_errprint_no_evaluation_quoted.m4");

    let test: TestSnapshot =
        read_test("fixtures/integration_tests/macro_errprint_no_evaluation_quoted.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_recurse() {
    init();
    let output = run_command("fixtures/integration_tests/recurse.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/recurse.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_recursive_defines() {
    init();
    let output = run_command("fixtures/integration_tests/recursive_defines.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/recursive_defines.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_reverse() {
    init();
    let output = run_command("fixtures/integration_tests/reverse.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/reverse.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_shift() {
    init();
    let output = run_command("fixtures/integration_tests/shift.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/shift.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}

#[test]
fn test_sinclude() {
    init();
    let output = run_command("fixtures/integration_tests/sinclude.m4");

    let test: TestSnapshot = read_test("fixtures/integration_tests/sinclude.out");
    assert_eq!(
        output.status,
        std::process::ExitStatus::from_raw(test.status),
        "status (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        test.stdout,
        "stdout (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        test.stderr,
        "stderr (\x1b[31mcurrent\x1b[0m|\x1b[32mexpected\x1b[0m)"
    );
}
