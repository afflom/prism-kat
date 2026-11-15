//! The `stakeholder_view` suite: one test per scenario in `features/suites/stakeholder_view.feature`.
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
    let model = module("app/src/StakeholderViewProofs.lex.tex");
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

/// `SV-01`: The app never answers kappa-label-check when viewpoint is leader
#[test]
fn stakeholder_view_sv_01() {
    pinned(
        "breaksInv001",
        "body: (((((project request viewpoint) <? 6) && ((project request need) <? 6)) && ((project request viewpoint) =? (StakeholderView::stakeholderViewViewpointLeader))) && ((StakeholderView::stakeholderViewJudge request) =? (StakeholderView::stakeholderViewAnswerKappaLabelCheck)))",
        "inv001Holds",
    );
}

/// `SV-02`: The app never answers kappa-label-check when viewpoint is business
#[test]
fn stakeholder_view_sv_02() {
    pinned(
        "breaksInv002",
        "body: (((((project request viewpoint) <? 6) && ((project request need) <? 6)) && ((project request viewpoint) =? (StakeholderView::stakeholderViewViewpointBusiness))) && ((StakeholderView::stakeholderViewJudge request) =? (StakeholderView::stakeholderViewAnswerKappaLabelCheck)))",
        "inv002Holds",
    );
}

/// `SV-03`: The app always answers rollout-gate when need is rollout and viewpoint is leader
#[test]
fn stakeholder_view_sv_03() {
    pinned(
        "breaksInv003",
        "body: ((((((project request viewpoint) <? 6) && ((project request need) <? 6)) && ((project request need) =? (StakeholderView::stakeholderViewNeedRollout))) && ((project request viewpoint) =? (StakeholderView::stakeholderViewViewpointLeader))) && (not ((StakeholderView::stakeholderViewJudge request) =? (StakeholderView::stakeholderViewAnswerRolloutGate))))",
        "inv003Holds",
    );
}

/// `SV-04`: The app always answers vm-image-admission when need is image and viewpoint is security
#[test]
fn stakeholder_view_sv_04() {
    pinned(
        "breaksInv004",
        "body: ((((((project request viewpoint) <? 6) && ((project request need) <? 6)) && ((project request need) =? (StakeholderView::stakeholderViewNeedImage))) && ((project request viewpoint) =? (StakeholderView::stakeholderViewViewpointSecurity))) && (not ((StakeholderView::stakeholderViewJudge request) =? (StakeholderView::stakeholderViewAnswerVmImageAdmission))))",
        "inv004Holds",
    );
}
