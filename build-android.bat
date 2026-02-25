@echo off
chcp 65001 >nul
setlocal EnableDelayedExpansion

title Fly Strike - Android 构建脚本

echo.
echo ================================================
echo        Fly Strike Android 构建脚本 (Windows)
echo ================================================
echo.

:: ==================== 配置部分 ====================
set "TARGET_ROOT=E:\Develop\Andriod\android_example\app\src\main"
set "TARGET_DIR=%TARGET_ROOT%\jniLibs"
set "ARCH=arm64-v8a"
set "BUILD_MODE=--release --lib"

:: 可选：如果你想同时构建多个架构，取消下面注释并修改
:: set "ARCHS=arm64-v8a armeabi-v7a"
:: for %%a in (%ARCHS%) do (
::     set "ARCH=%%a"
::     goto :build
:: )

:: ==================== 检查环境 ====================
echo [检查] Cargo 和 cargo-ndk 是否可用...
cargo --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [错误] cargo 命令不可用，请检查 Rust 安装和 PATH
    goto :error_end
)

cargo ndk --version >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo [错误] cargo-ndk 未安装或不在 PATH 中
    echo 请运行: cargo install cargo-ndk
    goto :error_end
)

:: ==================== 创建输出目录 ====================
if not exist "%TARGET_DIR%\%ARCH%" (
    echo [创建] 输出目录: %TARGET_DIR%\%ARCH%
    mkdir "%TARGET_DIR%\%ARCH%"
    if %ERRORLEVEL% NEQ 0 (
        echo [错误] 创建目录失败，请检查路径权限
        goto :error_end
    )
)

:: ==================== 执行构建 ====================
echo.
echo [构建] 开始构建 %ARCH% %BUILD_MODE% ...
echo 目标目录: %TARGET_DIR%\%ARCH%
echo.

cargo ndk -t %ARCH% -o "%TARGET_DIR%" build %BUILD_MODE%

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [失败] cargo ndk 构建出错！退出码: %ERRORLEVEL%
    echo 请检查：
    echo   1. ANDROID_NDK_ROOT 环境变量是否正确设置
    echo   2. rustup target add %ARCH%-linux-android 是否已执行
    echo   3. 项目 Cargo.toml 是否正确
    goto :error_end
)

:: ==================== 成功提示 ====================
echo.
echo [成功] 构建完成！
echo   .so 文件位置: %TARGET_DIR%\%ARCH%\libfly_strike.so
echo   （如果你的 crate 名不同，请替换 lib 名确认文件）
echo.

:: 可选：自动复制 libc++_shared.so（如果你需要）
:: echo [可选] 正在尝试复制 libc++_shared.so ...
:: set "NDK_PATH=%ANDROID_NDK_ROOT%"
:: if defined NDK_PATH (
::     set "CPP_SO=%NDK_PATH%\toolchains\llvm\prebuilt\windows-x86_64\sysroot\usr\lib\aarch64-linux-android\libc++_shared.so"
::     if exist "!CPP_SO!" (
::         copy "!CPP_SO!" "%TARGET_DIR%\%ARCH%\" >nul
::         echo 已复制 libc++_shared.so 到 %TARGET_DIR%\%ARCH%
::     ) else (
::         echo 找不到 libc++_shared.so，请手动复制
::     )
:: ) else (
::     echo ANDROID_NDK_ROOT 未设置，跳过复制 libc++_shared.so
:: )

xcopy /Y /I /E /Q ".\assets" "%TARGET_ROOT%\assets"
if %ERRORLEVEL% NEQ 0 (
    echo [警告] 资源复制失败，请检查路径
)

echo.
exit /b 0