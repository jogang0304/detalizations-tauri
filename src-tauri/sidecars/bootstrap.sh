#!/bin/bash
cd "${0%/*}"

target=$(rustc -Vv | grep host | cut -f2 -d' ')
echo "${target}"
pyinstaller price.py --distpath ./executables -y --onefile --name getNameAndPrice-"${target}"
