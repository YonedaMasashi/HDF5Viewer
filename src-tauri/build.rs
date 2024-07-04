use std::fs;
use std::path::Path;

fn main() {
  let target_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  let src_dll = "C:/soft/install/hdf5/HDF5-1.12.3-win64/bin/hdf5.dll";
  let dest_dll = Path::new(&target_dir).join("hdf5.dll");
  fs::copy(src_dll, dest_dll).unwrap();
  
  tauri_build::build()
}
