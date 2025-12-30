@echo off
REM Wrapper to run Makefile targets via WSL
REM Usage: make.cmd [target]
REM Example: make.cmd build

setlocal EnableDelayedExpansion

REM Get the directory this script is in
set "SCRIPT_DIR=%~dp0"

REM Remove trailing backslash
if "%SCRIPT_DIR:~-1%"=="\" set "SCRIPT_DIR=%SCRIPT_DIR:~0,-1%"

REM Extract drive letter (lowercase it for WSL)
set "DRIVE=%SCRIPT_DIR:~0,1%"
for %%a in (a b c d e f g h i j k l m n o p q r s t u v w x y z) do (
    if /i "%DRIVE%"=="%%a" set "DRIVE=%%a"
)

REM Get path after drive letter, convert backslashes to forward slashes
set "REST=%SCRIPT_DIR:~2%"
set "REST=%REST:\=/%"

REM Build WSL path
set "WSL_PATH=/mnt/%DRIVE%%REST%"

REM Default to 'help' if no target specified
if "%~1"=="" (
    set "TARGET=help"
) else (
    set "TARGET=%*"
)

REM Run make in WSL (source cargo env first)
wsl -e bash -c "source $HOME/.cargo/env 2>/dev/null; cd '%WSL_PATH%' && make %TARGET%"
