#!/bin/bash

cd "${0%/*}"

docker build -t detailing-builder .
docker run -v $(pwd):/app detailing-builder
