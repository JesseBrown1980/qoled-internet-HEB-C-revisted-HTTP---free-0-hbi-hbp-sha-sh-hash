//! Headless HTTP-0 equation-only qutrit graph states; no amplitudes stored.
#![forbid(unsafe_code)]
#![deny(warnings)]
use std::env;
use std::io::{Read, Write};
use std::net::TcpListener;

const MAX_N: u32=23_887_872;
const MAX_T: u16=243;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
struct Query { n:u32, t:u16, cut:u32, probe:u32 }
impl Query {
    fn new(n:u32,t:u16,cut:u32,probe:u32)->Result<Self,&'static str>{
        if !(3..=MAX_N).contains(&n){return Err("INVALID_N")}
        if t>MAX_T{return Err("INVALID_SLICE")}
        if cut==0 || cut>=n{return Err("INVALID_CUT")}
        if probe>=n{return Err("INVALID_PROBE")}
        Ok(Self{n,t,cut,probe})
    }
    fn w(self)->u8{(self.t%3) as u8}
    fn phase(self)->u8{self.w()}
    fn rank(self)->u8{
        if self.w()==0{0}
        else if self.cut==1 || self.cut+1==self.n{1}
        else{2}
    }
    fn schmidt(self)->u8{match self.rank(){0=>1,1=>3,_=>9}}
}
fn state_sha(n:u32,w:u8)->String{
    let spec=format!("OMEGA_QUTRIT_RING_V1|q=3|n={n}|edges=cycle|weight={w}|normalization=3^(-n/2)|json=0\n");
    hebc::sha256::hash_bytes(spec.as_bytes())
}
fn history_sha(n:u32,t:u16)->String{
    let start=format!("OMEGA_QUTRIT_GENESIS_V1|n={n}|json=0\n");
    let mut prev=hebc::sha256::hash_bytes(start.as_bytes());
    for s in 0..=t{
        let hash=state_sha(n,(s%3) as u8);
        let row=format!("OMEGA_SLICE_V1|n={n}|slice={s}|state_sha256={hash}|previous={prev}|json=0\n");
        prev=hebc::sha256::hash_bytes(row.as_bytes());
    }
    prev
}
fn report(q:Query)->String{
    format!("OMEGA_EQUATIONS|kind=qutrit_cycle_graph|n={}|slice={}|cut={}|probe={}|edge_weight={}|phase_mod3={}|cross_rank_GF3={}|schmidt_rank={}|entropy_unit=log2(3)|entropy_coefficient={}|amplitude_formula=3^(-n/2)*omega^phase|state_sha256={}|history_sha256={}|equation_only=1|materialized_amplitudes=0|physical_quantum_hardware=0|learning_not_measured=1|json=0\n",
        q.n,q.t,q.cut,q.probe,q.w(),q.phase(),q.rank(),q.schmidt(),q.rank(),state_sha(q.n,q.w()),history_sha(q.n,q.t))
}
fn parse(parts:&[&str])->Result<Query,&'static str>{
    if parts.len()!=4{return Err("EXPECTED_N_SLICE_CUT_PROBE")}
    let n=parts[0].parse::<u32>().map_err(|_|"INVALID_N")?;
    let t=parts[1].parse::<u16>().map_err(|_|"INVALID_SLICE")?;
    let c=parts[2].parse::<u32>().map_err(|_|"INVALID_CUT")?;
    let p=parts[3].parse::<u32>().map_err(|_|"INVALID_PROBE")?;
    Query::new(n,t,c,p)
}
fn route(req:&str)->(&'static str,String){
    let mut fields=req.lines().next().unwrap_or("").split_whitespace();
    let (Some(method),Some(path),Some(version))=(fields.next(),fields.next(),fields.next())
    else{return("400 Bad Request","OMEGA_ERROR|reason=MALFORMED_REQUEST|json=0\n".into())};
    if method!="GET" || !version.starts_with("HTTP/1.") || fields.next().is_some(){
        return("405 Method Not Allowed","OMEGA_ERROR|reason=GET_ONLY|json=0\n".into())
    }
    if path=="/health"{return("200 OK","OMEGA_HEALTH|backend=equation_only|json=0\n".into())}
    let p:Vec<_>=path.split('/').collect();
    if p.len()!=8 || p[0]!="" || p[1]!="equations" || p[2]!="v1" || p[3]!="graph"{
        return("404 Not Found","OMEGA_ERROR|reason=UNKNOWN_ROUTE|json=0\n".into())
    }
    match parse(&p[4..]){
        Ok(q)=>("200 OK",report(q)),
        Err(e)=>("400 Bad Request",format!("OMEGA_ERROR|reason={e}|json=0\n"))
    }
}
fn serve(port:u16)->std::io::Result<()>{
    let server=TcpListener::bind(("127.0.0.1",port))?;
    println!("OMEGA_HTTP0|mode=headless|bind=127.0.0.1|port={port}|json=0");
    for client in server.incoming(){
        let mut stream=client?;
        let mut buf=[0u8;1024];
        let n=stream.read(&mut buf)?;
        let req=std::str::from_utf8(&buf[..n]).unwrap_or("");
        let (status,body)=route(req);
        let header=format!("HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",body.len());
        stream.write_all(header.as_bytes())?;
        stream.write_all(body.as_bytes())?;
    }
    Ok(())
}
fn main()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<String>=env::args().collect();
    match args.get(1).map(String::as_str){
        Some("--query")=>{
            let p:Vec<_>=args[2..].iter().map(String::as_str).collect();
            let q=parse(&p).map_err(std::io::Error::other)?;
            print!("{}",report(q));
        }
        Some("--serve")=>{
            if args.len()!=3{return Err("usage: --serve PORT".into())}
            serve(args[2].parse::<u16>()?)?;
        }
        _=>return Err("usage: --query N SLICE CUT PROBE | --serve PORT".into()),
    }
    Ok(())
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn prediction_first(){
        for (n,t,cut,phase,rank) in [
            (3,0,1,0,0),(3,1,1,1,1),(3,2,1,2,1),(3,3,1,0,0),
            (6,1,3,1,2),(6,2,3,2,2),(23_887_872,1,11_943_936,1,2)
        ]{
            let q=Query::new(n,t,cut,0).unwrap();
            assert_eq!((q.phase(),q.rank()),(phase,rank));
        }
    }
    #[test]
    fn repeating_state_is_not_repeating_history(){
        assert_eq!(state_sha(6,0),state_sha(6,3%3));
        assert_ne!(state_sha(6,0),state_sha(6,1));
        assert_ne!(history_sha(6,0),history_sha(6,3));
    }
    #[test]
    fn reject_faults(){
        assert!(Query::new(2,0,1,0).is_err());
        assert!(Query::new(MAX_N+1,0,1,0).is_err());
        assert!(Query::new(3,MAX_T+1,1,0).is_err());
        assert!(Query::new(3,1,0,0).is_err());
        assert!(Query::new(3,1,1,3).is_err());
        assert_eq!(route("GET /pixel.bmp HTTP/1.1\r\n\r\n").0,"404 Not Found");
        assert_eq!(route("POST /equations/v1/graph/6/1/3/2 HTTP/1.1\r\n\r\n").0,"405 Method Not Allowed");
    }
    #[test]
    fn http_and_cli_equations_match(){
        let q=Query::new(6,1,3,2).unwrap();
        let (status,body)=route("GET /equations/v1/graph/6/1/3/2 HTTP/1.1\r\n\r\n");
        assert_eq!(status,"200 OK");
        assert_eq!(body,report(q));
        assert!(body.contains("schmidt_rank=9"));
    }
}
