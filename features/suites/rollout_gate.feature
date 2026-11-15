Feature: rollout_gate

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Decide, as a DMN decision table with hit policy FIRST, whether a kappa-registry platform may roll out, from its conformance verdict, its signing key backup, its data backup, whether its environment is pinned and whether its images were admitted.

  @RG-01 @build
  Scenario: The app never answers go when key is missing
    Given the model in app/src/RolloutGate.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @RG-02 @build
  Scenario: The app never answers go when conformance is untested
    Given the model in app/src/RolloutGate.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @RG-03 @build
  Scenario: The app never answers go when backup is not complete
    Given the model in app/src/RolloutGate.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003

  @RG-04 @build
  Scenario: The app always answers no-go when conformance is nonconformant
    Given the model in app/src/RolloutGate.lex.tex
    When the Lean kernel checks inv004Holds during prismpm verify
    Then no request meets breaksInv004
