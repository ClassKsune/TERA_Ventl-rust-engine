#!/bin/sh

echo -n "Clean debug build? [y]"
read V_Str_Resp

if [ "$V_Str_Resp" = "y" ]; then
	cargo clean
	cargo build
fi

cp -r ./FLD_Resources ./target/debug/
cp -r ./FLD_GuideFiles/ ./target/debug/

cargo build
clear