# Getting Started with utrace

This document provides an example of using `utrace` to analyze
the `hypervisor` in [crosvm](https://github.com/google/crosvm) on Ubuntu.

## Prerequisites

First, clone `crosvm` and `utrace` in your home directory (`~/utrace`, `~/crosvm`)
and set them up for building:

- **crosvm**: 
  ```sh
  cd ~/crosvm
  git submodule update --init && sudo apt install libcap-dev
  ```
- **utrace**: 
  ```sh
  cd utrace
  make init
  ```

## Analyzing Unsafe Code

### 1. Summarize Unsafe Code

Check the summary to see the overview of unsafe code.

```sh
$ cd ~/utrace && CRATE_PATH=~/crosvm/hypervisor make summary

## Summary
Crate                Functions  Blocks     Impls      Traits
futures_executor     0          4          0          0
probe4               0          0          0          0
intrusive_collections 183        89         62         7
serde_keyvalue_derive 0          0          0          0
cexpr                0          0          0          0
shlex                0          3          0          0
anyhow               20         23         2          0
byteorder            0          53         0          0
...
getrandom            6          9          1          0
cros_async           11         48         16         1
hashbrown            61         82         17         1
either               0          0          0          0
quote                0          0          0          0
libc                 41         2          0          0
lz4_flex             0          3          0          0
futures              0          0          0          0
pin_utils            0          0          0          0
regex_syntax         0          0          0          0
clang_sys            348        389        0          0
futures_channel      3          9          8          0
thiserror_impl       0          0          0          0
futures_core         0          3          2          0
base                 35         184        26         4
memchr               54         23         0          0
probe0               0          0          0          0
kvm_sys              6          84         72         0
hypervisor           4          74         0          0
futures_util         9          308        24         0
argh_derive          0          0          0          0
futures_task         21         13         12         1
...
```

### 2. List Unsafe Items

This command lists all items that contain unsafe code.

```sh
$ CRATE_PATH=~/crosvm/hypervisor CRATE_NAME=hypervisor make unsafe-list

- type: Block, id: hypervisor::<kvm::Kvm as Hypervisor>::check_capability
- type: Block, id: hypervisor::<kvm::KvmVcpu as Vcpu>::handle_io
- type: Block, id: hypervisor::<kvm::KvmVcpu as Vcpu>::handle_mmio
- type: Block, id: hypervisor::<kvm::KvmVcpu as Vcpu>::on_suspend
- type: Block, id: hypervisor::<kvm::KvmVcpu as Vcpu>::run
- type: Block, id: hypervisor::<kvm::KvmVcpu as Vcpu>::set_immediate_exit
 type: Block, id: hypervisor::kvm::KvmVm::ioeventfd
- type: Block, id: hypervisor::kvm::KvmVm::new
- type: Block, id: hypervisor::kvm::KvmVm::register_irqfd
- type: Block, id: hypervisor::kvm::KvmVm::set_gsi_routing
- type: Block, id: hypervisor::kvm::KvmVm::set_irq_line
- type: Block, id: hypervisor::kvm::KvmVm::unregister_irqfd
- type: Block, id: hypervisor::kvm::x86_64::<impl kvm::KvmVcpu>::get_interrupt_bitmap
- type: Block, id: hypervisor::kvm::x86_64::<impl kvm::KvmVcpu>::get_lapic
- type: Block, id: hypervisor::kvm::x86_64::<impl kvm::KvmVcpu>::handle_vm_exit_arch
 type: Block, id: hypervisor::kvm::x86_64::<impl kvm::KvmVm>::set_pit_state
- type: Block, id: hypervisor::kvm::x86_64::<impl kvm::KvmVm>::set_platform_info_read_access
- type: Block, id: hypervisor::kvm::x86_64::<impl kvm::KvmVm>::set_pvclock_arch
- type: Block, id: hypervisor::kvm::x86_64::<impl std::convert::From<&kvm_sys::kvm_ioapic_state__bindgen_ty_1> for x86_64::IoapicRedirectionTableEntry>::from
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::HypervisorX86_64 for kvm::Kvm>::get_msr_index_list
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_all_msrs
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_debugregs
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_fpu
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_interrupt_state
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_msr
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_regs
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_sregs
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_xcrs
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::get_xsave
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::inject_nmi
- type: Block, id: hypervisor::kvm::x86_64::<impl x86_64::VcpuX86_64 for kvm::KvmVcpu>::interrupt
...
- type: Block, id: hypervisor::x86_64::VcpuX86_64::set_tsc_offset
- type: Block, id: hypervisor::x86_64::host_phys_addr_bits
- type: Function, id: hypervisor::<kvm::KvmVcpu as Vcpu>::enable_raw_capability
- type: Function, id: hypervisor::Vcpu::enable_raw_capability
- type: Function, id: hypervisor::kvm::KvmVm::enable_raw_capability
- type: Function, id: hypervisor::kvm::set_user_memory_region
```

### 3. Analyze Call Trace for Unsafe Code

This command analyzes the call trace and highlights unsafe code within functions.

```sh
CRATE_PATH=~/crosvm/hypervisor CRATE_NAME=hypervisor make call-trace

- hypervisor::x86_64::_::<impl _::_serde::Serialize for x86_64::PitRWMode>::serialize
    - serde::ser::Serializer::serialize_unit_variant
    - serde::ser::Serializer::serialize_unit_variant
    - serde::ser::Serializer::serialize_unit_variant
    - serde::ser::Serializer::serialize_unit_variant
- hypervisor::x86_64::_::<impl _::_serde::Serialize for x86_64::PitRWState>::serialize
    - serde::ser::Serializer::serialize_unit_variant
    - serde::ser::Serializer::serialize_unit_variant
    - serde::ser::Serializer::serialize_unit_variant
    - serde::ser::Serializer::serialize_unit_variant
    - serde::ser::Serializer::serialize_unit_variant
- hypervisor::x86_64::_::<impl _::_serde::Serialize for x86_64::Sregs>::serialize
    - serde::ser::Serializer::serialize_struct
    - core::result::Result::Err::{constructor#0}
...
- hypervisor::x86_64::_::<impl _::_serde::Serialize for x86_64::VcpuSnapshot>::serialize
    - serde::ser::Serializer::serialize_struct
    - core::result::Result::Err::{constructor#0}
    - serde::ser::SerializeStruct::serialize_field
    - serde::ser::SerializeStruct::end
- hypervisor::x86_64::_::<impl _::_serde::Serialize for x86_64::Xsave>::serialize
    - serde::ser::Serializer::serialize_struct
    - core::result::Result::Err::{constructor#0}
    - serde::ser::SerializeStruct::serialize_field
    - core::result::Result::Err::{constructor#0}
    - serde::ser::SerializeStruct::serialize_field
    - core::result::Result::Err::{constructor#0}
    - serde::ser::SerializeStruct::end
- hypervisor::x86_64::host_phys_addr_bits (unsafe)
    - core::core_arch::x86::cpuid::__cpuid
    - core::core_arch::x86::cpuid::__cpuid
```

---

With just a few simple commands, we can analyze unsafe code in a Rust project!
