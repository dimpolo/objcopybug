use probe_rs::flashing::FlashLoader;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    run_objcopy("made_by_objcopy.bin");
    run_probe("made_by_probe.bin");
}

fn run_objcopy(out_path: &str) {
    Command::new("rust-objcopy")
        .args(["test.elf", out_path, "-O", "binary"])
        .output()
        .unwrap();
}

fn run_probe(out_path: &str) {
    let p = Path::new("test.elf");
    let mut file = std::fs::File::open(p).unwrap();

    let target = probe_rs::config::get_target_by_name("STM32G041G8Ux").unwrap();
    let mut loader = FlashLoader::new(target.memory_map.to_vec(), target.source().clone());
    loader.load_elf_data(&mut file).unwrap();

    let data = loader.data().next().unwrap().1;

    let mut f = std::fs::File::create(Path::new(out_path)).unwrap();
    f.write_all(data).unwrap();
}
