#!/bin/sh

echo "This script builds .deb package of Servus inside docker image rust_builder"

echo "WARNING this builds code from git repository, not this folder ! ! !"

rm servus.deb

docker rm -f servus_builder

docker run -di --name servus_builder --rm rust_builder bash

docker exec servus_builder sh -c "git clone https://github.com/vctibor/servus"

docker exec servus_builder sh -c "cd servus && cargo deb --output servus.deb"

docker cp servus_builder:/servus/servus.deb .

docker rm -f servus_builder

