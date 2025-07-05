# LLVM

## Language Reference

https://llvm.org/docs/LangRef.html

## llvm version

```bash
clang++ --version
lli --version
```

## Generate llvm code

```bash
clang++ -S -emit-llvm test.cc
lli test.ll
```

## Compile llvm code

```bash
clang++ -o minimal minimal.ll
./test
```

## LLVM Interpreter

```bash
lli minimal.ll
```

## LLVM assembler and disassembler

```bash
llvm-as  minimal.ll
llvm-dis minimal.bc
```

## LLVM Native assembler

```bash
clang++ -S minimal.ll
clang++ -o minimal minimal.s
./minimal
```
