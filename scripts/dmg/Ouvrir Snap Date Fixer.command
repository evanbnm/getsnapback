#!/bin/bash
# This script removes the macOS quarantine attribute from Snap Date Fixer.app
# so it can be opened without the "is damaged" Gatekeeper error.
# Required only on the very first launch — afterwards the app opens normally.

set -e

APP_NAME="Snap Date Fixer"
APP_PATH="/Applications/${APP_NAME}.app"

if [ ! -d "$APP_PATH" ]; then
  osascript <<EOF
display dialog "Avant de continuer, glissez l'icône « ${APP_NAME} » dans le dossier « Applications » à côté, puis double-cliquez à nouveau sur ce fichier.

Before continuing, drag the « ${APP_NAME} » icon into the « Applications » folder next to it, then double-click this file again." with icon stop buttons {"OK"} default button "OK" with title "${APP_NAME}"
EOF
  exit 1
fi

xattr -cr "$APP_PATH" 2>/dev/null || true

open "$APP_PATH"

osascript -e "tell application \"Terminal\" to close (every window whose name contains \"Ouvrir ${APP_NAME}\")" 2>/dev/null || true
exit 0
