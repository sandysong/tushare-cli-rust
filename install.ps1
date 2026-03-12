# Tushare CLI Rust Skill 安装脚本 (Windows PowerShell)

$ErrorActionPreference = "Stop"

# 版本号
$VERSION = if ($env:VERSION) { $env:VERSION } else { "v1.0.2" }
$REPO_URL = "https://github.com/sandysong/tushare-cli-rust"
$RELEASE_URL = "$REPO_URL/releases/download/$VERSION"

# Claude Code skills 目录
$SKILLS_DIR = "$env:USERPROFILE\.claude\skills"
$SKILL_NAME = "tushare-cli"

function Write-Color {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

function Write-Success {
    Write-Color "✅ $args" -Color Green
}

function Write-Info {
    Write-Color "ℹ $args" -Color Cyan
}

function Write-Warning {
    Write-Color "⚠️ $args" -Color Yellow
}

function Write-Error {
    Write-Color "❌ $args" -Color Red
}

function Write-Step {
    Write-Color ""
    Write-Color "▶ $args" -Color Cyan
}

function Detect-Platform {
    Write-Step "检测系统信息..."

    $OS = [System.Environment]::OSVersion.Platform
    $Arch = [System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")

    if ($OS -eq "Win32NT") {
        $platform = "win32"
    } else {
        Write-Error "不支持的操作系统: $OS"
        exit 1
    }

    switch ($Arch) {
        "AMD64" { $arch = "x64" }
        "ARM64" { $arch = "arm64" }
        default {
            Write-Error "不支持的架构: $Arch"
            exit 1
        }
    }

    $global:PLATFORM_STR = "$platform-$arch"
    Write-Success "检测到平台: $PLATFORM_STR"
}

function Check-Dependencies {
    Write-Step "检查依赖..."

    # 检查是否有 PowerShell 5+
    $PSVersion = $PSVersionTable.PSVersion
    if ($PSVersion.Major -lt 5) {
        Write-Error "需要 PowerShell 5.0 或更高版本"
        exit 1
    }

    Write-Success "依赖检查通过"
}

function Download-File {
    param(
        [string]$Url,
        [string]$Output
    )

    Invoke-WebRequest -Uri $Url -OutFile $Output -UseBasicParsing
}

function Install-Skill {
    Write-Step "下载 Tushare CLI Rust Skill $VERSION..."

    # 创建临时目录
    $TEMP_DIR = Join-Path $env:TEMP "tushare-cli-rust-install"
    if (Test-Path $TEMP_DIR) {
        Remove-Item -Recurse -Force $TEMP_DIR
    }
    New-Item -ItemType Directory -Path $TEMP_DIR | Out-Null

    # 确定 skill 文件名
    $SKILL_FILE = "tushare-cli-rust-$PLATFORM_STR.skill"

    $DOWNLOAD_URL = "$RELEASE_URL/$SKILL_FILE"
    Write-Info "下载地址: $DOWNLOAD_URL"

    # 下载 skill 文件
    $SKILL_PATH = Join-Path $TEMP_DIR $SKILL_FILE
    try {
        Download-File -Url $DOWNLOAD_URL -Output $SKILL_PATH
        Write-Success "下载完成"
    } catch {
        Write-Error "下载失败"
        Write-Info "请检查版本号是否正确，或访问 $REPO_URL/releases 查看可用版本"
        exit 1
    }

    # 解压安装
    Write-Step "安装到 Claude Code Skills 目录..."

    # 创建 skills 目录（如果不存在）
    if (!(Test-Path $SKILLS_DIR)) {
        New-Item -ItemType Directory -Path $SKILLS_DIR | Out-Null
    }

    # 删除旧版本（如果存在）
    $OLD_SKILL = Join-Path $SKILLS_DIR $SKILL_NAME
    if (Test-Path $OLD_SKILL) {
        Write-Warning "发现旧版本，正在删除..."
        Remove-Item -Recurse -Force $OLD_SKILL
    }

    # 解压 skill 文件
    $DEST_PATH = Join-Path $SKILLS_DIR $SKILL_NAME
    Expand-Archive -Path $SKILL_PATH -DestinationPath $DEST_PATH -Force

    Write-Success "安装完成"

    # 清理临时文件
    Remove-Item -Recurse -Force $TEMP_DIR
}

function Verify-Installation {
    Write-Step "验证安装..."

    $EXECUTABLE = Join-Path $SKILLS_DIR "$SKILL_NAME\scripts\tushare.exe"

    if (!(Test-Path $EXECUTABLE)) {
        Write-Error "安装验证失败：可执行文件不存在"
        exit 1
    }

    try {
        $VERSION_INFO = & $EXECUTABLE --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Success "版本: $VERSION_INFO"
        } else {
            Write-Warning "无法获取版本信息，但文件已存在"
        }
    } catch {
        Write-Warning "无法执行版本检查，但文件已存在"
    }
}

function Show-ConfigHelp {
    Write-Color ""
    Write-Color "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -Color Green
    Write-Color "          安装成功！🎉" -Color Green
    Write-Color "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -Color Green
    Write-Color ""
    Write-Color "安装位置: " -NoNewline
    Write-Color "$DEST_PATH" -Color Cyan
    Write-Color "可执行文件: " -NoNewline
    Write-Color "$EXECUTABLE" -Color Cyan
    Write-Color ""
    Write-Color "下一步：配置 Tushare Token" -Color Yellow
    Write-Color ""
    Write-Color "1. 获取 Token:"
    Write-Color "   访问 https://tushare.pro 注册账号并获取 Token"
    Write-Color ""
    Write-Color "2. 配置环境变量:"
    Write-Color "   " -NoNewline
    Write-Color '`$env:TUSHARE_TOKEN = "your_token_here"`' -Color Cyan
    Write-Color ""
    Write-Color "   永久配置（系统环境变量）:"
    Write-Color "   1. 打开系统属性 -> 高级系统设置 -> 环境变量"
    Write-Color "   2. 新建用户变量 TUSHARE_TOKEN"
    Write-Color "   3. 值为你的 Token"
    Write-Color ""
    Write-Color "3. 测试安装:"
    Write-Color "   " -NoNewline
    Write-Color "& '$EXECUTABLE' --version" -Color Cyan
    Write-Color ""
    Write-Color "4. 在 Claude Code 中使用:"
    Write-Color "   直接对 Claude 说：\"帮我查询平安银行的基本信息\""
    Write-Color ""
    Write-Color "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -Color Green
}

function Uninstall-Skill {
    Write-Step "卸载 Tushare CLI Rust Skill..."

    $OLD_SKILL = Join-Path $SKILLS_DIR $SKILL_NAME
    if (Test-Path $OLD_SKILL) {
        Remove-Item -Recurse -Force $OLD_SKILL
        Write-Success "卸载完成"
    } else {
        Write-Warning "未找到已安装的 Tushare CLI Rust Skill"
    }
}

function Update-Skill {
    Write-Info "更新到最新版本..."

    # 获取最新版本号
    try {
        $LATEST_VERSION = (Invoke-WebRequest -Uri "$REPO_URL/releases/latest" -UseBasicParsing | Select-String -Pattern 'tag/[^"]*' | ForEach-Object { $_ -replace 'tag/', '' } | Select-Object -First 1)
    } catch {
        Write-Error "无法获取最新版本信息"
        exit 1
    }

    if ([string]::IsNullOrEmpty($LATEST_VERSION)) {
        Write-Error "无法获取最新版本信息"
        exit 1
    }

    Write-Info "最新版本: $LATEST_VERSION"

    # 设置版本号并安装
    $global:VERSION = $LATEST_VERSION
    Install-Skill
}

function Show-Help {
    Write-Color @"
Tushare CLI Rust Skill 安装脚本 (Windows)

用法:
  .\install.ps1 [命令]

命令:
  install     安装 Tushare CLI Rust Skill (默认)
  uninstall   卸载 Tushare CLI Rust Skill
  update      更新到最新版本
  help        显示此帮助信息

环境变量:
  VERSION     指定安装版本 (默认: v1.0.0)

示例:
  # 安装默认版本
  .\install.ps1

  # 安装指定版本
  $env:VERSION="v1.1.0"; .\install.ps1

  # 更新到最新版本
  .\install.ps1 update

  # 卸载
  .\install.ps1 uninstall

一键安装:
  iex (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/sandysong/tushare-cli-rust/main/install.ps1" -UseBasicParsing).Content

更多信息:
  https://github.com/sandysong/tushare-cli-rust
"@ -Color White
}

# 主函数
function Main {
    Write-Color @"
╔═══════════════════════════════════════════════════════════╗
║   Tushare CLI Rust for Claude Code - 安装向导             ║
╚═══════════════════════════════════════════════════════════╝
"@ -Color Cyan

    $COMMAND = if ($args.Count -eq 0) { "install" } else { $args[0] }

    switch ($COMMAND) {
        "install" {
            Check-Dependencies
            Detect-Platform
            Install-Skill
            Verify-Installation
            Show-ConfigHelp
        }
        "uninstall" {
            Uninstall-Skill
        }
        "update" {
            Check-Dependencies
            Detect-Platform
            Update-Skill
            Verify-Installation
            Show-ConfigHelp
        }
        "help" {
            Show-Help
        }
        default {
            Write-Error "未知命令: $COMMAND"
            Write-Color ""
            Show-Help
            exit 1
        }
    }
}

# 运行主函数
Main
