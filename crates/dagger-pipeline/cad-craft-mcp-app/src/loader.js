const pending = new Map(); let sequence = 0;
const embedded = parent !== window;
let snapshot = null;
let viewerReady = false;
function status(text) {
 let label = document.getElementById('cadcraft_status');
 if (!label) {
  label = document.createElement('div'); label.id = 'cadcraft_status';
  label.style.cssText = 'position:fixed;top:0;left:0;right:0;z-index:9999;padding:6px 12px;background:#17212b;color:white;font:12px sans-serif;pointer-events:none';
  document.body.appendChild(label);
 }
 label.textContent = text;
}
function displaySnapshot() {
 if (!snapshot || !viewerReady) return;
 try {
  if (snapshot.version !== 1 || typeof snapshot.data !== 'string' || snapshot.data.length > 4 * 1024 * 1024) throw new Error('Invalid drawing snapshot');
  const bytes = Uint8Array.from(atob(snapshot.data), c => c.charCodeAt(0));
  window.postMessage({type:'cadcraft:open',name:'Drawing.dxf',bytes}, '*', [bytes.buffer]);
  status('Model-generated drawing · revision '+snapshot.revision+' · view only · scroll to zoom, middle-drag to pan');
 } catch (error) { status('Drawing could not be loaded: '+error.message); }
}
function request(method, params) { return new Promise((resolve,reject) => {
 const id = `cadcraft-${++sequence}`;
 const timer = setTimeout(() => {pending.delete(id);reject(new Error('Host request timed out'));},120000);
 pending.set(id,{resolve,reject,timer}); parent.postMessage({jsonrpc:'2.0',id,method,params},'*');
}); }
window.addEventListener('message', event => {
 if(event.source === window && event.data?.type === 'cadcraft:ready') { viewerReady = true; displaySnapshot(); return; }
 if(event.source === window && event.data?.type === 'cadcraft:failed') { status('CADCraft failed: '+event.data.error); return; }
 if(event.source !== parent || !embedded) return;
 if(event.data?.method === 'ui/notifications/tool-result') {
  const result = event.data.params;
  snapshot = result?._meta?.['cadcraft/snapshot'] || null;
  if (result?._meta?.['cadcraft/snapshotError']) status('Drawing preview unavailable: '+result._meta['cadcraft/snapshotError']);
  else if (result?.isError && !snapshot) status('CADCraft: '+(result.content || []).filter(item => item.type === 'text').map(item => item.text).join('\n'));
  else displaySnapshot();
  return;
 }
 const item = pending.get(event.data?.id); if(!item) return;
 clearTimeout(item.timer); pending.delete(event.data.id);
 if(event.data.error) item.reject(new Error(event.data.error.message)); else item.resolve(event.data.result);
});
async function asset(path) { if(embedded) return (await request('bionic/assets/read',{path})).bytes; const r=await fetch('/assets/'+path); if(!r.ok) throw new Error('Asset HTTP '+r.status); return r.arrayBuffer(); }
try {
 status('Model-generated drawing · view only');
 if(embedded) { await request('ui/initialize',{protocolVersion:'2026-01-26',appInfo:{name:'CADCraft',version:'0.1.0'},appCapabilities:{availableDisplayModes:['inline','fullscreen','pip']}}); parent.postMessage({jsonrpc:'2.0',method:'ui/notifications/initialized'},'*'); await request('ui/request-display-mode',{mode:'pip'}); parent.postMessage({jsonrpc:'2.0',method:'ui/notifications/size-changed',params:{height:700}},'*'); }
 const [js,wasm]=await Promise.all([asset('__JS__'),asset('__WASM__')]);
 const moduleUrl=URL.createObjectURL(new Blob([js],{type:'text/javascript'}));
 const bindings=await import(moduleUrl); await bindings.default({module_or_path:wasm}); URL.revokeObjectURL(moduleUrl);
} catch(error) { document.getElementById('cadcraft_loading').textContent='CADCraft failed to start: '+error.message; }
