extern crate gcc;

fn main() {
    gcc::compile_library("libmusahi.a",
                         &["native/m68kcpu.c",
                           "native/m68kdasm.c",
                           "native/m68kopac.c",
                           "native/m68kopdm.c",
                           "native/m68kopnz.c",
                           "native/m68kops.c",
                           "native/m68k_wrapper.c"]);
}
