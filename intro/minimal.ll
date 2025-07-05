; ModuleID = 'minimal.bc'
source_filename = "minimal.ll"

; To avoid the warning: overriding the module target triple with x86_64-apple-macosx13.0.0 [-Woverride-module]
target triple = "x86_64-apple-macosx13.0.0"

define i32 @main() {
  ret i32 42
}
