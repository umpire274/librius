# ============================================================
#  Librius - Submodule Check Script (PowerShell)
# ============================================================
# Verifica lo stato del submodule tools_private
# e controlla se è sincronizzato con l'ultimo commit remoto.
# ============================================================

$submodulePath = "tools_private"
$remoteUrl = "git@github.com:umpire274/rust_dev_scripts.git"
$branch = "main"

Write-Host "🔍 Checking submodule status for '$submodulePath'..." -ForegroundColor Cyan

if (-not (Test-Path "$submodulePath/.git"))
{
    Write-Host "❌ Submodule not initialized. Run:" -ForegroundColor Red
    Write-Host "   git submodule update --init --recursive"
    exit 1
}

Set-Location $submodulePath

# Get local HEAD commit
$localCommit = git rev-parse HEAD
# Fetch latest remote
git fetch origin $branch --quiet
$remoteCommit = git rev-parse origin/$branch

Set-Location ..

if ($localCommit -eq $remoteCommit)
{
    Write-Host "✅ Submodule '$submodulePath' is up to date with origin/$branch." -ForegroundColor Green
    exit 0
}
else
{
    Write-Host "⚠️  Submodule '$submodulePath' is out of sync!" -ForegroundColor Yellow
    Write-Host "   Local : $localCommit"
    Write-Host "   Remote: $remoteCommit"
    Write-Host ""
    Write-Host "👉 To update, run:" -ForegroundColor Cyan
    Write-Host "   cd $submodulePath; git pull origin $branch; cd .."
    exit 2
}
