//! The `gc_eviction` suite: one test per scenario in `features/suites/gc_eviction.feature`.
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
    let model = module("app/src/GcEvictionProofs.lex.tex");
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

/// `GE-01`: The app never answers evict when finalizer is outstanding
#[test]
fn gc_eviction_ge_01() {
    pinned(
        "breaksInv001",
        "body: (((((((project request tag) <? 2) && ((project request pin) <? 4)) && ((project request finalizer) <? 3)) && ((project request upload) <? 2)) && ((project request finalizer) =? (GcEviction::gcEvictionFinalizerOutstanding))) && ((GcEviction::gcEvictionJudge request) =? (GcEviction::gcEvictionAnswerEvict)))",
        "inv001Holds",
    );
}

/// `GE-02`: The app never answers evict when tag is bound
#[test]
fn gc_eviction_ge_02() {
    pinned(
        "breaksInv002",
        "body: (((((((project request tag) <? 2) && ((project request pin) <? 4)) && ((project request finalizer) <? 3)) && ((project request upload) <? 2)) && ((project request tag) =? (GcEviction::gcEvictionTagBound))) && ((GcEviction::gcEvictionJudge request) =? (GcEviction::gcEvictionAnswerEvict)))",
        "inv002Holds",
    );
}

/// `GE-03`: The app always answers evict when tag is none and pin is expired and finalizer is not outstanding and upload is none
#[test]
fn gc_eviction_ge_03() {
    pinned(
        "breaksInv003",
        "body: ((((((((((project request tag) <? 2) && ((project request pin) <? 4)) && ((project request finalizer) <? 3)) && ((project request upload) <? 2)) && ((project request tag) =? (GcEviction::gcEvictionTagNone))) && ((project request pin) =? (GcEviction::gcEvictionPinExpired))) && (not ((project request finalizer) =? (GcEviction::gcEvictionFinalizerOutstanding)))) && ((project request upload) =? (GcEviction::gcEvictionUploadNone))) && (not ((GcEviction::gcEvictionJudge request) =? (GcEviction::gcEvictionAnswerEvict))))",
        "inv003Holds",
    );
}
