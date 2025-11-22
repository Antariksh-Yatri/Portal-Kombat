
set -euo pipefail
IFS=$'\n\t'

owner='antariksh-yatri'
repo='portal-kombat'
required_tools=(gh jq curl sudo)


err() { printf '%s\n' "$*" >&2; exit 1; }
cleanup() {
  if [[ -n "${TMPDIR:-}" && -d "$TMPDIR" ]]; then
    rm -rf "$TMPDIR"
  fi
}

on_interrupt() {
  echo ""
  echo "Interrupted. Cleaning up and exiting."
  cleanup
  exit 130  
}
which_or_err() {
  for t in "${required_tools[@]}"; do
    command -v "$t" >/dev/null 2>&1 || err "required tool '$t' not found. install it and retry."
  done
}
trap on_interrupt INT
trap cleanup EXIT

get_platform() {
  case "$(uname -s)" in
    Linux) echo "linux" ;;
    Darwin) echo "darwin" ;;
    *) echo "unknown" ;;
  esac
}

get_arch() {
  case "$(uname -m)" in
    x86_64) echo "x86_64" ;;
    aarch64|arm64) echo "aarch64" ;;
    *) echo "unknown" ;;
  esac
}


choose_bin_dir() {
  if [[ "$1" == "darwin" ]]; then
    echo "/usr/local/bin"; 
  else
    if [[ -d "/usr/local/bin" ]]; then echo "/usr/local/bin"; else echo "/usr/bin"; fi
  fi
}

cleanup() { if [[ -n "${TMPDIR:-}" && -d "$TMPDIR" ]]; then rm -rf "$TMPDIR"; fi }
trap cleanup EXIT


which_or_err

platform="$(get_platform)"
arch="$(get_arch)"
[[ "$platform" != "unknown" ]] || err "unsupported platform"
[[ "$arch" != "unknown" ]] || err "unsupported arch"

BIN_DIR="$(choose_bin_dir "$platform")"


TMPDIR="$(mktemp -d /tmp/portalkombat.XXXX)"
cd "$TMPDIR"


latest_tag="$(gh release list --repo "$owner/$repo" --limit 200 --json tagName,publishedAt \
  | jq -r '.[].tagName' \
  | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+-daemon$' \
  | sort -V \
  | tail -n1 || true)"

[[ -n "$latest_tag" ]] || err "no matching release tag found (expected tags matching vX.Y.Z-daemon)."

echo "selected release: $latest_tag"


asset_name="$(gh release view "$latest_tag" --repo "$owner/$repo" --json assets \
  | jq -r '.assets[].name' \
  | grep -E "portalkombatd-${arch}-.*-${platform}.*" \
  | head -n1 || true)"

[[ -n "$asset_name" ]] || err "could not find release asset for arch/platform: ${arch}/${platform}."

echo "asset: $asset_name"


gh release download --repo "$owner/$repo" --pattern "$asset_name" --clobber --archive=zip >/dev/null 2>&1 || {
  gh release download --repo "$owner/$repo" "$latest_tag" --pattern "$asset_name" --clobber || err "failed to download asset"
}


if [[ ! -f "$asset_name" ]]; then err "download failed, $asset_name not found"; fi


case "$asset_name" in
  *.zip) unzip -o "$asset_name" ;; 
  *.tar.gz|*.tgz) tar -xzf "$asset_name" ;;
  *) : ;; 
esac


binary_file="$(ls -1 | grep -E "^portalkombatd($|.*$)" | head -n1 || true)"
[[ -n "$binary_file" ]] || err "binary not found after download/extract"

chmod +x "$binary_file"
sudo mv -f "$binary_file" "$BIN_DIR/portalkombatd"
sudo chown root:wheel "$BIN_DIR/portalkombatd"
sudo chmod 0755 "$BIN_DIR/portalkombatd"
echo "installed binary -> $BIN_DIR/portalkombatd"

CONFIG_PATH="$HOME/.portalkombatd.toml"
DEFAULT_REFRESH=5
DEFAULT_TIMEOUTS=5

if [[ -f "$CONFIG_PATH" ]]; then
  printf '%s\n' "found existing config at $CONFIG_PATH"
  read -r -p "overwrite existing config? [y/N]: " _ans < /dev/tty
  _ans_lc="$(printf '%s' "$_ans" | tr '[:upper:]' '[:lower:]')"

  case "$_ans_lc" in
    y|yes) 
      ;; 
    *) 
      printf '%s\n' "keeping existing config; skipping write."
      WRITE_CONFIG_SKIP=1 
      ;;
  esac
else
  printf '%s\n' "No previous config found..."
fi

if [[ -z "${WRITE_CONFIG_SKIP:-}" ]]; then
read -r -p "Enter roll number: " ROLLNO < /dev/tty
read -r -p "Enter captive-portal password: " PASSWORD < /dev/tty
echo

if [[ -z "$ROLLNO" || -z "$PASSWORD" ]]; then
echo "roll number and password must not be empty." >&2
exit 1
fi
cat > "$CONFIG_PATH" <<EOF
refresh = ${DEFAULT_REFRESH}
timeouts = ${DEFAULT_TIMEOUTS}

[profile]
rollno = "${ROLLNO}"
password = "${PASSWORD}"
EOF

echo "wrote config -> $CONFIG_PATH"
fi


if [[ "$platform" == "linux" ]]; then
  svc_url="https://raw.githubusercontent.com/$owner/$repo/main/daemon/resources/portalkombatd.service"
  svc_dest="/etc/systemd/system/portalkombatd.service"
  if curl -fL "$svc_url" -o portalkombatd.service; then
    sudo mv -f portalkombatd.service "$svc_dest"
    sudo chown root:wheel "$svc_dest"
    sudo chmod 0644 "$svc_dest"
    echo "installed systemd unit -> $svc_dest"
    if command -v systemctl >/dev/null 2>&1; then
      sudo systemctl daemon-reload
      sudo systemctl enable --now portalkombatd.service || {
        echo "warning: failed to enable/start service. check journalctl -u portalkombatd.service"
      }
    else
      echo "systemctl not present. unit installed but not enabled."
    fi
  else
    echo "no systemd unit found at $svc_url; binary installed only."
  fi
elif [[ "$platform" == "darwin" ]]; then
  plist_url="https://raw.githubusercontent.com/$owner/$repo/main/daemon/resources/com.arjunmnath.portalkombatd.plist"
  plist_dest="/Library/LaunchDaemons/com.arjunmnath.portalkombatd.plist"
  if curl -fL "$plist_url" -o com.arjunmnath.portalkombatd.plist; then
    sudo mv -f com.arjunmnath.portalkombatd.plist "$plist_dest"
    sudo chown root:wheel "$plist_dest"
    sudo chmod 0644 "$plist_dest"
    echo "installed plist -> $plist_dest"
    
    if sudo launchctl print system/com.arjunmnath.portalkombatd >/dev/null 2>&1; then
      echo "service appears registered. try to bootstrap/reload manually if needed."
    else
      sudo launchctl bootstrap system "$plist_dest" || echo "launchctl bootstrap failed; try: sudo launchctl load $plist_dest"
    fi
  else
    echo "no plist found at $plist_url; binary installed only."
  fi
fi

echo "done."
