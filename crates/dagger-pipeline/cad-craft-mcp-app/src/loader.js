const pending = new Map(); let sequence = 0;
const embedded = parent !== window;
function request(method, params) { return new Promise((resolve,reject) => {
 const id = `cadcraft-${++sequence}`;
 const timer = setTimeout(() => {pending.delete(id);reject(new Error('Host request timed out'));},120000);
 pending.set(id,{resolve,reject,timer}); parent.postMessage({jsonrpc:'2.0',id,method,params},'*');
}); }
window.addEventListener('message', event => {
 if(event.source !== parent) return;
 const item = pending.get(event.data?.id); if(!item) return;
 clearTimeout(item.timer); pending.delete(event.data.id);
 if(event.data.error) item.reject(new Error(event.data.error.message)); else item.resolve(event.data.result);
});
async function asset(path) { if(embedded) return (await request('bionic/assets/read',{path})).bytes; const r=await fetch('/assets/'+path); if(!r.ok) throw new Error('Asset HTTP '+r.status); return r.arrayBuffer(); }
try {
 if(embedded) { await request('ui/initialize',{protocolVersion:'2026-01-26',appInfo:{name:'CADCraft',version:'0.1.0'},appCapabilities:{}}); parent.postMessage({jsonrpc:'2.0',method:'ui/notifications/initialized'},'*'); parent.postMessage({jsonrpc:'2.0',method:'ui/notifications/size-changed',params:{height:700}},'*'); }
 const [js,wasm]=await Promise.all([asset('__JS__'),asset('__WASM__')]);
 const moduleUrl=URL.createObjectURL(new Blob([js],{type:'text/javascript'}));
 const bindings=await import(moduleUrl); await bindings.default({module_or_path:wasm}); URL.revokeObjectURL(moduleUrl);
} catch(error) { document.getElementById('cadcraft_loading').textContent='CADCraft failed to start: '+error.message; }
