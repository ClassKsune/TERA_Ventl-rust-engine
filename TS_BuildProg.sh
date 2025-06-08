#!/bin/sh

echo -n "Clean debug build? [y]"
read V_Str_Resp

if [ "$V_Str_Resp" = "y" ]; then
	cargo clean
	cargo build
fi

echo -n "Re-Add resources? [y]"
read V_Str_Resp2

if [ "$V_Str_Resp2" = "y" ]; then
	cp -r ./FLD_Resources ./target/debug/
	cp -r ./FLD_GuideFiles/ ./target/debug/
fi

cargo build
clear