//! The `vm_image_admission` suite: one test per scenario in `features/suites/vm_image_admission.feature`.
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
    let model = module("app/src/VmImageAdmissionProofs.lex.tex");
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

/// `VIA-01`: The app never answers permit when signature is not verified
#[test]
fn vm_image_admission_via_01() {
    pinned(
        "breaksInv001",
        "body: (((((((((project request signature) <? 3) && ((project request digest) <? 2)) && ((project request arch) <? 3)) && ((project request source) <? 2)) && ((project request site) <? 2)) && ((project request verifier) <? 2)) && (not ((project request signature) =? (VmImageAdmission::vmImageAdmissionSignatureVerified)))) && ((VmImageAdmission::vmImageAdmissionJudge request) =? (VmImageAdmission::vmImageAdmissionAnswerPermit)))",
        "inv001Holds",
    );
}

/// `VIA-02`: The app never answers permit when digest is mismatch
#[test]
fn vm_image_admission_via_02() {
    pinned(
        "breaksInv002",
        "body: (((((((((project request signature) <? 3) && ((project request digest) <? 2)) && ((project request arch) <? 3)) && ((project request source) <? 2)) && ((project request site) <? 2)) && ((project request verifier) <? 2)) && ((project request digest) =? (VmImageAdmission::vmImageAdmissionDigestMismatch))) && ((VmImageAdmission::vmImageAdmissionJudge request) =? (VmImageAdmission::vmImageAdmissionAnswerPermit)))",
        "inv002Holds",
    );
}

/// `VIA-03`: The app never answers permit when site is airgapped and source is upstream
#[test]
fn vm_image_admission_via_03() {
    pinned(
        "breaksInv003",
        "body: ((((((((((project request signature) <? 3) && ((project request digest) <? 2)) && ((project request arch) <? 3)) && ((project request source) <? 2)) && ((project request site) <? 2)) && ((project request verifier) <? 2)) && ((project request site) =? (VmImageAdmission::vmImageAdmissionSiteAirgapped))) && ((project request source) =? (VmImageAdmission::vmImageAdmissionSourceUpstream))) && ((VmImageAdmission::vmImageAdmissionJudge request) =? (VmImageAdmission::vmImageAdmissionAnswerPermit)))",
        "inv003Holds",
    );
}

/// `VIA-04`: The app always answers indeterminate when verifier is error
#[test]
fn vm_image_admission_via_04() {
    pinned(
        "breaksInv004",
        "body: (((((((((project request signature) <? 3) && ((project request digest) <? 2)) && ((project request arch) <? 3)) && ((project request source) <? 2)) && ((project request site) <? 2)) && ((project request verifier) <? 2)) && ((project request verifier) =? (VmImageAdmission::vmImageAdmissionVerifierError))) && (not ((VmImageAdmission::vmImageAdmissionJudge request) =? (VmImageAdmission::vmImageAdmissionAnswerIndeterminate))))",
        "inv004Holds",
    );
}
