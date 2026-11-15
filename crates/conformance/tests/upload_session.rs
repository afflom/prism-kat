//! The `upload_session` suite: one test per scenario in `features/suites/upload_session.feature`.
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
    let model = module("app/src/UploadSessionProofs.lex.tex");
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

/// `US-01`: The app never answers committed when label is mismatch and state is not committed
#[test]
fn upload_session_us_01() {
    pinned(
        "breaksInv001",
        "body: (((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request label) =? (UploadSession::uploadSessionLabelMismatch))) && (not ((project request state) =? (UploadSession::uploadSessionStateCommitted)))) && ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerCommitted)))",
        "inv001Holds",
    );
}

/// `US-02`: The app always answers abandoned when event is timeout and state is not committed and state is not rejected
#[test]
fn upload_session_us_02() {
    pinned(
        "breaksInv002",
        "body: ((((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request event) =? (UploadSession::uploadSessionEventTimeout))) && (not ((project request state) =? (UploadSession::uploadSessionStateCommitted)))) && (not ((project request state) =? (UploadSession::uploadSessionStateRejected)))) && (not ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerAbandoned))))",
        "inv002Holds",
    );
}

/// `US-03`: The app always answers committed when state is committed
#[test]
fn upload_session_us_03() {
    pinned(
        "breaksInv003",
        "body: ((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request state) =? (UploadSession::uploadSessionStateCommitted))) && (not ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerCommitted))))",
        "inv003Holds",
    );
}

/// `US-04`: The app always answers rejected when state is rejected
#[test]
fn upload_session_us_04() {
    pinned(
        "breaksInv004",
        "body: ((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request state) =? (UploadSession::uploadSessionStateRejected))) && (not ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerRejected))))",
        "inv004Holds",
    );
}

/// `US-05`: The app always answers abandoned when state is abandoned
#[test]
fn upload_session_us_05() {
    pinned(
        "breaksInv005",
        "body: ((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request state) =? (UploadSession::uploadSessionStateAbandoned))) && (not ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerAbandoned))))",
        "inv005Holds",
    );
}
