# Tauri Window Save State Bug

### Setup
1. `pnpm i`
1. `RUST_BACKTRACE=full pnpm tauri dev`


🛑 Blocked on startup with the following stack dump

```
thread 'main' panicked at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cocoa-0.24.1/src/appkit.rs:1183:9:
Uncaught exception <NSException: 0x600003cea160>
stack backtrace:
   0:        0x104fef7e0 - std::backtrace_rs::backtrace::libunwind::trace::he4f0a5f56afe8e37
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/libunwind.rs:105:5
   1:        0x104fef7e0 - std::backtrace_rs::backtrace::trace_unsynchronized::habb302958e80f800
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x104fef7e0 - std::sys_common::backtrace::_print_fmt::h9819d35e2a5cda77
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:68:5
   3:        0x104fef7e0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1f3776e0b5c7517d
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x10500d848 - core::fmt::rt::Argument::fmt::h626862aa6242248a
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/rt.rs:165:63
   5:        0x10500d848 - core::fmt::write::heedef092c8c0962e
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/mod.rs:1157:21
   6:        0x104fed1cc - std::io::Write::write_fmt::h7178e8e2ea928914
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/io/mod.rs:1832:15
   7:        0x104fef638 - std::sys_common::backtrace::_print::ha0f584bc7bfb9d2b
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:47:5
   8:        0x104fef638 - std::sys_common::backtrace::print::h417292deb95532ed
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:34:9
   9:        0x104ff0794 - std::panicking::default_hook::{{closure}}::h0cb68f1228c4613a
  10:        0x104ff0488 - std::panicking::default_hook::h24535936bc1f51de
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:298:9
  11:        0x104ff104c - std::panicking::rust_panic_with_hook::h5db4d2345b297bed
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:795:13
  12:        0x104ff0a7c - std::panicking::begin_panic_handler::{{closure}}::h3fd558f09a0d5492
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:664:13
  13:        0x104fefc68 - std::sys_common::backtrace::__rust_end_short_backtrace::hfc76eebe1ce501b2
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:171:18
  14:        0x104ff07ec - rust_begin_unwind
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
  15:        0x10502acb4 - core::panicking::panic_fmt::hc2b459a5bd3dce66
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:72:14
  16:        0x104fc697c - core::panicking::panic_display::h69e346896f5b054a
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:263:5
  17:        0x104c81ee8 - <*mut objc::runtime::Object as cocoa::appkit::NSWindow>::setStyleMask_::h098447cb28d2cca0
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cocoa-0.24.1/src/appkit.rs:1183:9
  18:        0x104b89bd8 - tao::platform_impl::platform::util::async::set_style_mask::hba50aadbca8bc76a
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.16.9/src/platform_impl/macos/util/async.rs:50:3
  19:        0x104b89cd0 - tao::platform_impl::platform::util::async::set_style_mask_async::{{closure}}::h836b65d74cd5d7c2
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.16.9/src/platform_impl/macos/util/async.rs:64:5
  20:        0x104ba12dc - dispatch::context_and_function::work_execute_closure::h516f47ab7ecb9548
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/dispatch-0.2.0/src/lib.rs:94:9
  21:        0x19d7463e8 - <unknown>
  22:        0x19d754bb8 - <unknown>
  23:        0x19d7547cc - <unknown>
  24:        0x19da17ad4 - <unknown>
  25:        0x19d9d5258 - <unknown>
  26:        0x19d9d4434 - <unknown>
  27:        0x1a817819c - <unknown>
  28:        0x1a8177fd8 - <unknown>
  29:        0x1a8177d30 - <unknown>
  30:        0x1a1233d68 - <unknown>
  31:        0x1a1a29808 - <unknown>
  32:        0x1a122709c - <unknown>
  33:        0x104fc722c - <() as objc::message::MessageArguments>::invoke::he2a9f5e157602ad3
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:128:17
  34:        0x104fc495c - objc::message::platform::send_unverified::{{closure}}::h1de18d70506d8a66
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/apple/mod.rs:27:9
  35:        0x104fc1dd0 - objc_exception::try::{{closure}}::h503f0df89e179fdb
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:68:31
  36:        0x104fc06f4 - objc_exception::try_no_ret::try_objc_execute_closure::h2a3a7b88012b221c
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:34:9
  37:        0x104fce768 - RustObjCExceptionTryCatch
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/extern/exception.m:10:9
  38:        0x104fbfa0c - objc_exception::try_no_ret::h7a87fbf87301c277
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:44:19
  39:        0x104fc1640 - objc_exception::try::hb0fa7117000458cf
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc_exception-0.1.2/src/lib.rs:67:9
  40:        0x104fc6af4 - objc::exception::try::h4de6ba1989c61214
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/exception.rs:8:5
  41:        0x104fc4404 - objc::message::platform::send_unverified::hdd52eec107c95d84
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:12:9
  42:        0x1049ce808 - objc::message::send_message::he5fba4ee7cf2b9a0
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/objc-0.2.7/src/message/mod.rs:178:5
  43:        0x1049ce808 - tao::platform_impl::platform::event_loop::EventLoop<T>::run_return::heba0781fe7b593f8
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.16.9/src/platform_impl/macos/event_loop.rs:193:16
  44:        0x1049cf68c - tao::platform_impl::platform::event_loop::EventLoop<T>::run::h133a10348c2b692b
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.16.9/src/platform_impl/macos/event_loop.rs:160:21
  45:        0x104a6f180 - tao::event_loop::EventLoop<T>::run::h3942928b83a8e218
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tao-0.16.9/src/event_loop.rs:179:5
  46:        0x1049d4bd0 - <tauri_runtime_wry::Wry<T> as tauri_runtime::Runtime<T>>::run::h131094f1f0142c72
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-runtime-wry-0.14.8/src/lib.rs:2299:5
  47:        0x10493b9b0 - tauri::app::App<R>::run::hd69b39225c29766a
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-1.6.8/src/app.rs:868:5
  48:        0x10493bbd4 - tauri::app::Builder<R>::run::h98a97b6452aec0b3
                               at /Users/hacksore/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tauri-1.6.8/src/app.rs:1727:5
  49:        0x1049df4d0 - window_save_state_bug::main::ha0dc1a1a02dd7d33
                               at /Users/hacksore/code/repro/tauri-window-save-state-bug/src-tauri/src/main.rs:23:3
  50:        0x1049a9c34 - core::ops::function::FnOnce::call_once::h25f3ad00820c7fc3
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/ops/function.rs:250:5
  51:        0x1049044f4 - std::sys_common::backtrace::__rust_begin_short_backtrace::hb2d4b355decfc6d6
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:155:18
  52:        0x104a1b468 - std::rt::lang_start::{{closure}}::hec495c55d1096433
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/rt.rs:159:18
  53:        0x104fe85c4 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h2f86a413382a979d
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/ops/function.rs:284:13
  54:        0x104fe85c4 - std::panicking::try::do_call::hd40c9eb4d233b111
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:559:40
  55:        0x104fe85c4 - std::panicking::try::h13ac68ffa70c387b
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:523:19
  56:        0x104fe85c4 - std::panic::catch_unwind::habea7b6fc986e966
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panic.rs:149:14
  57:        0x104fe85c4 - std::rt::lang_start_internal::{{closure}}::h6b16436250c3cf62
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/rt.rs:141:48
  58:        0x104fe85c4 - std::panicking::try::do_call::h9970b928a0b20951
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:559:40
  59:        0x104fe85c4 - std::panicking::try::h4dfbe3cb4cc8f253
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:523:19
  60:        0x104fe85c4 - std::panic::catch_unwind::hf6a5e1e8ce5a10f5
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panic.rs:149:14
  61:        0x104fe85c4 - std::rt::lang_start_internal::hecc68fef83c8f44d
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/rt.rs:141:20
  62:        0x104a1b434 - std::rt::lang_start::h1760092ae641734b
                               at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/rt.rs:158:17
  63:        0x1049dfc64 - _main
libc++abi: terminating due to uncaught foreign exception
```
