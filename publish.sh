#!/bin/sh

set -e

if [ x"$1" = "x4real" ]; then
  #/bin/sh tag.sh
  cargo publish
else
  cargo publish --dry-run
  cargo package --list
fi

# vim: set ft=sh et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
