svd2rust.exe -i RV32M1_RI5CY.svd --target riscv
rm src/* -Recurse -Force
form -i lib.rs -o src
rm lib.rs
cargo fmt -p rv32m1_ri5cy-pac
