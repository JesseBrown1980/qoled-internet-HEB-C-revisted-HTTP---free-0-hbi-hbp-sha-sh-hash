# Requires rustup with Rust 1.81.0 available. Headless, no Node/JS/JSON.
$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
Set-Location (Resolve-Path (Join-Path $PSScriptRoot ".."))
$root = (Get-Location).Path
$out = Join-Path $root "repro-output"
foreach ($name in @("fixture","synthetic","physical","synthetic-again","physical-again")) {
    New-Item -ItemType Directory -Force (Join-Path $out $name) | Out-Null
}
function Checked($command, [string[]]$args) {
    & $command @args
    if ($LASTEXITCODE -ne 0) { throw "Command failed: $command" }
}
Checked cargo @("+1.81.0", "fmt", "--all", "--", "--check")
Checked cargo @("+1.81.0", "clippy", "--all-targets", "--locked", "--offline", "--", "-D", "warnings")
Checked cargo @("+1.81.0", "test", "--all-targets", "--release", "--locked", "--offline")
Checked cargo @("+1.81.0", "build", "--release", "--locked", "--offline")
$bin = Join-Path $root "target\release\hebc-portal.exe"
$fixture = Join-Path $out "fixture\sector-000-window0.rooms"
Checked $bin @("--write-host8-window", $fixture)
$sha = (Get-FileHash $fixture -Algorithm SHA256).Hash.ToLowerInvariant()
if ($sha -ne "2dfe7d757ecb012475806b86fcdefc602680d042db836ffbcb2bfe924f2a18ee") { throw "Physical block SHA mismatch" }

function NativeRun([string]$name, [bool]$physical) {
    $dir = Join-Path $out $name
    $p = @("--run-243", "--headless")
    if ($physical) { $p += @("--physical-input", $fixture, "--expect-input-sha", "2dfe7d757ecb012475806b86fcdefc602680d042db836ffbcb2bfe924f2a18ee") }
    $p += @("--hbp", (Join-Path $dir "run.hbp"), "--hbi", (Join-Path $dir "run.hbi"), "--anchor", (Join-Path $dir "run.anchor"))
    & $bin @p | Out-File (Join-Path $dir "stdout.hbp") -Encoding utf8
    if ($LASTEXITCODE -ne 0) { throw "Native execution failed: $name" }
    Checked $bin @("--verify-anchor", (Join-Path $dir "run.anchor"), (Join-Path $dir "run.hbp"), (Join-Path $dir "run.hbi"))
}
NativeRun "synthetic" $false
NativeRun "physical" $true
NativeRun "synthetic-again" $false
NativeRun "physical-again" $true
foreach ($mode in @("synthetic","physical")) {
    foreach ($filename in @("run.hbp","run.hbi","run.anchor")) {
        $a = (Get-FileHash (Join-Path $out "$mode\$filename") -Algorithm SHA256).Hash
        $b = (Get-FileHash (Join-Path $out "$mode-again\$filename") -Algorithm SHA256).Hash
        if ($a -ne $b) { throw "Mismatch in $mode/$filename" }
    }
}
if (-not (Select-String -Path (Join-Path $out "synthetic\stdout.hbp") -Pattern "42fb223a755aef7c4bbefbef87af830d1e19ed2fc64de018d9f412493f41b475" -Quiet)) { throw "Synthetic state mismatch" }
if (-not (Select-String -Path (Join-Path $out "physical\stdout.hbp") -Pattern "d33eaf8d8d156377957a42f8536e667b46ce6f22b3cb0b0256147ea74d246426" -Quiet)) { throw "Physical state mismatch" }
Write-Output "PASS: Windows Rust HEB-C exact 244-state physical replay and receipts"

Set-Location (Join-Path $root "research\yang_mills_stage2")
Checked cargo @("+1.81.0", "fmt", "--all", "--", "--check")
Checked cargo @("+1.81.0", "test", "--all-targets", "--locked", "--offline")
Checked cargo @("+1.81.0", "clippy", "--all-targets", "--locked", "--offline", "--", "-D", "warnings")
Checked cargo @("+1.81.0", "build", "--release", "--locked", "--offline")
$ym = Join-Path (Get-Location).Path "target\release\hebc.exe"
Checked $ym @("sector","single")
Checked $ym @("sector","double")
Checked $ym @("sector","torus2")
Checked $ym @("spectrum","1","1")
Checked $ym @("run","3","512","0",(Join-Path $out "ym-stress-512"))
Checked $ym @("verify",(Join-Path $out "ym-stress-512"))
Write-Output "PASS: Windows native Rust S3 spectral and million-Q-trit encoded stress tests"
