#!/usr/bin/env bash
# Auto-detect the Android NDK and invoke the correct clang for the target.
#
# Env vars (all optional):
#   ANDROID_NDK_HOME  – explicit NDK root (e.g. .../ndk/28.0.13004108)
#   ANDROID_HOME      – SDK root; the latest NDK under $ANDROID_HOME/ndk/ is used
#   ANDROID_API_LEVEL – minimum API level (default: 34)
#
# The script reads TARGET (set by cargo) to pick the right clang prefix.
set -euo pipefail

API_LEVEL="${ANDROID_API_LEVEL:-34}"

# --- Resolve NDK root ---
find_ndk_root() {
    if [ -n "${ANDROID_NDK_HOME:-}" ] && [ -d "$ANDROID_NDK_HOME" ]; then
        echo "$ANDROID_NDK_HOME"
        return
    fi
    local sdk_dir="${ANDROID_HOME:-}"
    # Common SDK locations
    for candidate in \
        "$sdk_dir" \
        "$HOME/Library/Android/sdk" \
        "$HOME/Android/Sdk" \
        "/usr/local/lib/android/sdk"; do
        if [ -n "$candidate" ] && [ -d "$candidate/ndk" ]; then
            # Pick the latest installed NDK (lexicographic sort, highest last)
            local latest
            latest=$(ls -1d "$candidate/ndk"/*/ 2>/dev/null | sort -V | tail -1)
            if [ -n "$latest" ]; then
                echo "${latest%/}"
                return
            fi
        fi
    done
    echo "ERROR: Could not find Android NDK." >&2
    echo "Set ANDROID_NDK_HOME or ANDROID_HOME, or install the NDK via Android Studio." >&2
    exit 1
}

NDK_ROOT=$(find_ndk_root)

# --- Resolve host prebuilt directory ---
HOST_OS=$(uname -s)
HOST_ARCH=$(uname -m)
case "${HOST_OS}-${HOST_ARCH}" in
    Darwin-*)   HOST_TAG="darwin-x86_64" ;;
    Linux-*)    HOST_TAG="linux-x86_64"  ;;
    *)          echo "ERROR: Unsupported host ${HOST_OS}-${HOST_ARCH}" >&2; exit 1 ;;
esac

TOOLCHAIN_BIN="$NDK_ROOT/toolchains/llvm/prebuilt/$HOST_TAG/bin"
if [ ! -d "$TOOLCHAIN_BIN" ]; then
    echo "ERROR: NDK toolchain not found at $TOOLCHAIN_BIN" >&2
    exit 1
fi

# --- Map cargo TARGET to NDK clang triple ---
case "${TARGET:-}" in
    aarch64-linux-android*)  TRIPLE="aarch64-linux-android" ;;
    x86_64-linux-android*)   TRIPLE="x86_64-linux-android"  ;;
    armv7-linux-androideabi*) TRIPLE="armv7a-linux-androideabi" ;;
    i686-linux-android*)     TRIPLE="i686-linux-android"    ;;
    *)
        echo "ERROR: Unsupported Android target '${TARGET:-<unset>}'" >&2
        exit 1
        ;;
esac

exec "$TOOLCHAIN_BIN/${TRIPLE}${API_LEVEL}-clang" "$@"
