<#
.SYNOPSIS
Create a .mumble_plugin package for easy installs
#>
[CmdletBinding()]
param(
    # DLL path, relative to the root of this repo.
    $Dll = "./target/debug/mumble_mute_plugin.dll",

    # Output location, including file name and extension
    $Output = "./output/universal_mute.mumble_plugin"
)

$paths = @(
    "./packaging/manifest.xml",
    $Dll
)
$repoRoot = Join-Path $PSScriptRoot "../"
$fullPaths = $paths | %{ Join-Path $repoRoot $_ }

$outputPath =  (Join-Path $repoRoot $Output)
$outputDir = Split-Path $outputPath
if (!(Test-Path $outputDir)) {
    mkdir $outputDir | Out-Null
}

# https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.archive/compress-archive?view=powershell-7.3
$compress = @{
    Path = @($fullPaths)
    CompressionLevel = "Fastest"
    DestinationPath = $outputPath
}
Compress-Archive @compress