"""Exercise real upstream geometry through the HTTP adapter, without Docker.

python3 tests/smoke.py /absolute/path/to/server /absolute/path/to/cadcraft-cli
Requires Linux with Landlock ABI v3. No credentials or external services are used.
"""
import base64
import concurrent.futures
import json
import os
from pathlib import Path
import secrets
import socket
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request

server, cli = map(lambda p: str(Path(p).resolve()), sys.argv[1:])
token = secrets.token_hex(32)
with tempfile.TemporaryDirectory() as directory:
    root = Path(directory)
    (root / 'index.html').write_text('<html><body><div id="cadcraft_loading"></div></body></html>')
    (root / 'viewer.js').write_text('export default function() {}')
    (root / 'viewer.wasm').write_bytes(b'')
    secret = root / 'private.txt'
    secret.write_text('must not be readable by workers')
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        port = sock.getsockname()[1]
    endpoint = f'http://127.0.0.1:{port}/mcp'
    env = {**os.environ, 'CADCRAFT_AUTH_TOKEN': token, 'CADCRAFT_CLI': cli,
           'CADCRAFT_ASSETS': directory, 'CADCRAFT_LISTEN': f'127.0.0.1:{port}'}
    process = subprocess.Popen([server], env=env, stdout=subprocess.DEVNULL)
    def rpc(method, params=None, authorized=True):
        headers = {'Content-Type': 'application/json'}
        if authorized:
            headers['Authorization'] = 'Bearer ' + token
        request = urllib.request.Request(endpoint, json.dumps({'jsonrpc':'2.0','id':1,'method':method,'params':params or {}}).encode(), headers)
        with urllib.request.urlopen(request, timeout=130) as response:
            body = json.load(response)
        assert 'error' not in body, body
        return body['result']
    def call(name, arguments=None, owner='user-a:conversation-1'):
        return rpc('tools/call', {'name':name, 'arguments':arguments or {}, '_meta':{'bionic/conversation':owner}})
    def text(result):
        assert not result.get('isError'), result
        return json.loads(result['content'][0]['text'])
    try:
        for _ in range(200):
            if process.poll() is not None:
                raise RuntimeError('Server exited during initialization')
            try:
                rpc('ping')
                break
            except urllib.error.URLError:
                time.sleep(.1)
        else:
            raise RuntimeError('Server failed to start')
        try:
            rpc('ping', authorized=False)
            raise AssertionError('Missing authentication was accepted')
        except urllib.error.HTTPError as error:
            assert error.code == 401
        tools = rpc('tools/list')['tools']
        assert all(t['_meta']['bionic/conversationScope'] for t in tools)
        assert {'command_line','script','execute','render','open_cadcraft'} <= {t['name'] for t in tools}
        assert not {'open','save'} & {t['name'] for t in tools}
        assert rpc('tools/call', {'name':'open_cadcraft','arguments':{}})['isError']
        opened = call('open_cadcraft')
        drawing = opened['structuredContent']['drawing_id']
        def args(**kw): return {'drawing_id':drawing, **kw}
        rectangle = call('command_line', args(text='rectang 0,0 100,60'))
        assert not rectangle.get('isError'), rectangle
        circle = call('execute', args(command='circle', params={'center':[10,10],'radius':4}))
        assert not circle.get('isError'), circle
        entities = text(call('query_entities', args()))
        assert entities['count'] == 2, entities
        snapshot = circle['_meta']['cadcraft/snapshot']
        dxf = base64.b64decode(snapshot['data'])
        assert b'CIRCLE' in dxf and b'LWPOLYLINE' in dxf
        assert snapshot['revision'] > rectangle['_meta']['cadcraft/snapshot']['revision']
        assert call('inspect_drawing', args(), owner='user-a:conversation-2')['isError']
        assert call('inspect_drawing', args(), owner='user-b:conversation-1')['isError']
        other = call('open_cadcraft')['structuredContent']['drawing_id']
        assert text(call('query_entities', {'drawing_id':other}))['count'] == 0
        with concurrent.futures.ThreadPoolExecutor(2) as executor:
            results = list(executor.map(lambda x: call('execute', args(command='circle', params={'center':[x,20],'radius':2})), [20,30]))
        assert all(not r.get('isError') for r in results)
        assert text(call('query_entities', args()))['count'] == 4
        assert call('execute', args(command='does-not-exist'))['isError']
        assert call('command_line', args())['isError']
        assert call('execute', args(command='open', params={'path':str(secret)}))['isError']
        assert call('execute', args(command='saveas', params={'path':str(root / 'escape.dxf')}))['isError']
        assert not (root / 'escape.dxf').exists()
        assert not call('execute', args(command='saveas', params={'path':'drawing.dxf'})).get('isError')
        rendered = call('render', args(width=320, height=240))
        assert not rendered.get('isError'), rendered
        assert base64.b64decode(rendered['content'][0]['data']).startswith(b'\x89PNG')
        assert b'CIRCLE' in dxf, 'Saved snapshots must remain independent of later calls'
        print('PASS: real commands, DXF snapshots, PNG render, continuity, concurrent calls, conversation isolation, authentication, validation and filesystem isolation')
    finally:
        process.terminate()
        process.wait(timeout=10)
