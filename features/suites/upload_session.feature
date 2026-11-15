Feature: upload_session

  This repository's Prism application, in app/, built by the Prism-DevRel
  builder from its prompt set: Give, as a UML state machine, the next state of a chunked upload session from its state, the event the registry received, and whether the assembled content hashes to the kappa-label the closing PUT names.

  @US-01 @build
  Scenario: The app never answers committed when label is mismatch and state is not committed
    Given the model in app/src/UploadSession.lex.tex
    When the Lean kernel checks inv001Holds during prismpm verify
    Then no request meets breaksInv001

  @US-02 @build
  Scenario: The app always answers abandoned when event is timeout and state is not committed and state is not rejected
    Given the model in app/src/UploadSession.lex.tex
    When the Lean kernel checks inv002Holds during prismpm verify
    Then no request meets breaksInv002

  @US-03 @build
  Scenario: The app always answers committed when state is committed
    Given the model in app/src/UploadSession.lex.tex
    When the Lean kernel checks inv003Holds during prismpm verify
    Then no request meets breaksInv003

  @US-04 @build
  Scenario: The app always answers rejected when state is rejected
    Given the model in app/src/UploadSession.lex.tex
    When the Lean kernel checks inv004Holds during prismpm verify
    Then no request meets breaksInv004

  @US-05 @build
  Scenario: The app always answers abandoned when state is abandoned
    Given the model in app/src/UploadSession.lex.tex
    When the Lean kernel checks inv005Holds during prismpm verify
    Then no request meets breaksInv005
