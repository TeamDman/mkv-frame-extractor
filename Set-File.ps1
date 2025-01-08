Write-Host "Reading file from clipboard"
Get-Clipboard | Set-Content -Path file.txt
