# utrace

A easy-to-use and intuitive tool for analyzing unsafe code in Rust.

## Features
- **Unsafe Code Summary**:
  - Categorizes unsafe code based on [Rust's Unsafe Keywords](https://doc.rust-lang.org/reference/unsafe-keyword.html),
    including all dependencies, and provides a summary.
- **Unsafe Item Listing**:
  - Lists items containing unsafe code.
- **Unsafe Call Trace**:
  - Displays the call trace of functions within a crate,
    indicating the presence of unsafe code at each step.

## How to Use
Install `utrace` with `make init` and specify the project path with `CRATE_PATH`.

### Unsafe Code Summary

```
$ CRATE_PATH=~/islet/rmm make summary

## Summary
Crate                Functions  Blocks     Impls      Traits
spinning_top         1          0          1          0
islet_rmm            15         18         2          0
spki                 0          0          0          0
log                  2          3          0          0
serde                0          1          0          0
subtle               0          2          0          0
quote                0          0          0          0
base16ct             0          4          0          0
safe_abstraction     2          3          0          0
lazy_static          0          0          0          0
...
generic_array        6          493        20         8
```

### Unsafe Item Listing

```
## Unsafe Item List (islet_rmm)
- type: Block, id: islet_rmm::<mm::page_table::entry::Entry as vmsa::page_table::Entry>::set
- type: Block, id: islet_rmm::<realm::mm::entry::Entry as vmsa::page_table::Entry>::set
- type: Block, id: islet_rmm::<realm::mm::stage2_translation::Stage2Translation<'a> as realm::mm::IPATranslation>::clean
- type: Block, id: islet_rmm::allocator::get_used_size
- type: Block, id: islet_rmm::asm::dcache_flush
- type: Block, id: islet_rmm::asm::smc
- type: Block, id: islet_rmm::granule::array::entry::Granule::zeroize
...
- type: Function, id: islet_rmm::rmm_exit
- type: Function, id: islet_rmm::setup_el2
- type: Impl, id: islet_rmm::<event::rsihandle::RsiHandle as core::marker::Send>
- type: Impl, id: islet_rmm::<event::rsihandle::RsiHandle as core::marker::Sync>
## Unsafe Item List (vmsa)
- type: Block, id: vmsa::<page_table::PageTable<A, L, E, N> as page_table::PageTableMethods<A, L, E, N>>::drop
- type: Block, id: vmsa::page_table::PageTable::<A, L, E, N>::new_in
- type: Block, id: vmsa::page_table::PageTable::<A, L, E, N>::new_init_in
- type: Block, id: vmsa::page_table::PageTable::<A, L, E, N>::subtable
- type: Function, id: vmsa::page_table::MemAlloc::allocate
- type: Function, id: vmsa::page_table::MemAlloc::deallocate
```

### Unsafe Call Trace

```
## Unsafe Call Trace
- islet_rmm::<mm::page_table::entry::Entry as vmsa::page_table::Entry>::points_to_table_or_page
    - vmsa::vmsa::page_table::Entry::is_valid
    - islet_rmm::mm::page_table::entry::PTDesc::get_masked_value
        - core::core::num::<impl u64>::trailing_zeros
- islet_rmm::<mm::page_table::entry::Entry as vmsa::page_table::Entry>::pte
    - islet_rmm::mm::page_table::entry::PTDesc::get
- islet_rmm::<mm::page_table::entry::Entry as vmsa::page_table::Entry>::set (unsafe)
    - islet_rmm::mm::page_table::entry::PTDesc::set
    - vmsa::vmsa::address::PhysAddr::as_u64
    - islet_rmm::mm::page_table::entry::PTDesc::set_bits
    - islet_rmm::mm::page_table::entry::PTDesc::set_masked_value
    - core::core::fmt::Formatter::<'a>::debug_struct
- islet_rmm::<mm::translation::Inner<'a> as core::ops::Drop>::drop
    - log::max_level (unsafe)
        - core::intrinsics::{extern#1}::transmute
        - core::core::sync::atomic::AtomicUsize::load
    - log::__private_api::log
        - log::__private_api::log_impl
            - core::core::option::Option::<T>::is_some
            - core::panicking::panic_fmt
...
- uart::<pl011::DeviceInner as io::Device>::initialized
- uart::<pl011::DeviceInner as io::Write>::write_all
    - uart::pl011::DeviceInner::putc (unsafe)
        - core::core::ptr::mut_ptr::<impl *mut T>::read_volatile
        - core::core::ptr::mut_ptr::<impl *mut T>::offset
        - core::core::ptr::mut_ptr::<impl *mut T>::write_volatile
        - core::core::ptr::mut_ptr::<impl *mut T>::offset
        - core::result::Result::Ok::{constructor#0}
        - core::result::Result::Err::{constructor#0}
    - uart::pl011::DeviceInner::putc (unsafe)
        - core::core::ptr::mut_ptr::<impl *mut T>::read_volatile
        - core::core::ptr::mut_ptr::<impl *mut T>::offset
        - core::core::ptr::mut_ptr::<impl *mut T>::write_volatile
```

---

*This Project is inspired by [cargo-geiger](https://github.com/geiger-rs/cargo-geiger)
and [unsafe-ast](https://github.com/alex-ozdemir/unsafe-ast).*
