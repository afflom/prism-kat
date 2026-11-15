Feature: backend_capability

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Decide, as a DMN decision table with hit policy FIRST, which backend capability serves a front end feature at a site that is online or air-gapped: the verified guest, the static model record, acceptance-vector replay, the fingerprint check, the proof bundled with the site, or a link to the CI attestation.

  @BC-01 @build
  Scenario: The app never answers attestation-link when site is airgapped
    Given the model in app/src/BackendCapability.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @BC-02 @build
  Scenario: The app always answers fingerprint-check when feature is seal-badge
    Given the model in app/src/BackendCapability.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @BC-03 @build
  Scenario: The app always answers guest-dispatch when feature is verdict-card
    Given the model in app/src/BackendCapability.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003
