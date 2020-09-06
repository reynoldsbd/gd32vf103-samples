#!/usr/bin/pwsh

<#
.SYNOPSIS
Downloads and extracts Nuclei's RISC-V toolchain
#>

[CmdletBinding()]

param (
    # If set, toolchain directory is returned on the pipeline
    [switch]
    $PassThru
)

$ErrorActionPreference = "Stop"

if (!$IsLinux) {
    throw "Only Linux is supported at this time"
}

$repoRoot = (Resolve-Path $PSScriptRoot/..).Path
Write-Debug "repoRoot: $repoRoot"

$toolchainDir = "$repoRoot/tools/nuclei-toolchain"
Write-Debug "toolchainDir: $toolchainDir"

$toolchainUrl = "https://nucleisys.com/upload/files/toochain/gcc/nuclei_riscv_newlibc_prebuilt_linux64_2020.08.tar.bz2"
Write-Debug "toolchainUrl: $toolchainUrl"

$toolchainHash = "398C25B9385B8122D2E864BF71E47B1D871F6C326C21D0AE6D3AFD2858F36041"
Write-Debug "toolchainHash: $toolchainHash"

$downloadDest = "/tmp/nuclei-gcc.tar.bz2"
Write-Debug "downloadDest: $downloadDest"

if (Test-Path $toolchainDir/hash.txt) {

    $hash = Get-Content $toolchainDir/hash.txt

    if ($hash -eq $toolchainHash) {

        Write-Verbose "Toolchain already available, skipping download and extraction"

	if ($PassThru) {
	    $toolchainDir
	}

        return
    }
}

$downloadNeeded = $true

if (Test-Path $downloadDest) {

    $hash = (Get-FileHash $downloadDest).Hash

    if ($hash -ne $toolchainHash) {
        Write-Verbose "Removing pre-existing download with unrecognized hash $actualHash"
	Remove-Item $downloadDest
    } else {
        Write-Verbose "Found pre-existing download with matching hash, skipping download"
	$downloadNeeded = $false
    }
}

if ($downloadNeeded) {

    Write-Host "Downloading Nuclei toolchain"

    $origProgPref = $global:ProgressPreference
    $global:ProgressPreference = "SilentlyContinue"

    try {
        Invoke-WebRequest $toolchainUrl -OutFile $downloadDest
    } finally {
        $global:ProgressPreference = $origProgPref
    }

    $hash = (Get-FileHash $downloadDest).Hash

    if ($hash -ne $toolchainHash) {
        throw "Downloaded toolchain has unexpected hash $hash"
    }
}

if (Test-Path $toolchainDir) {

    Write-Verbose "Cleaning previous toolchain"
    Remove-Item -Recurse $toolchainDir/*

} else {

    Write-Verbose "Automatically creating toolchain directory"
    $null = New-Item -ItemType Directory $toolchainDir
}

Write-Host "Extracing Nuclei toolchain"
tar -xf $downloadDest -C $toolchainDir
if ($LASTEXITCODE -ne 0) {
    throw "tar returned exit code $LASTEXITCODE"
}

$toolchainHash > $toolchainDir/hash.txt

if ($PassThru) {
    $toolchainDir
}
