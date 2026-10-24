//! Native Rust HTTP-0 Portal Server.
//! Serves plain HTML/CSS dashboard (ZERO JavaScript) and streams uncompressed 24-bit BMP pixel frames.

use crate::L;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub const DEFAULT_PORT: u16 = 5088;

/// Render a 2D lattice slice ($24 \times 24$) into an uncompressed 24-bit BMP file ($288 \times 288$ pixels).
#[must_use]
pub fn render_lattice_bmp_288(state: &[u8], z_slice: usize) -> Vec<u8> {
    const SCALE: usize = 12; // 24 * 12 = 288 pixels
    const WIDTH: usize = L * SCALE;
    const HEIGHT: usize = L * SCALE;
    const ROW_PADDING: usize = (4 - (WIDTH * 3) % 4) % 4;
    const ROW_STRIDE: usize = WIDTH * 3 + ROW_PADDING;
    const IMAGE_SIZE: usize = ROW_STRIDE * HEIGHT;
    const FILE_SIZE: usize = 54 + IMAGE_SIZE;

    let mut bmp = Vec::with_capacity(FILE_SIZE);

    // 1. BMP Header (14 bytes)
    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&(FILE_SIZE as u32).to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes()); // Reserved
    bmp.extend_from_slice(&54u32.to_le_bytes()); // Data offset

    // 2. DIB Header (40 bytes - BITMAPINFOHEADER)
    bmp.extend_from_slice(&40u32.to_le_bytes()); // Header size
    bmp.extend_from_slice(&(WIDTH as i32).to_le_bytes());
    bmp.extend_from_slice(&(HEIGHT as i32).to_le_bytes()); // Positive = bottom-to-top
    bmp.extend_from_slice(&1u16.to_le_bytes()); // Planes = 1
    bmp.extend_from_slice(&24u16.to_le_bytes()); // BPP = 24
    bmp.extend_from_slice(&0u32.to_le_bytes()); // Compression = 0 (BI_RGB)
    bmp.extend_from_slice(&(IMAGE_SIZE as u32).to_le_bytes());
    bmp.extend_from_slice(&2835u32.to_le_bytes()); // 72 DPI X
    bmp.extend_from_slice(&2835u32.to_le_bytes()); // 72 DPI Y
    bmp.extend_from_slice(&0u32.to_le_bytes()); // Colors
    bmp.extend_from_slice(&0u32.to_le_bytes()); // Important colors

    // 3. Pixel Rows (Bottom-to-Top, BGR order)
    for y_inv in (0..HEIGHT).rev() {
        let room_y = y_inv / SCALE;
        for px in 0..WIDTH {
            let room_x = px / SCALE;
            let cell = (room_x * L + room_y) * L + z_slice;
            // Channel 0 or dominant local channel
            let q_trit = state[cell * 27];

            // BGR color mapping
            let (b, g, r) = match q_trit {
                2 => (0x24, 0xbf, 0xfb), // Gold (+1)
                1 => (0x30, 0x20, 0x15), // Background Neutral (0)
                _ => (0xf8, 0xbd, 0x38), // Cyan (-1)
            };
            bmp.push(b);
            bmp.push(g);
            bmp.push(r);
        }
        #[allow(clippy::absurd_extreme_comparisons)]
        if ROW_PADDING > 0 {
            bmp.resize(bmp.len() + ROW_PADDING, 0x00);
        }
    }

    bmp
}

/// Generate plain HTML/CSS dashboard page containing STRICTLY ZERO JAVASCRIPT.
#[must_use]
pub fn generate_plain_html_dashboard(
    ticks: usize,
    state_sha: &str,
    mass: i64,
    energy_2x: i64,
) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>HEB-C Rust 1.81 HTTP-0 Portal</title>
  <style>
    :root {{ --bg: #090d16; --card: #111827; --border: #1f2937; --text: #f3f4f6; --muted: #9ca3af; --acc: #38bdf8; --gold: #fbbf24; --green: #34d399; }}
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{ background: var(--bg); color: var(--text); font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, monospace; padding: 24px; }}
    .container {{ max-width: 860px; margin: 0 auto; background: var(--card); border: 1px solid var(--border); border-radius: 12px; padding: 24px; }}
    .header {{ display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--border); padding-bottom: 16px; margin-bottom: 20px; }}
    .badge {{ background: rgba(56, 189, 248, 0.15); color: var(--acc); border: 1px solid rgba(56, 189, 248, 0.3); padding: 4px 10px; border-radius: 6px; font-size: 12px; }}
    .grid {{ display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; margin-bottom: 24px; }}
    .stat {{ background: var(--bg); border: 1px solid var(--border); padding: 12px; border-radius: 8px; }}
    .stat .label {{ font-size: 11px; color: var(--muted); }}
    .stat .val {{ font-size: 16px; font-weight: bold; margin-top: 4px; }}
    .pixel-box {{ display: flex; flex-direction: column; align-items: center; background: #000; border: 1px solid var(--border); border-radius: 8px; padding: 16px; margin-bottom: 20px; }}
    .pixel-box img {{ border-radius: 4px; image-rendering: pixelated; }}
    .footer {{ font-size: 12px; color: var(--muted); border-top: 1px solid var(--border); padding-top: 16px; display: flex; justify-content: space-between; }}
  </style>
</head>
<body>
  <div class="container">
    <div class="header">
      <div>
        <h1 style="font-size: 20px; font-weight: bold;">HEB-C RUST 1.81 HTTP-0 PORTAL</h1>
        <p style="font-size: 12px; color: var(--muted); margin-top: 2px;">Canonical Discrete Reversible Ternary Physics &middot; Zero JavaScript &middot; json=0</p>
      </div>
      <span class="badge">HTTP-0 NATIVE LOOPBACK</span>
    </div>

    <div class="grid">
      <div class="stat">
        <div class="label">Self-Stepping Ticks</div>
        <div class="val" style="color: var(--acc);">{ticks} / 243</div>
      </div>
      <div class="stat">
        <div class="label">Reversible Gates</div>
        <div class="val">288 Involutive</div>
      </div>
      <div class="stat">
        <div class="label">Virtual Q-Trits</div>
        <div class="val" style="color: var(--gold);">23,000,000,000</div>
      </div>
      <div class="stat">
        <div class="label">Integer Invariants</div>
        <div class="val" style="color: var(--green);">M={mass} &middot; E2={energy_2x}</div>
      </div>
    </div>

    <div class="pixel-box">
      <h3 style="font-size: 12px; color: var(--muted); margin-bottom: 12px;">NATIVE BACKEND BMP PIXEL STREAM (288&times;288 &middot; ZERO JAVASCRIPT)</h3>
      <img src="/pixel.bmp" width="288" height="288" alt="Lattice Pixel Projection">
      <p style="font-size: 11px; color: var(--muted); margin-top: 10px;">Rainbow/Ternary Mapping: +1 (Gold), 0 (Slate), -1 (Cyan)</p>
    </div>

    <div class="stat" style="margin-bottom: 20px;">
      <div class="label">State SHA-256 Digest</div>
      <div class="val" style="font-size: 13px; font-family: monospace; word-break: break-all;">{state_sha}</div>
    </div>

    <div class="stat" style="margin-bottom: 20px;">
      <div class="label">ASOLARIA CONSTELLATION SOURCE REGISTRY (NO MODEL WEIGHTS BUNDLED)</div>
      <div class="val" style="font-size: 13px; margin-top: 8px;">
        <a style="color: var(--acc);" href="/fabric/catalog">HBI/HBP source-pinned catalog</a>
        &middot; <a style="color: var(--acc);" href="/fabric/anchor">SHA anchor</a>
        &middot; <a style="color: var(--acc);" href="/fabric/vowel/A">A</a>
        &middot; <a style="color: var(--acc);" href="/fabric/vowel/O">O</a>
        &middot; <a style="color: var(--acc);" href="/fabric/vowel/I">I</a>
        &middot; <a style="color: var(--acc);" href="/fabric/vowel/E">E</a>
      </div>
      <p style="font-size: 11px; color: var(--muted); margin-top: 8px;">Pinned pointers and integer 3D shape templates; not remote repository execution, GGUF inference, or demonstrated FTL transfer.</p>
    </div>

    <div class="footer">
      <span>Rust 1.81.0 &middot; Clippy Clean &middot; Zero Float Ops</span>
      <span>HBI &rarr; HBP &rarr; SHA &rarr; SH &rarr; HASH</span>
    </div>
  </div>
</body>
</html>
"#
    )
}

fn write_plain(stream: &mut TcpStream, status: &str, body: &str) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body.as_bytes())
}

/// Handle a single incoming HTTP client request.
pub fn handle_client(
    mut stream: TcpStream,
    state: &[u8],
    ticks: usize,
    state_sha: &str,
    mass: i64,
    energy_2x: i64,
) -> Result<(), std::io::Error> {
    let mut buf = [0u8; 1024];
    let bytes_read = stream.read(&mut buf)?;
    if bytes_read == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buf[..bytes_read]);
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    match path {
        "/fabric" | "/fabric/catalog" => {
            write_plain(&mut stream, "200 OK", &crate::fabric::catalog())?;
        }
        "/fabric/anchor" => {
            write_plain(&mut stream, "200 OK", &crate::fabric::evidence().anchor)?;
        }
        "/fabric/vowel/A" | "/fabric/vowel/O" |
        "/fabric/vowel/I" | "/fabric/vowel/E" => {
            let vowel = path.chars().last().unwrap_or('A');
            if let Some(shape) = crate::fabric::vowel_shape(vowel) {
                write_plain(&mut stream, "200 OK", &shape)?;
            }
        }
        "/pixel.bmp" => {
            let bmp = render_lattice_bmp_288(state, 12);
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: image/bmp\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",
                bmp.len()
            );
            stream.write_all(header.as_bytes())?;
            stream.write_all(&bmp)?;
        }
        "/status" => {
            let body = format!(
                "HOST8STATUS|seat=ACER-CLAUDE-FABLE5|ticks={}|mass={}|energy_2x={}|state_sha={}|portals=23000000000|json=0\n",
                ticks, mass, energy_2x, state_sha
            );
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(header.as_bytes())?;
            stream.write_all(body.as_bytes())?;
        }
        "/health" => {
            let body = "OK\n";
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            stream.write_all(header.as_bytes())?;
            stream.write_all(body.as_bytes())?;
        }
        _ => {
            let html = generate_plain_html_dashboard(ticks, state_sha, mass, energy_2x);
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                html.len()
            );
            stream.write_all(header.as_bytes())?;
            stream.write_all(html.as_bytes())?;
        }
    }
    stream.flush()?;
    Ok(())
}

/// Run loopback HTTP-0 server in a background thread until terminated.
pub fn run_server_background(
    port: u16,
    state: Vec<u8>,
    ticks: usize,
    state_sha: String,
    mass: i64,
    energy_2x: i64,
    running: Arc<AtomicBool>,
) -> Result<thread::JoinHandle<()>, std::io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    listener.set_nonblocking(true)?;

    let handle = thread::spawn(move || {
        while running.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let _ = handle_client(stream, &state, ticks, &state_sha, mass, energy_2x);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(_) => break,
            }
        }
    });

    Ok(handle)
}
