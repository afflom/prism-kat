//! The `conformance_verdict` suite: one test per scenario in `features/suites/conformance_verdict.feature`.
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
    let model = module("app/src/ConformanceVerdictProofs.lex.tex");
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

/// `CV-01`: The app never answers ideal when must is above 0
#[test]
fn conformance_verdict_cv_01() {
    pinned(
        "breaksInv001",
        "body: (((((((project request config) <? 2) && ((project request network) <? 2)) && ((project request must) <=? 999)) && ((project request should) <=? 999)) && (0 <? (project request must))) && ((ConformanceVerdict::conformanceVerdictJudge request) =? (ConformanceVerdict::conformanceVerdictAnswerIdeal)))",
        "inv001Holds",
    );
}

/// `CV-02`: The app never answers conformant when must is above 0
#[test]
fn conformance_verdict_cv_02() {
    pinned(
        "breaksInv002",
        "body: (((((((project request config) <? 2) && ((project request network) <? 2)) && ((project request must) <=? 999)) && ((project request should) <=? 999)) && (0 <? (project request must))) && ((ConformanceVerdict::conformanceVerdictJudge request) =? (ConformanceVerdict::conformanceVerdictAnswerConformant)))",
        "inv002Holds",
    );
}

/// `CV-03`: The app always answers conformant when config is ok and network is reachable and must is at most 0 and should is above 0
#[test]
fn conformance_verdict_cv_03() {
    pinned(
        "breaksInv003",
        "body: ((((((((((project request config) <? 2) && ((project request network) <? 2)) && ((project request must) <=? 999)) && ((project request should) <=? 999)) && ((project request config) =? (ConformanceVerdict::conformanceVerdictConfigOk))) && ((project request network) =? (ConformanceVerdict::conformanceVerdictNetworkReachable))) && ((project request must) <=? 0)) && (0 <? (project request should))) && (not ((ConformanceVerdict::conformanceVerdictJudge request) =? (ConformanceVerdict::conformanceVerdictAnswerConformant))))",
        "inv003Holds",
    );
}
