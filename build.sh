#!/usr/bin/env bash
set -euo pipefail

TARGETS=(
  x86_64-unknown-linux-gnu
  aarch64-unknown-linux-gnu
  x86_64-pc-windows-msvc
  aarch64-pc-windows-msvc
  x86_64-apple-darwin
  aarch64-apple-darwin
)

DIST_DIR="dist"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Ensure all targets are installed
for target in "${TARGETS[@]}"; do
  rustup target add "$target" 2>/dev/null || true
done

for target in "${TARGETS[@]}"; do
  echo "========================================"
  echo "Building: $target"
  echo "========================================"

  export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

  if ! cargo build --release --target "$target"; then
    echo "WARN: failed to build $target (missing toolchain or linker?), skipping."
    continue
  fi

  # Determine binary name and archive format
  case "$target" in
    *windows*)
      bin="jpg2ascii.exe"
      artifact="jpg2ascii-${target}.zip"
      staging=$(mktemp -d)
      cp "target/${target}/release/${bin}" "$staging/"
      (cd "$staging" && 7z a -tzip "$OLDPWD/$DIST_DIR/$artifact" "$bin" > /dev/null 2>&1) \
        || (cd "$staging" && zip "$OLDPWD/$DIST_DIR/$artifact" "$bin")
      rm -rf "$staging"
      ;;
    *)
      bin="jpg2ascii"
      artifact="jpg2ascii-${target}.tar.gz"
      tar czf "$DIST_DIR/$artifact" -C "target/${target}/release" "$bin"
      ;;
  esac

  echo "-> $DIST_DIR/$artifact"
done

echo ""
echo "Done. Artifacts:"
ls -lh "$DIST_DIR"/
