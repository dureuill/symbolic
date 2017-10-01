use std::mem;
use std::fmt;
use std::ffi::CStr;

#[cfg(feature = "with_objects")]
use mach_object;
#[cfg(feature = "with_dwarf")]
use gimli;

use errors::{ErrorKind, Result};

/// Represents endianess.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Endianness {
    Little,
    Big,
}

impl Default for Endianness {
    #[cfg(target_endian = "little")]
    #[inline]
    fn default() -> Endianness {
        Endianness::Little
    }

    #[cfg(target_endian = "big")]
    #[inline]
    fn default() -> Endianness {
        Endianness::Big
    }
}

#[cfg(feature = "with_dwarf")]
impl gimli::Endianity for Endianness {
    #[inline]
    fn is_big_endian(self) -> bool {
        self == Endianness::Big
    }
}

/// Represents a family of CPUs
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CpuFamily {
    Intel32,
    Intel64,
    Arm32,
    Arm64,
    Unknown,
}

/// An enum of supported architectures.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum Arch {
    Unknown,
    X86,
    X86_64,
    ArmV5,
    ArmV6,
    ArmV7,
    ArmV7f,
    ArmV7s,
    ArmV7k,
    ArmV7m,
    ArmV7em,
    Arm64,
    #[doc(hidden)]
    __Max
}

impl Default for Arch {
    fn default() -> Arch {
        Arch::Unknown
    }
}

impl Arch {
    /// Creates an arch from the u32 it represents
    pub fn from_u32(val: u32) -> Result<Arch> {
        if val >= (Arch::__Max as u32) {
            Err(ErrorKind::Parse("unknown architecture").into())
        } else {
            Ok(unsafe { mem::transmute(val) })
        }
    }

    /// Constructs an architecture from mach CPU types
    #[cfg(feature = "with_objects")]
    pub fn from_mach(cputype: u32, cpusubtype: u32) -> Result<Arch> {
        let ty = cputype as i32;
        let subty = cpusubtype as i32;
        if let Some(arch) = mach_object::get_arch_name_from_types(ty, subty) {
            Arch::parse(arch)
        } else {
            Err(ErrorKind::Parse("unknown architecture").into())
        }
    }

    /// Constructs an architecture from ELF flags
    #[cfg(feature = "with_objects")]
    pub fn from_elf(machine: u16) -> Result<Arch> {
        use goblin::elf::header::*;
        Ok(match machine {
            EM_386 => Arch::X86,
            EM_X86_64 => Arch::X86_64,
            // FIXME: This is incorrect! ARM information is located in the .ARM.attributes section
            EM_ARM => Arch::ArmV7,
            _ => return Err(ErrorKind::Parse("unknown architecture").into()),
        })
    }

    /// Parses an architecture from a string.
    pub fn parse(string: &str) -> Result<Arch> {
        use Arch::*;
        Ok(match string {
            "x86" => X86,
            "x86_64" => X86_64,
            "arm64" => Arm64,
            "armv5" => ArmV5,
            "armv6" => ArmV6,
            "armv7" => ArmV7,
            "armv7f" => ArmV7f,
            "armv7s" => ArmV7s,
            "armv7k" => ArmV7k,
            "armv7m" => ArmV7m,
            "armv7em" => ArmV7em,
            _ => {
                return Err(ErrorKind::Parse("unknown architecture").into());
            }
        })
    }

    /// Returns the CPU family
    pub fn cpu_family(&self) -> CpuFamily {
        use Arch::*;
        match *self {
            Unknown | __Max => CpuFamily::Unknown,
            X86 => CpuFamily::Intel32,
            X86_64 => CpuFamily::Intel64,
            Arm64 => CpuFamily::Arm64,
            ArmV5 | ArmV6 | ArmV7 | ArmV7f | ArmV7s | ArmV7k | ArmV7m | ArmV7em => CpuFamily::Arm32,
        }
    }

    /// Returns the native pointer size
    pub fn pointer_size(&self) -> Option<usize> {
        use Arch::*;
        match *self {
            Unknown | __Max => None,
            X86_64 | Arm64 => Some(8),
            X86 | ArmV5 | ArmV6 | ArmV7 | ArmV7f | ArmV7s | ArmV7k | ArmV7m | ArmV7em => Some(4),
        }
    }

    #[doc(hidden)]
    pub fn name_cstr(&self) -> &'static CStr {
        use Arch::*;
        CStr::from_bytes_with_nul(match *self {
            Unknown | __Max => b"unknown\0",
            X86 => b"x86\0",
            X86_64 => b"x86_64\0",
            Arm64 => b"arm64\0",
            ArmV5 => b"armv5\0",
            ArmV6 => b"armv6\0",
            ArmV7 => b"armv7\0",
            ArmV7f => b"armv7f\0",
            ArmV7s => b"armv7s\0",
            ArmV7k => b"armv7k\0",
            ArmV7m => b"armv7m\0",
            ArmV7em => b"armv7em\0",
        }).unwrap()
    }

    /// Returns the name of the arch
    pub fn name(&self) -> &'static str {
        let s_with_nul = self.name_cstr().to_str().unwrap();
        &s_with_nul[..s_with_nul.len() - 1]
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Supported programming languages for demangling
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[repr(u32)]
pub enum Language {
    Unknown,
    C,
    Cpp,
    D,
    Go,
    ObjC,
    ObjCpp,
    Rust,
    Swift,
    #[doc(hidden)]
    __Max
}

impl Language {
    /// Creates a language from the u32 it represents
    pub fn from_u32(val: u32) -> Result<Language> {
        if val >= (Language::__Max as u32) {
            Err(ErrorKind::Parse("unknown language").into())
        } else {
            Ok(unsafe { mem::transmute(val) })
        }
    }

    /// Converts a DWARF language tag into a supported language.
    #[cfg(feature="with_dwarf")]
    pub fn from_dwarf_lang(lang: gimli::DwLang) -> Option<Language> {
        match lang {
            gimli::DW_LANG_C | gimli::DW_LANG_C11 |
            gimli::DW_LANG_C89 | gimli::DW_LANG_C99 => Some(Language::C),
            gimli::DW_LANG_C_plus_plus | gimli::DW_LANG_C_plus_plus_03 |
            gimli::DW_LANG_C_plus_plus_11 |
            gimli::DW_LANG_C_plus_plus_14 => Some(Language::Cpp),
            gimli::DW_LANG_D => Some(Language::D),
            gimli::DW_LANG_Go => Some(Language::Go),
            gimli::DW_LANG_ObjC => Some(Language::ObjC),
            gimli::DW_LANG_ObjC_plus_plus => Some(Language::ObjCpp),
            gimli::DW_LANG_Rust => Some(Language::Rust),
            gimli::DW_LANG_Swift => Some(Language::Swift),
            _ => None,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Language::Unknown | Language::__Max => "unknown",
            Language::C => "C",
            Language::Cpp => "C++",
            Language::D => "D",
            Language::Go => "Go",
            Language::ObjC => "Objective-C",
            Language::ObjCpp => "Objective-C++",
            Language::Rust => "Rust",
            Language::Swift => "Swift"
        })
    }
}

/// Represents the kind of an object.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum ObjectKind {
    MachO,
    Elf,
}
