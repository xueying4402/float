//! Wasmer will let you easily run Wasm module in a Rust host.
//!
//! This example illustrates the basics of using Wasmer through a "Hello World"-like project:
//!
//!   1. How to load a Wasm modules as bytes
//!   2. How to compile the module
//!   3. How to create an instance of the module
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example instance --release --features "cranelift"
//! ```
//!
//! Ready?

use wasmer::{imports, wat2wasm, Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module.
    //
    // We are using the text representation of the module here but you can also load `.wasm`
    // files using the `include_bytes!` macro.
    let wasm_bytes = wat2wasm(
        br#"
        (module
            (type $add_one_t (func (param f32) (result f32)))
            (func $add_one_f (type $add_one_t)  (param $value f32) (result f32)
              local.get $value
              f32.const 0.0
              f32.div)
            (export "add_one" (func $add_one_f))
          )"#,
    )?;

    // Create a Store.
    // Note that we don't need to specify the engine/compiler if we want to use
    // the default provided by Wasmer.
    // You can use `Store::default()` for that.
    let store = Store::new(&Universal::new(Cranelift::default()).engine());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    // We now have an instance ready to be used.
    //
    // From an `Instance` we can retrieve any exported entities.
    // Each of these entities is covered in others examples.
    //
    // Here we are retrieving the exported function. We won't go into details here
    // as the main focus of this example is to show how to create an instance out
    // of a Wasm module and have basic interactions with it.
    let add_one = instance
        .exports
        .get_function("add_one")?
        .native::<f32, f32>()?;

    println!("Calling `add_one` function...");
    let result = add_one.call(0.0)?;
    println!("{:?}", result.to_be_bytes());

    println!("Results of `add_one`: {:?}", result);
    assert_eq!(result, f32::NAN);

    Ok(())
}

#[test]
fn test_exported_function() -> Result<(), Box<dyn std::error::Error>> {
    main()
}