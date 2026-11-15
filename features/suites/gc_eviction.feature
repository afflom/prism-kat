Feature: gc_eviction

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Decide, as a DMN decision table with hit policy FIRST, whether a sweep retains or evicts a blob, from whether a tag reaches it, the state of its pin, any finalizer on it and whether an upload session protects it.

  @GE-01 @build
  Scenario: The app never answers evict when finalizer is outstanding
    Given the model in app/src/GcEviction.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @GE-02 @build
  Scenario: The app never answers evict when tag is bound
    Given the model in app/src/GcEviction.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @GE-03 @build
  Scenario: The app always answers evict when tag is none and pin is expired and finalizer is not outstanding and upload is none
    Given the model in app/src/GcEviction.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003
