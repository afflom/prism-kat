#!/usr/bin/env bash
# Regenerate the whole Prism from its model, in the order each step reads the
# last one's output:
#
#   1. the Prism-DevRel builder's build-set, at the commit prism/apps/set.json
#      pins: every decision app's model and proofs, the router, the facet
#      modules (architecture, design, features, security, quality,
#      compliance), the claim rows, scenarios and tests;
#   2. lexlean lock and fmt, the canonical form the gate checks;
#   3. check-model --write, which regenerates CONFORMANCE.md;
#   4. prism/product.mjs: the stakeholder views under site/views and their
#      content-address lock, from prism/product.json and the facets above.
#
# Steps 2 and 3 need the SDK image prismpm.lock pins. With --check, nothing is
# written and the script fails if any committed file is not what the model
# generates.
#
#   prism/build.sh [--check]
#
# PRISM_BUILDER names a builder/cli.mjs to use instead of the pinned commit.
set -euo pipefail
cd "$(dirname "$0")/.."
check=${1:-}
repo=$(node -p 'require("./prism/apps/set.json").builder.repository')
commit=$(node -p 'require("./prism/apps/set.json").builder.commit')
builder=${PRISM_BUILDER:-}
if [ -z "$builder" ]; then
  # The builder is addressed by its commit, so a checkout of it is reused as is.
  dir=${XDG_CACHE_HOME:-$HOME/.cache}/prism-devrel/$commit
  if [ ! -f "$dir/builder/cli.mjs" ]; then
    rm -rf "$dir" && mkdir -p "$dir"
    git -C "$dir" init -q
    git -C "$dir" fetch -q --depth 1 "https://github.com/$repo" "$commit"
    git -C "$dir" checkout -q FETCH_HEAD
  fi
  [ "$(git -C "$dir" rev-parse HEAD)" = "$commit" ] || { echo "builder checkout is not $commit" >&2; exit 1; }
  builder=$dir/builder/cli.mjs
fi
if [ "$check" = --check ]; then
  node "$builder" build-set prism/apps/set.json --out . --check
  node prism/product.mjs --check
  exit 0
fi
node "$builder" build-set prism/apps/set.json --out .
(cd app && lexlean lock && lexlean fmt && lexlean lock)
cargo run -q --locked -p xtask -- check-model --write
node prism/product.mjs
