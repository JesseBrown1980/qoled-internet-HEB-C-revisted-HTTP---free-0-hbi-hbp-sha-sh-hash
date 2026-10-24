#!/usr/bin/env python3
"""Read-only static evidence checks; validates baseline and 23B virtual-portal datasets."""
from pathlib import Path
import json, hashlib, math

R = Path(__file__).resolve().parents[1]

# 1. Verify Baseline 243-step simulation
B = R / 'baseline'
b = json.loads((B / 'heb_c_243_measurement.json').read_text(encoding='utf-8'))
b_hbp = (B / 'heb_c_243_trace.hbp').read_bytes()
b_hbi = (B / 'heb_c_243_index.hbi').read_bytes()

assert b['done'] and b['pass'] and b['stopped_at_target']
assert b['ticks'] == b['target'] == 243
assert b['room_transitions'] == 3359232 and b['trit_transitions'] == 90699264
assert b['invariant_deviations'] == b['loschmidt_mismatches'] == 0
assert hashlib.sha256(b_hbp).hexdigest() == b['hbp_sha256']
assert hashlib.sha256(b_hbi).hexdigest() == b['hbi_sha256']
assert b['state_sha256'] == '42fb223a755aef7c4bbefbef87af830d1e19ed2fc64de018d9f412493f41b475'

b_rows = b_hbp.splitlines(keepends=True)
b_idx = b_hbi.splitlines(keepends=True)
assert len(b_rows) == len(b_idx) == 243
b_start = 0
for tick, (r, idx_line) in enumerate(zip(b_rows, b_idx), 1):
    def parse_fields(line):
        return dict(part.split(b'=', 1) for part in line.strip().split(b'|')[1:])
    p1, p2 = parse_fields(r), parse_fields(idx_line)
    assert int(p1[b'tick']) == int(p2[b'tick']) == tick
    assert int(p2[b'hbp_offset']) == b_start and int(p2[b'length']) == len(r)
    assert b_hbp[b_start:b_start + len(r)] == r
    b_start += len(r)
assert b_start == len(b_hbp)

# 2. Verify Evidence with 23-Billion Virtual Address Namespace
E = R / 'evidence'
a = json.loads((E / 'heb_c_23b_measurement.json').read_text(encoding='utf-8'))
hbp = (E / 'heb_c_23b_trace.hbp').read_bytes()
hbi = (E / 'heb_c_23b_index.hbi').read_bytes()

assert a['done'] and a['pass'] and a['stopped_at_target']
assert a['ticks'] == a['target'] == 243
assert a['virtual_http0_address_count'] == 23000000000
assert a['virtual_http0_instances_allocated'] == a['actual_webrtc_connections'] == a['actual_http_requests'] == 0
assert a['portal_route_probe_count'] == 243 * 8192 == 1990656
assert math.gcd(a['portal_route_probe_stride'], a['virtual_http0_address_count']) == 1
assert a['portal_endpoints_valid'] and a['portal_all_sample_addresses_distinct']
assert a['room_transitions'] == 3359232 and a['trit_transitions'] == 90699264
assert a['invariant_deviations'] == a['loschmidt_mismatches'] == 0
assert hashlib.sha256(hbp).hexdigest() == a['hbp_sha256']
assert hashlib.sha256(hbi).hexdigest() == a['hbi_sha256']
assert a['state_sha256'] == '42fb223a755aef7c4bbefbef87af830d1e19ed2fc64de018d9f412493f41b475'

rows = hbp.splitlines(keepends=True)
idx = hbi.splitlines(keepends=True)
assert len(rows) == len(idx) == 243
start = 0
firstinv = None
for tick, (r, b_line) in enumerate(zip(rows, idx), 1):
    assert r.startswith(b'HOST8WIN|') and b_line.startswith(b'HBI|')
    assert r.endswith(b'\n') and b_line.endswith(b'\n')
    def parse(line):
        return dict(part.split(b'=', 1) for part in line.strip().split(b'|')[1:])
    a1, a2 = parse(r), parse(b_line)
    assert int(a1[b'tick']) == int(a2[b'tick']) == tick
    assert int(a2[b'hbp_offset']) == start and int(a2[b'length']) == len(r)
    assert hbp[start:start + len(r)] == r
    assert a1[b'fp_fnv32'] == a2[b'fp_fnv32']
    assert a1[b'json'] == a2[b'json'] == b'0'
    assert int(a1[b'virtual_portals']) == 23000000000
    assert int(a1[b'portal_probes']) == tick * 8192
    assert a1[b'verified'] == b'1'
    v = tuple(a1[k] for k in (b'M', b'Px', b'Py', b'Pz', b'E2'))
    if firstinv is None:
        firstinv = v
    assert v == firstinv
    start += len(r)
assert start == len(hbp)

print('PASS: verified 243 baseline records, 243 23B-evidence records, 1,990,656 virtual-address probes, and exact integer conservation')
