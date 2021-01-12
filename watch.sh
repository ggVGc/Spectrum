#!/usr/bin/env bash

watchexec -cr --exts rs 'bspc rule -a \*:\*:\* -o state=floating focus=false rectangle=400x400+1520+0 && cargo run '
