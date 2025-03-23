#!/bin/bash

commit=$(git log -1 --pretty=%B)

version=$(cat VERSION)
# Trim whitespace if theres any
version=$(echo "$version" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')  

echo "current version: $version"

IFS='.' read -r major minor patch <<< "$version"

if [[ "$commit" == *"BREAKING CHANGE"* ]] || \
   [[ "$commit" =~ feat(\([a-zA-Z0-9]+\))?!: ]] || \
   [[ "$commit" =~ fix(\([a-zA-Z0-9]+\))?!: ]]; then
    ((major++))
    minor=0
    patch=0
elif [[ "$commit" =~ feat(\([a-zA-Z0-9]+\))?: ]]; then
    ((minor++))
    patch=0
elif [[ "$commit" =~ fix(\([a-zA-Z0-9]+\))?: ]]; then
    ((patch++))
fi

new_version="${major}.${minor}.${patch}"

if [ "$new_version" != "$version" ]; then
    echo "bump version: $version to $new_version"
    echo -n "$new_version" > VERSION
    sed -i "/\[package\]/,/^$/ { /name = \"juro\"/ { n; s/version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$new_version\"/ } }" Cargo.toml
    sed -i "/\[package\]/,/^$/ { /name = \"juro\"/ { n; s/version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$new_version\"/ } }" Cargo.lock
else
    echo "new version not needed"
fi