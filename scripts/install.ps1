function Get-LatestVersion {
    $releases = Invoke-RestMethod -Uri "https://api.github.com/repos/fgbm/apc/releases/latest"
    return $releases.tag_name
}

function Install-APC {
    $version = Get-LatestVersion
    $versionNum = $version.Substring(1)  # Удаляем 'v' из начала строки

    Write-Host "Installing APC $version for Windows..."

    $installDir = "$env:LOCALAPPDATA\Programs\apc"
    if (!(Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    }

    $downloadUrl = "https://github.com/fgbm/apc/releases/download/$version/apc-windows-amd64.exe"
    $outputPath = "$installDir\apc.exe"

    Write-Host "Downloading from $downloadUrl..."
    Invoke-WebRequest -Uri $downloadUrl -OutFile $outputPath

    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath -notlike "*$installDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$currentPath;$installDir", "User")
        Write-Host "Added $installDir to PATH"
    }

    Write-Host "APC $version installed successfully to $outputPath"
    Write-Host "You may need to restart your terminal for changes to take effect."
}

Install-APC
