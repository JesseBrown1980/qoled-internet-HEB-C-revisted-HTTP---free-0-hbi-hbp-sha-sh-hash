//! Source-pinned Asolaria constellation portal registry.
//! HBI/HBP hash commitments are to *catalog pointers*, not remote file bytes.
//! No source is fetched, reconstructed, executed or presumed installed by this module.
#![forbid(unsafe_code)]
use crate::sha256::hash_bytes;
use std::path::Path;

#[derive(Clone,Copy)]
pub struct Source {
    pub id:&'static str,
    pub repo:&'static str,
    pub commit:&'static str,
    pub path:&'static str,
    /// Git blob SHA-1 (NOT SHA-256); checked against the source revision
    /// only when an external fetch and Git blob verification is performed.
    pub blob:&'static str,
}
pub const SOURCES:&[Source]=&[
    Source { id:"CANON_LAWS", repo:"JesseBrown1980/ASOLARIA-UNIVERSE-SIMULATOR-with-gravity-light-time-space-color-rime-winding-drills", commit:"3f9f42dab6247c40b30921b253e3f73c9e942ca5", path:"books/COMBINED-BOOK-OF-LAWS.md", blob:"82b3fe3e833ab34a21451e57f4fb9b0a8d5304f5" },
    Source { id:"BOOK_OF_SHADOWS", repo:"JesseBrown1980/ASOLARIA-UNIVERSE-SIMULATOR-with-gravity-light-time-space-color-rime-winding-drills", commit:"3f9f42dab6247c40b30921b253e3f73c9e942ca5", path:"books/BOOKS-OF-SHADOWS.md", blob:"6369030024fc110c77f31ce0fd0c2d4ab8628d3c" },
    Source { id:"RIME_RELATION", repo:"JesseBrown1980/ASOLARIA-UNIVERSE-SIMULATOR-with-gravity-light-time-space-color-rime-winding-drills", commit:"3f9f42dab6247c40b30921b253e3f73c9e942ca5", path:"books/LAW-56-THE-RIME-IS-THE-RELATION.md", blob:"659c832a848cd7af12b6a7c98cad680e6e695848" },
    Source { id:"THIRDS_LAW", repo:"JesseBrown1980/ASOLARIA-UNIVERSE-SIMULATOR-with-gravity-light-time-space-color-rime-winding-drills", commit:"3f9f42dab6247c40b30921b253e3f73c9e942ca5", path:"books/LAW-70-LEVELS-OF-SHADOW-CATS.md", blob:"ae05ff790b1c8ce76564d58f5a9dbc95405092d8" },
    Source { id:"TRIANTI_LAW", repo:"JesseBrown1980/ASOLARIA-UNIVERSE-SIMULATOR-with-gravity-light-time-space-color-rime-winding-drills", commit:"3f9f42dab6247c40b30921b253e3f73c9e942ca5", path:"books/LAW-71-HOTTER-BLACK-SHADOW-CAT-SPHERICAL-AMPLIFICATION.md", blob:"24e5e8837e40804a19bb0bf5048985c179d35272" },
    Source { id:"CONSTELLATION", repo:"JesseBrown1980/Clasy-repo-x-3-", commit:"63cbf283b900457f8dfc59ac0e42d3efcbc9aedb", path:"vendor/lattice/oscillator.rs", blob:"619344880359eaa5f0fcd0d3e12a83cd39425ff5" },
    Source { id:"TRIT", repo:"JesseBrown1980/Clasy-repo-x-3-", commit:"63cbf283b900457f8dfc59ac0e42d3efcbc9aedb", path:"vendor/lattice/trit.rs", blob:"3fb7ba9228cab4c40cdfc2732588e868a45b76ed" },
    Source { id:"MATRIX_3D", repo:"JesseBrown1980/qprism-3d-slice-harness", commit:"2f48332921057570814a1519b27e272eb501711c", path:"src/lib.rs", blob:"6b2d133e6a111db570fae4a17d453abe665cd4cb" },
    Source { id:"YANG_MILLS_TORUS", repo:"JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C", commit:"c0ce4f4233d3a27016bf682318811f92e0369279", path:"native/src/torus2.rs", blob:"b8edc277b1ed62e94f6886102aa0772ffb58fbcc" },
    Source { id:"YANG_MILLS_CONSTELLATION", repo:"JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C", commit:"c0ce4f4233d3a27016bf682318811f92e0369279", path:"native/src/constellation_gate.rs", blob:"b70a800a13858787095d6411850b0e1b58d7494a" },
    Source { id:"YANG_MILLS_RESIDUAL", repo:"JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C", commit:"c0ce4f4233d3a27016bf682318811f92e0369279", path:"native/src/shadow_residual.rs", blob:"2c98bd6204eaf94366b7f744f4463f9b21025596" },
    Source { id:"YANG_MILLS_RIME_GEARS", repo:"JesseBrown1980/Finite-universe-Yang-mills-existence-and-mass-gap-using-HEB-C", commit:"c0ce4f4233d3a27016bf682318811f92e0369279", path:"native/src/rime_gear.rs", blob:"74ea468d5bfdd8d4c2ba599bac2d081027c046e1" },
];

pub const LINKS:&[(&str,&str,&str)]=&[
    ("CANON_LAWS","BOOK_OF_SHADOWS","contains"),
    ("CANON_LAWS","THIRDS_LAW","contains"),
    ("CANON_LAWS","TRIANTI_LAW","contains"),
    ("BOOK_OF_SHADOWS","MATRIX_3D","view_candidate"),
    ("RIME_RELATION","CONSTELLATION","interpretation_candidate"),
    ("CONSTELLATION","TRIT","uses"),
    ("CONSTELLATION","YANG_MILLS_CONSTELLATION","pinned_source_in_adapter"),
    ("YANG_MILLS_TORUS","YANG_MILLS_CONSTELLATION","same_finite_project"),
    ("TRIANTI_LAW","YANG_MILLS_RESIDUAL","structural_C3_candidate_only"),
    ("THIRDS_LAW","YANG_MILLS_RIME_GEARS","bookkeeping_test_only"),
];

/// Git blob OID and commit pin cannot decompress or execute model weights.
/// GGUF is a model container and its runtime needs full weights and a compatible loader.
pub const MODEL_GATE:&str="MODEL|format=GGUF|repo=UNRESOLVED|revision=UNRESOLVED|path=UNRESOLVED|sha256=UNRESOLVED|bytes=NOT_PRESENT|loader=NOT_CONNECTED|executable=0|catalog_only=1|json=0";

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Vowel {
    pub letter:char,
    pub grid:[u8;3],
    /// Illustrative 3D vowel formant labels, not a validated voice model.
    pub formants_hz:[u16;3],
}
pub const VOWELS:[Vowel;4]=[
    Vowel{letter:'A',grid:[0b101,0b111,0b101],formants_hz:[800,1150,2900]},
    Vowel{letter:'O',grid:[0b111,0b101,0b111],formants_hz:[500,900,2500]},
    Vowel{letter:'I',grid:[0b111,0b010,0b111],formants_hz:[300,2200,3000]},
    Vowel{letter:'E',grid:[0b111,0b110,0b111],formants_hz:[500,1850,2600]},
];
pub fn vowel_shape(letter:char)->Option<String>{
    let vowel=VOWELS.iter().find(|v|v.letter==letter.to_ascii_uppercase())?;
    let mut body=format!(
        "VOWEL|letter={}|kind=TERNARY_3D_GLYPH_TEMPLATE|axes=x_y_z|formants_hz={},{},{}|speech_synthesis=NOT_VALIDATED|json=0\n",
        vowel.letter,vowel.formants_hz[0],vowel.formants_hz[1],vowel.formants_hz[2]);
    // Three nested y-layers; x/z space is a 3x3 glyph plane.
    // The center layer is complete, flanking layers retain only endpoints.
    for y in -1..=1 {
        for z in -1..=1 {
            for x in -1..=1 {
                let bit=1u8 << (x+1);
                let on=(vowel.grid[(z+1) as usize]&bit)!=0;
                if on && (y==0 || x!=0) {
                    body.push_str(&format!("POINT|vowel={}|x={x}|y={y}|z={z}|color=RGB_TEMPLATE|json=0\n",vowel.letter));
                }
            }
        }
    }
    let sha=hash_bytes(body.as_bytes());
    body.push_str(&format!("SHAPE_SHA|vowel={}|sha256={sha}|json=0\n",vowel.letter));
    Some(body)
}

/// Portable, host-independent integer derivation of a VIEW coordinate.
/// This is a lossy 3D *projection* of the 256-bit node identity, not a
/// unique reconstruction key or a geometric description of the universe.
fn projected_xyz(id:&str)->[i8;3] {
    let sha=hash_bytes(id.as_bytes());
    let mut coords=[0_i8;3];
    for (i,c) in coords.iter_mut().enumerate(){
        let hex=sha.as_bytes()[i];
        *c=match hex%3 {0=>-1,1=>0,_=>1};
    }
    coords
}
pub fn catalog()->String{
    let mut text=String::from("FABRIC|schema=hebc.constellation.catalog.v1|transport=HTTP0_LOCAL_ABSTRACTION|wire=HTTP1_1_OPTIONAL|source_pinned=1|source_bytes_embedded=0|json=0\n");
    text.push_str("LAWSET|series=RIME_0_TO_71|book=CANON_LAWS|unassigned=27|duplicate=58|book9_reported_rules=1940|all_external_evidence_audited=0|json=0\n");
    for source in SOURCES {
        let xyz=projected_xyz(source.id);
        text.push_str(&format!(
            "SOURCE|id={}|repo={}|commit={}|path={}|git_blob_oid={}|local_loaded=0|remote_verified_on_this_run=0|x={}|y={}|z={}|rgb=DERIVED_VIEW_ONLY|json=0\n",
            source.id,source.repo,source.commit,source.path,source.blob,xyz[0],xyz[1],xyz[2]
        ));
    }
    for &(a,b,relation) in LINKS {
        text.push_str(&format!("LINK|from={a}|to={b}|relation={relation}|exec=0|json=0\n"));
    }
    text.push_str(MODEL_GATE);
    text.push('\n');
    for vowel in VOWELS {
        if let Some(shape)=vowel_shape(vowel.letter) {
            text.push_str(&format!("VOWELREF|letter={}|sha256={}|kind=REGISTERED_GLYPH_NOT_EXECUTABLE_AUDIO|json=0\n",vowel.letter,hash_bytes(shape.as_bytes())));
        }
    }
    let commit=hash_bytes(text.as_bytes());
    text.push_str(&format!("CATALOGSHA|algorithm=SHA256|value={commit}|sh={}|json=0\n",&commit[..16]));
    text
}
pub struct Evidence {
    pub hbp:Vec<u8>,pub hbi:Vec<u8>,pub anchor:String
}
pub fn evidence()->Evidence{
    let catalog=catalog();
    let payload_hash=hash_bytes(catalog.as_bytes());
    let hbp=format!("HBP|scope=PINNED_SOURCE_CATALOG_NOT_REMOTE_WEIGHTS|source_sha={payload_hash}|payload_length={}|json=0\n{catalog}",catalog.len()).into_bytes();
    let hbp_sha=hash_bytes(&hbp);
    let hbi=format!("HBI|schema=hebc.constellation.catalog.v1|offset=0|len={}|record_sha={hbp_sha}|sh={}|json=0\n",hbp.len(),&hbp_sha[..16]).into_bytes();
    let anchor=format!("ANCHOR|schema=hebc.constellation.catalog.v1|hbp_sha={hbp_sha}|hbi_sha={}|independently_signed=0|json=0\n",hash_bytes(&hbi));
    Evidence {hbp,hbi,anchor}
}
pub fn write(dir:&Path)->Result<(),String>{
    std::fs::create_dir_all(dir).map_err(|e|e.to_string())?;
    let e=evidence();
    for (name,data) in [
        ("catalog.hbp",e.hbp.as_slice()),
        ("catalog.hbi",e.hbi.as_slice()),
        ("catalog.anchor",e.anchor.as_bytes())
    ] {
        std::fs::write(dir.join(name),data).map_err(|e|e.to_string())?;
    }
    Ok(())
}
pub fn verify(dir:&Path)->Result<(),String>{
    let actual=evidence();
    for (name,expected) in [
        ("catalog.hbp",actual.hbp.as_slice()),
        ("catalog.hbi",actual.hbi.as_slice()),
        ("catalog.anchor",actual.anchor.as_bytes())
    ] {
        let found=std::fs::read(dir.join(name)).map_err(|e|e.to_string())?;
        if found!=expected{return Err(format!("fabric source catalog corrupt or altered: {name}"));}
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pinned_source_manifest_and_mapped_links(){
        let rendered=catalog();
        for s in SOURCES {
            assert!(rendered.contains(&format!("id={}",s.id)));
            assert_eq!(s.commit.len(),40);
            assert_eq!(s.blob.len(),40);
        }
        for &(a,b,_) in LINKS {
            assert!(SOURCES.iter().any(|s|s.id==a));
            assert!(SOURCES.iter().any(|s|s.id==b));
        }
        assert!(rendered.contains("bytes=NOT_PRESENT"));
    }
    #[test]
    fn nested_vowel_glyphs_are_deterministic_but_not_speech(){
        let mut hashes=std::collections::BTreeSet::new();
        for v in VOWELS {
            let shape=vowel_shape(v.letter).unwrap();
            assert!(shape.contains("|y=0|"));
            assert!(shape.contains("speech_synthesis=NOT_VALIDATED"));
            assert!(hashes.insert(hash_bytes(shape.as_bytes())));
        }
        assert!(vowel_shape('U').is_none());
    }
    #[test]
    fn evidence_replay_and_corruption_rejection(){
        let d=std::env::temp_dir().join(format!("hebc-fabric-{}-{}",std::process::id(),std::thread::current().name().unwrap_or("test")));
        write(&d).unwrap();
        verify(&d).unwrap();
        let p=d.join("catalog.hbp");
        let mut b=std::fs::read(&p).unwrap();
        b[10]^=1;
        std::fs::write(&p,b).unwrap();
        assert!(verify(&d).is_err());
        std::fs::remove_dir_all(d).unwrap();
    }
}
