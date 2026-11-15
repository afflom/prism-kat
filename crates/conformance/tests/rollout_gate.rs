//! The `rollout_gate` suite: one test per scenario in `features/suites/rollout_gate.feature`.
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
    let model = module("app/src/RolloutGateProofs.lex.tex");
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

/// `RG-01`: The app never answers go when key is missing
#[test]
fn rollout_gate_rg_01() {
    pinned(
        "breaksInv001",
        "body: ((((((((project request conformance) <? 4) && ((project request key) <? 2)) && ((project request backup) <? 3)) && ((project request env) <? 2)) && ((project request images) <? 2)) && ((project request key) =? (RolloutGate::rolloutGateKeyMissing))) && ((RolloutGate::rolloutGateJudge request) =? (RolloutGate::rolloutGateAnswerGo)))",
        "inv001Holds",
    );
}

/// `RG-02`: The app never answers go when conformance is untested
#[test]
fn rollout_gate_rg_02() {
    pinned(
        "breaksInv002",
        "body: ((((((((project request conformance) <? 4) && ((project request key) <? 2)) && ((project request backup) <? 3)) && ((project request env) <? 2)) && ((project request images) <? 2)) && ((project request conformance) =? (RolloutGate::rolloutGateConformanceUntested))) && ((RolloutGate::rolloutGateJudge request) =? (RolloutGate::rolloutGateAnswerGo)))",
        "inv002Holds",
    );
}

/// `RG-03`: The app never answers go when backup is not complete
#[test]
fn rollout_gate_rg_03() {
    pinned(
        "breaksInv003",
        "body: ((((((((project request conformance) <? 4) && ((project request key) <? 2)) && ((project request backup) <? 3)) && ((project request env) <? 2)) && ((project request images) <? 2)) && (not ((project request backup) =? (RolloutGate::rolloutGateBackupComplete)))) && ((RolloutGate::rolloutGateJudge request) =? (RolloutGate::rolloutGateAnswerGo)))",
        "inv003Holds",
    );
}

/// `RG-04`: The app always answers no-go when conformance is nonconformant
#[test]
fn rollout_gate_rg_04() {
    pinned(
        "breaksInv004",
        "body: ((((((((project request conformance) <? 4) && ((project request key) <? 2)) && ((project request backup) <? 3)) && ((project request env) <? 2)) && ((project request images) <? 2)) && ((project request conformance) =? (RolloutGate::rolloutGateConformanceNonconformant))) && (not ((RolloutGate::rolloutGateJudge request) =? (RolloutGate::rolloutGateAnswerNoGo))))",
        "inv004Holds",
    );
}
