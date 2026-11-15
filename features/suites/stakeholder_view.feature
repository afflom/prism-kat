Feature: stakeholder_view

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Decide, as a DMN decision table with hit policy FIRST, which decision app answers a stakeholder's question, from the stakeholder's viewpoint and the kind of question they ask, or that the question is outside their view.

  @SV-01 @build
  Scenario: The app never answers kappa-label-check when viewpoint is leader
    Given the model in app/src/StakeholderView.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @SV-02 @build
  Scenario: The app never answers kappa-label-check when viewpoint is business
    Given the model in app/src/StakeholderView.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @SV-03 @build
  Scenario: The app always answers rollout-gate when need is rollout and viewpoint is leader
    Given the model in app/src/StakeholderView.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003

  @SV-04 @build
  Scenario: The app always answers vm-image-admission when need is image and viewpoint is security
    Given the model in app/src/StakeholderView.lex.tex
    When the Lean kernel checks inv004Holds during prismpm verify
    Then no request meets breaksInv004
