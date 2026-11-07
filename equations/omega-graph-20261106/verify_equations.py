#!/usr/bin/env python3
"""Independent exact GF(3), SHA-256 and native HTTP verification; integer only."""
import hashlib
import socket
import subprocess
import sys
import time

def sha(s):
    return hashlib.sha256(s.encode("ascii")).hexdigest()

def state(n,w):
    return sha("OMEGA_QUTRIT_RING_V1|q=3|n="+str(n)+"|edges=cycle|weight="+str(w)+"|normalization=3^(-n/2)|json=0\n")

def history(n,t):
    prev=sha("OMEGA_QUTRIT_GENESIS_V1|n="+str(n)+"|json=0\n")
    for s in range(t+1):
        h=state(n,s%3)
        prev=sha("OMEGA_SLICE_V1|n="+str(n)+"|slice="+str(s)+"|state_sha256="+h+"|previous="+prev+"|json=0\n")
    return prev

def rank_gf3(a):
    a=[r[:] for r in a]
    p=0
    for c in range(len(a[0])):
        i=next((j for j in range(p,len(a)) if a[j][c]),None)
        if i is None:continue
        a[p],a[i]=a[i],a[p]
        inv=1 if a[p][c]==1 else 2
        a[p]=[v*inv%3 for v in a[p]]
        for j in range(len(a)):
            if j!=p:
                z=a[j][c]
                a[j]=[(x-z*y)%3 for x,y in zip(a[j],a[p])]
        p+=1
        if p==len(a):break
    return p

def reference_rank(n,t,cut):
    w=t%3
    return rank_gf3([[w if (i-j)%n in (1,n-1) else 0 for j in range(cut,n)] for i in range(cut)])

def fields(row):
    cells=row.strip().split("|")
    assert cells[0]=="OMEGA_EQUATIONS"
    out={}
    for cell in cells[1:]:
        k,sep,v=cell.partition("=")
        assert sep and k not in out
        out[k]=v
    assert out["json"]=="0"
    return out

def verify(binary):
    checks=0
    for n in range(3,13):
        for c in range(1,n):
            for t in range(4):
                actual=reference_rank(n,t,c)
                predicted=0 if t%3==0 else (1 if c in (1,n-1) else 2)
                assert actual==predicted,(n,t,c,actual,predicted)
                checks+=1
    ladder=((16,43046721),(17,129140163),(18,387420489))
    examples=[(3,0,1,0),(3,1,1,0),(3,2,1,1),(3,3,2,0),(6,1,3,5),(6,2,3,4),(23887872,1,11943936,23887871)]
    for power,n in ladder:
        assert n==3**power
        for t in (0,1,2,3,243):
            examples.append((n,t,n//2,n-1))
    for n,t,c,p in examples:
        raw=subprocess.check_output([binary,"--query",str(n),str(t),str(c),str(p)],text=True)
        f=fields(raw)
        assert int(f["phase_mod3"])==t%3
        expected=reference_rank(n,t,c) if n<100 else (0 if t%3==0 else (1 if c==1 or c==n-1 else 2))
        assert int(f["cross_rank_GF3"])==expected
        assert int(f["schmidt_rank"])==3**int(f["cross_rank_GF3"])
        assert f["state_sha256"]==state(n,t%3)
        assert f["history_sha256"]==history(n,t)
        assert f["materialized_amplitudes"]=="0"
        checks+=1
    rows=subprocess.check_output([binary,"--ladder"],text=True).splitlines()
    assert len(rows)==3
    for row,(power,n) in zip(rows,ladder):
        cells=row.split("|")
        assert cells[0]=="OMEGA_LADDER" and cells[1]=="power="+str(power)
        v=fields("OMEGA_EQUATIONS|"+"|".join(cells[2:]))
        assert int(v["n"])==n and int(v["cross_rank_GF3"])==2
        assert int(v["schmidt_rank"])==9
        assert v["state_sha256"]==state(n,1)
        assert v["history_sha256"]==history(n,1)
        checks+=1
    assert state(6,0)==state(6,3%3)
    assert history(6,0)!=history(6,3)
    checks+=2
    open_socket=socket.socket()
    open_socket.bind(("127.0.0.1",0))
    port=open_socket.getsockname()[1]
    open_socket.close()
    process=subprocess.Popen([binary,"--serve",str(port)],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
    try:
        sock=None
        for unused in range(10):
            try:
                sock=socket.create_connection(("127.0.0.1",port),timeout=3)
                break
            except OSError:time.sleep(1)
        assert sock is not None
        sock.sendall(b"GET /equations/v1/graph/6/1/3/2 HTTP/1.1\r\nHost: localhost\r\n\r\n")
        sock.shutdown(socket.SHUT_WR)
        body=b""
        while True:
            part=sock.recv(8192)
            if not part:break
            body+=part
        sock.close()
        assert body.startswith(b"HTTP/1.1 200 OK")
        assert b"schmidt_rank=9" in body
        assert b"Content-Type: text/plain" in body
        assert b"<html" not in body
        checks+=1
        for path in (
            "/equations/v1/ladder",
            "/equations/v1/graph/43046721/1/21523360/43046720",
            "/equations/v1/graph/129140163/1/64570081/129140162",
            "/equations/v1/graph/387420489/1/193710244/387420488",
        ):
            with socket.create_connection(("127.0.0.1",port),timeout=3) as client:
                client.sendall(("GET "+path+" HTTP/1.1\r\nHost: localhost\r\n\r\n").encode("ascii"))
                client.shutdown(socket.SHUT_WR)
                response=b""
                while True:
                    chunk=client.recv(8192)
                    if not chunk:break
                    response+=chunk
            assert response.startswith(b"HTTP/1.1 200 OK")
            assert b"materialized_amplitudes=0" in response
            assert b"<html" not in response
            if path.endswith("ladder"):
                assert response.count(b"OMEGA_LADDER|")==3
            checks+=1
        with socket.create_connection(("127.0.0.1",port),timeout=3) as client:
            client.sendall(b"GET /equations/v1/graph/387420490/1/1/0 HTTP/1.1\r\nHost: localhost\r\n\r\n")
            client.shutdown(socket.SHUT_WR)
            response=client.recv(8192)
        assert response.startswith(b"HTTP/1.1 400 Bad Request")
        checks+=1
    finally:
        process.terminate()
        process.wait(timeout=10)
    print("OMEGA_INDEPENDENT|checks="+str(checks)+"|GF3=PASS|hash=PASS|HTTP0=PASS|ladder=3^16,3^17,3^18|max_symbolic_n=387420489|json=0")

if __name__=="__main__":
    if len(sys.argv)!=2:raise SystemExit("usage: verify_equations.py compiled_binary")
    verify(sys.argv[1])
