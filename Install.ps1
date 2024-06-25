Write-Host 'Cloning repo.'
Set-Location $env:USERPROFILE -ErrorAction Ignore
git clone https://github.com/Tarun-Sri-Sai/jsondiff.git

Write-Host 'Building release binary.'
Set-Location 'jsondiff'
git checkout release
cargo build --release

$destDirectory = 'C:\Program Files\jsondiff'
Write-Host "Moving binary to $destDirectory."
New-Item -ItemType Directory -ErrorAction Ignore "$destDirectory"
Move-Item -Force '.\target\release\jsondiff.exe' "$destDirectory"

Write-Host "Adding $destDirectory to PATH."
if (Test-Path -Path $destDirectory) {
    $currentPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
    
    if (-not $currentPath.Split(";") -contains $destDirectory) {
        [System.Environment]::SetEnvironmentVariable("Path", "$currentPath;$destDirectory", [System.EnvironmentVariableTarget]::Machine)
        Write-Host "Added $destDirectory to PATH."
    } else {
        Write-Host "$destDirectory is already in PATH."
    }
} else {
    Write-Host "Directory $destDirectory does not exist."
}

Write-Host 'Cleaning up.'
Set-Location ..
Remove-Item -Recurse -Force 'jsondiff'
