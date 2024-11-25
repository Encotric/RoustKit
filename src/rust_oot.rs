//! Sample Rust Linux Out-of-tree kernel module
use kernel::prelude::*;

module! {
    type: RustOot,
    name: "rust_oot",
    author: "0xor0ne",
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustOot {
    numbers: Vec<i32>,
}

impl kernel::Module for RustOot {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust OOT sample (init)\n");

        let mut numbers = Vec::new();
        numbers.push(72, GFP_KERNEL)?;
        numbers.push(108, GFP_KERNEL)?;
        numbers.push(200, GFP_KERNEL)?;

        Ok(RustOot { numbers })
    }
}

impl Drop for RustOot {
    fn drop(&mut self) {
        pr_info!("Rust OOT: numbers {:?}\n", self.numbers);
        pr_info!("Rust OOT sample (exit)\n");
    }
}
