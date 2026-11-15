//! The `kat_platform_prism` suite: one test per claim, each named for its ID.
//!
//! Written by the Prism-DevRel builder from the prompts. The theorems are
//! checked by the Lean kernel when `just vv` runs `prismpm verify` on `app/`.
//! What these tests hold fixed is what each theorem is about, so a weakened
//! statement that still verifies fails the gate. One line per claim keeps the
//! generated Rust small, as the template's only-Prism ratchet asks.

/// The top-level declaration `name` of `app/src/<module>.lex.tex` contains `needle`.
fn pins(module: &str, name: &str, needle: &str) {
    let model = std::fs::read_to_string(format!("../../app/src/{module}.lex.tex")).unwrap();
    let found = &model[model.find(&format!("\n  name: {name},\n")).expect(name)..];
    let found = &found[..found.find("\n}\n").expect("closed")];
    assert!(found.contains(needle), "{name}: {needle}\n{found}");
}

#[rustfmt::skip]
mod claims {

    // CV-01: The app never answers ideal when must is above 0
    #[test]
    fn conformance_verdict_cv_01() { super::pins("ConformanceVerdictProofs", "breaksInv001", "body: (((((((project request config) <? 2) && ((project request network) <? 2)) && ((project request must) <=? 999)) && ((project request should) <=? 999)) && (0 <? (project request must))) && ((ConformanceVerdict::conformanceVerdictJudge request) =? (ConformanceVerdict::conformanceVerdictAnswerIdeal)))"); super::pins("ConformanceVerdictProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // CV-02: The app never answers conformant when must is above 0
    #[test]
    fn conformance_verdict_cv_02() { super::pins("ConformanceVerdictProofs", "breaksInv002", "body: (((((((project request config) <? 2) && ((project request network) <? 2)) && ((project request must) <=? 999)) && ((project request should) <=? 999)) && (0 <? (project request must))) && ((ConformanceVerdict::conformanceVerdictJudge request) =? (ConformanceVerdict::conformanceVerdictAnswerConformant)))"); super::pins("ConformanceVerdictProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // CV-03: The app always answers conformant when config is ok and network is reachable and must is at most 0 and should is above 0
    #[test]
    fn conformance_verdict_cv_03() { super::pins("ConformanceVerdictProofs", "breaksInv003", "body: ((((((((((project request config) <? 2) && ((project request network) <? 2)) && ((project request must) <=? 999)) && ((project request should) <=? 999)) && ((project request config) =? (ConformanceVerdict::conformanceVerdictConfigOk))) && ((project request network) =? (ConformanceVerdict::conformanceVerdictNetworkReachable))) && ((project request must) <=? 0)) && (0 <? (project request should))) && (not ((ConformanceVerdict::conformanceVerdictJudge request) =? (ConformanceVerdict::conformanceVerdictAnswerConformant))))"); super::pins("ConformanceVerdictProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // KLC-01: The app never answers permit when axis is other
    #[test]
    fn kappa_label_check_klc_01() { super::pins("KappaLabelCheckProofs", "breaksInv001", "body: (((((((project request axis) <? 6) && ((project request colons) <=? 9)) && ((project request hex) <? 2)) && ((project request digits) <=? 200)) && ((project request axis) =? (KappaLabelCheck::kappaLabelCheckAxisOther))) && ((KappaLabelCheck::kappaLabelCheckJudge request) =? (KappaLabelCheck::kappaLabelCheckAnswerPermit)))"); super::pins("KappaLabelCheckProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // KLC-02: The app never answers permit when hex is other
    #[test]
    fn kappa_label_check_klc_02() { super::pins("KappaLabelCheckProofs", "breaksInv002", "body: (((((((project request axis) <? 6) && ((project request colons) <=? 9)) && ((project request hex) <? 2)) && ((project request digits) <=? 200)) && ((project request hex) =? (KappaLabelCheck::kappaLabelCheckHexOther))) && ((KappaLabelCheck::kappaLabelCheckJudge request) =? (KappaLabelCheck::kappaLabelCheckAnswerPermit)))"); super::pins("KappaLabelCheckProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // KLC-03: The app never answers permit when digits is below 64
    #[test]
    fn kappa_label_check_klc_03() { super::pins("KappaLabelCheckProofs", "breaksInv003", "body: (((((((project request axis) <? 6) && ((project request colons) <=? 9)) && ((project request hex) <? 2)) && ((project request digits) <=? 200)) && ((project request digits) <? 64)) && ((KappaLabelCheck::kappaLabelCheckJudge request) =? (KappaLabelCheck::kappaLabelCheckAnswerPermit)))"); super::pins("KappaLabelCheckProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // US-01: The app never answers committed when label is mismatch and state is not committed
    #[test]
    fn upload_session_us_01() { super::pins("UploadSessionProofs", "breaksInv001", "body: (((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request label) =? (UploadSession::uploadSessionLabelMismatch))) && (not ((project request state) =? (UploadSession::uploadSessionStateCommitted)))) && ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerCommitted)))"); super::pins("UploadSessionProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // US-02: The app always answers abandoned when event is timeout and state is not committed and state is not rejected
    #[test]
    fn upload_session_us_02() { super::pins("UploadSessionProofs", "breaksInv002", "body: ((((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request event) =? (UploadSession::uploadSessionEventTimeout))) && (not ((project request state) =? (UploadSession::uploadSessionStateCommitted)))) && (not ((project request state) =? (UploadSession::uploadSessionStateRejected)))) && (not ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerAbandoned))))"); super::pins("UploadSessionProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // US-03: The app always answers committed when state is committed
    #[test]
    fn upload_session_us_03() { super::pins("UploadSessionProofs", "breaksInv003", "body: ((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request state) =? (UploadSession::uploadSessionStateCommitted))) && (not ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerCommitted))))"); super::pins("UploadSessionProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // US-04: The app always answers rejected when state is rejected
    #[test]
    fn upload_session_us_04() { super::pins("UploadSessionProofs", "breaksInv004", "body: ((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request state) =? (UploadSession::uploadSessionStateRejected))) && (not ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerRejected))))"); super::pins("UploadSessionProofs", "inv004Holds", "statement: ((breaksInv004 request) = false),"); }
    // US-05: The app always answers abandoned when state is abandoned
    #[test]
    fn upload_session_us_05() { super::pins("UploadSessionProofs", "breaksInv005", "body: ((((((project request state) <? 5) && ((project request event) <? 5)) && ((project request label) <? 2)) && ((project request state) =? (UploadSession::uploadSessionStateAbandoned))) && (not ((UploadSession::uploadSessionJudge request) =? (UploadSession::uploadSessionAnswerAbandoned))))"); super::pins("UploadSessionProofs", "inv005Holds", "statement: ((breaksInv005 request) = false),"); }
    // GE-01: The app never answers evict when finalizer is outstanding
    #[test]
    fn gc_eviction_ge_01() { super::pins("GcEvictionProofs", "breaksInv001", "body: (((((((project request tag) <? 2) && ((project request pin) <? 4)) && ((project request finalizer) <? 3)) && ((project request upload) <? 2)) && ((project request finalizer) =? (GcEviction::gcEvictionFinalizerOutstanding))) && ((GcEviction::gcEvictionJudge request) =? (GcEviction::gcEvictionAnswerEvict)))"); super::pins("GcEvictionProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // GE-02: The app never answers evict when tag is bound
    #[test]
    fn gc_eviction_ge_02() { super::pins("GcEvictionProofs", "breaksInv002", "body: (((((((project request tag) <? 2) && ((project request pin) <? 4)) && ((project request finalizer) <? 3)) && ((project request upload) <? 2)) && ((project request tag) =? (GcEviction::gcEvictionTagBound))) && ((GcEviction::gcEvictionJudge request) =? (GcEviction::gcEvictionAnswerEvict)))"); super::pins("GcEvictionProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // GE-03: The app always answers evict when tag is none and pin is expired and finalizer is not outstanding and upload is none
    #[test]
    fn gc_eviction_ge_03() { super::pins("GcEvictionProofs", "breaksInv003", "body: ((((((((((project request tag) <? 2) && ((project request pin) <? 4)) && ((project request finalizer) <? 3)) && ((project request upload) <? 2)) && ((project request tag) =? (GcEviction::gcEvictionTagNone))) && ((project request pin) =? (GcEviction::gcEvictionPinExpired))) && (not ((project request finalizer) =? (GcEviction::gcEvictionFinalizerOutstanding)))) && ((project request upload) =? (GcEviction::gcEvictionUploadNone))) && (not ((GcEviction::gcEvictionJudge request) =? (GcEviction::gcEvictionAnswerEvict))))"); super::pins("GcEvictionProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // VIA-01: The app never answers permit when signature is not verified
    #[test]
    fn vm_image_admission_via_01() { super::pins("VmImageAdmissionProofs", "breaksInv001", "body: (((((((((project request signature) <? 3) && ((project request digest) <? 2)) && ((project request arch) <? 3)) && ((project request source) <? 2)) && ((project request site) <? 2)) && ((project request verifier) <? 2)) && (not ((project request signature) =? (VmImageAdmission::vmImageAdmissionSignatureVerified)))) && ((VmImageAdmission::vmImageAdmissionJudge request) =? (VmImageAdmission::vmImageAdmissionAnswerPermit)))"); super::pins("VmImageAdmissionProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // VIA-02: The app never answers permit when digest is mismatch
    #[test]
    fn vm_image_admission_via_02() { super::pins("VmImageAdmissionProofs", "breaksInv002", "body: (((((((((project request signature) <? 3) && ((project request digest) <? 2)) && ((project request arch) <? 3)) && ((project request source) <? 2)) && ((project request site) <? 2)) && ((project request verifier) <? 2)) && ((project request digest) =? (VmImageAdmission::vmImageAdmissionDigestMismatch))) && ((VmImageAdmission::vmImageAdmissionJudge request) =? (VmImageAdmission::vmImageAdmissionAnswerPermit)))"); super::pins("VmImageAdmissionProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // VIA-03: The app never answers permit when site is airgapped and source is upstream
    #[test]
    fn vm_image_admission_via_03() { super::pins("VmImageAdmissionProofs", "breaksInv003", "body: ((((((((((project request signature) <? 3) && ((project request digest) <? 2)) && ((project request arch) <? 3)) && ((project request source) <? 2)) && ((project request site) <? 2)) && ((project request verifier) <? 2)) && ((project request site) =? (VmImageAdmission::vmImageAdmissionSiteAirgapped))) && ((project request source) =? (VmImageAdmission::vmImageAdmissionSourceUpstream))) && ((VmImageAdmission::vmImageAdmissionJudge request) =? (VmImageAdmission::vmImageAdmissionAnswerPermit)))"); super::pins("VmImageAdmissionProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // VIA-04: The app always answers indeterminate when verifier is error
    #[test]
    fn vm_image_admission_via_04() { super::pins("VmImageAdmissionProofs", "breaksInv004", "body: (((((((((project request signature) <? 3) && ((project request digest) <? 2)) && ((project request arch) <? 3)) && ((project request source) <? 2)) && ((project request site) <? 2)) && ((project request verifier) <? 2)) && ((project request verifier) =? (VmImageAdmission::vmImageAdmissionVerifierError))) && (not ((VmImageAdmission::vmImageAdmissionJudge request) =? (VmImageAdmission::vmImageAdmissionAnswerIndeterminate))))"); super::pins("VmImageAdmissionProofs", "inv004Holds", "statement: ((breaksInv004 request) = false),"); }
    // RG-01: The app never answers go when key is missing
    #[test]
    fn rollout_gate_rg_01() { super::pins("RolloutGateProofs", "breaksInv001", "body: ((((((((project request conformance) <? 4) && ((project request key) <? 2)) && ((project request backup) <? 3)) && ((project request env) <? 2)) && ((project request images) <? 2)) && ((project request key) =? (RolloutGate::rolloutGateKeyMissing))) && ((RolloutGate::rolloutGateJudge request) =? (RolloutGate::rolloutGateAnswerGo)))"); super::pins("RolloutGateProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // RG-02: The app never answers go when conformance is untested
    #[test]
    fn rollout_gate_rg_02() { super::pins("RolloutGateProofs", "breaksInv002", "body: ((((((((project request conformance) <? 4) && ((project request key) <? 2)) && ((project request backup) <? 3)) && ((project request env) <? 2)) && ((project request images) <? 2)) && ((project request conformance) =? (RolloutGate::rolloutGateConformanceUntested))) && ((RolloutGate::rolloutGateJudge request) =? (RolloutGate::rolloutGateAnswerGo)))"); super::pins("RolloutGateProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // RG-03: The app never answers go when backup is not complete
    #[test]
    fn rollout_gate_rg_03() { super::pins("RolloutGateProofs", "breaksInv003", "body: ((((((((project request conformance) <? 4) && ((project request key) <? 2)) && ((project request backup) <? 3)) && ((project request env) <? 2)) && ((project request images) <? 2)) && (not ((project request backup) =? (RolloutGate::rolloutGateBackupComplete)))) && ((RolloutGate::rolloutGateJudge request) =? (RolloutGate::rolloutGateAnswerGo)))"); super::pins("RolloutGateProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // RG-04: The app always answers no-go when conformance is nonconformant
    #[test]
    fn rollout_gate_rg_04() { super::pins("RolloutGateProofs", "breaksInv004", "body: ((((((((project request conformance) <? 4) && ((project request key) <? 2)) && ((project request backup) <? 3)) && ((project request env) <? 2)) && ((project request images) <? 2)) && ((project request conformance) =? (RolloutGate::rolloutGateConformanceNonconformant))) && (not ((RolloutGate::rolloutGateJudge request) =? (RolloutGate::rolloutGateAnswerNoGo))))"); super::pins("RolloutGateProofs", "inv004Holds", "statement: ((breaksInv004 request) = false),"); }
    // SV-01: The app never answers kappa-label-check when viewpoint is leader
    #[test]
    fn stakeholder_view_sv_01() { super::pins("StakeholderViewProofs", "breaksInv001", "body: (((((project request viewpoint) <? 6) && ((project request need) <? 6)) && ((project request viewpoint) =? (StakeholderView::stakeholderViewViewpointLeader))) && ((StakeholderView::stakeholderViewJudge request) =? (StakeholderView::stakeholderViewAnswerKappaLabelCheck)))"); super::pins("StakeholderViewProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // SV-02: The app never answers kappa-label-check when viewpoint is business
    #[test]
    fn stakeholder_view_sv_02() { super::pins("StakeholderViewProofs", "breaksInv002", "body: (((((project request viewpoint) <? 6) && ((project request need) <? 6)) && ((project request viewpoint) =? (StakeholderView::stakeholderViewViewpointBusiness))) && ((StakeholderView::stakeholderViewJudge request) =? (StakeholderView::stakeholderViewAnswerKappaLabelCheck)))"); super::pins("StakeholderViewProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // SV-03: The app always answers rollout-gate when need is rollout and viewpoint is leader
    #[test]
    fn stakeholder_view_sv_03() { super::pins("StakeholderViewProofs", "breaksInv003", "body: ((((((project request viewpoint) <? 6) && ((project request need) <? 6)) && ((project request need) =? (StakeholderView::stakeholderViewNeedRollout))) && ((project request viewpoint) =? (StakeholderView::stakeholderViewViewpointLeader))) && (not ((StakeholderView::stakeholderViewJudge request) =? (StakeholderView::stakeholderViewAnswerRolloutGate))))"); super::pins("StakeholderViewProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // SV-04: The app always answers vm-image-admission when need is image and viewpoint is security
    #[test]
    fn stakeholder_view_sv_04() { super::pins("StakeholderViewProofs", "breaksInv004", "body: ((((((project request viewpoint) <? 6) && ((project request need) <? 6)) && ((project request need) =? (StakeholderView::stakeholderViewNeedImage))) && ((project request viewpoint) =? (StakeholderView::stakeholderViewViewpointSecurity))) && (not ((StakeholderView::stakeholderViewJudge request) =? (StakeholderView::stakeholderViewAnswerVmImageAdmission))))"); super::pins("StakeholderViewProofs", "inv004Holds", "statement: ((breaksInv004 request) = false),"); }
    // FF-01: The app never answers rule-trace when viewpoint is leader
    #[test]
    fn frontend_feature_ff_01() { super::pins("FrontendFeatureProofs", "breaksInv001", "body: (((((project request viewpoint) <? 6) && ((project request surface) <? 7)) && ((project request viewpoint) =? (FrontendFeature::frontendFeatureViewpointLeader))) && ((FrontendFeature::frontendFeatureJudge request) =? (FrontendFeature::frontendFeatureAnswerRuleTrace)))"); super::pins("FrontendFeatureProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // FF-02: The app always answers seal-badge when surface is seal
    #[test]
    fn frontend_feature_ff_02() { super::pins("FrontendFeatureProofs", "breaksInv002", "body: (((((project request viewpoint) <? 6) && ((project request surface) <? 7)) && ((project request surface) =? (FrontendFeature::frontendFeatureSurfaceSeal))) && (not ((FrontendFeature::frontendFeatureJudge request) =? (FrontendFeature::frontendFeatureAnswerSealBadge))))"); super::pins("FrontendFeatureProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // FF-03: The app always answers verdict-card when surface is answer
    #[test]
    fn frontend_feature_ff_03() { super::pins("FrontendFeatureProofs", "breaksInv003", "body: (((((project request viewpoint) <? 6) && ((project request surface) <? 7)) && ((project request surface) =? (FrontendFeature::frontendFeatureSurfaceAnswer))) && (not ((FrontendFeature::frontendFeatureJudge request) =? (FrontendFeature::frontendFeatureAnswerVerdictCard))))"); super::pins("FrontendFeatureProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // BC-01: The app never answers attestation-link when site is airgapped
    #[test]
    fn backend_capability_bc_01() { super::pins("BackendCapabilityProofs", "breaksInv001", "body: (((((project request feature) <? 7) && ((project request site) <? 2)) && ((project request site) =? (BackendCapability::backendCapabilitySiteAirgapped))) && ((BackendCapability::backendCapabilityJudge request) =? (BackendCapability::backendCapabilityAnswerAttestationLink)))"); super::pins("BackendCapabilityProofs", "inv001Holds", "statement: ((breaksInv001 request) = false),"); }
    // BC-02: The app always answers fingerprint-check when feature is seal-badge
    #[test]
    fn backend_capability_bc_02() { super::pins("BackendCapabilityProofs", "breaksInv002", "body: (((((project request feature) <? 7) && ((project request site) <? 2)) && ((project request feature) =? (BackendCapability::backendCapabilityFeatureSealBadge))) && (not ((BackendCapability::backendCapabilityJudge request) =? (BackendCapability::backendCapabilityAnswerFingerprintCheck))))"); super::pins("BackendCapabilityProofs", "inv002Holds", "statement: ((breaksInv002 request) = false),"); }
    // BC-03: The app always answers guest-dispatch when feature is verdict-card
    #[test]
    fn backend_capability_bc_03() { super::pins("BackendCapabilityProofs", "breaksInv003", "body: (((((project request feature) <? 7) && ((project request site) <? 2)) && ((project request feature) =? (BackendCapability::backendCapabilityFeatureVerdictCard))) && (not ((BackendCapability::backendCapabilityJudge request) =? (BackendCapability::backendCapabilityAnswerGuestDispatch))))"); super::pins("BackendCapabilityProofs", "inv003Holds", "statement: ((breaksInv003 request) = false),"); }
    // KPP-01: The architecture description declares every stakeholder, concern, component, edge, viewpoint and view it references
    #[test]
    fn kat_platform_prism_kpp_01() { super::pins("Architecture", "architectureDescriptionHolds", "statement: ((architectureValid) = true),"); super::pins("Architecture", "architectureDescriptionHolds", "proof: decide{}"); }
    // KPP-02: The ISO/IEC 25010:2023 quality model declares all 9 characteristics and 40 subcharacteristics, states each subcharacteristic by a requirement or lists it open, and every parent it names is declared
    #[test]
    fn kat_platform_prism_kpp_02() { super::pins("Quality", "qualityModelHolds", "statement: ((qualityValid) = true),"); super::pins("Quality", "qualityModelHolds", "proof: decide{}"); }
    // KPP-03: Every STRIDE category each component and data flow is open to is answered by a threat or stated open, and no other pair is stated open
    #[test]
    fn kat_platform_prism_kpp_03() { super::pins("Security", "strideEnumerated", "statement: ((securityValid) = true),"); super::pins("Security", "strideEnumerated", "proof: decide{}"); }
    // KPP-04: Every compliance obligation is numbered in order and names its evidence
    #[test]
    fn kat_platform_prism_kpp_04() { super::pins("Compliance", "everyObligationEvidenced", "statement: ((obligationsEvidenced 0 (obligations)) = true),"); super::pins("Compliance", "everyObligationEvidenced", "proof: decide{}"); }
}
