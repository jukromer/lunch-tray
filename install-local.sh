#!/bin/sh
# Install Lunch Tray for the current user: binary, desktop file and icons.
set -eu

APP_ID=de.jukromer.LunchTray
PREFIX="${PREFIX:-$HOME/.local}"

cargo build --release

install -Dm755 target/release/lunch-tray "$PREFIX/bin/lunch-tray"
install -Dm644 "data/icons/hicolor/scalable/apps/$APP_ID.svg" \
    "$PREFIX/share/icons/hicolor/scalable/apps/$APP_ID.svg"
install -Dm644 "data/icons/hicolor/symbolic/apps/$APP_ID-symbolic.svg" \
    "$PREFIX/share/icons/hicolor/symbolic/apps/$APP_ID-symbolic.svg"

mkdir -p "$PREFIX/share/applications"
sed "s|^Exec=.*|Exec=$PREFIX/bin/lunch-tray|" "data/$APP_ID.desktop" \
    > "$PREFIX/share/applications/$APP_ID.desktop"

echo "Lunch Tray is installed in $PREFIX and shows up in the GNOME overview."
