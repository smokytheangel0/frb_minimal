set shell := ["powershell"]



gen_and_lint_all:
    flutter pub get
    @just lint
    cd native; cargo build;
    @just gen "C:/Users/wiggle/Desktop/flutter_rust_bridge_template/" "--llvm-path C:/Program Files/LLVM/bin --rust-crate-dir C:/Users/wiggle/Desktop/flutter_rust_bridge_template/native"

gen_and_lint_flutter:
    flutter pub get
    @just gen "C:/Users/wiggle/Desktop/flutter_rust_bridge_template/" "--llvm-path C:/Program Files/LLVM/bin --rust-crate-dir C:/Users/wiggle/Desktop/flutter_rust_bridge_template/native"
    @just lint_flutter

default: gen_and_lint_all

bridge:
    @just gen "C:/Users/wiggle/Desktop/flutter_rust_bridge_template/" "--llvm-path C:/Program Files/LLVM/bin --rust-crate-dir C:/Users/wiggle/Desktop/flutter_rust_bridge_template/native"

gen working_dir llvm_path:
    cd /; flutter_rust_bridge_codegen {{llvm_path}} \
    --rust-input "{{working_dir}}/native/src/api.rs" \
    --dart-output "{{working_dir}}/lib/bridge_generated.dart" \
    --c-output "{{working_dir}}/ios/Runner/bridge_generated.h" \
    --c-output "{{working_dir}}/macos/Runner/bridge_generated.h"

lint:
    dart format .
    cd native; cargo fmt; cargo clippy;

lint_flutter:
    dart format .

attempt MESSAGE:
    git pull
    @just gen_and_lint_all
    flutter test
    git commit -m "{{MESSAGE}}"

push:
    git push

run:
    @just gen_and_lint_flutter
    flutter run

all:
    @just gen_and_lint_all

release:
    cd native; cargo build --release;
    flutter run --release
    mkdir run
    mkdir run/windows
    cp -R build/windows/runner/Release/* run/windows
#also strip bin in linux
#and handle browser releases

# name {{name}}:
# change all the places the package name shows up
# this needs another binary (hopefully packaged by cargo)

# write {{author}}:
# change all the places the author shows up
# this needs another binary (name_writer)

# unit:
# run doctests and unit tests which cover the last modified module

# test
# run driver tests with which cover the last modified module

clean:
    flutter clean
    cd native; cargo clean
    rm -R run

# vim:expandtab:sw=4:ts=4
