#!/bin/bash
set -e

if [ ! -d "third_party/librdkafka" ]; then
  git clone https://github.com/confluentinc/librdkafka.git build/librdkafka
fi


cd build/librdkafka

./configure --disable-shared --enable-static
make -j$(nproc)
