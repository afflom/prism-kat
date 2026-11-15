Feature: kappa_label_check

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Decide, as an XACML policy decision, whether a candidate kappa-label is admitted, from its sigma-axis token, the number of colons, whether its digest is lowercase hex, and how many hex digits it carries.

  @KLC-01 @build
  Scenario: The app never answers permit when axis is other
    Given the model in app/src/KappaLabelCheck.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @KLC-02 @build
  Scenario: The app never answers permit when hex is other
    Given the model in app/src/KappaLabelCheck.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @KLC-03 @build
  Scenario: The app never answers permit when digits is below 64
    Given the model in app/src/KappaLabelCheck.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003
