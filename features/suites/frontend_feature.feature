Feature: frontend_feature

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Decide, as a DMN decision table with hit policy FIRST, which front end feature renders one surface of an answer (the answer, its reason, its rules, its proof, the journey, the words or the seal) for a stakeholder viewpoint, or that the surface is not shown in that view.

  @FF-01 @build
  Scenario: The app never answers rule-trace when viewpoint is leader
    Given the model in app/src/FrontendFeature.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @FF-02 @build
  Scenario: The app always answers seal-badge when surface is seal
    Given the model in app/src/FrontendFeature.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @FF-03 @build
  Scenario: The app always answers verdict-card when surface is answer
    Given the model in app/src/FrontendFeature.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003
