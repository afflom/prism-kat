#!/usr/bin/env node
/**
 * The product generator: turns the product record (prism/product.json), the
 * prompt set (prism/apps/set.json) and the facet modules the Prism-DevRel
 * builder wrote (app/src) into the stakeholder pages under site/views/.
 *
 * Nothing on a page is a second source. Every rule, property and example is
 * read from the prompt the holo was built from, every count from the generated
 * facet modules, and every live answer comes from the verified holo through
 * the template's own guest worker. The pages only move bytes.
 *
 * Every input and every written file is named by its content address
 * (sha256 over canonical JSON for records, over the exact bytes for files)
 * in prism/product.lock.json, so a reader can check that a page is the one
 * this model generates.
 *
 *   node prism/product.mjs            write site/views and prism/product.lock.json
 *   node prism/product.mjs --check    write nothing; exit 1 if anything is stale
 *
 * A record the generator cannot make into a sound product is refused with its
 * reason and exit code 2.
 */
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

export class ProductError extends Error {
  constructor(message, field) {
    super(message);
    this.field = field;
  }
}

const refuse = (message, field) => {
  throw new ProductError(message, field);
};

/** RFC 8785-style canonical JSON: sorted keys, no insignificant whitespace, NFC strings. */
export function canonical(value) {
  if (value === null || typeof value === 'boolean') return JSON.stringify(value);
  if (typeof value === 'number') {
    if (!Number.isFinite(value)) refuse('A record holds a number that is not finite.', 'number');
    return JSON.stringify(value);
  }
  if (typeof value === 'string') return JSON.stringify(value.normalize('NFC'));
  if (Array.isArray(value)) return `[${value.map(canonical).join(',')}]`;
  const keys = Object.keys(value).sort();
  return `{${keys.map((k) => `${JSON.stringify(k.normalize('NFC'))}:${canonical(value[k])}`).join(',')}}`;
}

export const address = (bytes) => `sha256:${createHash('sha256').update(bytes).digest('hex')}`;
export const recordAddress = (value) => address(Buffer.from(canonical(value), 'utf8'));

const ESC = { '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' };
export const esc = (s) => String(s).replace(/[&<>"']/g, (c) => ESC[c]);

/** Read and check the product record against the set and its prompts. */
export function readProduct(product, set, prompts) {
  if (!product || product.schema !== 'prism/product/1') refuse('A product record is `prism/product/1`.', 'schema');
  for (const key of ['name', 'tagline', 'summary', 'accent', 'accentDark'])
    if (typeof product.brand?.[key] !== 'string' || !product.brand[key].trim()) refuse(`The brand needs \`${key}\`.`, `brand.${key}`);
  for (const key of ['accent', 'accentDark'])
    if (!/^#[0-9a-f]{6}$/i.test(product.brand[key])) refuse(`\`brand.${key}\` is a #rrggbb colour.`, `brand.${key}`);
  if (!Array.isArray(product.viewpoints) || product.viewpoints.length < 1) refuse('The product declares at least one viewpoint.', 'viewpoints');

  const apps = new Map(prompts.map((p) => [p.app.name, p]));
  const stakeholders = new Map();
  for (const p of prompts) for (const s of p.stakeholders ?? []) stakeholders.set(s.name, s);
  const lexicon = new Map((product.lexicon ?? []).map((t) => [t.key, t]));
  if (lexicon.size !== (product.lexicon ?? []).length) refuse('Two lexicon terms share a key.', 'lexicon');

  const keys = new Set();
  const seen = new Set();
  const covered = new Set();
  const usedTerms = new Set();
  for (const v of product.viewpoints) {
    if (!/^[a-z][a-z0-9-]*$/.test(v.key ?? '')) refuse('A viewpoint key is lowercase words joined by hyphens.', 'viewpoints.key');
    if (keys.has(v.key)) refuse(`Two viewpoints are both \`${v.key}\`.`, 'viewpoints.key');
    keys.add(v.key);
    if (!stakeholders.has(v.stakeholder)) refuse(`\`${v.stakeholder}\` is not a stakeholder any prompt declares.`, `viewpoints.${v.key}.stakeholder`);
    if (seen.has(v.stakeholder)) refuse(`\`${v.stakeholder}\` has two viewpoints.`, `viewpoints.${v.key}.stakeholder`);
    seen.add(v.stakeholder);
    if (!['technical', 'non-technical'].includes(v.audience)) refuse('A viewpoint audience is technical or non-technical.', `viewpoints.${v.key}.audience`);
    for (const key of ['title', 'question', 'frame']) if (typeof v[key] !== 'string' || !v[key].trim()) refuse(`Viewpoint \`${v.key}\` needs \`${key}\`.`, `viewpoints.${v.key}.${key}`);
    if (!Array.isArray(v.apps) || v.apps.length < 1) refuse(`Viewpoint \`${v.key}\` names at least one app.`, `viewpoints.${v.key}.apps`);
    for (const a of v.apps) {
      if (!apps.has(a)) refuse(`\`${a}\` is not an app of the set.`, `viewpoints.${v.key}.apps`);
      covered.add(a);
    }
    for (const t of v.lexicon ?? []) {
      if (!lexicon.has(t)) refuse(`\`${t}\` is not a lexicon term.`, `viewpoints.${v.key}.lexicon`);
      usedTerms.add(t);
    }
    if (!Array.isArray(v.journey) || v.journey.length < 1) refuse(`Viewpoint \`${v.key}\` walks at least one step.`, `viewpoints.${v.key}.journey`);
    for (const step of v.journey) {
      if (!v.apps.includes(step.app)) refuse(`A step of \`${v.key}\` uses \`${step.app}\`, which the viewpoint does not name.`, `viewpoints.${v.key}.journey`);
      const example = apps.get(step.app).app.examples.find((e) => e.request === step.request);
      if (!example) refuse(`\`${step.app} ${step.request}\` is not one of that app's examples, so its answer is not checked.`, `viewpoints.${v.key}.journey`);
      if (typeof step.says !== 'string' || !step.says.trim()) refuse(`A step of \`${v.key}\` says nothing.`, `viewpoints.${v.key}.journey`);
      step.expect = example.response;
    }
  }
  for (const name of stakeholders.keys()) if (!seen.has(name)) refuse(`Stakeholder \`${name}\` has no viewpoint.`, 'viewpoints');
  for (const name of apps.keys()) if (!covered.has(name)) refuse(`App \`${name}\` is in no viewpoint.`, 'viewpoints');
  for (const key of lexicon.keys()) if (!usedTerms.has(key)) refuse(`Lexicon term \`${key}\` is used by no viewpoint.`, 'lexicon');
  return { product, apps, stakeholders, lexicon };
}

const FACETS = ['Architecture', 'Design', 'Features', 'Security', 'Quality', 'Compliance'];

/** Records per facet, counted from the generated modules as the SDK numbers them. */
export function facetCounts(root) {
  const count = (file) => {
    const path = join(root, 'app', 'src', `${file}.lex.tex`);
    if (!existsSync(path)) refuse(`app/src/${file}.lex.tex is missing; run the builder's build-set first.`, 'facets');
    const text = readFileSync(path, 'utf8');
    return { definitions: (text.match(/^definition\{/gm) ?? []).length, theorems: (text.match(/^theorem\{/gm) ?? []).length };
  };
  return Object.fromEntries(FACETS.map((f) => [f, count(f)]));
}

const pascal = (s) =>
  String(s)
    .split(/[^A-Za-z0-9]+/)
    .filter(Boolean)
    .map((w) => w[0].toUpperCase() + w.slice(1))
    .join('');

function appProofs(root, name) {
  const path = join(root, 'app', 'src', `${pascal(name)}Proofs.lex.tex`);
  if (!existsSync(path)) refuse(`app/src/${pascal(name)}Proofs.lex.tex is missing; run the builder's build-set first.`, 'facets');
  return (readFileSync(path, 'utf8').match(/^theorem\{/gm) ?? []).length;
}

const PATTERN = { decisionTable: 'Decision table (OMG DMN 1.5, hit policy FIRST)', policyDecision: 'Policy decision (OASIS XACML 3.0)', stateMachine: 'State machine (OMG UML 2.5.1)' };

const cond = (c) => `${esc(c.field)} ${esc(c.op)} ${esc(c.value)}`;
const conds = (when) => when.map(cond).join(' and ');

const CSP = "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; font-src 'self'; img-src 'self' data:; connect-src 'self'; worker-src 'self'; base-uri 'none'; form-action 'none'; object-src 'none'";

function shell({ title, description, depth, body, brand }) {
  const up = depth === 0 ? './' : '../'.repeat(depth);
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="color-scheme" content="light dark" />
    <meta http-equiv="Content-Security-Policy" content="${CSP}" />
    <meta name="description" content="${esc(description)}" />
    <title>${esc(title)}</title>
    <link rel="stylesheet" href="${up}views.css" />
  </head>
  <body>
    <header class="top">
      <a class="mark" href="${up}index.html">${esc(brand.name)}</a>
      <span class="seal" data-seal>Checking the seal on the verified app…</span>
    </header>
    <main>
${body}
    </main>
    <footer>
      <p>Built with Prism from the UOR Framework. Every rule here is read from the model, every promise is a theorem the Lean kernel checks, and every answer comes from the verified app, never from this page. <a href="https://uor.foundation">uor.foundation</a></p>
    </footer>
    <script type="module" src="${up}view.mjs"></script>
  </body>
</html>
`;
}

function indexPage(ctx) {
  const { product, counts, total } = ctx;
  const b = product.brand;
  const cards = product.viewpoints
    .map(
      (v) => `        <a class="card" href="./${esc(v.key)}/index.html">
          <span class="tag ${v.audience === 'technical' ? 'tech' : 'plain'}">${esc(v.audience)}</span>
          <h3>${esc(v.title)}</h3>
          <p>${esc(v.question)}</p>
        </a>`,
    )
    .join('\n');
  const facets = Object.entries(counts)
    .map(([f, c]) => `          <li><b>${c.definitions + c.theorems}</b> ${esc(f)}</li>`)
    .join('\n');
  const steps = [
    ['Canonicalize', 'canonical-form'],
    ['Fingerprint', 'content-address'],
    ['Address', 'kappa-label'],
  ]
    .map(([verb, key]) => {
      const t = ctx.lexicon.get(key);
      return `          <li><h3>${esc(verb)}</h3><p><b>${esc(t.uor)}</b>: ${esc(t.plain)}.</p><p class="tech-note">On your platform: ${esc(t.technical)}.</p></li>`;
    })
    .join('\n');
  const body = `      <section class="hero">
        <h1>${esc(b.name)}</h1>
        <p class="tagline">${esc(b.tagline)}</p>
        <p>${esc(b.summary)}</p>
      </section>
      <section>
        <h2>Pick your view</h2>
        <p>One product, seen by everyone who depends on it. Each view asks the question that reader asks, in their own words, and answers it with the same verified app.</p>
        <div class="cards">
${cards}
        </div>
      </section>
      <section>
        <h2>UOR in three steps, in platform words</h2>
        <ol class="steps">
${steps}
        </ol>
      </section>
      <section>
        <h2>What one model generated</h2>
        <p>${ctx.appCount} decision apps, ${total.properties} proved promises and ${total.examples} worked examples, plus every facet below, all written from <code>prism/apps</code> by one command.</p>
        <ul class="facets">
${facets}
        </ul>
      </section>
      <section>
        <h2>Who this is for</h2>
        <p><b>${esc(product.persona.title)}.</b> ${esc(product.persona.source)}</p>
        <ul>
${product.persona.domain.map((d) => `          <li>${esc(d)}</li>`).join('\n')}
        </ul>
      </section>`;
  return shell({ title: b.name, description: b.summary, depth: 0, body, brand: b });
}

function viewPage(ctx, v) {
  const { product, apps, lexicon } = ctx;
  const words = (v.lexicon ?? [])
    .map((k) => {
      const t = lexicon.get(k);
      return `          <tr><th scope="row">${esc(t.uor)}</th><td>${esc(v.audience === 'technical' ? t.technical : t.plain)}</td></tr>`;
    })
    .join('\n');
  const journey = v.journey
    .map(
      (s, i) => `          <li class="step" data-app="${esc(s.app)}" data-request="${esc(s.request)}" data-expect="${esc(s.expect)}">
            <p class="says">${esc(s.says)}</p>
            <p class="req"><code>${esc(s.app)} ${esc(s.request)}</code> <span class="expect">model answer: <b>${esc(s.expect)}</b></span></p>
            <button type="button" data-ask="${i}">Ask the verified app</button>
            <output aria-live="polite"></output>
          </li>`,
    )
    .join('\n');
  const appSections = v.apps
    .map((name) => {
      const p = apps.get(name);
      const a = p.app;
      const rules = a.rules.map((r, i) => `              <tr><td>${i + 1}</td><td>${conds(r.when)}</td><td><b>${esc(r.then)}</b></td><td>${esc(r.why)}</td></tr>`).join('\n');
      const props = (a.properties ?? []).map((pr) => `            <li>It <b>${esc(pr.kind)}</b> answers <b>${esc(pr.then)}</b> when ${conds(pr.when)}: ${esc(pr.why)}.</li>`).join('\n');
      const fields = a.fields.map((f) => `<code>${esc(f.name)}</code> (${esc(f.means)})`).join(', ');
      return `        <article class="app" id="${esc(name)}">
          <h3>${esc(a.title)}</h3>
          <p class="pattern">${esc(PATTERN[p.pattern] ?? p.pattern)} · grounded in <code>${esc(p.repository)}</code></p>
          <p>${esc(p.goal)}</p>
          <p class="fields">A request is ${fields}, in that order.</p>
          <h4>Promises the Lean kernel proves for every request</h4>
          <ul class="props">
${props}
          </ul>
          <details>
            <summary>Every rule, first match wins, with where it comes from</summary>
            <table>
              <thead><tr><th>#</th><th>When</th><th>Answer</th><th>Why</th></tr></thead>
              <tbody>
${rules}
              <tr><td>else</td><td>nothing above matched</td><td><b>${esc(a.otherwise)}</b></td><td>${esc(a.otherwiseWhy)}</td></tr>
              </tbody>
            </table>
          </details>
          <form class="try" data-app="${esc(name)}">
            <label>Try your own request <input name="request" autocomplete="off" spellcheck="false" placeholder="${esc(a.examples[0].request)}" /></label>
            <button type="submit">Decide</button>
            <output aria-live="polite"></output>
          </form>
        </article>`;
    })
    .join('\n');
  const body = `      <section class="hero">
        <p class="kicker">${esc(v.title)} · ${esc(v.stakeholder)}</p>
        <h1>${esc(v.question)}</h1>
        <p>${esc(v.frame)}</p>
      </section>
      <section>
        <h2>Walk it</h2>
        <ol class="journey">
${journey}
        </ol>
      </section>
      <section>
        <h2>The words, in your terms</h2>
        <table class="lexicon">
          <tbody>
${words}
          </tbody>
        </table>
      </section>
      <section>
        <h2>The decisions behind this view</h2>
${appSections}
      </section>`;
  return shell({ title: `${v.title} · ${product.brand.name}`, description: v.question, depth: 1, body, brand: product.brand });
}

function css(brand) {
  return `:root {
  --bg: #f6f5f1; --fg: #1c1a19; --muted: #5d5853; --line: #dcd8d0; --card: #ffffff;
  --accent: ${brand.accent}; --ok: #1f7a4d; --bad: #b3261e; --tech: #2b5fb8; --plain: #8a5a00;
  color-scheme: light dark;
}
@media (prefers-color-scheme: dark) {
  :root { --bg: #141312; --fg: #f1efea; --muted: #a8a29b; --line: #34302d; --card: #1d1b1a; --accent: ${brand.accentDark}; --ok: #6fd39d; --bad: #ff8a80; --tech: #8fb4ff; --plain: #f0c060; }
}
@font-face { font-family: Geist; src: url("../fonts/Geist-Regular.woff2") format("woff2"); font-weight: 400; }
@font-face { font-family: Geist; src: url("../fonts/Geist-Medium.woff2") format("woff2"); font-weight: 600; }
@font-face { font-family: GeistMono; src: url("../fonts/GeistMono-Regular.woff2") format("woff2"); }
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--fg); font: 16px/1.55 Geist, system-ui, sans-serif; }
main, header, footer { max-width: 60rem; margin: 0 auto; padding: 0 16px; }
header.top { display: flex; justify-content: space-between; align-items: center; gap: 1rem; padding-top: 1rem; padding-bottom: 1rem; border-bottom: 1px solid var(--line); flex-wrap: wrap; }
.mark { font-weight: 600; color: var(--fg); text-decoration: none; letter-spacing: .01em; }
.mark::before { content: ""; display: inline-block; width: .8em; height: .8em; margin-right: .5em; border-radius: 50%; background: conic-gradient(from 90deg, #ff5f6d, #ffc371, #7cf29c, #5ec8ff, var(--accent), #ff5f6d); vertical-align: -.05em; }
.seal { font-size: .85rem; color: var(--muted); }
.seal.ok { color: var(--ok); } .seal.bad { color: var(--bad); }
h1 { font-size: clamp(1.8rem, 4vw, 2.6rem); line-height: 1.15; margin: 2rem 0 .5rem; }
h2 { margin-top: 2.5rem; font-size: 1.35rem; }
h3 { margin: .2rem 0; }
.tagline { font-size: 1.2rem; color: var(--accent); margin: 0 0 1rem; }
.kicker { color: var(--muted); margin: 2rem 0 0; font-size: .9rem; }
a { color: var(--accent); }
code { font-family: GeistMono, ui-monospace, monospace; font-size: .9em; overflow-wrap: anywhere; }
.cards { display: grid; grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr)); gap: 12px; }
.card { display: block; padding: 1rem; border: 1px solid var(--line); border-radius: 12px; background: var(--card); color: var(--fg); text-decoration: none; }
.card:hover, .card:focus-visible { border-color: var(--accent); }
.card p { color: var(--muted); margin: .3rem 0 0; }
.tag { font-size: .75rem; text-transform: uppercase; letter-spacing: .06em; }
.tag.tech { color: var(--tech); } .tag.plain { color: var(--plain); }
.steps { list-style: none; padding: 0; display: grid; grid-template-columns: repeat(auto-fill, minmax(15rem, 1fr)); gap: 12px; counter-reset: s; }
.steps li { border: 1px solid var(--line); border-radius: 12px; padding: 1rem; background: var(--card); }
.tech-note { color: var(--muted); font-size: .9rem; }
.facets { list-style: none; padding: 0; display: flex; flex-wrap: wrap; gap: 8px; }
.facets li { border: 1px solid var(--line); border-radius: 999px; padding: .25rem .8rem; background: var(--card); }
.journey { padding-left: 1.2rem; }
.step { margin-bottom: 1.2rem; }
.says { font-weight: 600; margin: 0; }
.req { margin: .2rem 0 .4rem; color: var(--muted); }
.expect { margin-left: .4rem; }
button { font: inherit; padding: .35rem .8rem; border-radius: 8px; border: 1px solid var(--accent); background: transparent; color: var(--accent); cursor: pointer; }
button:disabled { opacity: .5; cursor: not-allowed; }
output { display: block; margin-top: .3rem; font-family: GeistMono, ui-monospace, monospace; font-size: .9rem; }
output.ok { color: var(--ok); } output.bad { color: var(--bad); }
table { border-collapse: collapse; width: 100%; font-size: .9rem; display: block; overflow-x: auto; }
th, td { text-align: left; vertical-align: top; padding: .4rem .6rem; border-bottom: 1px solid var(--line); }
.lexicon th { width: 12rem; }
.app { border: 1px solid var(--line); border-radius: 12px; padding: 1rem; margin: 1rem 0; background: var(--card); }
.pattern, .fields { color: var(--muted); font-size: .9rem; }
.props li { margin-bottom: .3rem; }
details { margin: .8rem 0; }
.try label { display: block; margin-bottom: .4rem; }
.try input { font: inherit; font-family: GeistMono, ui-monospace, monospace; width: 100%; max-width: 32rem; padding: .35rem .5rem; border: 1px solid var(--line); border-radius: 8px; background: var(--bg); color: var(--fg); display: block; margin-top: .3rem; }
footer { margin-top: 3rem; padding-top: 1rem; padding-bottom: 2rem; border-top: 1px solid var(--line); color: var(--muted); font-size: .85rem; }
`;
}

/** The page client: it loads the verified holo through the template's guest worker and only moves bytes. */
const CLIENT = `// Generated by prism/product.mjs. Loads ../app.holo in the template's guest
// worker, which verifies its fingerprint and acceptance vectors before it will
// dispatch anything, then sends "<app> <fields>" and shows the answer. No rule
// lives here: an answer the holo does not give is never shown.
const base = new URL(document.querySelector('script[src$="view.mjs"]').src);
const seal = document.querySelector("[data-seal]");
const encoder = new TextEncoder();
const decoder = new TextDecoder();
let call = null;

function start() {
  let worker;
  try { worker = new Worker(new URL("guest-worker.mjs", new URL("..", base)), { type: "module" }); } catch { return null; }
  const waiting = new Map();
  let next = 0;
  worker.onmessage = ({ data }) => {
    const settle = waiting.get(data?.id);
    if (!settle) return;
    waiting.delete(data.id);
    data.ok ? settle.resolve(data.value) : settle.reject(new Error(data.error));
  };
  worker.onerror = (event) => { event.preventDefault?.(); for (const s of waiting.values()) s.reject(new Error("the guest worker failed")); waiting.clear(); };
  return (kind, payload = {}) => new Promise((resolve, reject) => {
    const id = next++;
    waiting.set(id, { resolve, reject });
    worker.postMessage({ id, kind, ...payload });
  });
}

function setSeal(text, cls) { if (seal) { seal.textContent = text; seal.className = "seal " + cls; } }

async function boot() {
  const holo = new URL("app.holo", new URL("..", base)).href;
  const head = await fetch(holo, { method: "HEAD" }).catch(() => null);
  if (!head || !head.ok) { setSeal("The verified app is published with the site's release; answers appear once it is live.", "bad"); return; }
  const c = start();
  if (!c) { setSeal("This browser cannot run the verified app.", "bad"); return; }
  try {
    const identities = await c("init", { source: holo });
    if (!identities?.verified) { setSeal("The app's seal did not verify, so it will not answer.", "bad"); return; }
    call = c;
    setSeal("Seal verified: answers come from the proved app.", "ok");
  } catch (error) {
    setSeal("The app's seal did not verify (" + error.message + "), so it will not answer.", "bad");
  }
}

async function ask(app, request) {
  if (!call) throw new Error("the verified app is not loaded");
  const bytes = await call("dispatch", { request: encoder.encode(app + " " + request) });
  return decoder.decode(bytes);
}

document.addEventListener("click", async (event) => {
  const button = event.target.closest("button[data-ask]");
  if (!button) return;
  const step = button.closest(".step");
  const out = step.querySelector("output");
  try {
    const answer = await ask(step.dataset.app, step.dataset.request);
    const same = answer === step.dataset.expect;
    out.textContent = "verified app: " + answer + (same ? "  (matches the model)" : "  (differs from the model's example)");
    out.className = same ? "ok" : "bad";
  } catch (error) { out.textContent = error.message; out.className = "bad"; }
});

document.addEventListener("submit", async (event) => {
  const form = event.target.closest("form.try");
  if (!form) return;
  event.preventDefault();
  const out = form.querySelector("output");
  const request = form.elements.request.value.trim() || form.elements.request.placeholder;
  try { out.textContent = "verified app: " + await ask(form.dataset.app, request); out.className = "ok"; }
  catch (error) { out.textContent = error.message; out.className = "bad"; }
});

boot();
`;

function llms(ctx) {
  const { product, apps } = ctx;
  const lines = [`# ${product.brand.name}`, '', `> ${product.brand.tagline}`, '', product.brand.summary, '', '## Views', ''];
  for (const v of product.viewpoints) lines.push(`- [${v.title}](./${v.key}/index.html): ${v.question} (${v.stakeholder}, ${v.audience})`);
  lines.push('', '## Decision apps', '');
  for (const [name, p] of apps) lines.push(`- ${name}: ${p.goal} Request fields: ${p.app.fields.map((f) => f.name).join(' ')}. Answers: ${p.app.outcomes.join(', ')}.`);
  lines.push('', '## Words', '');
  for (const t of product.lexicon) lines.push(`- ${t.uor}: ${t.plain}. Technically: ${t.technical}.`);
  return `${lines.join('\n')}\n`;
}

/** Everything the product writes, as relative path to bytes, plus the lock. */
export function generate(root) {
  const read = (p) => JSON.parse(readFileSync(join(root, p), 'utf8'));
  const set = read('prism/apps/set.json');
  const prompts = set.prompts.map((f) => read(join('prism/apps', f)));
  const product = read('prism/product.json');
  const ctx = readProduct(structuredClone(product), set, prompts);
  ctx.counts = facetCounts(root);
  ctx.appCount = prompts.length;
  ctx.total = {
    properties: prompts.reduce((n, p) => n + (p.app.properties?.length ?? 0), 0),
    examples: prompts.reduce((n, p) => n + p.app.examples.length, 0),
    theorems: prompts.reduce((n, p) => n + appProofs(root, p.app.name), 0),
  };
  const files = new Map();
  files.set('site/views/index.html', indexPage(ctx));
  for (const v of ctx.product.viewpoints) files.set(`site/views/${v.key}/index.html`, viewPage(ctx, v));
  files.set('site/views/views.css', css(ctx.product.brand));
  files.set('site/views/view.mjs', CLIENT);
  files.set('site/views/llms.txt', llms(ctx));
  const lock = {
    schema: 'prism/product-lock/1',
    comment: 'Generated by prism/product.mjs. Content addresses of every input record and every written file; --check fails when any is stale.',
    inputs: {
      'prism/product.json': recordAddress(product),
      'prism/apps/set.json': recordAddress(set),
      ...Object.fromEntries(set.prompts.map((f, i) => [`prism/apps/${f}`, recordAddress(prompts[i])])),
    },
    facets: ctx.counts,
    theorems: ctx.total.theorems,
    outputs: Object.fromEntries([...files].map(([p, text]) => [p, address(Buffer.from(text, 'utf8'))])),
  };
  files.set('prism/product.lock.json', `${JSON.stringify(lock, null, 2)}\n`);
  return files;
}

function walk(dir) {
  if (!existsSync(dir)) return [];
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? walk(join(dir, e.name)) : [join(dir, e.name)]));
}

export function run(root, { check = false } = {}) {
  const files = generate(root);
  const stale = [];
  for (const [p, text] of files) {
    const path = join(root, p);
    if (!existsSync(path) || readFileSync(path, 'utf8') !== text) stale.push(p);
  }
  const extra = walk(join(root, 'site/views'))
    .map((f) => relative(root, f))
    .filter((p) => !files.has(p));
  if (check) return { stale, extra };
  rmSync(join(root, 'site/views'), { recursive: true, force: true });
  for (const [p, text] of files) {
    mkdirSync(dirname(join(root, p)), { recursive: true });
    writeFileSync(join(root, p), text);
  }
  return { stale, extra, written: [...files.keys()] };
}

if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  const root = join(dirname(fileURLToPath(import.meta.url)), '..');
  const check = process.argv.includes('--check');
  try {
    const { stale, extra, written } = run(root, { check });
    if (check) {
      if (stale.length || extra.length) {
        for (const p of stale) console.error(`stale: ${p}`);
        for (const p of extra) console.error(`not generated: ${p}`);
        process.exit(1);
      }
      console.log('product: every generated file is current');
    } else {
      console.log(`product: wrote ${written.length} files`);
    }
  } catch (error) {
    if (error instanceof ProductError) {
      console.error(`refused: ${error.message}`);
      process.exit(2);
    }
    throw error;
  }
}
