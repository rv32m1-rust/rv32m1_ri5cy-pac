#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::CR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x8000_0000
  }
}
#[doc = "Task completion with software error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCSEIE_A {
  #[doc = "0: Disables task completion with software error to generate an interrupt request"]
  TCSEIE_0,
  #[doc = "1: Enables task completion with software error to generate an interrupt request"]
  TCSEIE_1,
}
impl From<TCSEIE_A> for bool {
  #[inline(always)]
  fn from(variant: TCSEIE_A) -> Self {
    match variant {
      TCSEIE_A::TCSEIE_0 => false,
      TCSEIE_A::TCSEIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TCSEIE`"]
pub type TCSEIE_R = crate::R<bool, TCSEIE_A>;
impl TCSEIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TCSEIE_A {
    match self.bits {
      false => TCSEIE_A::TCSEIE_0,
      true => TCSEIE_A::TCSEIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TCSEIE_0`"]
  #[inline(always)]
  pub fn is_tcseie_0(&self) -> bool {
    *self == TCSEIE_A::TCSEIE_0
  }
  #[doc = "Checks if the value of the field is `TCSEIE_1`"]
  #[inline(always)]
  pub fn is_tcseie_1(&self) -> bool {
    *self == TCSEIE_A::TCSEIE_1
  }
}
#[doc = "Write proxy for field `TCSEIE`"]
pub struct TCSEIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TCSEIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCSEIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables task completion with software error to generate an interrupt request"]
  #[inline(always)]
  pub fn tcseie_0(self) -> &'a mut W {
    self.variant(TCSEIE_A::TCSEIE_0)
  }
  #[doc = "Enables task completion with software error to generate an interrupt request"]
  #[inline(always)]
  pub fn tcseie_1(self) -> &'a mut W {
    self.variant(TCSEIE_A::TCSEIE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
    self.w
  }
}
#[doc = "Illegal Instruction Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILLIE_A {
  #[doc = "0: Illegal instruction interrupt requests are disabled"]
  ILLIE_0,
  #[doc = "1: illegal Instruction interrupt requests are enabled"]
  ILLIE_1,
}
impl From<ILLIE_A> for bool {
  #[inline(always)]
  fn from(variant: ILLIE_A) -> Self {
    match variant {
      ILLIE_A::ILLIE_0 => false,
      ILLIE_A::ILLIE_1 => true,
    }
  }
}
#[doc = "Reader of field `ILLIE`"]
pub type ILLIE_R = crate::R<bool, ILLIE_A>;
impl ILLIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ILLIE_A {
    match self.bits {
      false => ILLIE_A::ILLIE_0,
      true => ILLIE_A::ILLIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `ILLIE_0`"]
  #[inline(always)]
  pub fn is_illie_0(&self) -> bool {
    *self == ILLIE_A::ILLIE_0
  }
  #[doc = "Checks if the value of the field is `ILLIE_1`"]
  #[inline(always)]
  pub fn is_illie_1(&self) -> bool {
    *self == ILLIE_A::ILLIE_1
  }
}
#[doc = "Write proxy for field `ILLIE`"]
pub struct ILLIE_W<'a> {
  w: &'a mut W,
}
impl<'a> ILLIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ILLIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Illegal instruction interrupt requests are disabled"]
  #[inline(always)]
  pub fn illie_0(self) -> &'a mut W {
    self.variant(ILLIE_A::ILLIE_0)
  }
  #[doc = "illegal Instruction interrupt requests are enabled"]
  #[inline(always)]
  pub fn illie_1(self) -> &'a mut W {
    self.variant(ILLIE_A::ILLIE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
    self.w
  }
}
#[doc = "AHB Slave Response Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASREIE_A {
  #[doc = "0: AHB slave response error interruption is not enabled"]
  ASREIE_0,
  #[doc = "1: AHB slave response error interruption is enabled"]
  ASREIE_1,
}
impl From<ASREIE_A> for bool {
  #[inline(always)]
  fn from(variant: ASREIE_A) -> Self {
    match variant {
      ASREIE_A::ASREIE_0 => false,
      ASREIE_A::ASREIE_1 => true,
    }
  }
}
#[doc = "Reader of field `ASREIE`"]
pub type ASREIE_R = crate::R<bool, ASREIE_A>;
impl ASREIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ASREIE_A {
    match self.bits {
      false => ASREIE_A::ASREIE_0,
      true => ASREIE_A::ASREIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `ASREIE_0`"]
  #[inline(always)]
  pub fn is_asreie_0(&self) -> bool {
    *self == ASREIE_A::ASREIE_0
  }
  #[doc = "Checks if the value of the field is `ASREIE_1`"]
  #[inline(always)]
  pub fn is_asreie_1(&self) -> bool {
    *self == ASREIE_A::ASREIE_1
  }
}
#[doc = "Write proxy for field `ASREIE`"]
pub struct ASREIE_W<'a> {
  w: &'a mut W,
}
impl<'a> ASREIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ASREIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "AHB slave response error interruption is not enabled"]
  #[inline(always)]
  pub fn asreie_0(self) -> &'a mut W {
    self.variant(ASREIE_A::ASREIE_0)
  }
  #[doc = "AHB slave response error interruption is enabled"]
  #[inline(always)]
  pub fn asreie_1(self) -> &'a mut W {
    self.variant(ASREIE_A::ASREIE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
    self.w
  }
}
#[doc = "IMEM Illegal Address Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IIADIE_A {
  #[doc = "0: IMEM illegal address interruption is not enabled"]
  IIADIE_0,
  #[doc = "1: IMEM illegal address interruption is enabled"]
  IIADIE_1,
}
impl From<IIADIE_A> for bool {
  #[inline(always)]
  fn from(variant: IIADIE_A) -> Self {
    match variant {
      IIADIE_A::IIADIE_0 => false,
      IIADIE_A::IIADIE_1 => true,
    }
  }
}
#[doc = "Reader of field `IIADIE`"]
pub type IIADIE_R = crate::R<bool, IIADIE_A>;
impl IIADIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IIADIE_A {
    match self.bits {
      false => IIADIE_A::IIADIE_0,
      true => IIADIE_A::IIADIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `IIADIE_0`"]
  #[inline(always)]
  pub fn is_iiadie_0(&self) -> bool {
    *self == IIADIE_A::IIADIE_0
  }
  #[doc = "Checks if the value of the field is `IIADIE_1`"]
  #[inline(always)]
  pub fn is_iiadie_1(&self) -> bool {
    *self == IIADIE_A::IIADIE_1
  }
}
#[doc = "Write proxy for field `IIADIE`"]
pub struct IIADIE_W<'a> {
  w: &'a mut W,
}
impl<'a> IIADIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IIADIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "IMEM illegal address interruption is not enabled"]
  #[inline(always)]
  pub fn iiadie_0(self) -> &'a mut W {
    self.variant(IIADIE_A::IIADIE_0)
  }
  #[doc = "IMEM illegal address interruption is enabled"]
  #[inline(always)]
  pub fn iiadie_1(self) -> &'a mut W {
    self.variant(IIADIE_A::IIADIE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
#[doc = "DMEM Illegal Address Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIADIE_A {
  #[doc = "0: DMEM illegal address interruption is not enabled"]
  DIADIE_0,
  #[doc = "1: DMEM illegal address interruption is enabled"]
  DIADIE_1,
}
impl From<DIADIE_A> for bool {
  #[inline(always)]
  fn from(variant: DIADIE_A) -> Self {
    match variant {
      DIADIE_A::DIADIE_0 => false,
      DIADIE_A::DIADIE_1 => true,
    }
  }
}
#[doc = "Reader of field `DIADIE`"]
pub type DIADIE_R = crate::R<bool, DIADIE_A>;
impl DIADIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DIADIE_A {
    match self.bits {
      false => DIADIE_A::DIADIE_0,
      true => DIADIE_A::DIADIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `DIADIE_0`"]
  #[inline(always)]
  pub fn is_diadie_0(&self) -> bool {
    *self == DIADIE_A::DIADIE_0
  }
  #[doc = "Checks if the value of the field is `DIADIE_1`"]
  #[inline(always)]
  pub fn is_diadie_1(&self) -> bool {
    *self == DIADIE_A::DIADIE_1
  }
}
#[doc = "Write proxy for field `DIADIE`"]
pub struct DIADIE_W<'a> {
  w: &'a mut W,
}
impl<'a> DIADIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DIADIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "DMEM illegal address interruption is not enabled"]
  #[inline(always)]
  pub fn diadie_0(self) -> &'a mut W {
    self.variant(DIADIE_A::DIADIE_0)
  }
  #[doc = "DMEM illegal address interruption is enabled"]
  #[inline(always)]
  pub fn diadie_1(self) -> &'a mut W {
    self.variant(DIADIE_A::DIADIE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
#[doc = "Security Violation Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVIE_A {
  #[doc = "0: Security violation interruption is not enabled"]
  SVIE_0,
  #[doc = "1: Security violation interruption is enabled"]
  SVIE_1,
}
impl From<SVIE_A> for bool {
  #[inline(always)]
  fn from(variant: SVIE_A) -> Self {
    match variant {
      SVIE_A::SVIE_0 => false,
      SVIE_A::SVIE_1 => true,
    }
  }
}
#[doc = "Reader of field `SVIE`"]
pub type SVIE_R = crate::R<bool, SVIE_A>;
impl SVIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SVIE_A {
    match self.bits {
      false => SVIE_A::SVIE_0,
      true => SVIE_A::SVIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `SVIE_0`"]
  #[inline(always)]
  pub fn is_svie_0(&self) -> bool {
    *self == SVIE_A::SVIE_0
  }
  #[doc = "Checks if the value of the field is `SVIE_1`"]
  #[inline(always)]
  pub fn is_svie_1(&self) -> bool {
    *self == SVIE_A::SVIE_1
  }
}
#[doc = "Write proxy for field `SVIE`"]
pub struct SVIE_W<'a> {
  w: &'a mut W,
}
impl<'a> SVIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SVIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Security violation interruption is not enabled"]
  #[inline(always)]
  pub fn svie_0(self) -> &'a mut W {
    self.variant(SVIE_A::SVIE_0)
  }
  #[doc = "Security violation interruption is enabled"]
  #[inline(always)]
  pub fn svie_1(self) -> &'a mut W {
    self.variant(SVIE_A::SVIE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
#[doc = "Task completion with no error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
  #[doc = "0: Disables task completion with no error to generate an interrupt request"]
  TCIE_0,
  #[doc = "1: Enables task completion with no error to generate an interrupt request"]
  TCIE_1,
}
impl From<TCIE_A> for bool {
  #[inline(always)]
  fn from(variant: TCIE_A) -> Self {
    match variant {
      TCIE_A::TCIE_0 => false,
      TCIE_A::TCIE_1 => true,
    }
  }
}
#[doc = "Reader of field `TCIE`"]
pub type TCIE_R = crate::R<bool, TCIE_A>;
impl TCIE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TCIE_A {
    match self.bits {
      false => TCIE_A::TCIE_0,
      true => TCIE_A::TCIE_1,
    }
  }
  #[doc = "Checks if the value of the field is `TCIE_0`"]
  #[inline(always)]
  pub fn is_tcie_0(&self) -> bool {
    *self == TCIE_A::TCIE_0
  }
  #[doc = "Checks if the value of the field is `TCIE_1`"]
  #[inline(always)]
  pub fn is_tcie_1(&self) -> bool {
    *self == TCIE_A::TCIE_1
  }
}
#[doc = "Write proxy for field `TCIE`"]
pub struct TCIE_W<'a> {
  w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCIE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disables task completion with no error to generate an interrupt request"]
  #[inline(always)]
  pub fn tcie_0(self) -> &'a mut W {
    self.variant(TCIE_A::TCIE_0)
  }
  #[doc = "Enables task completion with no error to generate an interrupt request"]
  #[inline(always)]
  pub fn tcie_1(self) -> &'a mut W {
    self.variant(TCIE_A::TCIE_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
    self.w
  }
}
#[doc = "Reset Semaphore\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTSM4_A {
  #[doc = "0: Idle state"]
  RSTSM4_0,
  #[doc = "1: Wait for second write"]
  RSTSM4_1,
  #[doc = "2: Clears semaphore if previous state was \"01\""]
  RSTSM4_2,
}
impl From<RSTSM4_A> for u8 {
  #[inline(always)]
  fn from(variant: RSTSM4_A) -> Self {
    match variant {
      RSTSM4_A::RSTSM4_0 => 0,
      RSTSM4_A::RSTSM4_1 => 1,
      RSTSM4_A::RSTSM4_2 => 2,
    }
  }
}
#[doc = "Reader of field `RSTSM4`"]
pub type RSTSM4_R = crate::R<u8, RSTSM4_A>;
impl RSTSM4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RSTSM4_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(RSTSM4_A::RSTSM4_0),
      1 => Val(RSTSM4_A::RSTSM4_1),
      2 => Val(RSTSM4_A::RSTSM4_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RSTSM4_0`"]
  #[inline(always)]
  pub fn is_rstsm4_0(&self) -> bool {
    *self == RSTSM4_A::RSTSM4_0
  }
  #[doc = "Checks if the value of the field is `RSTSM4_1`"]
  #[inline(always)]
  pub fn is_rstsm4_1(&self) -> bool {
    *self == RSTSM4_A::RSTSM4_1
  }
  #[doc = "Checks if the value of the field is `RSTSM4_2`"]
  #[inline(always)]
  pub fn is_rstsm4_2(&self) -> bool {
    *self == RSTSM4_A::RSTSM4_2
  }
}
#[doc = "Write proxy for field `RSTSM4`"]
pub struct RSTSM4_W<'a> {
  w: &'a mut W,
}
impl<'a> RSTSM4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RSTSM4_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Idle state"]
  #[inline(always)]
  pub fn rstsm4_0(self) -> &'a mut W {
    self.variant(RSTSM4_A::RSTSM4_0)
  }
  #[doc = "Wait for second write"]
  #[inline(always)]
  pub fn rstsm4_1(self) -> &'a mut W {
    self.variant(RSTSM4_A::RSTSM4_1)
  }
  #[doc = "Clears semaphore if previous state was \"01\""]
  #[inline(always)]
  pub fn rstsm4_2(self) -> &'a mut W {
    self.variant(RSTSM4_A::RSTSM4_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
    self.w
  }
}
#[doc = "Module Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRST_AW {
  #[doc = "0: no action"]
  MRST_0,
  #[doc = "1: reset"]
  MRST_1,
}
impl From<MRST_AW> for bool {
  #[inline(always)]
  fn from(variant: MRST_AW) -> Self {
    match variant {
      MRST_AW::MRST_0 => false,
      MRST_AW::MRST_1 => true,
    }
  }
}
#[doc = "Write proxy for field `MRST`"]
pub struct MRST_W<'a> {
  w: &'a mut W,
}
impl<'a> MRST_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MRST_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no action"]
  #[inline(always)]
  pub fn mrst_0(self) -> &'a mut W {
    self.variant(MRST_AW::MRST_0)
  }
  #[doc = "reset"]
  #[inline(always)]
  pub fn mrst_1(self) -> &'a mut W {
    self.variant(MRST_AW::MRST_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
    self.w
  }
}
#[doc = "Force Security Violation Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSV_A {
  #[doc = "0: no violation is forced"]
  FSV_0,
  #[doc = "1: force security violation"]
  FSV_1,
}
impl From<FSV_A> for bool {
  #[inline(always)]
  fn from(variant: FSV_A) -> Self {
    match variant {
      FSV_A::FSV_0 => false,
      FSV_A::FSV_1 => true,
    }
  }
}
#[doc = "Reader of field `FSV`"]
pub type FSV_R = crate::R<bool, FSV_A>;
impl FSV_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FSV_A {
    match self.bits {
      false => FSV_A::FSV_0,
      true => FSV_A::FSV_1,
    }
  }
  #[doc = "Checks if the value of the field is `FSV_0`"]
  #[inline(always)]
  pub fn is_fsv_0(&self) -> bool {
    *self == FSV_A::FSV_0
  }
  #[doc = "Checks if the value of the field is `FSV_1`"]
  #[inline(always)]
  pub fn is_fsv_1(&self) -> bool {
    *self == FSV_A::FSV_1
  }
}
#[doc = "Write proxy for field `FSV`"]
pub struct FSV_W<'a> {
  w: &'a mut W,
}
impl<'a> FSV_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FSV_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no violation is forced"]
  #[inline(always)]
  pub fn fsv_0(self) -> &'a mut W {
    self.variant(FSV_A::FSV_0)
  }
  #[doc = "force security violation"]
  #[inline(always)]
  pub fn fsv_1(self) -> &'a mut W {
    self.variant(FSV_A::FSV_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
#[doc = "Default Task Completion Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTCCFG_A {
  #[doc = "0: no explicit action"]
  DTCCFG_0,
  #[doc = "1: Issue an Interrupt Request"]
  DTCCFG_1,
  #[doc = "2: Assert Event Completion Signal"]
  DTCCFG_2,
  #[doc = "4: Issue a DMA request"]
  DTCCFG_4,
}
impl From<DTCCFG_A> for u8 {
  #[inline(always)]
  fn from(variant: DTCCFG_A) -> Self {
    match variant {
      DTCCFG_A::DTCCFG_0 => 0,
      DTCCFG_A::DTCCFG_1 => 1,
      DTCCFG_A::DTCCFG_2 => 2,
      DTCCFG_A::DTCCFG_4 => 4,
    }
  }
}
#[doc = "Reader of field `DTCCFG`"]
pub type DTCCFG_R = crate::R<u8, DTCCFG_A>;
impl DTCCFG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, DTCCFG_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(DTCCFG_A::DTCCFG_0),
      1 => Val(DTCCFG_A::DTCCFG_1),
      2 => Val(DTCCFG_A::DTCCFG_2),
      4 => Val(DTCCFG_A::DTCCFG_4),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DTCCFG_0`"]
  #[inline(always)]
  pub fn is_dtccfg_0(&self) -> bool {
    *self == DTCCFG_A::DTCCFG_0
  }
  #[doc = "Checks if the value of the field is `DTCCFG_1`"]
  #[inline(always)]
  pub fn is_dtccfg_1(&self) -> bool {
    *self == DTCCFG_A::DTCCFG_1
  }
  #[doc = "Checks if the value of the field is `DTCCFG_2`"]
  #[inline(always)]
  pub fn is_dtccfg_2(&self) -> bool {
    *self == DTCCFG_A::DTCCFG_2
  }
  #[doc = "Checks if the value of the field is `DTCCFG_4`"]
  #[inline(always)]
  pub fn is_dtccfg_4(&self) -> bool {
    *self == DTCCFG_A::DTCCFG_4
  }
}
#[doc = "Write proxy for field `DTCCFG`"]
pub struct DTCCFG_W<'a> {
  w: &'a mut W,
}
impl<'a> DTCCFG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DTCCFG_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no explicit action"]
  #[inline(always)]
  pub fn dtccfg_0(self) -> &'a mut W {
    self.variant(DTCCFG_A::DTCCFG_0)
  }
  #[doc = "Issue an Interrupt Request"]
  #[inline(always)]
  pub fn dtccfg_1(self) -> &'a mut W {
    self.variant(DTCCFG_A::DTCCFG_1)
  }
  #[doc = "Assert Event Completion Signal"]
  #[inline(always)]
  pub fn dtccfg_2(self) -> &'a mut W {
    self.variant(DTCCFG_A::DTCCFG_2)
  }
  #[doc = "Issue a DMA request"]
  #[inline(always)]
  pub fn dtccfg_4(self) -> &'a mut W {
    self.variant(DTCCFG_A::DTCCFG_4)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
    self.w
  }
}
#[doc = "Disable Secure Hash Function Instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSHFI_A {
  #[doc = "0: Secure Hash Functions are enabled"]
  DSHFI_0,
  #[doc = "1: Secure Hash Functions are disabled"]
  DSHFI_1,
}
impl From<DSHFI_A> for bool {
  #[inline(always)]
  fn from(variant: DSHFI_A) -> Self {
    match variant {
      DSHFI_A::DSHFI_0 => false,
      DSHFI_A::DSHFI_1 => true,
    }
  }
}
#[doc = "Reader of field `DSHFI`"]
pub type DSHFI_R = crate::R<bool, DSHFI_A>;
impl DSHFI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DSHFI_A {
    match self.bits {
      false => DSHFI_A::DSHFI_0,
      true => DSHFI_A::DSHFI_1,
    }
  }
  #[doc = "Checks if the value of the field is `DSHFI_0`"]
  #[inline(always)]
  pub fn is_dshfi_0(&self) -> bool {
    *self == DSHFI_A::DSHFI_0
  }
  #[doc = "Checks if the value of the field is `DSHFI_1`"]
  #[inline(always)]
  pub fn is_dshfi_1(&self) -> bool {
    *self == DSHFI_A::DSHFI_1
  }
}
#[doc = "Write proxy for field `DSHFI`"]
pub struct DSHFI_W<'a> {
  w: &'a mut W,
}
impl<'a> DSHFI_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DSHFI_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Secure Hash Functions are enabled"]
  #[inline(always)]
  pub fn dshfi_0(self) -> &'a mut W {
    self.variant(DSHFI_A::DSHFI_0)
  }
  #[doc = "Secure Hash Functions are disabled"]
  #[inline(always)]
  pub fn dshfi_1(self) -> &'a mut W {
    self.variant(DSHFI_A::DSHFI_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
    self.w
  }
}
#[doc = "Disable DES Instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDESI_A {
  #[doc = "0: DES instructions are enabled"]
  DDESI_0,
  #[doc = "1: DES instructions are disabled"]
  DDESI_1,
}
impl From<DDESI_A> for bool {
  #[inline(always)]
  fn from(variant: DDESI_A) -> Self {
    match variant {
      DDESI_A::DDESI_0 => false,
      DDESI_A::DDESI_1 => true,
    }
  }
}
#[doc = "Reader of field `DDESI`"]
pub type DDESI_R = crate::R<bool, DDESI_A>;
impl DDESI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DDESI_A {
    match self.bits {
      false => DDESI_A::DDESI_0,
      true => DDESI_A::DDESI_1,
    }
  }
  #[doc = "Checks if the value of the field is `DDESI_0`"]
  #[inline(always)]
  pub fn is_ddesi_0(&self) -> bool {
    *self == DDESI_A::DDESI_0
  }
  #[doc = "Checks if the value of the field is `DDESI_1`"]
  #[inline(always)]
  pub fn is_ddesi_1(&self) -> bool {
    *self == DDESI_A::DDESI_1
  }
}
#[doc = "Write proxy for field `DDESI`"]
pub struct DDESI_W<'a> {
  w: &'a mut W,
}
impl<'a> DDESI_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DDESI_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "DES instructions are enabled"]
  #[inline(always)]
  pub fn ddesi_0(self) -> &'a mut W {
    self.variant(DDESI_A::DDESI_0)
  }
  #[doc = "DES instructions are disabled"]
  #[inline(always)]
  pub fn ddesi_1(self) -> &'a mut W {
    self.variant(DDESI_A::DDESI_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
    self.w
  }
}
#[doc = "Disable AES Instructions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAESI_A {
  #[doc = "0: AES instructions are enabled"]
  DAESI_0,
  #[doc = "1: AES instructions are disabled"]
  DAESI_1,
}
impl From<DAESI_A> for bool {
  #[inline(always)]
  fn from(variant: DAESI_A) -> Self {
    match variant {
      DAESI_A::DAESI_0 => false,
      DAESI_A::DAESI_1 => true,
    }
  }
}
#[doc = "Reader of field `DAESI`"]
pub type DAESI_R = crate::R<bool, DAESI_A>;
impl DAESI_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DAESI_A {
    match self.bits {
      false => DAESI_A::DAESI_0,
      true => DAESI_A::DAESI_1,
    }
  }
  #[doc = "Checks if the value of the field is `DAESI_0`"]
  #[inline(always)]
  pub fn is_daesi_0(&self) -> bool {
    *self == DAESI_A::DAESI_0
  }
  #[doc = "Checks if the value of the field is `DAESI_1`"]
  #[inline(always)]
  pub fn is_daesi_1(&self) -> bool {
    *self == DAESI_A::DAESI_1
  }
}
#[doc = "Write proxy for field `DAESI`"]
pub struct DAESI_W<'a> {
  w: &'a mut W,
}
impl<'a> DAESI_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DAESI_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "AES instructions are enabled"]
  #[inline(always)]
  pub fn daesi_0(self) -> &'a mut W {
    self.variant(DAESI_A::DAESI_0)
  }
  #[doc = "AES instructions are disabled"]
  #[inline(always)]
  pub fn daesi_1(self) -> &'a mut W {
    self.variant(DAESI_A::DAESI_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
    self.w
  }
}
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
  #[doc = "0: CAU3 exits from low power mode"]
  MDIS_0,
  #[doc = "1: CAU3 enters low power mode"]
  MDIS_1,
}
impl From<MDIS_A> for bool {
  #[inline(always)]
  fn from(variant: MDIS_A) -> Self {
    match variant {
      MDIS_A::MDIS_0 => false,
      MDIS_A::MDIS_1 => true,
    }
  }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, MDIS_A>;
impl MDIS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MDIS_A {
    match self.bits {
      false => MDIS_A::MDIS_0,
      true => MDIS_A::MDIS_1,
    }
  }
  #[doc = "Checks if the value of the field is `MDIS_0`"]
  #[inline(always)]
  pub fn is_mdis_0(&self) -> bool {
    *self == MDIS_A::MDIS_0
  }
  #[doc = "Checks if the value of the field is `MDIS_1`"]
  #[inline(always)]
  pub fn is_mdis_1(&self) -> bool {
    *self == MDIS_A::MDIS_1
  }
}
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
  w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: MDIS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "CAU3 exits from low power mode"]
  #[inline(always)]
  pub fn mdis_0(self) -> &'a mut W {
    self.variant(MDIS_A::MDIS_0)
  }
  #[doc = "CAU3 enters low power mode"]
  #[inline(always)]
  pub fn mdis_1(self) -> &'a mut W {
    self.variant(MDIS_A::MDIS_1)
  }
  #[doc = r"Sets the field bit"]
  #[inline(always)]
  pub fn set_bit(self) -> &'a mut W {
    self.bit(true)
  }
  #[doc = r"Clears the field bit"]
  #[inline(always)]
  pub fn clear_bit(self) -> &'a mut W {
    self.bit(false)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bit(self, value: bool) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Task completion with software error interrupt enable"]
  #[inline(always)]
  pub fn tcseie(&self) -> TCSEIE_R {
    TCSEIE_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Illegal Instruction Interrupt Enable"]
  #[inline(always)]
  pub fn illie(&self) -> ILLIE_R {
    ILLIE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 3 - AHB Slave Response Error Interrupt Enable"]
  #[inline(always)]
  pub fn asreie(&self) -> ASREIE_R {
    ASREIE_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - IMEM Illegal Address Interrupt Enable"]
  #[inline(always)]
  pub fn iiadie(&self) -> IIADIE_R {
    IIADIE_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - DMEM Illegal Address Interrupt Enable"]
  #[inline(always)]
  pub fn diadie(&self) -> DIADIE_R {
    DIADIE_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Security Violation Interrupt Enable"]
  #[inline(always)]
  pub fn svie(&self) -> SVIE_R {
    SVIE_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Task completion with no error interrupt enable"]
  #[inline(always)]
  pub fn tcie(&self) -> TCIE_R {
    TCIE_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bits 12:13 - Reset Semaphore"]
  #[inline(always)]
  pub fn rstsm4(&self) -> RSTSM4_R {
    RSTSM4_R::new(((self.bits >> 12) & 0x03) as u8)
  }
  #[doc = "Bit 16 - Force Security Violation Test"]
  #[inline(always)]
  pub fn fsv(&self) -> FSV_R {
    FSV_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bits 24:26 - Default Task Completion Configuration"]
  #[inline(always)]
  pub fn dtccfg(&self) -> DTCCFG_R {
    DTCCFG_R::new(((self.bits >> 24) & 0x07) as u8)
  }
  #[doc = "Bit 28 - Disable Secure Hash Function Instructions"]
  #[inline(always)]
  pub fn dshfi(&self) -> DSHFI_R {
    DSHFI_R::new(((self.bits >> 28) & 0x01) != 0)
  }
  #[doc = "Bit 29 - Disable DES Instructions"]
  #[inline(always)]
  pub fn ddesi(&self) -> DDESI_R {
    DDESI_R::new(((self.bits >> 29) & 0x01) != 0)
  }
  #[doc = "Bit 30 - Disable AES Instructions"]
  #[inline(always)]
  pub fn daesi(&self) -> DAESI_R {
    DAESI_R::new(((self.bits >> 30) & 0x01) != 0)
  }
  #[doc = "Bit 31 - Module Disable"]
  #[inline(always)]
  pub fn mdis(&self) -> MDIS_R {
    MDIS_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Task completion with software error interrupt enable"]
  #[inline(always)]
  pub fn tcseie(&mut self) -> TCSEIE_W {
    TCSEIE_W { w: self }
  }
  #[doc = "Bit 1 - Illegal Instruction Interrupt Enable"]
  #[inline(always)]
  pub fn illie(&mut self) -> ILLIE_W {
    ILLIE_W { w: self }
  }
  #[doc = "Bit 3 - AHB Slave Response Error Interrupt Enable"]
  #[inline(always)]
  pub fn asreie(&mut self) -> ASREIE_W {
    ASREIE_W { w: self }
  }
  #[doc = "Bit 4 - IMEM Illegal Address Interrupt Enable"]
  #[inline(always)]
  pub fn iiadie(&mut self) -> IIADIE_W {
    IIADIE_W { w: self }
  }
  #[doc = "Bit 5 - DMEM Illegal Address Interrupt Enable"]
  #[inline(always)]
  pub fn diadie(&mut self) -> DIADIE_W {
    DIADIE_W { w: self }
  }
  #[doc = "Bit 6 - Security Violation Interrupt Enable"]
  #[inline(always)]
  pub fn svie(&mut self) -> SVIE_W {
    SVIE_W { w: self }
  }
  #[doc = "Bit 7 - Task completion with no error interrupt enable"]
  #[inline(always)]
  pub fn tcie(&mut self) -> TCIE_W {
    TCIE_W { w: self }
  }
  #[doc = "Bits 12:13 - Reset Semaphore"]
  #[inline(always)]
  pub fn rstsm4(&mut self) -> RSTSM4_W {
    RSTSM4_W { w: self }
  }
  #[doc = "Bit 15 - Module Reset"]
  #[inline(always)]
  pub fn mrst(&mut self) -> MRST_W {
    MRST_W { w: self }
  }
  #[doc = "Bit 16 - Force Security Violation Test"]
  #[inline(always)]
  pub fn fsv(&mut self) -> FSV_W {
    FSV_W { w: self }
  }
  #[doc = "Bits 24:26 - Default Task Completion Configuration"]
  #[inline(always)]
  pub fn dtccfg(&mut self) -> DTCCFG_W {
    DTCCFG_W { w: self }
  }
  #[doc = "Bit 28 - Disable Secure Hash Function Instructions"]
  #[inline(always)]
  pub fn dshfi(&mut self) -> DSHFI_W {
    DSHFI_W { w: self }
  }
  #[doc = "Bit 29 - Disable DES Instructions"]
  #[inline(always)]
  pub fn ddesi(&mut self) -> DDESI_W {
    DDESI_W { w: self }
  }
  #[doc = "Bit 30 - Disable AES Instructions"]
  #[inline(always)]
  pub fn daesi(&mut self) -> DAESI_W {
    DAESI_W { w: self }
  }
  #[doc = "Bit 31 - Module Disable"]
  #[inline(always)]
  pub fn mdis(&mut self) -> MDIS_W {
    MDIS_W { w: self }
  }
}
