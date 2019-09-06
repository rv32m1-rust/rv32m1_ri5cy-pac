#[doc = "Reader of register HWCFG2"]
pub type R = crate::R<u32, super::HWCFG2>;
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP0_A {
  #[doc = "0: Bus master 0 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP0_0,
  #[doc = "1: Bus master 0 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP0_1,
}
impl From<PIDP0_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP0_A) -> Self {
    match variant {
      PIDP0_A::PIDP0_0 => false,
      PIDP0_A::PIDP0_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP0`"]
pub type PIDP0_R = crate::R<bool, PIDP0_A>;
impl PIDP0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP0_A {
    match self.bits {
      false => PIDP0_A::PIDP0_0,
      true => PIDP0_A::PIDP0_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP0_0`"]
  #[inline(always)]
  pub fn is_pidp0_0(&self) -> bool {
    *self == PIDP0_A::PIDP0_0
  }
  #[doc = "Checks if the value of the field is `PIDP0_1`"]
  #[inline(always)]
  pub fn is_pidp0_1(&self) -> bool {
    *self == PIDP0_A::PIDP0_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP1_A {
  #[doc = "0: Bus master 1 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP1_0,
  #[doc = "1: Bus master 1 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP1_1,
}
impl From<PIDP1_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP1_A) -> Self {
    match variant {
      PIDP1_A::PIDP1_0 => false,
      PIDP1_A::PIDP1_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP1`"]
pub type PIDP1_R = crate::R<bool, PIDP1_A>;
impl PIDP1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP1_A {
    match self.bits {
      false => PIDP1_A::PIDP1_0,
      true => PIDP1_A::PIDP1_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP1_0`"]
  #[inline(always)]
  pub fn is_pidp1_0(&self) -> bool {
    *self == PIDP1_A::PIDP1_0
  }
  #[doc = "Checks if the value of the field is `PIDP1_1`"]
  #[inline(always)]
  pub fn is_pidp1_1(&self) -> bool {
    *self == PIDP1_A::PIDP1_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP2_A {
  #[doc = "0: Bus master 2 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP2_0,
  #[doc = "1: Bus master 2 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP2_1,
}
impl From<PIDP2_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP2_A) -> Self {
    match variant {
      PIDP2_A::PIDP2_0 => false,
      PIDP2_A::PIDP2_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP2`"]
pub type PIDP2_R = crate::R<bool, PIDP2_A>;
impl PIDP2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP2_A {
    match self.bits {
      false => PIDP2_A::PIDP2_0,
      true => PIDP2_A::PIDP2_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP2_0`"]
  #[inline(always)]
  pub fn is_pidp2_0(&self) -> bool {
    *self == PIDP2_A::PIDP2_0
  }
  #[doc = "Checks if the value of the field is `PIDP2_1`"]
  #[inline(always)]
  pub fn is_pidp2_1(&self) -> bool {
    *self == PIDP2_A::PIDP2_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP3_A {
  #[doc = "0: Bus master 3 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP3_0,
  #[doc = "1: Bus master 3 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP3_1,
}
impl From<PIDP3_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP3_A) -> Self {
    match variant {
      PIDP3_A::PIDP3_0 => false,
      PIDP3_A::PIDP3_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP3`"]
pub type PIDP3_R = crate::R<bool, PIDP3_A>;
impl PIDP3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP3_A {
    match self.bits {
      false => PIDP3_A::PIDP3_0,
      true => PIDP3_A::PIDP3_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP3_0`"]
  #[inline(always)]
  pub fn is_pidp3_0(&self) -> bool {
    *self == PIDP3_A::PIDP3_0
  }
  #[doc = "Checks if the value of the field is `PIDP3_1`"]
  #[inline(always)]
  pub fn is_pidp3_1(&self) -> bool {
    *self == PIDP3_A::PIDP3_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP4_A {
  #[doc = "0: Bus master 4 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP4_0,
  #[doc = "1: Bus master 4 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP4_1,
}
impl From<PIDP4_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP4_A) -> Self {
    match variant {
      PIDP4_A::PIDP4_0 => false,
      PIDP4_A::PIDP4_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP4`"]
pub type PIDP4_R = crate::R<bool, PIDP4_A>;
impl PIDP4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP4_A {
    match self.bits {
      false => PIDP4_A::PIDP4_0,
      true => PIDP4_A::PIDP4_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP4_0`"]
  #[inline(always)]
  pub fn is_pidp4_0(&self) -> bool {
    *self == PIDP4_A::PIDP4_0
  }
  #[doc = "Checks if the value of the field is `PIDP4_1`"]
  #[inline(always)]
  pub fn is_pidp4_1(&self) -> bool {
    *self == PIDP4_A::PIDP4_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP5_A {
  #[doc = "0: Bus master 5 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP5_0,
  #[doc = "1: Bus master 5 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP5_1,
}
impl From<PIDP5_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP5_A) -> Self {
    match variant {
      PIDP5_A::PIDP5_0 => false,
      PIDP5_A::PIDP5_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP5`"]
pub type PIDP5_R = crate::R<bool, PIDP5_A>;
impl PIDP5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP5_A {
    match self.bits {
      false => PIDP5_A::PIDP5_0,
      true => PIDP5_A::PIDP5_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP5_0`"]
  #[inline(always)]
  pub fn is_pidp5_0(&self) -> bool {
    *self == PIDP5_A::PIDP5_0
  }
  #[doc = "Checks if the value of the field is `PIDP5_1`"]
  #[inline(always)]
  pub fn is_pidp5_1(&self) -> bool {
    *self == PIDP5_A::PIDP5_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP6_A {
  #[doc = "0: Bus master 6 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP6_0,
  #[doc = "1: Bus master 6 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP6_1,
}
impl From<PIDP6_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP6_A) -> Self {
    match variant {
      PIDP6_A::PIDP6_0 => false,
      PIDP6_A::PIDP6_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP6`"]
pub type PIDP6_R = crate::R<bool, PIDP6_A>;
impl PIDP6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP6_A {
    match self.bits {
      false => PIDP6_A::PIDP6_0,
      true => PIDP6_A::PIDP6_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP6_0`"]
  #[inline(always)]
  pub fn is_pidp6_0(&self) -> bool {
    *self == PIDP6_A::PIDP6_0
  }
  #[doc = "Checks if the value of the field is `PIDP6_1`"]
  #[inline(always)]
  pub fn is_pidp6_1(&self) -> bool {
    *self == PIDP6_A::PIDP6_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP7_A {
  #[doc = "0: Bus master 7 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP7_0,
  #[doc = "1: Bus master 7 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP7_1,
}
impl From<PIDP7_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP7_A) -> Self {
    match variant {
      PIDP7_A::PIDP7_0 => false,
      PIDP7_A::PIDP7_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP7`"]
pub type PIDP7_R = crate::R<bool, PIDP7_A>;
impl PIDP7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP7_A {
    match self.bits {
      false => PIDP7_A::PIDP7_0,
      true => PIDP7_A::PIDP7_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP7_0`"]
  #[inline(always)]
  pub fn is_pidp7_0(&self) -> bool {
    *self == PIDP7_A::PIDP7_0
  }
  #[doc = "Checks if the value of the field is `PIDP7_1`"]
  #[inline(always)]
  pub fn is_pidp7_1(&self) -> bool {
    *self == PIDP7_A::PIDP7_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP8_A {
  #[doc = "0: Bus master 8 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP8_0,
  #[doc = "1: Bus master 8 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP8_1,
}
impl From<PIDP8_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP8_A) -> Self {
    match variant {
      PIDP8_A::PIDP8_0 => false,
      PIDP8_A::PIDP8_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP8`"]
pub type PIDP8_R = crate::R<bool, PIDP8_A>;
impl PIDP8_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP8_A {
    match self.bits {
      false => PIDP8_A::PIDP8_0,
      true => PIDP8_A::PIDP8_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP8_0`"]
  #[inline(always)]
  pub fn is_pidp8_0(&self) -> bool {
    *self == PIDP8_A::PIDP8_0
  }
  #[doc = "Checks if the value of the field is `PIDP8_1`"]
  #[inline(always)]
  pub fn is_pidp8_1(&self) -> bool {
    *self == PIDP8_A::PIDP8_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP9_A {
  #[doc = "0: Bus master 9 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP9_0,
  #[doc = "1: Bus master 9 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP9_1,
}
impl From<PIDP9_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP9_A) -> Self {
    match variant {
      PIDP9_A::PIDP9_0 => false,
      PIDP9_A::PIDP9_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP9`"]
pub type PIDP9_R = crate::R<bool, PIDP9_A>;
impl PIDP9_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP9_A {
    match self.bits {
      false => PIDP9_A::PIDP9_0,
      true => PIDP9_A::PIDP9_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP9_0`"]
  #[inline(always)]
  pub fn is_pidp9_0(&self) -> bool {
    *self == PIDP9_A::PIDP9_0
  }
  #[doc = "Checks if the value of the field is `PIDP9_1`"]
  #[inline(always)]
  pub fn is_pidp9_1(&self) -> bool {
    *self == PIDP9_A::PIDP9_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP10_A {
  #[doc = "0: Bus master 10 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP10_0,
  #[doc = "1: Bus master 10 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP10_1,
}
impl From<PIDP10_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP10_A) -> Self {
    match variant {
      PIDP10_A::PIDP10_0 => false,
      PIDP10_A::PIDP10_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP10`"]
pub type PIDP10_R = crate::R<bool, PIDP10_A>;
impl PIDP10_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP10_A {
    match self.bits {
      false => PIDP10_A::PIDP10_0,
      true => PIDP10_A::PIDP10_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP10_0`"]
  #[inline(always)]
  pub fn is_pidp10_0(&self) -> bool {
    *self == PIDP10_A::PIDP10_0
  }
  #[doc = "Checks if the value of the field is `PIDP10_1`"]
  #[inline(always)]
  pub fn is_pidp10_1(&self) -> bool {
    *self == PIDP10_A::PIDP10_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP11_A {
  #[doc = "0: Bus master 11 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP11_0,
  #[doc = "1: Bus master 11 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP11_1,
}
impl From<PIDP11_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP11_A) -> Self {
    match variant {
      PIDP11_A::PIDP11_0 => false,
      PIDP11_A::PIDP11_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP11`"]
pub type PIDP11_R = crate::R<bool, PIDP11_A>;
impl PIDP11_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP11_A {
    match self.bits {
      false => PIDP11_A::PIDP11_0,
      true => PIDP11_A::PIDP11_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP11_0`"]
  #[inline(always)]
  pub fn is_pidp11_0(&self) -> bool {
    *self == PIDP11_A::PIDP11_0
  }
  #[doc = "Checks if the value of the field is `PIDP11_1`"]
  #[inline(always)]
  pub fn is_pidp11_1(&self) -> bool {
    *self == PIDP11_A::PIDP11_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP12_A {
  #[doc = "0: Bus master 12 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP12_0,
  #[doc = "1: Bus master 12 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP12_1,
}
impl From<PIDP12_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP12_A) -> Self {
    match variant {
      PIDP12_A::PIDP12_0 => false,
      PIDP12_A::PIDP12_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP12`"]
pub type PIDP12_R = crate::R<bool, PIDP12_A>;
impl PIDP12_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP12_A {
    match self.bits {
      false => PIDP12_A::PIDP12_0,
      true => PIDP12_A::PIDP12_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP12_0`"]
  #[inline(always)]
  pub fn is_pidp12_0(&self) -> bool {
    *self == PIDP12_A::PIDP12_0
  }
  #[doc = "Checks if the value of the field is `PIDP12_1`"]
  #[inline(always)]
  pub fn is_pidp12_1(&self) -> bool {
    *self == PIDP12_A::PIDP12_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP13_A {
  #[doc = "0: Bus master 13 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP13_0,
  #[doc = "1: Bus master 13 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP13_1,
}
impl From<PIDP13_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP13_A) -> Self {
    match variant {
      PIDP13_A::PIDP13_0 => false,
      PIDP13_A::PIDP13_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP13`"]
pub type PIDP13_R = crate::R<bool, PIDP13_A>;
impl PIDP13_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP13_A {
    match self.bits {
      false => PIDP13_A::PIDP13_0,
      true => PIDP13_A::PIDP13_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP13_0`"]
  #[inline(always)]
  pub fn is_pidp13_0(&self) -> bool {
    *self == PIDP13_A::PIDP13_0
  }
  #[doc = "Checks if the value of the field is `PIDP13_1`"]
  #[inline(always)]
  pub fn is_pidp13_1(&self) -> bool {
    *self == PIDP13_A::PIDP13_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP14_A {
  #[doc = "0: Bus master 14 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP14_0,
  #[doc = "1: Bus master 14 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP14_1,
}
impl From<PIDP14_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP14_A) -> Self {
    match variant {
      PIDP14_A::PIDP14_0 => false,
      PIDP14_A::PIDP14_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP14`"]
pub type PIDP14_R = crate::R<bool, PIDP14_A>;
impl PIDP14_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP14_A {
    match self.bits {
      false => PIDP14_A::PIDP14_0,
      true => PIDP14_A::PIDP14_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP14_0`"]
  #[inline(always)]
  pub fn is_pidp14_0(&self) -> bool {
    *self == PIDP14_A::PIDP14_0
  }
  #[doc = "Checks if the value of the field is `PIDP14_1`"]
  #[inline(always)]
  pub fn is_pidp14_1(&self) -> bool {
    *self == PIDP14_A::PIDP14_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP15_A {
  #[doc = "0: Bus master 15 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP15_0,
  #[doc = "1: Bus master 15 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP15_1,
}
impl From<PIDP15_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP15_A) -> Self {
    match variant {
      PIDP15_A::PIDP15_0 => false,
      PIDP15_A::PIDP15_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP15`"]
pub type PIDP15_R = crate::R<bool, PIDP15_A>;
impl PIDP15_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP15_A {
    match self.bits {
      false => PIDP15_A::PIDP15_0,
      true => PIDP15_A::PIDP15_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP15_0`"]
  #[inline(always)]
  pub fn is_pidp15_0(&self) -> bool {
    *self == PIDP15_A::PIDP15_0
  }
  #[doc = "Checks if the value of the field is `PIDP15_1`"]
  #[inline(always)]
  pub fn is_pidp15_1(&self) -> bool {
    *self == PIDP15_A::PIDP15_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP16_A {
  #[doc = "0: Bus master 16 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP16_0,
  #[doc = "1: Bus master 16 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP16_1,
}
impl From<PIDP16_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP16_A) -> Self {
    match variant {
      PIDP16_A::PIDP16_0 => false,
      PIDP16_A::PIDP16_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP16`"]
pub type PIDP16_R = crate::R<bool, PIDP16_A>;
impl PIDP16_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP16_A {
    match self.bits {
      false => PIDP16_A::PIDP16_0,
      true => PIDP16_A::PIDP16_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP16_0`"]
  #[inline(always)]
  pub fn is_pidp16_0(&self) -> bool {
    *self == PIDP16_A::PIDP16_0
  }
  #[doc = "Checks if the value of the field is `PIDP16_1`"]
  #[inline(always)]
  pub fn is_pidp16_1(&self) -> bool {
    *self == PIDP16_A::PIDP16_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP17_A {
  #[doc = "0: Bus master 17 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP17_0,
  #[doc = "1: Bus master 17 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP17_1,
}
impl From<PIDP17_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP17_A) -> Self {
    match variant {
      PIDP17_A::PIDP17_0 => false,
      PIDP17_A::PIDP17_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP17`"]
pub type PIDP17_R = crate::R<bool, PIDP17_A>;
impl PIDP17_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP17_A {
    match self.bits {
      false => PIDP17_A::PIDP17_0,
      true => PIDP17_A::PIDP17_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP17_0`"]
  #[inline(always)]
  pub fn is_pidp17_0(&self) -> bool {
    *self == PIDP17_A::PIDP17_0
  }
  #[doc = "Checks if the value of the field is `PIDP17_1`"]
  #[inline(always)]
  pub fn is_pidp17_1(&self) -> bool {
    *self == PIDP17_A::PIDP17_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP18_A {
  #[doc = "0: Bus master 18 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP18_0,
  #[doc = "1: Bus master 18 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP18_1,
}
impl From<PIDP18_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP18_A) -> Self {
    match variant {
      PIDP18_A::PIDP18_0 => false,
      PIDP18_A::PIDP18_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP18`"]
pub type PIDP18_R = crate::R<bool, PIDP18_A>;
impl PIDP18_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP18_A {
    match self.bits {
      false => PIDP18_A::PIDP18_0,
      true => PIDP18_A::PIDP18_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP18_0`"]
  #[inline(always)]
  pub fn is_pidp18_0(&self) -> bool {
    *self == PIDP18_A::PIDP18_0
  }
  #[doc = "Checks if the value of the field is `PIDP18_1`"]
  #[inline(always)]
  pub fn is_pidp18_1(&self) -> bool {
    *self == PIDP18_A::PIDP18_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP19_A {
  #[doc = "0: Bus master 19 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP19_0,
  #[doc = "1: Bus master 19 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP19_1,
}
impl From<PIDP19_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP19_A) -> Self {
    match variant {
      PIDP19_A::PIDP19_0 => false,
      PIDP19_A::PIDP19_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP19`"]
pub type PIDP19_R = crate::R<bool, PIDP19_A>;
impl PIDP19_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP19_A {
    match self.bits {
      false => PIDP19_A::PIDP19_0,
      true => PIDP19_A::PIDP19_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP19_0`"]
  #[inline(always)]
  pub fn is_pidp19_0(&self) -> bool {
    *self == PIDP19_A::PIDP19_0
  }
  #[doc = "Checks if the value of the field is `PIDP19_1`"]
  #[inline(always)]
  pub fn is_pidp19_1(&self) -> bool {
    *self == PIDP19_A::PIDP19_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP20_A {
  #[doc = "0: Bus master 20 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP20_0,
  #[doc = "1: Bus master 20 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP20_1,
}
impl From<PIDP20_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP20_A) -> Self {
    match variant {
      PIDP20_A::PIDP20_0 => false,
      PIDP20_A::PIDP20_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP20`"]
pub type PIDP20_R = crate::R<bool, PIDP20_A>;
impl PIDP20_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP20_A {
    match self.bits {
      false => PIDP20_A::PIDP20_0,
      true => PIDP20_A::PIDP20_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP20_0`"]
  #[inline(always)]
  pub fn is_pidp20_0(&self) -> bool {
    *self == PIDP20_A::PIDP20_0
  }
  #[doc = "Checks if the value of the field is `PIDP20_1`"]
  #[inline(always)]
  pub fn is_pidp20_1(&self) -> bool {
    *self == PIDP20_A::PIDP20_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP21_A {
  #[doc = "0: Bus master 21 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP21_0,
  #[doc = "1: Bus master 21 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP21_1,
}
impl From<PIDP21_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP21_A) -> Self {
    match variant {
      PIDP21_A::PIDP21_0 => false,
      PIDP21_A::PIDP21_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP21`"]
pub type PIDP21_R = crate::R<bool, PIDP21_A>;
impl PIDP21_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP21_A {
    match self.bits {
      false => PIDP21_A::PIDP21_0,
      true => PIDP21_A::PIDP21_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP21_0`"]
  #[inline(always)]
  pub fn is_pidp21_0(&self) -> bool {
    *self == PIDP21_A::PIDP21_0
  }
  #[doc = "Checks if the value of the field is `PIDP21_1`"]
  #[inline(always)]
  pub fn is_pidp21_1(&self) -> bool {
    *self == PIDP21_A::PIDP21_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP22_A {
  #[doc = "0: Bus master 22 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP22_0,
  #[doc = "1: Bus master 22 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP22_1,
}
impl From<PIDP22_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP22_A) -> Self {
    match variant {
      PIDP22_A::PIDP22_0 => false,
      PIDP22_A::PIDP22_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP22`"]
pub type PIDP22_R = crate::R<bool, PIDP22_A>;
impl PIDP22_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP22_A {
    match self.bits {
      false => PIDP22_A::PIDP22_0,
      true => PIDP22_A::PIDP22_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP22_0`"]
  #[inline(always)]
  pub fn is_pidp22_0(&self) -> bool {
    *self == PIDP22_A::PIDP22_0
  }
  #[doc = "Checks if the value of the field is `PIDP22_1`"]
  #[inline(always)]
  pub fn is_pidp22_1(&self) -> bool {
    *self == PIDP22_A::PIDP22_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP23_A {
  #[doc = "0: Bus master 23 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP23_0,
  #[doc = "1: Bus master 23 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP23_1,
}
impl From<PIDP23_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP23_A) -> Self {
    match variant {
      PIDP23_A::PIDP23_0 => false,
      PIDP23_A::PIDP23_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP23`"]
pub type PIDP23_R = crate::R<bool, PIDP23_A>;
impl PIDP23_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP23_A {
    match self.bits {
      false => PIDP23_A::PIDP23_0,
      true => PIDP23_A::PIDP23_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP23_0`"]
  #[inline(always)]
  pub fn is_pidp23_0(&self) -> bool {
    *self == PIDP23_A::PIDP23_0
  }
  #[doc = "Checks if the value of the field is `PIDP23_1`"]
  #[inline(always)]
  pub fn is_pidp23_1(&self) -> bool {
    *self == PIDP23_A::PIDP23_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP24_A {
  #[doc = "0: Bus master 24 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP24_0,
  #[doc = "1: Bus master 24 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP24_1,
}
impl From<PIDP24_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP24_A) -> Self {
    match variant {
      PIDP24_A::PIDP24_0 => false,
      PIDP24_A::PIDP24_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP24`"]
pub type PIDP24_R = crate::R<bool, PIDP24_A>;
impl PIDP24_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP24_A {
    match self.bits {
      false => PIDP24_A::PIDP24_0,
      true => PIDP24_A::PIDP24_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP24_0`"]
  #[inline(always)]
  pub fn is_pidp24_0(&self) -> bool {
    *self == PIDP24_A::PIDP24_0
  }
  #[doc = "Checks if the value of the field is `PIDP24_1`"]
  #[inline(always)]
  pub fn is_pidp24_1(&self) -> bool {
    *self == PIDP24_A::PIDP24_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP25_A {
  #[doc = "0: Bus master 25 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP25_0,
  #[doc = "1: Bus master 25 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP25_1,
}
impl From<PIDP25_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP25_A) -> Self {
    match variant {
      PIDP25_A::PIDP25_0 => false,
      PIDP25_A::PIDP25_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP25`"]
pub type PIDP25_R = crate::R<bool, PIDP25_A>;
impl PIDP25_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP25_A {
    match self.bits {
      false => PIDP25_A::PIDP25_0,
      true => PIDP25_A::PIDP25_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP25_0`"]
  #[inline(always)]
  pub fn is_pidp25_0(&self) -> bool {
    *self == PIDP25_A::PIDP25_0
  }
  #[doc = "Checks if the value of the field is `PIDP25_1`"]
  #[inline(always)]
  pub fn is_pidp25_1(&self) -> bool {
    *self == PIDP25_A::PIDP25_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP26_A {
  #[doc = "0: Bus master 26 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP26_0,
  #[doc = "1: Bus master 26 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP26_1,
}
impl From<PIDP26_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP26_A) -> Self {
    match variant {
      PIDP26_A::PIDP26_0 => false,
      PIDP26_A::PIDP26_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP26`"]
pub type PIDP26_R = crate::R<bool, PIDP26_A>;
impl PIDP26_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP26_A {
    match self.bits {
      false => PIDP26_A::PIDP26_0,
      true => PIDP26_A::PIDP26_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP26_0`"]
  #[inline(always)]
  pub fn is_pidp26_0(&self) -> bool {
    *self == PIDP26_A::PIDP26_0
  }
  #[doc = "Checks if the value of the field is `PIDP26_1`"]
  #[inline(always)]
  pub fn is_pidp26_1(&self) -> bool {
    *self == PIDP26_A::PIDP26_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP27_A {
  #[doc = "0: Bus master 27 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP27_0,
  #[doc = "1: Bus master 27 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP27_1,
}
impl From<PIDP27_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP27_A) -> Self {
    match variant {
      PIDP27_A::PIDP27_0 => false,
      PIDP27_A::PIDP27_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP27`"]
pub type PIDP27_R = crate::R<bool, PIDP27_A>;
impl PIDP27_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP27_A {
    match self.bits {
      false => PIDP27_A::PIDP27_0,
      true => PIDP27_A::PIDP27_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP27_0`"]
  #[inline(always)]
  pub fn is_pidp27_0(&self) -> bool {
    *self == PIDP27_A::PIDP27_0
  }
  #[doc = "Checks if the value of the field is `PIDP27_1`"]
  #[inline(always)]
  pub fn is_pidp27_1(&self) -> bool {
    *self == PIDP27_A::PIDP27_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP28_A {
  #[doc = "0: Bus master 28 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP28_0,
  #[doc = "1: Bus master 28 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP28_1,
}
impl From<PIDP28_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP28_A) -> Self {
    match variant {
      PIDP28_A::PIDP28_0 => false,
      PIDP28_A::PIDP28_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP28`"]
pub type PIDP28_R = crate::R<bool, PIDP28_A>;
impl PIDP28_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP28_A {
    match self.bits {
      false => PIDP28_A::PIDP28_0,
      true => PIDP28_A::PIDP28_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP28_0`"]
  #[inline(always)]
  pub fn is_pidp28_0(&self) -> bool {
    *self == PIDP28_A::PIDP28_0
  }
  #[doc = "Checks if the value of the field is `PIDP28_1`"]
  #[inline(always)]
  pub fn is_pidp28_1(&self) -> bool {
    *self == PIDP28_A::PIDP28_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP29_A {
  #[doc = "0: Bus master 29 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP29_0,
  #[doc = "1: Bus master 29 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP29_1,
}
impl From<PIDP29_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP29_A) -> Self {
    match variant {
      PIDP29_A::PIDP29_0 => false,
      PIDP29_A::PIDP29_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP29`"]
pub type PIDP29_R = crate::R<bool, PIDP29_A>;
impl PIDP29_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP29_A {
    match self.bits {
      false => PIDP29_A::PIDP29_0,
      true => PIDP29_A::PIDP29_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP29_0`"]
  #[inline(always)]
  pub fn is_pidp29_0(&self) -> bool {
    *self == PIDP29_A::PIDP29_0
  }
  #[doc = "Checks if the value of the field is `PIDP29_1`"]
  #[inline(always)]
  pub fn is_pidp29_1(&self) -> bool {
    *self == PIDP29_A::PIDP29_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP30_A {
  #[doc = "0: Bus master 30 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP30_0,
  #[doc = "1: Bus master 30 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP30_1,
}
impl From<PIDP30_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP30_A) -> Self {
    match variant {
      PIDP30_A::PIDP30_0 => false,
      PIDP30_A::PIDP30_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP30`"]
pub type PIDP30_R = crate::R<bool, PIDP30_A>;
impl PIDP30_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP30_A {
    match self.bits {
      false => PIDP30_A::PIDP30_0,
      true => PIDP30_A::PIDP30_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP30_0`"]
  #[inline(always)]
  pub fn is_pidp30_0(&self) -> bool {
    *self == PIDP30_A::PIDP30_0
  }
  #[doc = "Checks if the value of the field is `PIDP30_1`"]
  #[inline(always)]
  pub fn is_pidp30_1(&self) -> bool {
    *self == PIDP30_A::PIDP30_1
  }
}
#[doc = "Process identifier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIDP31_A {
  #[doc = "0: Bus master 31 does not source a process identifier register. The XRDC_MDAC logic provides the needed PID for processor cores."]
  PIDP31_0,
  #[doc = "1: Bus master 31 sources a process identifier register to the XRDC_MDAC logic."]
  PIDP31_1,
}
impl From<PIDP31_A> for bool {
  #[inline(always)]
  fn from(variant: PIDP31_A) -> Self {
    match variant {
      PIDP31_A::PIDP31_0 => false,
      PIDP31_A::PIDP31_1 => true,
    }
  }
}
#[doc = "Reader of field `PIDP31`"]
pub type PIDP31_R = crate::R<bool, PIDP31_A>;
impl PIDP31_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PIDP31_A {
    match self.bits {
      false => PIDP31_A::PIDP31_0,
      true => PIDP31_A::PIDP31_1,
    }
  }
  #[doc = "Checks if the value of the field is `PIDP31_0`"]
  #[inline(always)]
  pub fn is_pidp31_0(&self) -> bool {
    *self == PIDP31_A::PIDP31_0
  }
  #[doc = "Checks if the value of the field is `PIDP31_1`"]
  #[inline(always)]
  pub fn is_pidp31_1(&self) -> bool {
    *self == PIDP31_A::PIDP31_1
  }
}
impl R {
  #[doc = "Bit 0 - Process identifier"]
  #[inline(always)]
  pub fn pidp0(&self) -> PIDP0_R {
    PIDP0_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Process identifier"]
  #[inline(always)]
  pub fn pidp1(&self) -> PIDP1_R {
    PIDP1_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Process identifier"]
  #[inline(always)]
  pub fn pidp2(&self) -> PIDP2_R {
    PIDP2_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Process identifier"]
  #[inline(always)]
  pub fn pidp3(&self) -> PIDP3_R {
    PIDP3_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Process identifier"]
  #[inline(always)]
  pub fn pidp4(&self) -> PIDP4_R {
    PIDP4_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Process identifier"]
  #[inline(always)]
  pub fn pidp5(&self) -> PIDP5_R {
    PIDP5_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Process identifier"]
  #[inline(always)]
  pub fn pidp6(&self) -> PIDP6_R {
    PIDP6_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Process identifier"]
  #[inline(always)]
  pub fn pidp7(&self) -> PIDP7_R {
    PIDP7_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bit 8 - Process identifier"]
  #[inline(always)]
  pub fn pidp8(&self) -> PIDP8_R {
    PIDP8_R::new(((self.bits >> 8) & 0x01) != 0)
  }
  #[doc = "Bit 9 - Process identifier"]
  #[inline(always)]
  pub fn pidp9(&self) -> PIDP9_R {
    PIDP9_R::new(((self.bits >> 9) & 0x01) != 0)
  }
  #[doc = "Bit 10 - Process identifier"]
  #[inline(always)]
  pub fn pidp10(&self) -> PIDP10_R {
    PIDP10_R::new(((self.bits >> 10) & 0x01) != 0)
  }
  #[doc = "Bit 11 - Process identifier"]
  #[inline(always)]
  pub fn pidp11(&self) -> PIDP11_R {
    PIDP11_R::new(((self.bits >> 11) & 0x01) != 0)
  }
  #[doc = "Bit 12 - Process identifier"]
  #[inline(always)]
  pub fn pidp12(&self) -> PIDP12_R {
    PIDP12_R::new(((self.bits >> 12) & 0x01) != 0)
  }
  #[doc = "Bit 13 - Process identifier"]
  #[inline(always)]
  pub fn pidp13(&self) -> PIDP13_R {
    PIDP13_R::new(((self.bits >> 13) & 0x01) != 0)
  }
  #[doc = "Bit 14 - Process identifier"]
  #[inline(always)]
  pub fn pidp14(&self) -> PIDP14_R {
    PIDP14_R::new(((self.bits >> 14) & 0x01) != 0)
  }
  #[doc = "Bit 15 - Process identifier"]
  #[inline(always)]
  pub fn pidp15(&self) -> PIDP15_R {
    PIDP15_R::new(((self.bits >> 15) & 0x01) != 0)
  }
  #[doc = "Bit 16 - Process identifier"]
  #[inline(always)]
  pub fn pidp16(&self) -> PIDP16_R {
    PIDP16_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Process identifier"]
  #[inline(always)]
  pub fn pidp17(&self) -> PIDP17_R {
    PIDP17_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - Process identifier"]
  #[inline(always)]
  pub fn pidp18(&self) -> PIDP18_R {
    PIDP18_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 19 - Process identifier"]
  #[inline(always)]
  pub fn pidp19(&self) -> PIDP19_R {
    PIDP19_R::new(((self.bits >> 19) & 0x01) != 0)
  }
  #[doc = "Bit 20 - Process identifier"]
  #[inline(always)]
  pub fn pidp20(&self) -> PIDP20_R {
    PIDP20_R::new(((self.bits >> 20) & 0x01) != 0)
  }
  #[doc = "Bit 21 - Process identifier"]
  #[inline(always)]
  pub fn pidp21(&self) -> PIDP21_R {
    PIDP21_R::new(((self.bits >> 21) & 0x01) != 0)
  }
  #[doc = "Bit 22 - Process identifier"]
  #[inline(always)]
  pub fn pidp22(&self) -> PIDP22_R {
    PIDP22_R::new(((self.bits >> 22) & 0x01) != 0)
  }
  #[doc = "Bit 23 - Process identifier"]
  #[inline(always)]
  pub fn pidp23(&self) -> PIDP23_R {
    PIDP23_R::new(((self.bits >> 23) & 0x01) != 0)
  }
  #[doc = "Bit 24 - Process identifier"]
  #[inline(always)]
  pub fn pidp24(&self) -> PIDP24_R {
    PIDP24_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - Process identifier"]
  #[inline(always)]
  pub fn pidp25(&self) -> PIDP25_R {
    PIDP25_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - Process identifier"]
  #[inline(always)]
  pub fn pidp26(&self) -> PIDP26_R {
    PIDP26_R::new(((self.bits >> 26) & 0x01) != 0)
  }
  #[doc = "Bit 27 - Process identifier"]
  #[inline(always)]
  pub fn pidp27(&self) -> PIDP27_R {
    PIDP27_R::new(((self.bits >> 27) & 0x01) != 0)
  }
  #[doc = "Bit 28 - Process identifier"]
  #[inline(always)]
  pub fn pidp28(&self) -> PIDP28_R {
    PIDP28_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Process identifier"]
  #[inline(always)]
  pub fn pidp29(&self) -> PIDP29_R {
    PIDP29_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Process identifier"]
  #[inline(always)]
  pub fn pidp30(&self) -> PIDP30_R {
    PIDP30_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Process identifier"]
  #[inline(always)]
  pub fn pidp31(&self) -> PIDP31_R {
    PIDP31_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
