//! The `kappa_label_check` suite: one test per scenario in `features/suites/kappa_label_check.feature`.
//!
//! Written by the Prism-DevRel builder from the app's prompt. The theorems are
//! checked by the Lean kernel when `just vv` runs `prismpm verify` on `app/`.
//! What these tests hold fixed is what each theorem is about, so a weakened
//! statement that still verifies fails the gate.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below the root")
        .to_path_buf()
}

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let start = model
        .find(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    let end = model[start..]
        .find("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"));
    &model[start..start + end]
}

fn module(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).expect("the module")
}

/// The condition no request may meet, and the theorem that says none does.
fn pinned(breaks: &str, condition: &str, theorem: &str) {
    let model = module("app/src/KappaLabelCheckProofs.lex.tex");
    let found = declaration(&model, breaks);
    assert!(
        found.contains(condition),
        "the condition of {breaks} changed:\n{found}"
    );
    let holds = declaration(&model, theorem);
    let statement = format!("statement: (({breaks} request) = false),");
    assert!(
        holds.contains(&statement),
        "{theorem} must state that no request meets {breaks}:\n{holds}"
    );
}

/// `KLC-01`: The app never answers permit when axis is other
#[test]
fn kappa_label_check_klc_01() {
    pinned(
        "breaksInv001",
        "body: (((((((project request axis) <? 6) && ((project request colons) <=? 9)) && ((project request hex) <? 2)) && ((project request digits) <=? 200)) && ((project request axis) =? (KappaLabelCheck::kappaLabelCheckAxisOther))) && ((KappaLabelCheck::kappaLabelCheckJudge request) =? (KappaLabelCheck::kappaLabelCheckAnswerPermit)))",
        "inv001Holds",
    );
}

/// `KLC-02`: The app never answers permit when hex is other
#[test]
fn kappa_label_check_klc_02() {
    pinned(
        "breaksInv002",
        "body: (((((((project request axis) <? 6) && ((project request colons) <=? 9)) && ((project request hex) <? 2)) && ((project request digits) <=? 200)) && ((project request hex) =? (KappaLabelCheck::kappaLabelCheckHexOther))) && ((KappaLabelCheck::kappaLabelCheckJudge request) =? (KappaLabelCheck::kappaLabelCheckAnswerPermit)))",
        "inv002Holds",
    );
}

/// `KLC-03`: The app never answers permit when digits is below 64
#[test]
fn kappa_label_check_klc_03() {
    pinned(
        "breaksInv003",
        "body: (((((((project request axis) <? 6) && ((project request colons) <=? 9)) && ((project request hex) <? 2)) && ((project request digits) <=? 200)) && ((project request digits) <? 64)) && ((KappaLabelCheck::kappaLabelCheckJudge request) =? (KappaLabelCheck::kappaLabelCheckAnswerPermit)))",
        "inv003Holds",
    );
}
