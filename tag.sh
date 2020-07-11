#!/bin/sh

set -e

PRJ=`grep '^name' Cargo.toml | sed -E 's/^name[[:space:]]*=[[:space:]]*"([^"]+)"/\1/'`
VERSION=`grep '^version' Cargo.toml | sed -E 's/^version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/'`

FSL_CHECKOUT=`fossil status | grep '^checkout:' | awk '{print $2}'`

TAGNAME=$PRJ-$VERSION

echo "Project: $PRJ"
echo "    Tag: $TAGNAME -> $FSL_CHECKOUT"

# fossil is for development
fossil tag add $TAGNAME $FSL_CHECKOUT
fossil sync && fossil update

# git is for publishing
git tag $TAGNAME
git push origin $TAGNAME

# vim: set ft=sh et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
