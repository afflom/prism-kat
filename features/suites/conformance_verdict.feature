Feature: conformance_verdict

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Decide, as a DMN decision table with hit policy FIRST, the verdict of a kappa-conformance run from whether its configuration was complete, whether the registry was reachable, and how many MUST failures and SHOULD warnings it recorded.

  @CV-01 @build
  Scenario: The app never answers ideal when must is above 0
    Given the model in app/src/ConformanceVerdict.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @CV-02 @build
  Scenario: The app never answers conformant when must is above 0
    Given the model in app/src/ConformanceVerdict.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @CV-03 @build
  Scenario: The app always answers conformant when config is ok and network is reachable and must is at most 0 and should is above 0
    Given the model in app/src/ConformanceVerdict.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003
