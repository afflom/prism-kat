# Platform Prism

**Every platform decision, proved before it ships.**

Platform Prism is a white-label Prism for teams that run content-addressed
platforms: registries, signed VM images, air-gapped sites and the rollouts that
depend on them. It was built for Kat Morgan's stakeholder view (platform and
cloud engineering, read from her public work on
[UOR-Foundation/kappa-distribution](https://github.com/UOR-Foundation/kappa-distribution),
[ContainerCraft](https://github.com/ContainerCraft) and Kubernetes platforms
from homelab to air-gapped), and it is also the demo: **this repository is a
tour of all of Prism, where every Prism capability is shown working on a
decision Kat already makes.**

**Live:** https://afflom.github.io/prism-kat/product/ is the product: six stakeholder views with real controls, a content-address lab, a live upload state machine and dashboards, every answer computed by the verified holo in your browser. https://afflom.github.io/prism-kat/ is the holo's own page. Entry point for UOR itself: https://uor.foundation
the verified holo. Entry point for UOR itself: https://uor.foundation

## The tour: all of Prism, in Kat's decisions

Each stop is a Prism capability, the place in this repository where you can see
it, and a request you can type into the live page to watch it happen.

| # | Prism capability | Where it lives here | Try it on the live page |
| --- | --- | --- | --- |
| 1 | **Content addressing** (UOR): an object's name is computed from its bytes | `kappa-label-check`: the exact rules for a kappa-label, from the κ-Distribution spec §3.2 | `kappa-label-check sha512 1 lower 64` → `deny` (sha512 needs 128 digits) |
| 2 | **One prompt, a whole app**: a structured prompt is the single source | [`prism/apps/*.prompt.json`](prism/apps), one per decision, each citing the spec line behind every rule | read [`upload-session.prompt.json`](prism/apps/upload-session.prompt.json) |
| 3 | **Three established patterns**: DMN decision tables, XACML policy decisions, UML state machines | `conformance-verdict`, `gc-eviction`, `rollout-gate`, `stakeholder-view`, `frontend-feature`, `backend-capability` (DMN); `kappa-label-check`, `vm-image-admission` (XACML); `upload-session` (UML) | `upload-session receiving complete mismatch` → `rejected` |
| 4 | **Proved, not tested**: properties are Lean theorems over every request | [`app/src/*Proofs.lex.tex`](app/src): 32 property theorems, such as "no unsigned image ever runs" (`VIA-01`) | `vm-image-admission missing match amd64 mirror connected ok` → `deny` |
| 5 | **State machine invariants**: a final state is never left, by induction | `upload-session`: committed, rejected and abandoned are final (`US-03`..`US-05`) | `upload-session committed cancel match` → `committed` |
| 6 | **One guest, many apps**: a router compiles every decision into one holo | [`app/src/KatPlatformPrism.lex.tex`](app/src/KatPlatformPrism.lex.tex): `<app> <fields>` dispatch to all 9 apps, 58 acceptance vectors | `conformance-verdict ok reachable 0 3` → `conformant` |
| 7 | **Verify before you trust**: the browser checks the holo's fingerprint and replays its vectors before it answers | `site/` (the shared Prism frame, pinned by `frame.lock`) | open the live page and watch the seal |
| 8 | **Architecture facet** (ISO/IEC/IEEE 42010): stakeholders, concerns, viewpoints, views | [`app/src/Architecture.lex.tex`](app/src/Architecture.lex.tex), 295 records | |
| 9 | **Design facet**: OPM object-process and UML component, sequence and state views | not in this set: `prismpm verify` refuses the `design/` files a multi-app set emits (PP5004), so the set ships the other five facets; the product page draws the upload state machine from the model | |
| 10 | **Features and journeys**: a feature per rule and property, a journey per stakeholder | [`app/src/Features.lex.tex`](app/src/Features.lex.tex), 181 records | |
| 11 | **Security facet**: assets, STRIDE threats, controls with evidence or accepted risk | [`app/src/Security.lex.tex`](app/src/Security.lex.tex), 1091 records, e.g. "Tampered VM image" answered by `VIA-01` and `VIA-02` | |
| 12 | **Quality facet** (ISO/IEC 25010:2023): requirements with measures and thresholds | [`app/src/Quality.lex.tex`](app/src/Quality.lex.tex), 211 records and 71 theorems, e.g. "Missed MUST failures = 0" | |
| 13 | **Compliance facet**: one obligation per standard with its evidence | [`app/src/Compliance.lex.tex`](app/src/Compliance.lex.tex) | |
| 14 | **Stakeholder views, proved**: the product's own information architecture | `stakeholder-view`: routes each reader's question to the decision that answers it; Lean proves a leader is never handed a protocol-level check (`SV-01`) | `stakeholder-view leader address` → `outside-view` |
| 15 | **Front end features, modeled from stakeholder requirements**: what each view shows of an answer | `frontend-feature`: picks the verdict card, plain reason, rule trace, proof panel, journey, lexicon or seal for each viewpoint and surface; Lean proves a leader is never handed a rule trace (`FF-01`), and every view shows the seal (`FF-02`) and leads with the answer (`FF-03`) | `frontend-feature leader rules` → `not-shown` |
| 16 | **Backend capabilities, modeled from front end requirements**: what serves each feature, online or air-gapped | `backend-capability`: guest dispatch, the model record, vector replay, the fingerprint check, the bundled proof or the CI attestation link; Lean proves nothing reaches the network at an air-gapped site (`BC-01`), the seal is always recomputed (`BC-02`) and every answer is computed by the verified guest (`BC-03`) | `backend-capability proof-panel airgapped` → `bundled-proof` |
| 17 | **The model is the authority** (R1..R6): claims, scenarios, tests and `CONFORMANCE.md` come from the model | [`model/ids.toml`](model/ids.toml), [`features/suites/`](features/suites), [`CONFORMANCE.md`](CONFORMANCE.md) | |
| 18 | **Only Prism is implemented here**: a shrink-only ratchet refuses new non-Prism code | `xtask` `audit-prism`, part of `just vv` | |
| 19 | **One gate in one pinned SDK**: `just vv` in the image `prismpm.lock` names | [`Justfile`](Justfile), [`prismpm.lock`](prismpm.lock) | |
| 20 | **Regenerate, never hand-edit**: the same prompts and commit always write the same bytes | the builder's `build-set --check` fails on any stale file | |

## How the layers trace

Each layer is an app modeled from the requirements of the layer above it, and
each is proved like any other decision:

| Layer | Input | App | Output |
| --- | --- | --- | --- |
| Stakeholders | a stakeholder viewpoint and its question | `stakeholder-view` | the decision app that answers it |
| Front end features | a surface of the answer (answer, reason, rules, proof, journey, words, seal) in that viewpoint | `frontend-feature` | the feature that renders it, or `not-shown` |
| Backend capabilities | that feature and the site (online or air-gapped) | `backend-capability` | the capability that serves it |

So `stakeholder-view leader rollout` → `rollout-gate`, then
`frontend-feature leader proof` → `plain-reason`, and
`backend-capability verdict-card airgapped` → `guest-dispatch`: the leader's
answer is the guest's, in plain words, with or without a network.

## Pick your view

The same Prism, read by everyone who depends on the platform. Each view asks
the question that reader asks; `stakeholder-view` is the proved map from that
question to the decision that answers it, and `frontend-feature` is the proved
map from each view to the features it shows. The words each view uses are in
[`prism/product.json`](prism/product.json).

| View | Who | Their question | Decisions | Front end features |
| --- | --- | --- | --- | --- |
| Platform engineering | Platform engineer | Can my teams build on this registry, and on these addresses? | `conformance-verdict`, `kappa-label-check`, `frontend-feature` | seal, verdict card, plain reason, rule trace, proof panel, journey, lexicon |
| Cloud engineering | Cloud engineer | Will my images and disks arrive whole, here and at the disconnected site? | `upload-session`, `vm-image-admission`, `backend-capability` | seal, verdict card, plain reason, rule trace, proof panel, journey, lexicon |
| Operations and SRE | Registry operator | What does the registry keep, and what can I safely reclaim tonight? | `gc-eviction`, `upload-session`, `conformance-verdict` | seal, verdict card, plain reason, rule trace, proof panel, journey, lexicon |
| Security and supply chain | Security engineer | Can anything unsigned, tampered or from the wrong place ever run? | `vm-image-admission`, `kappa-label-check`, `backend-capability` | seal, verdict card, plain reason, rule trace, proof panel, journey, lexicon |
| Engineering leadership | Engineering leader | Are we ready to roll out? Go, hold or no-go. | `rollout-gate`, `frontend-feature` | seal, verdict card, plain reason (also in place of the proof), journey, lexicon |
| Business and risk | Business owner | What are we paying for, and what could we lose? | `gc-eviction`, `rollout-gate`, `frontend-feature` | seal, verdict card, plain reason (also in place of the proof), journey, lexicon |

## The UOR words, in platform words

| UOR term | On your platform | In plain words |
| --- | --- | --- |
| content address | a digest, as in `image@sha256:...` | a fingerprint of the bytes: same bytes, same name, anywhere |
| kappa-label | `algorithm:hex`, such as `sha256:` and 64 hex digits | the written form of that fingerprint |
| canonical form | the exact bytes a digest is taken over | the one agreed way to write something, so everyone fingerprints the same thing |
| holo | a verified WebAssembly artifact with its fingerprint and acceptance vectors | the packaged decision, sealed so nobody can quietly change it |
| property | a theorem the Lean kernel checks for every possible request | a promise checked by a machine for every case, not sampled by tests |
| facet | one generated view of the model | one side of the same object, written for one kind of reader |
| verified guest | the sealed WebAssembly guest in the holo, run in the browser | the one place every answer is computed, with no server in between |

## How it is built

Everything above is generated from [`prism/apps/set.json`](prism/apps/set.json)
by the [Prism-DevRel builder](https://github.com/afflom/prism-devrel) at the
commit the set pins, inside the SDK image [`prismpm.lock`](prismpm.lock) pins:

```sh
node <builder>/cli.mjs build-set prism/apps/set.json --out .          # every app, facet, row, scenario and test
(cd app && lexlean lock && lexlean fmt && lexlean lock)               # the canonical, locked model
cargo run -q --locked -p xtask -- check-model --write                 # CONFORMANCE.md
node <builder>/cli.mjs build-set prism/apps/set.json --out . --check  # nothing is stale
just vv                                                               # the whole gate
```

To make a new white-label Prism for another stakeholder, write their decisions
as prompts, list them in a set, and run the same commands. The brand, persona
and words live in `prism/product.json`.

This repository is an instance of
[UOR-Foundation/template](https://github.com/UOR-Foundation/template) and
follows its contract: [AGENTS.md](AGENTS.md) holds rules R1 to R6 and
[TEMPLATE-CONTRACT.md](TEMPLATE-CONTRACT.md) the template's own terms.

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
