// The product generator's gate. Each refusal is a planted defect: a record
// the generator must not turn into pages. `node --test prism/product.test.mjs`.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';
import { ProductError, address, canonical, esc, generate, readProduct, recordAddress, run } from './product.mjs';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const read = (p) => JSON.parse(readFileSync(join(root, p), 'utf8'));
const set = read('prism/apps/set.json');
const prompts = set.prompts.map((f) => read(join('prism/apps', f)));
const product = read('prism/product.json');

const refused = (mutate, field) => {
  const p = structuredClone(product);
  mutate(p);
  assert.throws(() => readProduct(p, set, prompts), (e) => e instanceof ProductError && e.field.startsWith(field), `expected a refusal on ${field}`);
};

test('the committed product record is sound', () => {
  assert.doesNotThrow(() => readProduct(structuredClone(product), set, prompts));
});

test('every committed page is the one the model generates (--check)', () => {
  const { stale, extra } = run(root, { check: true });
  assert.deepEqual(stale, [], `stale: ${stale.join(', ')}; run node prism/product.mjs`);
  assert.deepEqual(extra, [], `not generated: ${extra.join(', ')}`);
});

test('generation is deterministic', () => {
  const a = generate(root);
  const b = generate(root);
  assert.deepEqual([...a], [...b]);
});

test('a viewpoint for a stakeholder no prompt declares is refused', () => {
  refused((p) => (p.viewpoints[0].stakeholder = 'Chief Vibes Officer'), 'viewpoints.platform.stakeholder');
});

test('a stakeholder with no viewpoint is refused', () => {
  refused((p) => (p.viewpoints = p.viewpoints.filter((v) => v.stakeholder !== 'Business owner')), 'viewpoints');
});

test('an app in no viewpoint is refused', () => {
  refused((p) => {
    for (const v of p.viewpoints) {
      v.apps = v.apps.filter((a) => a !== 'rollout-gate');
      v.journey = v.journey.filter((s) => s.app !== 'rollout-gate');
    }
    p.viewpoints = p.viewpoints.filter((v) => v.apps.length);
  }, 'viewpoint');
});

test('a journey step that is not one of the app’s checked examples is refused', () => {
  refused((p) => (p.viewpoints[0].journey[0].request = 'ok reachable 0 4'), 'viewpoints.platform.journey');
});

test('a journey step on an app the viewpoint does not name is refused', () => {
  refused((p) => (p.viewpoints[0].journey[0].app = 'gc-eviction'), 'viewpoints.platform.journey');
});

test('an unknown lexicon term, and an unused one, are refused', () => {
  refused((p) => p.viewpoints[0].lexicon.push('blockchain'), 'viewpoints.platform.lexicon');
  refused((p) => p.lexicon.push({ key: 'unused', uor: 'u', technical: 't', plain: 'p' }), 'lexicon');
});

test('a brand colour that is not #rrggbb is refused', () => {
  refused((p) => (p.brand.accent = 'red; } body { display:none'), 'brand.accent');
});

test('every journey step shows the answer the model gives for it', () => {
  const pages = generate(root);
  for (const v of product.viewpoints) {
    const html = pages.get(`site/views/${v.key}/index.html`);
    for (const s of v.journey) {
      const want = prompts.find((p) => p.app.name === s.app).app.examples.find((e) => e.request === s.request).response;
      assert.ok(html.includes(`data-request="${esc(s.request)}" data-expect="${esc(want)}"`), `${v.key}: ${s.app} ${s.request}`);
    }
  }
});

test('every rule of every app is on a page, with its source', () => {
  const pages = [...generate(root).values()].join('\n');
  for (const p of prompts) for (const r of p.app.rules) assert.ok(pages.includes(esc(r.why)), `${p.app.name}: ${r.why}`);
});

test('page text is escaped', () => {
  assert.equal(esc('<script>"&\''), '&lt;script&gt;&quot;&amp;&#39;');
  const pages = [...generate(root).values()].filter((_, i, a) => a);
  for (const html of pages) assert.ok(!/<script>(?!<\/script>)/.test(html));
});

test('the page client holds no rule: it only dispatches to the verified holo', () => {
  const client = generate(root).get('site/views/view.mjs');
  for (const p of prompts) for (const o of p.app.outcomes) assert.ok(!client.includes(`"${o}"`), `the client names the outcome ${o}`);
  assert.match(client, /identities\?\.verified/);
});

test('content addresses are canonical', () => {
  assert.equal(canonical({ b: 1, a: [true, null, 'é'] }), '{"a":[true,null,"é"],"b":1}');
  assert.equal(recordAddress({ b: 1, a: 2 }), recordAddress({ a: 2, b: 1 }));
  assert.equal(address(Buffer.from('')), 'sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855');
});

test('the lock names every input and output by its content address', () => {
  const lock = JSON.parse(generate(root).get('prism/product.lock.json'));
  assert.equal(lock.inputs['prism/product.json'], recordAddress(product));
  for (const f of set.prompts) assert.match(lock.inputs[`prism/apps/${f}`], /^sha256:[0-9a-f]{64}$/);
  assert.equal(Object.keys(lock.outputs).length, product.viewpoints.length + 4);
});
