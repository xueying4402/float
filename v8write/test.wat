(module
  (type $t0 (func (param f32)))
  (type $t1 (func (param f32)))
  (import "imports" "imported_func" (func $imports.imported_func (type $t0)))
  (func $exported_func (type $t1)
    f32.const 0.0
    f32.const 0.0
    f32.div
    call $imports.imported_func)
  (export "exported_func" (func $exported_func))
)