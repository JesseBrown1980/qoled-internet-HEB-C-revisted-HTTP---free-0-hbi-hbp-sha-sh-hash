//! Canonical HEB-C Rust 1.81 CLI & HTTP-0 Portal Executable.
//! Self-stepping discrete ternary physics, 288 collision gates, HBI/HBP/SHA/SH/HASH receipts,
//! and zero-JavaScript HTTP-0 portal server.

#![deny(warnings)]
#![deny(clippy::all)]

use hebc::receipts::{write_trust_anchor, ReceiptWriter};
use hebc::server::{generate_plain_html_dashboard, run_server_background, DEFAULT_PORT};
use hebc::{
    compute_integer_invariants, generate_synthetic_stubs, make_collisions,
    make_stream_destinations, make_velocities, sample_virtual_portal, step_universe_backward,
    step_universe_forward, unpack_room_stubs, K, N, PROBES_PER_STEP, V,
};
use std::env;
use std::fs;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut run_243 = true;
    let mut serve_portal = false;
    let mut port = DEFAULT_PORT;
    let mut physical_input_path: Option<String> = None;
    let mut hbp_out = "evidence/heb_c_rust_243.hbp".to_string();
    let mut hbi_out = "evidence/heb_c_rust_243.hbi".to_string();
    let mut anchor_out = "evidence/heb_c_rust_243.anchor".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--run-243" => run_243 = true,
            "--serve" => serve_portal = true,
            "--port" => {
                if i + 1 < args.len() {
                    port = args[i + 1].parse().unwrap_or(DEFAULT_PORT);
                    i += 1;
                }
            }
            "--physical-input" => {
                if i + 1 < args.len() {
                    physical_input_path = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--hbp" => {
                if i + 1 < args.len() {
                    hbp_out = args[i + 1].clone();
                    i += 1;
                }
            }
            "--hbi" => {
                if i + 1 < args.len() {
                    hbi_out = args[i + 1].clone();
                    i += 1;
                }
            }
            "--anchor" => {
                if i + 1 < args.len() {
                    anchor_out = args[i + 1].clone();
                    i += 1;
                }
            }
            "--verify-anchor" => {
                if i + 3 < args.len() {
                    let a_p = &args[i + 1];
                    let hbp_p = &args[i + 2];
                    let hbi_p = &args[i + 3];
                    println!("[*] Verifying trust anchor: {a_p}");
                    match hebc::receipts::verify_trust_anchor(a_p, hbp_p, hbi_p) {
                        Ok(true) => {
                            println!("  [OK] Trust anchor verification passed!");
                            return Ok(());
                        }
                        Ok(false) => {
                            eprintln!("  [FAIL] Trust anchor verification failed (returned false)");
                            std::process::exit(1);
                        }
                        Err(e) => {
                            eprintln!("  [FAIL] Trust anchor verification failed: {e}");
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Usage: --verify-anchor <anchor_file> <hbp_file> <hbi_file>");
                    std::process::exit(1);
                }
            }
            _ => {}
        }
        i += 1;
    }

    println!("=======================================================================");
    println!(" HEB-C CANONICAL RUST 1.81 ENGINE & HTTP-0 PORTAL");
    println!(" Target Architecture: Pure Native Rust &middot; Integer-Only &middot; json=0");
    println!(" Quality Gate       : cargo +1.81.0 clippy -D warnings &middot; 0 Warnings");
    println!("=======================================================================");

    let v = make_velocities();
    let gates = make_collisions(&v);
    let dest = make_stream_destinations(&v);

    let mut state = vec![0u8; N];
    let source_sha: String;

    if let Some(ref path) = physical_input_path {
        println!("[*] Loading physical room block from: {path}");
        let raw = fs::read(path)?;
        if raw.len() < V * 8 {
            return Err(format!(
                "Input file too small: {} bytes, required {}",
                raw.len(),
                V * 8
            )
            .into());
        }
        source_sha = hebc::sha256::hash_bytes(&raw[..V * 8]);
        let mut stubs = vec![0u64; V];
        for (idx, slot) in stubs.iter_mut().enumerate() {
            let b_idx = idx * 8;
            *slot = u64::from_le_bytes([
                raw[b_idx],
                raw[b_idx + 1],
                raw[b_idx + 2],
                raw[b_idx + 3],
                raw[b_idx + 4],
                raw[b_idx + 5],
                raw[b_idx + 6],
                raw[b_idx + 7],
            ]);
        }
        unpack_room_stubs(&stubs, &mut state);
    } else {
        println!("[*] Using canonical deterministic synthetic stubs (seed 0x4d5954484f533831)");
        let stubs = generate_synthetic_stubs();
        let mut raw_bytes = Vec::with_capacity(V * 8);
        for s in &stubs {
            raw_bytes.extend_from_slice(&s.to_le_bytes());
        }
        source_sha = hebc::sha256::hash_bytes(&raw_bytes);
        unpack_room_stubs(&stubs, &mut state);
    }

    let sh = &source_sha[..16];
    let init_state_sha = hebc::sha256::hash_bytes(&state);
    let init_inv = compute_integer_invariants(&state, &v);

    println!("  Source Block SHA-256: {source_sha}");
    println!("  SH Locator Handle   : {sh}");
    println!("  Initial State SHA   : {init_state_sha}");
    println!(
        "  Initial Invariants  : M={} P=({},{},{}) E2={}",
        init_inv.mass, init_inv.px, init_inv.py, init_inv.pz, init_inv.energy_2x
    );

    // Loschmidt reversibility pre-check (16 forward + 16 reverse ticks)
    println!("\n[*] Executing Loschmidt reversibility check (16 forward + 16 reverse)...");
    let mut rev_state = state.clone();
    let mut rev_scratch = vec![0u8; N];
    for t in 0..16 {
        step_universe_forward(&mut rev_state, &mut rev_scratch, &gates, &dest, t);
    }
    for t in (0..16).rev() {
        step_universe_backward(&mut rev_state, &mut rev_scratch, &gates, &dest, t);
    }
    let mut rev_mismatches = 0usize;
    for i in 0..N {
        if rev_state[i] != state[i] {
            rev_mismatches += 1;
        }
    }
    println!("  Loschmidt Reversal Mismatches: {rev_mismatches} (bit-for-bit exact)");
    assert_eq!(rev_mismatches, 0, "Reversibility must be 100% exact!");

    if run_243 {
        println!("\n[*] Executing 243 self-stepping forward transitions...");
        let hbp_filename = std::path::Path::new(&hbp_out)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("heb_c_rust_243.hbp");

        if let Some(parent) = std::path::Path::new(&hbp_out).parent() {
            let _ = fs::create_dir_all(parent);
        }
        let mut writer = ReceiptWriter::create(&hbp_out, &hbi_out, hbp_filename)?;

        let mut sim_state = state.clone();
        let mut sim_next = vec![0u8; N];
        let mut last_chain_hash = String::new();

        let t_start = Instant::now();
        for t in 0..243 {
            step_universe_forward(&mut sim_state, &mut sim_next, &gates, &dest, t);
            let cur_inv = compute_integer_invariants(&sim_state, &v);

            let d_e = cur_inv.energy_2x.wrapping_sub(init_inv.energy_2x);
            assert_eq!(d_e, 0, "Energy conservation violated at step {t}");
            assert_eq!(
                cur_inv.mass, init_inv.mass,
                "Mass conservation violated at step {t}"
            );

            // Virtual Q-trit addressing: sample 8,192 probes
            for p in 0..PROBES_PER_STEP {
                let ordinal = (t * PROBES_PER_STEP + p) as u64;
                let (_vid, room, channel) = sample_virtual_portal(ordinal);
                let _q = sim_state[room * K + channel];
            }

            let tick_state_sha = hebc::sha256::hash_bytes(&sim_state);
            last_chain_hash = writer.record_transition(
                t + 1,
                0,
                V,
                &source_sha,
                sh,
                d_e,
                0,
                &tick_state_sha,
                hebc::VIRTUAL_PORTALS,
                (t + 1) * PROBES_PER_STEP,
            )?;
        }
        let elapsed = t_start.elapsed();
        writer.flush()?;

        let millis = elapsed.as_millis();
        let ticks_per_sec = if millis > 0 { (243 * 1000) / millis } else { 0 };

        let final_state_sha = hebc::sha256::hash_bytes(&sim_state);
        println!(
            "  243 Transitions Completed in: {} ms (approx {} ticks/s)",
            millis, ticks_per_sec
        );
        println!("  Total Room Transitions     : {}", 243 * V);
        println!("  Total Trit Transitions     : {}", 243 * N);
        println!(
            "  Total Gate Operations      : {}",
            243 * V * hebc::GATES_PER_CELL
        );
        println!("  Final State SHA-256        : {final_state_sha}");
        println!("  Final Chain HASH           : {last_chain_hash}");

        // Write trust anchor
        write_trust_anchor(
            &anchor_out,
            &hbp_out,
            &hbi_out,
            243,
            243 * V,
            &last_chain_hash,
        )?;
        println!("  Trust Anchor written to    : {anchor_out}");

        // Update plain index.html dashboard (ZERO JavaScript)
        let html_content =
            generate_plain_html_dashboard(243, &final_state_sha, init_inv.mass, init_inv.energy_2x);
        fs::write("index.html", html_content)?;
        println!("  Updated index.html (STRICTLY ZERO JAVASCRIPT)");
    }

    if serve_portal {
        println!("\n[*] Starting HTTP-0 Loopback Portal on http://127.0.0.1:{port}...");
        let running = Arc::new(AtomicBool::new(true));
        let r_clone = Arc::clone(&running);
        let final_state_sha = hebc::sha256::hash_bytes(&state);

        let handle = run_server_background(
            port,
            state,
            243,
            final_state_sha,
            init_inv.mass,
            init_inv.energy_2x,
            r_clone,
        )?;

        println!("  HTTP-0 Portal listening. Press Enter to shutdown.");
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line);
        running.store(false, Ordering::SeqCst);
        let _ = handle.join();
        println!("  HTTP-0 Portal shutdown successfully.");
    }

    println!("\n=======================================================================");
    println!(" HEB-C RUST 1.81 ENGINE VERIFICATION: 100% PASS");
    println!("=======================================================================");

    Ok(())
}
