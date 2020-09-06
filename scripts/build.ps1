#!/usr/bin/pwsh

<#
.SYNOPSIS
Builds one of the examples provided by this project.
#>

[CmdletBinding()]

param (
    # Name of example program to build
    [Parameter(Mandatory=$true)]
    [string]
    $Name
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path $PSScriptRoot/..).Path
Write-Debug "repoRoot: $repoRoot"

$rustOutDir = "$repoRoot/target/riscv32imac-unknown-none-elf/release"
Write-Debug "rustOutDir: $rustOutDir"

$toolchainDir = & $PSScriptRoot/nuclei-toolchain.ps1 -PassThru

Write-Host "Building Rust code for $Name"
cargo build                              `
    --manifest-path $repoRoot/Cargo.toml `
    --release                            `
    --bin $Name
if ($LASTEXITCODE -ne 0) {
    throw "cargo returned exit code $LASTEXITCODE"
}

Write-Host "Preparing binary $repoRoot/$Name.bin"
& $toolchainDir/gcc/bin/riscv-nuclei-elf-objcopy $rustOutDir/$Name -O binary -S "$repoRoot/$Name.bin"
if ($LASTEXITCODE -ne 0) {
    throw "objcopy returned exit code $LASTEXITCODE"
}

