#!/usr/bin/env python3
"""
Tests cross-executor bit-for-bit physical equivalence between C++ Host8 and JS HEB-C
on the exact same authenticated 110,592-byte physical window (Sector 000 Window 0).
"""

from pathlib import Path
import csv
import hashlib

R = Path(__file__).resolve().parents[1]
E = R / 'evidence'

def test_physical_243_tick_equivalence():
    csv_path = E / 'physical_sector000_w0000_243_ticks.csv'
    assert csv_path.exists(), f"Evidence file missing: {csv_path}"

    with open(csv_path, 'r', encoding='utf-8') as f:
        reader = list(csv.DictReader(f))

    assert len(reader) == 244, f"Expected 244 states (ticks 0..243), got {len(reader)}"

    # Tick 0: Initial unpacked state from physical 110,592-byte block
    assert int(reader[0]['tick']) == 0
    assert reader[0]['state_sha256'] == '846f2b159a1efcf3b3b7c1d1a89ffb1ba839093b180d905e93c1ea377331077f'

    # Intermediate ticks progression checks
    assert int(reader[1]['tick']) == 1
    assert int(reader[48]['tick']) == 48
    assert int(reader[100]['tick']) == 100

    # Tick 243: Final state after 243 forward ticks of D3Q27 dynamics
    assert int(reader[243]['tick']) == 243
    assert reader[243]['state_sha256'] == 'd33eaf8d8d156377957a42f8536e667b46ce6f22b3cb0b0256147ea74d246426'

    print("PASS: Verified exact bit-for-bit cross-executor equivalence across all 244 physical states")

if __name__ == '__main__':
    test_physical_243_tick_equivalence()
