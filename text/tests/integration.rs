//
// Copyright (c) 2024 Jeff Garzik
// Copyright (c) 2024 Hemi Labs, Inc.
//
// This file is part of the posixutils-rs project covered under
// the MIT License.  For the full license text, please see the LICENSE
// file in the root directory of this project.
// SPDX-License-Identifier: MIT
//

use chrono::{DateTime, Local};
use plib::{run_test, run_test_with_checker, TestPlan};
use regex::Regex;
use std::fs;
use std::io::Read;
const PR_DATE_TIME_FORMAT: &str = "%b %d %H:%M %Y";

fn expand_test_noargs(test_data: &str, expected_output: &str) {
    run_test(TestPlan {
        cmd: String::from("expand"),
        args: Vec::new(),
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn head_test(test_data: &str, expected_output: &str) {
    run_test(TestPlan {
        cmd: String::from("head"),
        args: Vec::new(),
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn wc_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("wc"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn csplit_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("csplit"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn nl_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("nl"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn pr_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("pr"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn cut_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("cut"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn unexpand_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("unexpand"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn sort_test(
    args: &[&str],
    test_data: &str,
    expected_output: &str,
    expected_exit_code: i32,
    expected_err: &str,
) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("sort"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(expected_err),
        expected_exit_code,
    });
}

fn uniq_test(args: &[&str], test_data: &str, expected_output: &str) {
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    run_test(TestPlan {
        cmd: String::from("uniq"),
        args: str_args,
        stdin_data: String::from(test_data),
        expected_out: String::from(expected_output),
        expected_err: String::from(""),
        expected_exit_code: 0,
    });
}

fn pr_read_test_file(
    output_filename: &str,
    input_filename: &str,
    header: Option<&str>,
    date: Option<String>,
) -> String {
    let re = Regex::new(r"<DATE>|<FILENAME>").unwrap();

    let dt_string = date.unwrap_or_else(|| {
        let metadata = fs::metadata(input_filename).unwrap();
        let last_modified_time = metadata.modified().unwrap();
        let dt: DateTime<Local> = last_modified_time.into();
        dt.format(PR_DATE_TIME_FORMAT).to_string()
    });

    let mut file = fs::File::open(output_filename).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let s = re.replace_all(&buf, |captures: &regex::Captures<'_>| -> String {
        let marker = captures.get(0).unwrap();
        match marker.as_str() {
            "<DATE>" => dt_string.clone(),
            "<FILENAME>" => header.unwrap_or(input_filename).to_string(),
            _ => panic!("Unknown pattern"),
        }
    });

    s.to_string()
}

#[test]
fn test_expand_basic() {
    expand_test_noargs("", "");
    expand_test_noargs("a\tb\tc\n", "a       b       c\n");
}

#[test]
fn test_head_basic() {
    head_test("a\nb\nc\nd\n", "a\nb\nc\nd\n");
    head_test(
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n",
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n",
    );
    head_test(
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n0\na\n",
        "1\n2\n3\n4\n5\n6\n7\n8\n9\n0\n",
    );
}

#[test]
fn test_wc_empty() {
    wc_test(&["-c"], "", "0\n");
    wc_test(&["-l"], "", "0\n");
    wc_test(&["-w"], "", "0\n");
}

#[test]
fn test_wc_one() {
    wc_test(&["-c"], "x", "1\n");
    wc_test(&["-l"], "x", "0\n");
    wc_test(&["-w"], "x", "1\n");
}

#[test]
fn test_wc_two() {
    wc_test(&["-c"], "x y\n", "4\n");
    wc_test(&["-l"], "x y\n", "1\n");
    wc_test(&["-w"], "x y\n", "2\n");
}

#[test]
fn test_csplit_text_by_lines() {
    csplit_test(
        &["-f", "text", "-", "5", "{3}"],
        "1sdfghnm
2sadsgdhjmf
3zcxbncvm vbm
4asdbncv
5adsbfdgfnfm
6sdfcvncbmcg
7zsdgdgfndcgmncg
8asdbsfdndcgmn
9sfbdxgfndcgmncgmn
10dvsd
11
12
13
14
15
16
17",
        "43\n\n57\n\n31\n\n14\n\n",
    );
    std::fs::remove_file("text00").unwrap();
    std::fs::remove_file("text01").unwrap();
    std::fs::remove_file("text02").unwrap();
    std::fs::remove_file("text03").unwrap();
}

#[test]
fn test_csplit_text_by_lines_from_file() {
    csplit_test(
        &["-f", "text_f", "tests/assets/test_file.txt", "5", "{3}"],
        "",
        "43\n\n57\n\n31\n\n14\n\n",
    );
    std::fs::remove_file("text_f00").unwrap();
    std::fs::remove_file("text_f01").unwrap();
    std::fs::remove_file("text_f02").unwrap();
    std::fs::remove_file("text_f03").unwrap();
}

#[test]
fn test_csplit_c_code_by_regex() {
    csplit_test(
        &[
            "-f",
            "code_c",
            "tests/assets/test_file_c",
            r"%main\(%",
            "/^}/+1",
            "{3}",
        ],
        "",
        "59\n\n53\n\n53\n\n54\n\n",
    );
    std::fs::remove_file("code_c00").unwrap();
    std::fs::remove_file("code_c01").unwrap();
    std::fs::remove_file("code_c02").unwrap();
    std::fs::remove_file("code_c03").unwrap();
}

#[test]
fn test_csplit_c_code_by_regex_negative_offset() {
    csplit_test(
        &[
            "-f",
            "code_c_neg",
            "tests/assets/test_file_c",
            r"%main\(%",
            "/^}/-2",
            "{3}",
        ],
        "",
        "12\n\n46\n\n52\n\n107\n\n",
    );
    std::fs::remove_file("code_c_neg00").unwrap();
    std::fs::remove_file("code_c_neg01").unwrap();
    std::fs::remove_file("code_c_neg02").unwrap();
    std::fs::remove_file("code_c_neg03").unwrap();
}

#[test]
fn test_csplit_c_code_by_regex_suppress() {
    csplit_test(
        &[
            "-s",
            "-f",
            "code_c_s",
            "tests/assets/test_file_c",
            r"%main\(%",
            "/^}/+1",
            "{3}",
        ],
        "",
        "",
    );
    std::fs::remove_file("code_c_s00").unwrap();
    std::fs::remove_file("code_c_s01").unwrap();
    std::fs::remove_file("code_c_s02").unwrap();
    std::fs::remove_file("code_c_s03").unwrap();
}

#[test]
fn test_csplit_c_code_by_regex_with_number() {
    csplit_test(
        &[
            "-f",
            "code_c_n",
            "-n",
            "3",
            "tests/assets/test_file_c",
            r"%main\(%",
            "/^}/+1",
            "{3}",
        ],
        "",
        "59\n\n53\n\n53\n\n54\n\n",
    );
    std::fs::remove_file("code_c_n000").unwrap();
    std::fs::remove_file("code_c_n001").unwrap();
    std::fs::remove_file("code_c_n002").unwrap();
    std::fs::remove_file("code_c_n003").unwrap();
}

#[test]
fn test_csplit_regex_by_empty_lines() {
    csplit_test(
        &["-f", "empty_lines", "tests/assets/empty_line.txt", "/^$/"],
        "",
        "6\n\n7\n\n",
    );
    std::fs::remove_file("empty_lines00").unwrap();
    std::fs::remove_file("empty_lines01").unwrap();
}

#[test]
fn test_csplit_regex_would_infloop() {
    csplit_test(
        &[
            "-f",
            "would_infloop",
            "tests/assets/would_infloop.txt",
            "/a/-1",
            "{*}",
        ],
        "",
        "2\n\n",
    );
    std::fs::remove_file("would_infloop00").unwrap();
}

#[test]
fn test_csplit_regex_in_uniq() {
    csplit_test(
        &["-f", "in_uniq", "tests/assets/in_uniq", "/^$/", "{*}"],
        "",
        "6\n\n10\n\n8\n\n9\n\n",
    );
    std::fs::remove_file("in_uniq00").unwrap();
    std::fs::remove_file("in_uniq01").unwrap();
    std::fs::remove_file("in_uniq02").unwrap();
    std::fs::remove_file("in_uniq03").unwrap();
}

#[test]
fn test_csplit_regex_in_uniq_2() {
    csplit_test(
        &["-f", "in_uniq_2_", "tests/assets/in_uniq", "/^$/-1", "{*}"],
        "",
        "3\n\n10\n\n8\n\n12\n\n",
    );
    std::fs::remove_file("in_uniq_2_00").unwrap();
    std::fs::remove_file("in_uniq_2_01").unwrap();
    std::fs::remove_file("in_uniq_2_02").unwrap();
    std::fs::remove_file("in_uniq_2_03").unwrap();
}

#[test]
fn test_csplit_regex_in_uniq_3() {
    csplit_test(
        &["-f", "in_uniq_3_", "tests/assets/in_uniq", "/^$/1", "{*}"],
        "",
        "7\n\n10\n\n8\n\n8\n\n",
    );
    std::fs::remove_file("in_uniq_3_00").unwrap();
    std::fs::remove_file("in_uniq_3_01").unwrap();
    std::fs::remove_file("in_uniq_3_02").unwrap();
    std::fs::remove_file("in_uniq_3_03").unwrap();
}

#[test]
fn test_csplit_regex_in_seq() {
    csplit_test(
        &["-f", "in_seq", "tests/assets/in_seq", "/2/", "/4/", "/6/"],
        "",
        "1\n\n3\n\n3\n\n1\n\n",
    );
    std::fs::remove_file("in_seq00").unwrap();
    std::fs::remove_file("in_seq01").unwrap();
    std::fs::remove_file("in_seq02").unwrap();
    std::fs::remove_file("in_seq03").unwrap();
}

#[test]
fn test_nl_justification() {
    nl_test(&["-n", "ln"], "a", "1     \ta\n");
    nl_test(&["-n", "rn"], "b", "     1\tb\n");
    nl_test(&["-n", "rz"], "c", "000001\tc\n");
}

#[test]
fn test_nl_newlines_at_end() {
    nl_test(&[], "a\n\n", "     1\ta\n       \n");
}

#[test]
fn test_nl_starting_number() {
    nl_test(&["-v", "2"], "a", "     2\ta\n");
}

#[test]
fn test_nl_number_increment() {
    let input = "\\:\\:\\:\nheader\n\\:\\:\nbody\n\\:\nfooter";
    // Without -p, the counter resets on delimiters
    nl_test(
        &["-h", "a", "-f", "a"],
        input,
        "\n     1\theader\n\n     1\tbody\n\n     1\tfooter\n",
    );

    // With -p, the counter increments even when encountering delimiters
    nl_test(
        &["-h", "a", "-f", "a", "-p"],
        input,
        "\n     1\theader\n\n     2\tbody\n\n     3\tfooter\n",
    );

    nl_test(
        &["-h", "a", "-f", "a", "-p", "-i", "2"],
        input,
        "\n     1\theader\n\n     3\tbody\n\n     5\tfooter\n",
    );
}

#[test]
fn test_nl_delimiter() {
    // Single character delimiter should be appended with the default second
    // character, ':'
    nl_test(
        &["-h", "a", "-f", "a", "-d", "?"],
        "?:?:?:\nheader\n?:?:\nbody\n?:\nfooter",
        "\n     1\theader\n\n     1\tbody\n\n     1\tfooter\n",
    );

    nl_test(
        &["-h", "a", "-f", "a", "-d", "?!"],
        "?!?!?!\nheader\n?!?!\nbody\n?!\nfooter",
        "\n     1\theader\n\n     1\tbody\n\n     1\tfooter\n",
    );
}

#[test]
fn test_nl_regex() {
    // NOTE: The implementation has better regex support than the reference.
    // `nl -b p.+ng` would fail to match the words ending with "ng" in the
    // original whereas it would in this Rust implementation. Might be
    // considered a bug?
    nl_test(
        &["-b", "p.*ng"],
        "something\nanything\neverything\ncat\ndog",
        "     1\tsomething\n     2\tanything\n     3\teverything\n       cat\n       dog\n",
    );
}

#[test]
fn test_pr_single_column() {
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file(
        "tests/pr/lorem_ipsum_output_single_column.txt",
        input,
        None,
        None,
    );
    pr_test(&[&input], "", &output);
}

#[test]
fn test_pr_multi_column() {
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file("tests/pr/lorem_ipsum_output_9_cols.txt", input, None, None);
    pr_test(&["-9", &input], "", &output);
}

#[test]
fn test_pr_multi_column_across() {
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file(
        "tests/pr/lorem_ipsum_output_2_cols_across.txt",
        input,
        None,
        None,
    );
    pr_test(&["-2", "-a", &input], "", &output);
}

#[test]
fn test_pr_multi_column_merge() {
    // This test requires the current timestamp.
    //
    // It's possible to inject the current timestamp to the expected output
    // before calling `pr_test` but that would cause spurious errors when the
    // minute portion changes in between now and when the process is actually
    // ran:
    //
    // Apr 18 14:12 2024
    // Apr 18 14:13 2024

    let input = "tests/pr/lorem_ipsum.txt";
    let args = &["+1:1", "-m", &input, &input, &input];
    let str_args: Vec<String> = args.iter().map(|s| String::from(*s)).collect();

    let test_plan = TestPlan {
        cmd: String::from("pr"),
        args: str_args,
        stdin_data: String::from(""),
        expected_out: String::from(""),
        expected_err: String::from(""),
        expected_exit_code: 0,
    };

    run_test_with_checker(test_plan, |_, output| {
        let stdout = String::from_utf8_lossy(&output.stdout);

        // MMM++++++++++YYYY
        let re = Regex::new(r"\w{3}.+\d{4}").unwrap();
        let captures = re.captures(&stdout).unwrap();
        let date = captures.get(0).unwrap().as_str();

        let expected_out = pr_read_test_file(
            "tests/pr/lorem_ipsum_output_merge.txt",
            input,
            None,
            Some(date.to_string()),
        );

        assert_eq!(stdout, expected_out);
    });
}

#[test]
fn test_pr_page_skip() {
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file(
        "tests/pr/numbers_output_9_cols_page15.txt",
        input,
        None,
        None,
    );
    pr_test(&["-9", "+15", &input], "", &output);
}

#[test]
fn test_pr_header_replacement() {
    let header = "custom";
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file(
        "tests/pr/lorem_ipsum_output_page_1.txt",
        input,
        Some(header),
        None,
    );
    pr_test(&["+1:1", "-h", header, &input], "", &output);
}

#[test]
fn test_pr_limit_lines() {
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file("tests/pr/numbers_output_l20.txt", input, None, None);
    pr_test(&["+1:1", "-l20", &input], "", &output);
}

#[test]
fn test_pr_limit_lines_trim() {
    // Lines <= 10 behave like -t is used
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file("tests/pr/numbers_output_l10.txt", input, None, None);
    pr_test(&["+1:1", "-l10", &input], "", &output);
}

#[test]
fn test_pr_omit_header() {
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file("tests/pr/numbers_output_omit_header.txt", input, None, None);
    pr_test(&["+1:1", "-l20", "-t", &input], "", &output);
}

#[test]
fn test_pr_offset() {
    let input = "tests/pr/numbers.txt";
    let output = pr_read_test_file("tests/pr/numbers_output_offset.txt", input, None, None);
    pr_test(&["+1:1", "-o7", &input], "", &output);
}

#[test]
fn test_pr_width() {
    let input = "tests/pr/long_line.txt";
    let output = pr_read_test_file("tests/pr/long_line_output_w72.txt", input, None, None);
    pr_test(&["-2", "-t", "-w72", &input], "", &output);

    let output = pr_read_test_file("tests/pr/long_line_output_w200.txt", input, None, None);
    pr_test(&["-2", "-t", "-w200", &input], "", &output);

    // -s without -w causes the width to be 512
    let output = pr_read_test_file("tests/pr/long_line_output_s.txt", input, None, None);
    pr_test(&["-2", "-t", "-s", &input], "", &output);
}

#[test]
fn test_pr_number_line() {
    let input = "tests/pr/lorem_ipsum.txt";
    let output = pr_read_test_file(
        "tests/pr/lorem_ipsum_output_number_line.txt",
        input,
        None,
        None,
    );
    pr_test(&["-9", "-n3", &input], "", &output);
}

#[test]
fn test_pr_expand_and_replace() {
    let input = "tests/pr/spaces_and_tabs.txt";
    let output = pr_read_test_file(
        "tests/pr/spaces_and_tabs_expand_and_replace.txt",
        input,
        None,
        None,
    );
    pr_test(&["-i?3", "-e", "-t", &input], "", &output);
}

#[cfg(test)]
mod tests {
    use crate::cut_test;

    #[test]
    fn test_cut_0() {
        cut_test(&["-c", "1-3", "-"], "abcdef", "abc\n");
    }

    #[test]
    fn test_cut_1() {
        cut_test(
            &["-d", ":", "-f", "1,3", "-"],
            "field1:field2:field3",
            "field1:field3\n",
        );
    }

    #[test]
    fn test_cut_2() {
        cut_test(&["-d", ":", "-f", "1,3-", "-"], "a:b:c\n", "a:c\n");
    }

    #[test]
    fn test_cut_3() {
        cut_test(&["-d", ":", "-f", "2-", "-"], "a:b:c\n", "b:c\n");
    }

    #[test]
    fn test_cut_4() {
        cut_test(&["-d", ":", "-f", "4", "-"], "a:b:c\n", "\n");
    }

    #[test]
    fn test_cut_5() {
        cut_test(&["-d", ":", "-f", "4", "-"], "", "");
    }

    #[test]
    fn test_cut_6() {
        cut_test(&["-c", "4", "-"], "123\n", "\n");
    }

    #[test]
    fn test_cut_7() {
        cut_test(&["-c", "4", "-"], "123", "\n");
    }

    #[test]
    fn test_cut_8() {
        cut_test(&["-c", "4", "-"], "123\n1", "\n\n");
    }

    #[test]
    fn test_cut_9() {
        cut_test(&["-c", "4", "-"], "", "");
    }

    #[test]
    fn test_cut_a() {
        cut_test(&["-s", "-d", ":", "-f", "3-", "-"], "a:b:c\n", "c\n");
    }

    #[test]
    fn test_cut_b() {
        cut_test(&["-s", "-d", ":", "-f", "2,3", "-"], "a:b:c\n", "b:c\n");
    }

    #[test]
    fn test_cut_c() {
        cut_test(&["-s", "-d", ":", "-f", "1,3", "-"], "a:b:c\n", "a:c\n");
    }

    #[test]
    fn test_cut_e() {
        cut_test(&["-s", "-d", ":", "-f", "3-", "-"], "a:b:c:\n", "c:\n");
    }

    #[test]
    fn test_cut_f() {
        cut_test(&["-s", "-d", ":", "-f", "3-4", "-"], "a:b:c:\n", "c:\n");
    }

    #[test]
    fn test_cut_h() {
        cut_test(&["-s", "-d", ":", "-f", "2,3", "-"], "abc\n", "");
    }

    #[test]
    fn test_cut_i() {
        cut_test(&["-d", ":", "-f", "1-3", "-"], ":::\n", "::\n");
    }

    #[test]
    fn test_cut_j() {
        cut_test(&["-d", ":", "-f", "1-4", "-"], ":::\n", ":::\n");
    }

    #[test]
    fn test_cut_k() {
        cut_test(&["-d", ":", "-f", "2-3", "-"], ":::\n", ":\n");
    }

    #[test]
    fn test_cut_l() {
        cut_test(&["-d", ":", "-f", "2-4", "-"], ":::\n", "::\n");
    }

    #[test]
    fn test_cut_m() {
        cut_test(&["-s", "-d", ":", "-f", "1-3", "-"], ":::\n", "::\n");
    }

    #[test]
    fn test_cut_n() {
        cut_test(&["-s", "-d", ":", "-f", "1-4", "-"], ":::\n", ":::\n");
    }

    #[test]
    fn test_cut_o() {
        cut_test(&["-s", "-d", ":", "-f", "2-3", "-"], ":::\n", ":\n");
    }

    #[test]
    fn test_cut_p() {
        cut_test(&["-s", "-d", ":", "-f", "2-4", "-"], ":::\n", "::\n");
    }

    #[test]
    fn test_cut_q() {
        cut_test(&["-s", "-d", ":", "-f", "2-4", "-"], ":::\n:\n", "::\n\n");
    }

    #[test]
    fn test_cut_r() {
        cut_test(&["-s", "-d", ":", "-f", "2-4", "-"], ":::\n:1\n", "::\n1\n");
    }

    #[test]
    fn test_cut_s() {
        cut_test(
            &["-s", "-d", ":", "-f", "1-4", "-"],
            ":::\n:a\n",
            ":::\n:a\n",
        );
    }

    #[test]
    fn test_cut_t() {
        cut_test(&["-s", "-d", ":", "-f", "3-", "-"], ":::\n:1\n", ":\n\n");
    }

    #[test]
    fn test_cut_u() {
        cut_test(&["-s", "-f", "3-", "-"], "", "");
    }

    #[test]
    fn test_cut_v() {
        cut_test(&["-f", "3-", "-"], "", "");
    }

    #[test]
    fn test_cut_w() {
        cut_test(&["-b", "1", "-"], "", "");
    }

    #[test]
    fn test_cut_x() {
        cut_test(&["-s", "-d", ":", "-f", "2-4", "-"], ":\n", "\n");
    }

    #[test]
    fn test_cut_newline_1() {
        cut_test(&["-f", "1-", "-"], "a\nb", "a\nb\n");
    }

    #[test]
    fn test_cut_newline_2() {
        cut_test(&["-f", "1-", "-"], "", "");
    }

    #[test]
    fn test_cut_newline_3() {
        cut_test(&["-d", ":", "-f", "1", "-"], "a:1\nb:2\n", "a\nb\n");
    }

    #[test]
    fn test_cut_newline_4() {
        cut_test(&["-d", ":", "-f", "1", "-"], "a:1\nb:2", "a\nb\n");
    }

    #[test]
    fn test_cut_newline_5() {
        cut_test(&["-d", ":", "-f", "2", "-"], "a:1\nb:2\n", "1\n2\n");
    }

    #[test]
    fn test_cut_newline_6() {
        cut_test(&["-d", ":", "-f", "2", "-"], "a:1\nb:2", "1\n2\n");
    }

    #[test]
    fn test_cut_newline_7() {
        cut_test(&["-s", "-d", ":", "-f", "1", "-"], "a:1\nb:2", "a\nb\n");
    }

    #[test]
    fn test_cut_newline_8() {
        cut_test(&["-s", "-d", ":", "-f", "1", "-"], "a:1\nb:2\n", "a\nb\n");
    }

    #[test]
    fn test_cut_newline_9() {
        cut_test(&["-s", "-d", ":", "-f", "1", "-"], "a1\nb2", "");
    }

    #[test]
    fn test_cut_newline_10() {
        cut_test(
            &["-s", "-d", ":", "-f", "1,2", "-"],
            "a:1\nb:2",
            "a:1\nb:2\n",
        );
    }

    #[test]
    fn test_cut_newline_11() {
        cut_test(
            &["-s", "-d", ":", "-f", "1,2", "-"],
            "a:1\nb:2\n",
            "a:1\nb:2\n",
        );
    }

    #[test]
    fn test_cut_newline_12() {
        cut_test(&["-s", "-d", ":", "-f", "1", "-"], "a:1\nb:", "a\nb\n");
    }

    #[test]
    fn test_cut_newline_13() {
        cut_test(&["-d", ":", "-f", "1-", "-"], "a1:\n:", "a1:\n:\n");
    }

    #[test]
    fn test_cut_newline_14() {
        cut_test(&["-d", "\n", "-f", "1-", "-"], "\nb", "\nb\n");
    }

    #[test]
    fn test_out_delim_1() {
        cut_test(&["-d", ":", "-c", "1-3,5-", "-"], "abcdefg\n", "abc:efg\n");
    }

    #[test]
    fn test_out_delim_2() {
        cut_test(
            &["-d", ":", "-c", "1-3,2,5-", "-"],
            "abcdefg\n",
            "abc:efg\n",
        );
    }

    #[test]
    fn test_out_delim_3() {
        cut_test(
            &["-d", ":", "-c", "1-3,2-4,6", "-"],
            "abcdefg\n",
            "abcd:f\n",
        );
    }

    #[test]
    fn test_out_delim_3a() {
        cut_test(
            &["-d", ":", "-c", "1-3,2-4,6-", "-"],
            "abcdefg\n",
            "abcd:fg\n",
        );
    }

    #[test]
    fn test_out_delim_4() {
        cut_test(&["-d", ":", "-c", "4-,2-3", "-"], "abcdefg\n", "bc:defg\n");
    }

    #[test]
    fn test_out_delim_5() {
        cut_test(&["-d", ":", "-c", "2-3,4-", "-"], "abcdefg\n", "bc:defg\n");
    }

    #[test]
    fn test_out_delim_6() {
        cut_test(&["-d", ":", "-c", "2,1-3", "-"], "abc\n", "abc\n");
    }

    #[test]
    fn test_od_abut() {
        cut_test(&["-d", ":", "-b", "1-2,3-4", "-"], "abcd\n", "ab:cd\n");
    }

    #[test]
    fn test_od_overlap() {
        cut_test(&["-d", ":", "-b", "1-2,2", "-"], "abc\n", "ab\n");
    }

    #[test]
    fn test_od_overlap2() {
        cut_test(&["-d", ":", "-b", "1-2,2-", "-"], "abc\n", "abc\n");
    }

    #[test]
    fn test_od_overlap3() {
        cut_test(&["-d", ":", "-b", "1-3,2-", "-"], "abcd\n", "abcd\n");
    }

    #[test]
    fn test_od_overlap4() {
        cut_test(&["-d", ":", "-b", "1-3,2-3", "-"], "abcd\n", "abc\n");
    }

    #[test]
    fn test_od_overlap5() {
        cut_test(&["-d", ":", "-b", "1-3,1-4", "-"], "abcde\n", "abcd\n");
    }
}

#[cfg(test)]
mod sort_tests {
    use crate::sort_test;

    #[test]
    fn test_n1() {
        sort_test(&["-n"], ".01\n0\n", "0\n.01\n", 0, "");
    }

    #[test]
    fn test_n2() {
        sort_test(&["-n"], ".02\n.01\n", ".01\n.02\n", 0, "");
    }

    #[test]
    fn test_n3() {
        sort_test(&["-n"], ".02\n.00\n", ".00\n.02\n", 0, "");
    }

    #[test]
    fn test_n4() {
        sort_test(&["-n"], ".02\n.000\n", ".000\n.02\n", 0, "");
    }

    #[test]
    fn test_n5() {
        sort_test(&["-n"], ".021\n.029\n", ".021\n.029\n", 0, "");
    }

    #[test]
    fn test_n6() {
        sort_test(&["-n"], ".02\n.0*\n", ".0*\n.02\n", 0, "");
    }

    #[test]
    fn test_n7() {
        sort_test(&["-n"], ".02\n.*\n", ".*\n.02\n", 0, "");
    }

    #[test]
    fn test_n8a() {
        sort_test(&["-n", "-k1,1"], ".0a\n.0b\n", ".0a\n.0b\n", 0, "");
    }

    #[test]
    fn test_n8b() {
        sort_test(&["-n", "-k1,1"], ".0b\n.0a\n", ".0b\n.0a\n", 0, "");
    }

    #[test]
    fn test_n9a() {
        sort_test(&["-n", "-k1,1"], ".000a\n.000b\n", ".000a\n.000b\n", 0, "");
    }

    #[test]
    fn test_n9b() {
        sort_test(&["-n", "-k1,1"], ".000b\n.000a\n", ".000b\n.000a\n", 0, "");
    }

    #[test]
    fn test_n10a() {
        sort_test(&["-n", "-k1,1"], ".00a\n.000b\n", ".00a\n.000b\n", 0, "");
    }

    #[test]
    fn test_n10b() {
        sort_test(&["-n", "-k1,1"], ".00b\n.000a\n", ".00b\n.000a\n", 0, "");
    }

    #[test]
    fn test_n11a() {
        sort_test(&["-n", "-k1,1"], ".01a\n.010\n", ".01a\n.010\n", 0, "");
    }

    #[test]
    fn test_n11b() {
        sort_test(&["-n", "-k1,1"], ".010\n.01a\n", ".010\n.01a\n", 0, "");
    }

    #[test]
    fn test_02a() {
        sort_test(&["-c"], "A\nB\nC\n", "", 0, "");
    }

    #[test]
    fn test_02b() {
        sort_test(
            &["-c"],
            "A\nC\nB\n",
            "",
            1,
            "The order of the lines is not correct on line 2:`C`\n",
        );
    }

    #[test]
    fn test_02c() {
        sort_test(&["-c", "-k1,1"], "a\na b\n", "", 0, "");
    }

    #[test]
    fn test_02d() {
        sort_test(&["-C"], "A\nB\nC\n", "", 0, "");
    }

    #[test]
    fn test_02e() {
        sort_test(
            &["-C"],
            "A\nC\nB\n",
            "",
            1,
            "The order of the lines is not correct\n",
        );
    }

    #[test]
    fn test_02m() {
        sort_test(&["-cu"], "A\nA\n", "", 1, "Duplicate key was found! `A`\n");
    }

    #[test]
    fn test_02n() {
        sort_test(&["-cu"], "A\nB\n", "", 0, "");
    }

    #[test]
    fn test_02o() {
        sort_test(
            &["-cu"],
            "A\nB\nB\n",
            "",
            1,
            "Duplicate key was found! `B`\n",
        );
    }

    #[test]
    fn test_02p() {
        sort_test(
            &["-cu"],
            "B\nA\nB\n",
            "",
            1,
            "Duplicate key was found! `B`\n",
        );
    }

    #[test]
    fn test_03a() {
        sort_test(&["-k1", "-"], "B\nA\n", "A\nB\n", 0, "");
    }

    #[test]
    fn test_03b() {
        sort_test(&["-k1,1", "-"], "B\nA\n", "A\nB\n", 0, "");
    }

    #[test]
    fn test_03c() {
        sort_test(&["-k1", "-k2", "-"], "A b\nA a\n", "A a\nA b\n", 0, "");
    }

    #[test]
    fn test_03d() {
        // Fail with a diagnostic when -k specifies field == 0.
        sort_test(&["-k0", "-"], "", "", 1, "the key can't be zero.\n");
    }

    #[test]
    fn test_04a() {
        sort_test(&["-nc", "-"], "2\n11\n", "", 0, "");
    }

    #[test]
    fn test_04b() {
        sort_test(&["-n", "-"], "11\n2\n", "2\n11\n", 0, "");
    }

    #[test]
    fn test_04c() {
        sort_test(&["-k1n", "-"], "11\n2\n", "2\n11\n", 0, "");
    }

    #[test]
    fn test_04d() {
        sort_test(&["-k1", "-"], "11\n2\n", "11\n2\n", 0, "");
    }

    #[test]
    fn test_04e() {
        sort_test(
            &["-k2", "-"],
            "ignored B\nz-ig A\n",
            "z-ig A\nignored B\n",
            0,
            "",
        );
    }

    #[test]
    fn test_05a() {
        sort_test(&["-k1,2", "-"], "A B\nA A\n", "A A\nA B\n", 0, "");
    }

    #[test]
    fn test_05b() {
        sort_test(&["-k1,2", "-"], "A B A\nA A Z\n", "A A Z\nA B A\n", 0, "");
    }

    #[test]
    fn test_05c() {
        sort_test(
            &["-k1", "-k2", "-"],
            "A B A\nA A Z\n",
            "A A Z\nA B A\n",
            0,
            "",
        );
    }

    #[test]
    fn test_05d() {
        sort_test(&["-k2,2", "-"], "A B A\nA A Z\n", "A A Z\nA B A\n", 0, "");
    }

    #[test]
    fn test_05e() {
        sort_test(&["-k2,2", "-"], "A B Z\nA A A\n", "A A A\nA B Z\n", 0, "");
    }

    #[test]
    fn test_05f() {
        sort_test(&["-k2,2", "-"], "A B A\nA A Z\n", "A A Z\nA B A\n", 0, "");
    }

    #[test]
    fn test_07a() {
        sort_test(&["-k2,3", "-"], "9 a b\n7 a a\n", "7 a a\n9 a b\n", 0, "");
    }

    #[test]
    fn test_07b() {
        sort_test(&["-k2,3"], "a a b\nz a a\n", "z a a\na a b\n", 0, "");
    }

    #[test]
    fn test_07c() {
        sort_test(&["-k2,3", "-"], "y k b\nz k a\n", "z k a\ny k b\n", 0, "");
    }

    #[test]
    fn test_07e() {
        // ensure a character position of 0 includes whole field
        sort_test(&["-k2,3.0", "-"], "a a b\nz a a\n", "z a a\na a b\n", 0, "");
    }

    #[test]
    fn test_07f() {
        // ensure fields with end position before start are error
        sort_test(
            &["-n", "-k1.3,1.1", "-"],
            "a 2\nb 1\n",
            "",
            1,
            "keys fields with end position before start!\n",
        );
    }

    #[test]
    fn test_08a() {
        // report an error for '.' without following char spec
        sort_test(
            &["-k", "2.,3", "-"],
            "",
            "",
            1,
            "cannot parse integer from empty string\n",
        );
    }

    #[test]
    fn test_08b() {
        // report an error for ',' without following POS2
        sort_test(
            &["-k", "2,", "-"],
            "",
            "",
            1,
            "cannot parse integer from empty string\n",
        );
    }

    #[test]
    fn test_09b() {
        sort_test(&["-n", "-"], "1e2\n2e1\n", "1e2\n2e1\n", 0, "");
    }

    #[test]
    fn test_09c() {
        sort_test(&["-n", "-"], "2e1\n1e2\n", "1e2\n2e1\n", 0, "");
    }

    #[test]
    fn test_10a() {
        sort_test(
            &["-t", ":", "-k2.2,2.2", "-"],
            ":ba\n:ab\n",
            ":ba\n:ab\n",
            0,
            "",
        );
    }

    #[test]
    fn test_10c() {
        sort_test(
            &["-t", ":", "-k2.2,2.2", "-"],
            ":ab\n:ba\n",
            ":ba\n:ab\n",
            0,
            "",
        );
    }

    #[test]
    fn test_10a0() {
        sort_test(&["-k2.3,2.3", "-"], "z ba\nz ab\n", "z ba\nz ab\n", 0, "");
    }

    #[test]
    fn test_10a1() {
        sort_test(&["-k1.2,1.2", "-"], "ba\nab\n", "ba\nab\n", 0, "");
    }

    #[test]
    fn test_10a2() {
        sort_test(
            &["-b", "-k2.2,2.2", "-"],
            "z ba\nz ab\n",
            "z ba\nz ab\n",
            0,
            "",
        );
    }

    #[test]
    fn test_10e() {
        sort_test(&["-k1.2,1.2", "-"], "ab\nba\n", "ba\nab\n", 0, "");
    }

    #[test]
    fn test_11a() {
        // Exercise bug re using -b to skip trailing blanks.
        sort_test(
            &["-t:", "-k1,1b", "-k2,2", "-"],
            "a\t:a\na :b\n",
            "a\t:a\na :b\n",
            0,
            "",
        );
    }

    #[test]
    fn test_11b() {
        sort_test(
            &["-t:", "-k1,1b", "-k2,2", "-"],
            "a :b\na\t:a\n",
            "a\t:a\na :b\n",
            0,
            "",
        );
    }

    #[test]
    fn test_11c() {
        sort_test(
            &["-t:", "-k2,2b", "-k3,3", "-"],
            "z:a\t:a\na :b\n",
            "z:a\t:a\na :b\n",
            0,
            "",
        );
    }

    #[test]
    fn test_11d() {
        sort_test(
            &["-t:", "-k2,2b", "-k3,3", "-"],
            "z:a :b\na\t:a\n",
            "a\t:a\nz:a :b\n",
            0,
            "",
        );
    }

    #[test]
    fn test_14a() {
        sort_test(
            &["-d", "-u", "-"],
            "mal\nmal-\nmala\n",
            "mal\nmala\n",
            0,
            "",
        );
    }

    #[test]
    fn test_14b() {
        sort_test(
            &["-f", "-d", "-u", "-"],
            "mal\nmal-\nmala\n",
            "mal\nmala\n",
            0,
            "",
        );
    }

    #[test]
    fn test_15a() {
        sort_test(&["-i", "-u", "-"], "a\na\t\n", "a\n", 0, "");
    }

    #[test]
    fn test_15b() {
        sort_test(&["-i", "-u", "-"], "a\n\ta\n", "a\n", 0, "");
    }

    #[test]
    fn test_15c() {
        sort_test(&["-i", "-u", "-"], "a\t\na\n", "a\t\n", 0, "");
    }

    #[test]
    fn test_15d() {
        sort_test(&["-i", "-u", "-"], "\ta\na\n", "\ta\n", 0, "");
    }

    #[test]
    fn test_15e() {
        sort_test(&["-i", "-u", "-"], "a\n\t\t\t\t\ta\t\t\t\t\n", "a\n", 0, "");
    }

    #[test]
    fn test_18a() {
        sort_test(&["-k1.1,1.2n", "-"], " 901\n100\n", " 901\n100\n", 0, "");
    }

    #[test]
    fn test_18b() {
        sort_test(
            &["-b", "-k1.1,1.2n", "-"],
            " 901\n100\n",
            " 901\n100\n",
            0,
            "",
        );
    }

    #[test]
    fn test_18c() {
        sort_test(&["-k1.1,1.2nb", "-"], " 901\n100\n", "100\n 901\n", 0, "");
    }

    #[test]
    fn test_18d() {
        sort_test(&["-k1.1b,1.2n", "-"], " 901\n100\n", " 901\n100\n", 0, "");
    }

    #[test]
    fn test_18e() {
        sort_test(
            &["-nb", "-k1.1,1.2", "-"],
            " 901\n100\n",
            "100\n 901\n",
            0,
            "",
        );
    }

    #[test]
    fn test_18f() {
        sort_test(&["-k1,1b", "-"], "a  y\na z\n", "a  y\na z\n", 0, "");
    }
    #[test]
    fn test_19b() {
        sort_test(
            &["-k1,1", "-k2nr", "-"],
            "b 2\nb 1\nb 3\n",
            "b 3\nb 2\nb 1\n",
            0,
            "",
        );
    }

    #[test]
    fn test_20a() {
        sort_test(
            &["-"],
            "_________U__free\n_________U__malloc\n_________U__abort\n\
         _________U__memcpy\n_________U__memset\n_________U_dyld_stub_binding_helper\n\
         _________U__malloc\n_________U___iob\n_________U__abort\n_________U__fprintf\n",
            "_________U___iob\n_________U__abort\n_________U__abort\n\
         _________U__fprintf\n_________U__free\n_________U__malloc\n\
         _________U__malloc\n_________U__memcpy\n_________U__memset\n\
         _________U_dyld_stub_binding_helper\n",
            0,
            "",
        );
    }

    #[test]
    fn test_21a() {
        sort_test(&["-"], "A\na\n_\n", "A\n_\na\n", 0, "");
    }

    #[test]
    fn test_21b() {
        sort_test(&["-f", "-"], "A\na\n_\n", "A\na\n_\n", 0, "");
    }

    #[test]
    fn test_21c() {
        sort_test(&["-f", "-"], "a\nA\n_\n", "A\na\n_\n", 0, "");
    }

    #[test]
    fn test_21d() {
        sort_test(&["-f", "-"], "_\na\nA\n", "A\na\n_\n", 0, "");
    }

    #[test]
    fn test_21e() {
        sort_test(&["-f", "-"], "a\n_\nA\n", "A\na\n_\n", 0, "");
    }

    #[test]
    fn test_21g() {
        sort_test(&["-f", "-u", "-"], "a\n_\n", "a\n_\n", 0, "");
    }

    #[test]
    fn test_22a() {
        sort_test(
            &["-k2,2fd", "-k1,1r", "-"],
            "3 b\n4 B\n",
            "4 B\n3 b\n",
            0,
            "",
        );
    }

    #[test]
    fn test_neg_nls() {
        sort_test(&["-n", "-"], "-1\n-9\n", "-9\n-1\n", 0, "");
    }

    #[test]
    fn test_nul_nls() {
        sort_test(&["-"], "\0b\n\0a\n", "\0a\n\0b\n", 0, "");
    }

    #[test]
    fn test_use_nl() {
        sort_test(&["-"], "\n\t\n", "\n\t\n", 0, "");
    }

    #[test]
    fn test_files_sort_1() {
        sort_test(
            &["tests/assets/empty_line.txt", "tests/assets/in_uniq"],
            "",
            "\n\n\n\nXX\nXX\nXX\nYY\nYY\nYY\na\na\nb\nb\nc\nd\nd\nd\nline 1\nline 3\n",
            0,
            "",
        );
    }

    #[test]
    fn test_files_sort_2() {
        sort_test(
            &["-n", "tests/assets/in_seq", "tests/assets/test_file.txt"],
            "",
            "1\n1sdfghnm\n2\n2sadsgdhjmf\n3\n3zcxbncvm vbm\n4\n4asdbncv\n5\n5adsbfdgfnfm\n6\n6sdfcvncbmcg\n7zsdgdgfndcgmncg\n8asdbsfdndcgmn\n9sfbdxgfndcgmncgmn\n10dvsd\n11\n12\n13\n14\n15\n16\n17\n",
            0,
            "",
        );
    }
}

#[cfg(test)]
mod unexpand_tests {
    use crate::unexpand_test;

    #[test]
    fn unexpand_test_1() {
        unexpand_test(
            &["-t", "4,8,12"],
            "    Apple\n        Banana\n            Cherry\n                Date",
            "\tApple\n\t\tBanana\n\t\t\tCherry\n\t\t\t    Date\n",
        );
    }

    #[test]
    fn unexpand_test_2() {
        unexpand_test(
            &["-"],
            "    Apple\n        Banana\n            Cherry\n                Date",
            "    Apple\n\tBanana\n\t    Cherry\n\t        Date\n",
        );
    }

    #[test]
    fn unexpand_test_3() {
        unexpand_test(
            &["-t", "8"],
            "        leading spaces\n",
            "\tleading spaces\n",
        );
    }

    #[test]
    fn unexpand_test_4() {
        unexpand_test(&["-t", "4"], "    leading spaces\n", "\tleading spaces\n");
    }

    #[test]
    fn unexpand_test_5() {
        unexpand_test(
            &["-t", "8"],
            "text    with spaces\n",
            "text    with spaces\n",
        );
    }

    #[test]
    fn unexpand_test_6() {
        unexpand_test(
            &["-a"],
            "text        with                spaces",
            "text\twith\t\tspaces\n",
        );

    }

}

mod uniq_tests {
    use crate::uniq_test;
    #[test]
    fn test_uniq_2() {
        uniq_test(&[], "a\na\n", "a\n");
    }

    #[test]
    fn test_uniq_3() {
        uniq_test(&[], "a\na", "a\n");
    }

    #[test]
    fn test_uniq_4() {
        uniq_test(&[], "a\nb", "a\nb\n");
    }

    #[test]
    fn test_uniq_5() {
        uniq_test(&[], "a\na\nb", "a\nb\n");
    }

    #[test]
    fn test_uniq_6() {
        uniq_test(&[], "b\na\na\n", "b\na\n");
    }

    #[test]
    fn test_uniq_7() {
        uniq_test(&[], "a\nb\nc\n", "a\nb\nc\n");
    }

    #[test]
    fn test_uniq_8() {
        uniq_test(&[], "ö\nv\n", "ö\nv\n");
    }

    #[test]
    fn test_uniq_9() {
        uniq_test(&["-u"], "a\na\n", "");
    }

    #[test]
    fn test_uniq_10() {
        uniq_test(&["-u"], "a\nb\n", "a\nb\n");
    }

    #[test]
    fn test_uniq_11() {
        uniq_test(&["-u"], "a\nb\na\n", "a\nb\na\n");
    }

    #[test]
    fn test_uniq_12() {
        uniq_test(&["-u"], "a\na\n", "");
    }

    #[test]
    fn test_uniq_13() {
        uniq_test(&["-u"], "a\na\n", "");
    }

    #[test]
    fn test_uniq_20() {
        uniq_test(&["-d"], "a\na\n", "a\n");
    }

    #[test]
    fn test_uniq_21() {
        uniq_test(&["-d"], "a\nb\n", "");
    }

    #[test]
    fn test_uniq_22() {
        uniq_test(&["-d"], "a\nb\na\n", "");
    }

    #[test]
    fn test_uniq_23() {
        uniq_test(&["-d"], "a\na\nb\n", "a\n");
    }

    #[test]
    fn test_uniq_24() {
        uniq_test(&["-f", "1"], "a a\nb a\n", "a a\n");
    }

    #[test]
    fn test_uniq_25() {
        uniq_test(&["-f", "1"], "a a\nb b\n", "a a\nb b\n");
    }

    #[test]
    fn test_uniq_26() {
        uniq_test(&["-f", "1"], "a a a\nb a c\n", "a a a\nb a c\n");
    }

    #[test]
    fn test_uniq_27() {
        uniq_test(&["-f", "1"], "b a\na a\n", "b a\n");
    }

    #[test]
    fn test_uniq_28() {
        uniq_test(&["-f", "2"], "a a c\nb a c\n", "a a c\n");
    }

    #[test]
    fn test_uniq_29() {
        uniq_test(&["-s", "1"], "aaa\naaa\n", "aaa\n");
    }

    #[test]
    fn test_uniq_30() {
        uniq_test(&["-s", "2"], "baa\naaa\n", "baa\n");
    }

    #[test]
    fn test_uniq_31() {
        uniq_test(&["-f", "1", "-s", "1"], "a aaa\nb ab\n", "a aaa\nb ab\n");
    }

    #[test]
    fn test_uniq_32() {
        uniq_test(&["-f", "1", "-s", "1"], "a aaa\nb aaa\n", "a aaa\n");
    }

    #[test]
    fn test_uniq_33() {
        uniq_test(&["-f", "1", "-s", "1"], "a aaa\nb ab\n", "a aaa\nb ab\n");
    }

    #[test]
    fn test_uniq_34() {
        uniq_test(&["-f", "1", "-s", "1"], "a aaa\nb aaa\n", "a aaa\n");
    }

    #[test]
    fn test_uniq_35() {
        uniq_test(&["-s", "0"], "abc\nabcd\n", "abc\nabcd\n");
    }

    #[test]
    fn test_uniq_36() {
        uniq_test(&["-s", "0"], "abc\n", "abc\n");
    }

    #[test]
    fn test_uniq_37() {
        uniq_test(&[], "a\0a\na\n", "a\0a\na\n");
    }

    #[test]
    fn test_uniq_38() {
        uniq_test(&[], "a\ta\na a\n", "a\ta\na a\n");
    }

    #[test]
    fn test_uniq_39() {
        uniq_test(&["-f", "1"], "a\ta\na a\n", "a\ta\na a\n");
    }

    #[test]
    fn test_uniq_40() {
        uniq_test(&["-f", "2"], "a\ta a\na a a\n", "a\ta a\n");
    }

    #[test]
    fn test_uniq_41() {
        uniq_test(&["-f", "1"], "a\ta\na\ta\n", "a\ta\n");
    }

    #[test]
    fn test_uniq_42() {
        uniq_test(&["-c"], "a\nb\n", "1 a\n1 b\n");
    }

    #[test]
    fn test_uniq_43() {
        uniq_test(&["-c"], "a\na\n", "2 a\n");
    }
}
