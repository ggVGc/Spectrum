#!/usr/bin/env bash

watchexec -cr --exts rs 'bspc rule -a \*:\*:\* -o state=floating focus=false rectangle=800x600+1121+483 && cargo run '
