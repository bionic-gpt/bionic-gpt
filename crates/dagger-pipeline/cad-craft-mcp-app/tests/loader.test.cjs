// node --experimental-vm-modules --test tests/loader.test.cjs
const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');

test('buffers a snapshot until WASM is ready and ignores messages from other windows', async () => {
    const listeners = [];
    const sent = [];
    const elements = new Map([['cadcraft_loading', { textContent: '' }]]);
    let loaded = false;
    const window = {
        addEventListener: (_, listener) => listeners.push(listener),
        postMessage: data => {
            sent.push(data);
            queueMicrotask(() => emit(window, data));
        },
    };
    function emit(source, data) { listeners.forEach(listener => listener({ source, data })); }
    const snapshot = { version: 1, drawing_id: 'one', revision: 2, data: btoa('drawing-bytes') };
    const parent = {
        postMessage(message) {
            if (!message.id) return;
            queueMicrotask(() => {
                const result = message.method === 'ui/initialize' ? {} : { bytes: new Uint8Array() };
                emit(parent, { jsonrpc: '2.0', id: message.id, result });
                if (message.method === 'ui/initialize') {
                    emit(parent, { method: 'ui/notifications/tool-result', params: { _meta: { 'cadcraft/snapshot': snapshot } } });
                }
            });
        },
    };
    const document = {
        getElementById: id => elements.get(id),
        createElement: () => ({ style: {}, textContent: '' }),
        body: { appendChild: element => elements.set(element.id, element) },
    };
    const context = vm.createContext({ window, parent, document, setTimeout, clearTimeout, Blob, URL, Uint8Array, atob });
    const module = new vm.SyntheticModule(['default'], function () {
        this.setExport('default', async () => { loaded = true; });
    }, { context });
    await module.link(() => {});
    await module.evaluate();
    const code = readFileSync(path.join(__dirname, '../src/loader.js'), 'utf8');
    await vm.runInContext(`(async () => { ${code} })()`, context, {
        importModuleDynamically: async () => module,
    });
    assert.equal(loaded, true);
    assert.equal(sent.filter(x => x.type === 'cadcraft:open').length, 0);
    emit({}, { type: 'cadcraft:ready' });
    assert.equal(sent.filter(x => x.type === 'cadcraft:open').length, 0);
    emit(window, { type: 'cadcraft:ready' });
    const opened = sent.find(x => x.type === 'cadcraft:open');
    assert.equal(Buffer.from(opened.bytes).toString(), 'drawing-bytes');
    assert.match(elements.get('cadcraft_status').textContent, /revision 2/);
    emit({}, { method: 'ui/notifications/tool-result', params: { _meta: { 'cadcraft/snapshot': { ...snapshot, revision: 999 } } } });
    assert.match(elements.get('cadcraft_status').textContent, /revision 2/);
    emit(parent, { method: 'ui/notifications/tool-result', params: { _meta: { 'cadcraft/snapshotError': 'Export failed' } } });
    assert.match(elements.get('cadcraft_status').textContent, /Export failed/);
    emit(parent, { method: 'ui/notifications/tool-result', params: { isError: true, content: [{ type: 'text', text: 'Drawing expired' }] } });
    assert.match(elements.get('cadcraft_status').textContent, /Drawing expired/);
});
