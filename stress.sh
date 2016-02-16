#!/usr/bin/env bash

for i in `seq 1000`; do
  logger -n 127.0.0.1 -P 5555 -t stress -i one "Stress Test -- $i" &
  logger -n 127.0.0.1 -P 5555 -t stress -i two "Stress Test -- $i" &
  logger -n 127.0.0.1 -P 5555 -t stress -i three "Stress Test -- $i" &
  logger -n 127.0.0.1 -P 5555 -t stress -i four "Stress Test -- $i" &
done
