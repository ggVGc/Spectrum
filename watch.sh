#!/usr/bin/env bash

watchexec -cr --exts rs 'cargo build && bspc rule -a \*:\*:\* -o state=floating focus=false rectangle=400x400+1520+680 && cargo run '
