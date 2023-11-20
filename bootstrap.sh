#!/bin/bash

cd "${0%/*}"

./src-tauri/sidecars/bootstrap.sh
rustup update
npm i
npm run tauri build








