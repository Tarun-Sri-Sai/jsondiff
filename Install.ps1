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
    $machinePath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
    if (-not $machinePath.Split(";") -contains $destDirectory) {
        [System.Environment]::SetEnvironmentVariable("Path", "$machinePath;$destDirectory", [System.EnvironmentVariableTarget]::Machine)
        Write-Host "Added $destDirectory to Machine PATH."
    } else {
        Write-Host "$destDirectory is already in Machine PATH."
    }

    $userPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::User)
    if (-not $userPath.Split(";") -contains $destDirectory) {
        [System.Environment]::SetEnvironmentVariable("Path", "$userPath;$destDirectory", [System.EnvironmentVariableTarget]::User)
        Write-Host "Added $destDirectory to User PATH."
    } else {
        Write-Host "$destDirectory is already in User PATH."
    }
} else {
    Write-Host "Directory $destDirectory does not exist."
}

Write-Host 'Cleaning up.'
Set-Location ..
Remove-Item -Recurse -Force 'jsondiff'
