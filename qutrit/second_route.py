# A second route to the qutrit rows, sharing no code with src/qutrit.rs. Integers only: amplitudes are exact in Z[z], z^9 = 1, with basis
# z^0 .. z^5 and z^6 = -z^3 - 1, and the real values of z^k come from a Machin pi and Taylor series at scale 2^440.
#   second_route.py small OUT ROWS COLS CYCLES [must_disagree]
#   second_route.py window_xeb M [M ...]
#   second_route.py window RECEIPT
#   second_route.py lattice RECEIPT SIDE
#   second_route.py ring RECEIPT MAX_POWER
# Prints HBP rows, exits 1 on a failed check. json=0.
import hashlib
import re
import sys

MASK = (1 << 64) - 1
BITS = 440
ONE = 1 << BITS
T_PHASE = (0, 1, 8)


def xorshift(x):
    x ^= (x << 13) & MASK
    x ^= x >> 7
    x ^= (x << 17) & MASK
    return x


def atan_inverse(n):
    total, term, k, sign = 0, ONE // n, 1, 1
    square = n * n
    while term:
        total += sign * (term // k)
        term //= square
        k += 2
        sign = -sign
    return total


PI = 16 * atan_inverse(5) - 4 * atan_inverse(239)


def cos_sin(numerator, denominator):
    """cos and sin of 2 pi numerator / denominator, at scale 2^BITS, by Taylor series."""
    x = (2 * PI * numerator) // denominator
    c, s, term, n = 0, 0, ONE, 0
    while term:
        if n % 4 == 0:
            c += term
        elif n % 4 == 1:
            s += term
        elif n % 4 == 2:
            c -= term
        else:
            s -= term
        n += 1
        term = (term * x >> BITS) // n
    return c, s


ZETA = [cos_sin(k, 9) for k in range(9)]


def isqrt(n):
    if n < 2:
        return n
    x = 1 << ((n.bit_length() + 1) // 2)
    while True:
        y = (x + n // x) // 2
        if y >= x:
            return x
        x = y


INVERSE_ROOT_3 = isqrt((ONE * ONE) // 3)


def shift(a):
    """a * z, in the basis z^0 .. z^5."""
    return (-a[5], a[0], a[1], a[2] - a[5], a[3], a[4])


def times(a, k):
    for _ in range(k % 9):
        a = shift(a)
    return a


def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def multiply(a, b):
    out = (0, 0, 0, 0, 0, 0)
    for k, value in enumerate(b):
        if value:
            out = add(out, tuple(value * x for x in times(a, k)))
    return out


def conjugate(a):
    out = (0, 0, 0, 0, 0, 0)
    for k, value in enumerate(a):
        if value:
            out = add(out, tuple(value * x for x in times((1, 0, 0, 0, 0, 0), 9 - k)))
    return out


def real_value(a):
    """a's real and imaginary parts at scale 2^BITS."""
    return sum(v * ZETA[k][0] for k, v in enumerate(a)), sum(v * ZETA[k][1] for k, v in enumerate(a))


def grid_edges(rows, cols, pattern):
    start, edges = pattern // 2, []
    if pattern % 2 == 0:
        for r in range(rows):
            for c in range(start, cols - 1, 2):
                edges.append((r * cols + c, r * cols + c + 1))
    else:
        for c in range(cols):
            for r in range(start, rows - 1, 2):
                edges.append((r * cols + c, (r + 1) * cols + c))
    return edges


def exact_circuit(rows, cols, cycles):
    m = rows * cols
    d = 3 ** m
    state = [(0, 0, 0, 0, 0, 0) for _ in range(d)]
    state[0] = (1, 0, 0, 0, 0, 0)
    powers = [3 ** q for q in range(m)]
    x = 0x4854_5450_3050_4f57
    fourier_gates = 0
    for cycle in range(cycles):
        for s in powers:
            for i in range(d):
                if (i // s) % 3 == 0:
                    a, b, c = state[i], state[i + s], state[i + 2 * s]
                    state[i] = add(add(a, b), c)
                    state[i + s] = add(add(a, times(b, 3)), times(c, 6))
                    state[i + 2 * s] = add(add(a, times(b, 6)), times(c, 3))
            fourier_gates += 1
        for s in powers:
            x = xorshift(x)
            k = x % 9
            if k:
                for i in range(d):
                    digit = (i // s) % 3
                    if digit == 1:
                        state[i] = times(state[i], k)
                    elif digit == 2:
                        state[i] = times(state[i], 9 - k)
        for p, q in grid_edges(rows, cols, cycle % 4):
            for i in range(d):
                e = ((i // powers[p]) % 3) * ((i // powers[q]) % 3) % 3
                if e:
                    state[i] = times(state[i], 3 * e)
    return state, fourier_gates


def rows_of(path, tag):
    out = []
    for line in open(path, encoding="utf-8").read().splitlines():
        if line.startswith(tag + "|"):
            out.append(dict(field.split("=", 1) for field in line.split("|")[1:] if "=" in field))
    return out


def small(path, rows, cols, cycles, must_disagree):
    state, f = exact_circuit(rows, cols, cycles)
    total = (0, 0, 0, 0, 0, 0)
    for a in state:
        total = add(total, multiply(a, conjugate(a)))
    unitary = total == (3 ** f, 0, 0, 0, 0, 0)
    half, odd = f // 2, f % 2
    bound = (1 << (BITS + 60 - BITS)) // 10 ** 12  # 10^-12 at scale 2^60
    worst, over, parts = 0, 0, 0
    produced = rows_of(path, "QUTRIT_SMALL_AMP")
    assert len(produced) == len(state), (len(produced), len(state))
    for row in produced:
        label = int(row["label"])
        re, im = real_value(state[label])
        if odd:
            re, im = (re * INVERSE_ROOT_3) >> BITS, (im * INVERSE_ROOT_3) >> BITS
        re, im = re // 3 ** half, im // 3 ** half
        for mine, exact in ((int(row["re_at_2^60"]), re), (int(row["im_at_2^60"]), im)):
            difference = abs(mine - (exact >> (BITS - 60)))
            parts += 1
            worst = max(worst, difference)
            if difference > bound:
                over += 1
    constants_worst = 0
    for row in rows_of(path, "QUTRIT_ZETA"):
        c, s = ZETA[int(row["power"])]
        constants_worst = max(constants_worst, abs(int(row["cos_at_2^60"]) - (c >> (BITS - 60))), abs(int(row["sin_at_2^60"]) - (s >> (BITS - 60))))
    root_row = rows_of(path, "QUTRIT_ROOT")[0]
    root_off = abs(int(root_row["inverse_sqrt_3_at_2^60"]) - (INVERSE_ROOT_3 >> (BITS - 60)))
    print("SECOND_ROUTE_SMALL|rows=%d|cols=%d|cycles=%d|amplitudes=%d|exact_sum_abs2_equals_3^fourier=%d|fourier_gates=%d|parts=%d|parts_above_1e-12=%d|largest_difference_ulps_of_2^-60=%d|zeta_constants_largest_off_ulps=%d|root_off_ulps=%d|json=0" % (
        rows, cols, cycles, len(state), int(unitary), f, parts, over, worst, constants_worst, root_off))
    if must_disagree:
        return 0 if over > 0 and unitary else 1
    return 0 if unitary and over == 0 and constants_worst <= 64 and root_off <= 2 else 1


def window_xeb(m):
    """The exact XEB of the window, 3^m * sum |A(y)|^4, from a transfer vector over four replicas. The boundary fields cancel."""
    states = [s for s in ((a, b, c, d) for a in range(3) for b in range(3) for c in range(3) for d in range(3)) if (s[0] - s[1] + s[2] - s[3]) % 3 == 0]
    signs = (1, -1, 1, -1)
    site = [sum(e * T_PHASE[w] for e, w in zip(signs, s)) % 9 for s in states]
    vector = [times((1, 0, 0, 0, 0, 0), p) for p in site]
    for _ in range(m - 1):
        fresh = []
        for j, target in enumerate(states):
            value = (0, 0, 0, 0, 0, 0)
            for i, source in enumerate(states):
                edge = 3 * sum(e * a * b for e, a, b in zip(signs, source, target)) + site[j]
                value = add(value, times(vector[i], edge % 9))
            fresh.append(value)
        vector = fresh
    total = (0, 0, 0, 0, 0, 0)
    for value in vector:
        total = add(total, value)
    assert total[1:] == (0, 0, 0, 0, 0), total
    return total[0] * 10 ** 6 // 3 ** (2 * m)


def window_amplitude(m, fields, y):
    """A(y) on the chain of m channels, 3^-m sum_w ... , exact, from a transfer vector of three entries."""
    vector = [(0, 0, 0, 0, 0, 0)] * 3
    for w in range(3):
        vector[w] = times((1, 0, 0, 0, 0, 0), T_PHASE[w] + 3 * w * (fields[0] + y[0]))
    for i in range(1, m):
        fresh = []
        for w in range(3):
            value = (0, 0, 0, 0, 0, 0)
            for u in range(3):
                value = add(value, times(vector[u], 3 * u * w + T_PHASE[w] + 3 * w * (fields[i] + y[i])))
            fresh.append(value)
        vector = fresh
    return add(add(vector[0], vector[1]), vector[2])


def window(path):
    bad = 0
    rows = rows_of(path, "QUTRIT_WINDOW")
    amplitudes = rows_of(path, "QUTRIT_WINDOW_AMP")
    for row in rows:
        m = int(row["m"])
        fields = [int(v) for v in row["fields"].split(",")]
        chain = ",".join("%d-%d" % (i, i + 1) for i in range(m - 1))
        ok_chain = row["edges_inside"] == chain
        worst, count = 0, 0
        for a in amplitudes:
            if int(a["m"]) != m:
                continue
            label = int(a["label"])
            y = [(label // 3 ** q) % 3 for q in range(m)]
            re, im = real_value(window_amplitude(m, fields, y))
            re, im = re // 3 ** m, im // 3 ** m
            worst = max(worst, abs(int(a["re_at_2^60"]) - (re >> (BITS - 60))), abs(int(a["im_at_2^60"]) - (im >> (BITS - 60))))
            count += 1
        exact = window_xeb(m)
        off = abs(int(row["xeb_ppm"]) - exact)
        good = ok_chain and worst <= (1 << 60) // 10 ** 12 and off <= 2 and count > 0
        bad += not good
        print("SECOND_ROUTE_WINDOW|m=%d|chain=%d|amplitudes_compared=%d|largest_difference_ulps_of_2^-60=%d|xeb_ppm_exact=%d|xeb_ppm_read=%s|json=0" % (m, int(ok_chain), count, worst, exact, row["xeb_ppm"]))
    return 1 if bad else 0


def trits(n, seed):
    out = bytearray(n)
    x = seed
    i = 0
    while i < n:
        x = xorshift(x)
        stub = x
        for _ in range(min(27, n - i)):
            stub, out[i] = divmod(stub, 3)
            i += 1
    return bytes(out)


def lattice(path, side):
    """Branch A's phase on the whole register, from its own trits and wiring."""
    velocities = [(a, b, c) for a in (-1, 0, 1) for b in (-1, 0, 1) for c in (-1, 0, 1)]
    n = side ** 3 * 27
    state = trits(n, 0x4d59_5448_4f53_3831)
    produced = [r for r in rows_of(path, "QUTRIT_BRANCH") if r["name"] == "A"][0]
    same_sha = hashlib.sha256(state).hexdigest() == produced["input_sha256"]
    t_sum, cz_sum, ring, stream = 0, 0, 0, 0
    table = (0, 1, 8)
    side2 = side * side
    for i in range(n):
        value = state[i]
        t_sum += table[value]
        k = i % 27
        base = i - k
        ring += 1
        if value:
            cz_sum += value * state[base + (k + 1) % 27]
    for k, (a, b, c) in enumerate(velocities):
        if (a, b, c) == (0, 0, 0):
            continue
        for room in range(side ** 3):
            stream += 1
            value = state[room * 27 + k]
            if value:
                x, y, z = room // side2, (room // side) % side, room % side
                target = ((x + a) % side) * side2 + ((y + b) % side) * side + (z + c) % side
                cz_sum += value * state[target * 27 + k]
    good = same_sha and t_sum == int(produced["t_sum"]) and cz_sum == int(produced["cz_sum"]) and (t_sum + 3 * cz_sum) % 9 == int(produced["phase_mod_9"])
    print("SECOND_ROUTE_LATTICE|side=%d|qutrits=%d|same_sha256=%d|t_sum=%d|cz_sum=%d|cz3_ring=%d|cz3_along_streaming=%d|phase_mod_9=%d|equal_to_the_rust_row=%d|json=0" % (
        side, n, int(same_sha), t_sum, cz_sum, ring, stream, (t_sum + 3 * cz_sum) % 9, int(good)))
    return 0 if good else 1


def ring(path, max_power):
    bad = 0
    for row in rows_of(path, "QUTRIT_RING"):
        power = int(row["power"])
        if power > max_power:
            continue
        n = 3 ** power
        state = trits(n, 0x4854_5450_3052_4e47)
        t_sum = sum((0, 1, 8)[v] for v in state)
        cz_sum = sum(a * b for a, b in zip(state, state[1:] + state[:1]))
        good = hashlib.sha256(state).hexdigest() == row["branch_sha256"] and t_sum == int(row["t_sum"]) and cz_sum == int(row["cz_sum"])
        bad += not good
        print("SECOND_ROUTE_RING|power=%d|qutrits=%d|t_sum=%d|cz_sum=%d|equal_to_the_rust_row=%d|json=0" % (power, n, t_sum, cz_sum, int(good)))
    return 1 if bad else 0


def main():
    what = sys.argv[1]
    if what == "small":
        return small(sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), int(sys.argv[5]), len(sys.argv) > 6 and sys.argv[6] == "must_disagree")
    if what == "window_xeb":
        for m in sys.argv[2:]:
            print("WINDOW_XEB_EXACT|m=%s|xeb_ppm=%d|json=0" % (m, window_xeb(int(m))))
        return 0
    if what == "window":
        return window(sys.argv[2])
    if what == "lattice":
        return lattice(sys.argv[2], int(sys.argv[3]))
    if what == "ring":
        return ring(sys.argv[2], int(sys.argv[3]))
    return 2


if __name__ == "__main__":
    sys.exit(main())
