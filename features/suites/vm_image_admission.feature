Feature: vm_image_admission

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Decide, as an XACML policy decision, whether a VM disk image pulled as an OCI artifact is admitted to a cluster, from its signature, whether its bytes match its digest, its architecture, where it was pulled from, whether the site is air-gapped and whether the verifier answered.

  @VIA-01 @build
  Scenario: The app never answers permit when signature is not verified
    Given the model in app/src/VmImageAdmission.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @VIA-02 @build
  Scenario: The app never answers permit when digest is mismatch
    Given the model in app/src/VmImageAdmission.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @VIA-03 @build
  Scenario: The app never answers permit when site is airgapped and source is upstream
    Given the model in app/src/VmImageAdmission.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003

  @VIA-04 @build
  Scenario: The app always answers indeterminate when verifier is error
    Given the model in app/src/VmImageAdmission.lex.tex
    When the Lean kernel checks inv004Holds during prismpm verify
    Then no request meets breaksInv004
