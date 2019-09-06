#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0x0102_0900"]
impl crate::ResetValue for super::SR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0102_0900
  }
}
#[doc = "Task completion with software error interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCSEIRQ_A {
  #[doc = "0: Task not finished or finished with no software error"]
  TCSEIRQ_0,
  #[doc = "1: Task execution finished with software error"]
  TCSEIRQ_1,
}
impl From<TCSEIRQ_A> for bool {
  #[inline(always)]
  fn from(variant: TCSEIRQ_A) -> Self {
    match variant {
      TCSEIRQ_A::TCSEIRQ_0 => false,
      TCSEIRQ_A::TCSEIRQ_1 => true,
    }
  }
}
#[doc = "Reader of field `TCSEIRQ`"]
pub type TCSEIRQ_R = crate::R<bool, TCSEIRQ_A>;
impl TCSEIRQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TCSEIRQ_A {
    match self.bits {
      false => TCSEIRQ_A::TCSEIRQ_0,
      true => TCSEIRQ_A::TCSEIRQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `TCSEIRQ_0`"]
  #[inline(always)]
  pub fn is_tcseirq_0(&self) -> bool {
    *self == TCSEIRQ_A::TCSEIRQ_0
  }
  #[doc = "Checks if the value of the field is `TCSEIRQ_1`"]
  #[inline(always)]
  pub fn is_tcseirq_1(&self) -> bool {
    *self == TCSEIRQ_A::TCSEIRQ_1
  }
}
#[doc = "Write proxy for field `TCSEIRQ`"]
pub struct TCSEIRQ_W<'a> {
  w: &'a mut W,
}
impl<'a> TCSEIRQ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCSEIRQ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Task not finished or finished with no software error"]
  #[inline(always)]
  pub fn tcseirq_0(self) -> &'a mut W {
    self.variant(TCSEIRQ_A::TCSEIRQ_0)
  }
  #[doc = "Task execution finished with software error"]
  #[inline(always)]
  pub fn tcseirq_1(self) -> &'a mut W {
    self.variant(TCSEIRQ_A::TCSEIRQ_1)
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
#[doc = "Illegal instruction interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILLIRQ_A {
  #[doc = "0: no error"]
  ILLIRQ_0,
  #[doc = "1: illegal instruction detected"]
  ILLIRQ_1,
}
impl From<ILLIRQ_A> for bool {
  #[inline(always)]
  fn from(variant: ILLIRQ_A) -> Self {
    match variant {
      ILLIRQ_A::ILLIRQ_0 => false,
      ILLIRQ_A::ILLIRQ_1 => true,
    }
  }
}
#[doc = "Reader of field `ILLIRQ`"]
pub type ILLIRQ_R = crate::R<bool, ILLIRQ_A>;
impl ILLIRQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ILLIRQ_A {
    match self.bits {
      false => ILLIRQ_A::ILLIRQ_0,
      true => ILLIRQ_A::ILLIRQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `ILLIRQ_0`"]
  #[inline(always)]
  pub fn is_illirq_0(&self) -> bool {
    *self == ILLIRQ_A::ILLIRQ_0
  }
  #[doc = "Checks if the value of the field is `ILLIRQ_1`"]
  #[inline(always)]
  pub fn is_illirq_1(&self) -> bool {
    *self == ILLIRQ_A::ILLIRQ_1
  }
}
#[doc = "Write proxy for field `ILLIRQ`"]
pub struct ILLIRQ_W<'a> {
  w: &'a mut W,
}
impl<'a> ILLIRQ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ILLIRQ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no error"]
  #[inline(always)]
  pub fn illirq_0(self) -> &'a mut W {
    self.variant(ILLIRQ_A::ILLIRQ_0)
  }
  #[doc = "illegal instruction detected"]
  #[inline(always)]
  pub fn illirq_1(self) -> &'a mut W {
    self.variant(ILLIRQ_A::ILLIRQ_1)
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
#[doc = "AHB slave response error interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASREIRQ_A {
  #[doc = "0: no error"]
  ASREIRQ_0,
  #[doc = "1: AHB slave response error detected"]
  ASREIRQ_1,
}
impl From<ASREIRQ_A> for bool {
  #[inline(always)]
  fn from(variant: ASREIRQ_A) -> Self {
    match variant {
      ASREIRQ_A::ASREIRQ_0 => false,
      ASREIRQ_A::ASREIRQ_1 => true,
    }
  }
}
#[doc = "Reader of field `ASREIRQ`"]
pub type ASREIRQ_R = crate::R<bool, ASREIRQ_A>;
impl ASREIRQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ASREIRQ_A {
    match self.bits {
      false => ASREIRQ_A::ASREIRQ_0,
      true => ASREIRQ_A::ASREIRQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `ASREIRQ_0`"]
  #[inline(always)]
  pub fn is_asreirq_0(&self) -> bool {
    *self == ASREIRQ_A::ASREIRQ_0
  }
  #[doc = "Checks if the value of the field is `ASREIRQ_1`"]
  #[inline(always)]
  pub fn is_asreirq_1(&self) -> bool {
    *self == ASREIRQ_A::ASREIRQ_1
  }
}
#[doc = "Write proxy for field `ASREIRQ`"]
pub struct ASREIRQ_W<'a> {
  w: &'a mut W,
}
impl<'a> ASREIRQ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ASREIRQ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no error"]
  #[inline(always)]
  pub fn asreirq_0(self) -> &'a mut W {
    self.variant(ASREIRQ_A::ASREIRQ_0)
  }
  #[doc = "AHB slave response error detected"]
  #[inline(always)]
  pub fn asreirq_1(self) -> &'a mut W {
    self.variant(ASREIRQ_A::ASREIRQ_1)
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
#[doc = "IMEM Illegal address interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IIADIRQ_A {
  #[doc = "0: no error"]
  IIADIRQ_0,
  #[doc = "1: illegal IMEM address detected"]
  IIADIRQ_1,
}
impl From<IIADIRQ_A> for bool {
  #[inline(always)]
  fn from(variant: IIADIRQ_A) -> Self {
    match variant {
      IIADIRQ_A::IIADIRQ_0 => false,
      IIADIRQ_A::IIADIRQ_1 => true,
    }
  }
}
#[doc = "Reader of field `IIADIRQ`"]
pub type IIADIRQ_R = crate::R<bool, IIADIRQ_A>;
impl IIADIRQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IIADIRQ_A {
    match self.bits {
      false => IIADIRQ_A::IIADIRQ_0,
      true => IIADIRQ_A::IIADIRQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `IIADIRQ_0`"]
  #[inline(always)]
  pub fn is_iiadirq_0(&self) -> bool {
    *self == IIADIRQ_A::IIADIRQ_0
  }
  #[doc = "Checks if the value of the field is `IIADIRQ_1`"]
  #[inline(always)]
  pub fn is_iiadirq_1(&self) -> bool {
    *self == IIADIRQ_A::IIADIRQ_1
  }
}
#[doc = "Write proxy for field `IIADIRQ`"]
pub struct IIADIRQ_W<'a> {
  w: &'a mut W,
}
impl<'a> IIADIRQ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IIADIRQ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no error"]
  #[inline(always)]
  pub fn iiadirq_0(self) -> &'a mut W {
    self.variant(IIADIRQ_A::IIADIRQ_0)
  }
  #[doc = "illegal IMEM address detected"]
  #[inline(always)]
  pub fn iiadirq_1(self) -> &'a mut W {
    self.variant(IIADIRQ_A::IIADIRQ_1)
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
#[doc = "DMEM illegal access interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIADIRQ_A {
  #[doc = "0: no illegal address"]
  DIADIRQ_0,
  #[doc = "1: illegal address"]
  DIADIRQ_1,
}
impl From<DIADIRQ_A> for bool {
  #[inline(always)]
  fn from(variant: DIADIRQ_A) -> Self {
    match variant {
      DIADIRQ_A::DIADIRQ_0 => false,
      DIADIRQ_A::DIADIRQ_1 => true,
    }
  }
}
#[doc = "Reader of field `DIADIRQ`"]
pub type DIADIRQ_R = crate::R<bool, DIADIRQ_A>;
impl DIADIRQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DIADIRQ_A {
    match self.bits {
      false => DIADIRQ_A::DIADIRQ_0,
      true => DIADIRQ_A::DIADIRQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `DIADIRQ_0`"]
  #[inline(always)]
  pub fn is_diadirq_0(&self) -> bool {
    *self == DIADIRQ_A::DIADIRQ_0
  }
  #[doc = "Checks if the value of the field is `DIADIRQ_1`"]
  #[inline(always)]
  pub fn is_diadirq_1(&self) -> bool {
    *self == DIADIRQ_A::DIADIRQ_1
  }
}
#[doc = "Write proxy for field `DIADIRQ`"]
pub struct DIADIRQ_W<'a> {
  w: &'a mut W,
}
impl<'a> DIADIRQ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DIADIRQ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no illegal address"]
  #[inline(always)]
  pub fn diadirq_0(self) -> &'a mut W {
    self.variant(DIADIRQ_A::DIADIRQ_0)
  }
  #[doc = "illegal address"]
  #[inline(always)]
  pub fn diadirq_1(self) -> &'a mut W {
    self.variant(DIADIRQ_A::DIADIRQ_1)
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
#[doc = "Security violation interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVIRQ_A {
  #[doc = "0: No security violation"]
  SVIRQ_0,
  #[doc = "1: Security violation"]
  SVIRQ_1,
}
impl From<SVIRQ_A> for bool {
  #[inline(always)]
  fn from(variant: SVIRQ_A) -> Self {
    match variant {
      SVIRQ_A::SVIRQ_0 => false,
      SVIRQ_A::SVIRQ_1 => true,
    }
  }
}
#[doc = "Reader of field `SVIRQ`"]
pub type SVIRQ_R = crate::R<bool, SVIRQ_A>;
impl SVIRQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SVIRQ_A {
    match self.bits {
      false => SVIRQ_A::SVIRQ_0,
      true => SVIRQ_A::SVIRQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `SVIRQ_0`"]
  #[inline(always)]
  pub fn is_svirq_0(&self) -> bool {
    *self == SVIRQ_A::SVIRQ_0
  }
  #[doc = "Checks if the value of the field is `SVIRQ_1`"]
  #[inline(always)]
  pub fn is_svirq_1(&self) -> bool {
    *self == SVIRQ_A::SVIRQ_1
  }
}
#[doc = "Write proxy for field `SVIRQ`"]
pub struct SVIRQ_W<'a> {
  w: &'a mut W,
}
impl<'a> SVIRQ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SVIRQ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No security violation"]
  #[inline(always)]
  pub fn svirq_0(self) -> &'a mut W {
    self.variant(SVIRQ_A::SVIRQ_0)
  }
  #[doc = "Security violation"]
  #[inline(always)]
  pub fn svirq_1(self) -> &'a mut W {
    self.variant(SVIRQ_A::SVIRQ_1)
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
#[doc = "Task completion with no error interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIRQ_A {
  #[doc = "0: Task not finished or finished with error"]
  TCIRQ_0,
  #[doc = "1: Task execution finished with no error"]
  TCIRQ_1,
}
impl From<TCIRQ_A> for bool {
  #[inline(always)]
  fn from(variant: TCIRQ_A) -> Self {
    match variant {
      TCIRQ_A::TCIRQ_0 => false,
      TCIRQ_A::TCIRQ_1 => true,
    }
  }
}
#[doc = "Reader of field `TCIRQ`"]
pub type TCIRQ_R = crate::R<bool, TCIRQ_A>;
impl TCIRQ_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TCIRQ_A {
    match self.bits {
      false => TCIRQ_A::TCIRQ_0,
      true => TCIRQ_A::TCIRQ_1,
    }
  }
  #[doc = "Checks if the value of the field is `TCIRQ_0`"]
  #[inline(always)]
  pub fn is_tcirq_0(&self) -> bool {
    *self == TCIRQ_A::TCIRQ_0
  }
  #[doc = "Checks if the value of the field is `TCIRQ_1`"]
  #[inline(always)]
  pub fn is_tcirq_1(&self) -> bool {
    *self == TCIRQ_A::TCIRQ_1
  }
}
#[doc = "Write proxy for field `TCIRQ`"]
pub struct TCIRQ_W<'a> {
  w: &'a mut W,
}
impl<'a> TCIRQ_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCIRQ_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Task not finished or finished with error"]
  #[inline(always)]
  pub fn tcirq_0(self) -> &'a mut W {
    self.variant(TCIRQ_A::TCIRQ_0)
  }
  #[doc = "Task execution finished with no error"]
  #[inline(always)]
  pub fn tcirq_1(self) -> &'a mut W {
    self.variant(TCIRQ_A::TCIRQ_1)
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
#[doc = "Task completion status\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TKCS_A {
  #[doc = "0: Initialization RUN"]
  TKCS_0,
  #[doc = "1: Running"]
  TKCS_1,
  #[doc = "2: Debug Halted"]
  TKCS_2,
  #[doc = "9: Stop - Error Free"]
  TKCS_9,
  #[doc = "10: Stop - Error"]
  TKCS_10,
  #[doc = "14: Stop - Security Violation, assert security violation output signal and set SVIRQ"]
  TKCS_14,
  #[doc = "15: Stop - Security Violation and set SVIRQ"]
  TKCS_15,
}
impl From<TKCS_A> for u8 {
  #[inline(always)]
  fn from(variant: TKCS_A) -> Self {
    match variant {
      TKCS_A::TKCS_0 => 0,
      TKCS_A::TKCS_1 => 1,
      TKCS_A::TKCS_2 => 2,
      TKCS_A::TKCS_9 => 9,
      TKCS_A::TKCS_10 => 10,
      TKCS_A::TKCS_14 => 14,
      TKCS_A::TKCS_15 => 15,
    }
  }
}
#[doc = "Reader of field `TKCS`"]
pub type TKCS_R = crate::R<u8, TKCS_A>;
impl TKCS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TKCS_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TKCS_A::TKCS_0),
      1 => Val(TKCS_A::TKCS_1),
      2 => Val(TKCS_A::TKCS_2),
      9 => Val(TKCS_A::TKCS_9),
      10 => Val(TKCS_A::TKCS_10),
      14 => Val(TKCS_A::TKCS_14),
      15 => Val(TKCS_A::TKCS_15),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TKCS_0`"]
  #[inline(always)]
  pub fn is_tkcs_0(&self) -> bool {
    *self == TKCS_A::TKCS_0
  }
  #[doc = "Checks if the value of the field is `TKCS_1`"]
  #[inline(always)]
  pub fn is_tkcs_1(&self) -> bool {
    *self == TKCS_A::TKCS_1
  }
  #[doc = "Checks if the value of the field is `TKCS_2`"]
  #[inline(always)]
  pub fn is_tkcs_2(&self) -> bool {
    *self == TKCS_A::TKCS_2
  }
  #[doc = "Checks if the value of the field is `TKCS_9`"]
  #[inline(always)]
  pub fn is_tkcs_9(&self) -> bool {
    *self == TKCS_A::TKCS_9
  }
  #[doc = "Checks if the value of the field is `TKCS_10`"]
  #[inline(always)]
  pub fn is_tkcs_10(&self) -> bool {
    *self == TKCS_A::TKCS_10
  }
  #[doc = "Checks if the value of the field is `TKCS_14`"]
  #[inline(always)]
  pub fn is_tkcs_14(&self) -> bool {
    *self == TKCS_A::TKCS_14
  }
  #[doc = "Checks if the value of the field is `TKCS_15`"]
  #[inline(always)]
  pub fn is_tkcs_15(&self) -> bool {
    *self == TKCS_A::TKCS_15
  }
}
#[doc = "Security violation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVF_A {
  #[doc = "0: SoC security violation is not asserted"]
  SVF_0,
  #[doc = "1: SoC security violation was asserted"]
  SVF_1,
}
impl From<SVF_A> for bool {
  #[inline(always)]
  fn from(variant: SVF_A) -> Self {
    match variant {
      SVF_A::SVF_0 => false,
      SVF_A::SVF_1 => true,
    }
  }
}
#[doc = "Reader of field `SVF`"]
pub type SVF_R = crate::R<bool, SVF_A>;
impl SVF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SVF_A {
    match self.bits {
      false => SVF_A::SVF_0,
      true => SVF_A::SVF_1,
    }
  }
  #[doc = "Checks if the value of the field is `SVF_0`"]
  #[inline(always)]
  pub fn is_svf_0(&self) -> bool {
    *self == SVF_A::SVF_0
  }
  #[doc = "Checks if the value of the field is `SVF_1`"]
  #[inline(always)]
  pub fn is_svf_1(&self) -> bool {
    *self == SVF_A::SVF_1
  }
}
#[doc = "Debug mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_A {
  #[doc = "0: CAU3 is not in debug mode"]
  DBG_0,
  #[doc = "1: CAU3 is in debug mode"]
  DBG_1,
}
impl From<DBG_A> for bool {
  #[inline(always)]
  fn from(variant: DBG_A) -> Self {
    match variant {
      DBG_A::DBG_0 => false,
      DBG_A::DBG_1 => true,
    }
  }
}
#[doc = "Reader of field `DBG`"]
pub type DBG_R = crate::R<bool, DBG_A>;
impl DBG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DBG_A {
    match self.bits {
      false => DBG_A::DBG_0,
      true => DBG_A::DBG_1,
    }
  }
  #[doc = "Checks if the value of the field is `DBG_0`"]
  #[inline(always)]
  pub fn is_dbg_0(&self) -> bool {
    *self == DBG_A::DBG_0
  }
  #[doc = "Checks if the value of the field is `DBG_1`"]
  #[inline(always)]
  pub fn is_dbg_1(&self) -> bool {
    *self == DBG_A::DBG_1
  }
}
#[doc = "Task completion configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCFG_A {
  #[doc = "0: No action"]
  TCCFG_0,
  #[doc = "1: Assert an interrupt request"]
  TCCFG_1,
  #[doc = "2: Assert the Event Completion Signal"]
  TCCFG_2,
  #[doc = "4: Issue a DMA request"]
  TCCFG_4,
}
impl From<TCCFG_A> for u8 {
  #[inline(always)]
  fn from(variant: TCCFG_A) -> Self {
    match variant {
      TCCFG_A::TCCFG_0 => 0,
      TCCFG_A::TCCFG_1 => 1,
      TCCFG_A::TCCFG_2 => 2,
      TCCFG_A::TCCFG_4 => 4,
    }
  }
}
#[doc = "Reader of field `TCCFG`"]
pub type TCCFG_R = crate::R<u8, TCCFG_A>;
impl TCCFG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TCCFG_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TCCFG_A::TCCFG_0),
      1 => Val(TCCFG_A::TCCFG_1),
      2 => Val(TCCFG_A::TCCFG_2),
      4 => Val(TCCFG_A::TCCFG_4),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TCCFG_0`"]
  #[inline(always)]
  pub fn is_tccfg_0(&self) -> bool {
    *self == TCCFG_A::TCCFG_0
  }
  #[doc = "Checks if the value of the field is `TCCFG_1`"]
  #[inline(always)]
  pub fn is_tccfg_1(&self) -> bool {
    *self == TCCFG_A::TCCFG_1
  }
  #[doc = "Checks if the value of the field is `TCCFG_2`"]
  #[inline(always)]
  pub fn is_tccfg_2(&self) -> bool {
    *self == TCCFG_A::TCCFG_2
  }
  #[doc = "Checks if the value of the field is `TCCFG_4`"]
  #[inline(always)]
  pub fn is_tccfg_4(&self) -> bool {
    *self == TCCFG_A::TCCFG_4
  }
}
#[doc = "Module disable flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDISF_A {
  #[doc = "0: CCore is not in low power mode"]
  MDISF_0,
  #[doc = "1: CCore is in low power mode"]
  MDISF_1,
}
impl From<MDISF_A> for bool {
  #[inline(always)]
  fn from(variant: MDISF_A) -> Self {
    match variant {
      MDISF_A::MDISF_0 => false,
      MDISF_A::MDISF_1 => true,
    }
  }
}
#[doc = "Reader of field `MDISF`"]
pub type MDISF_R = crate::R<bool, MDISF_A>;
impl MDISF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> MDISF_A {
    match self.bits {
      false => MDISF_A::MDISF_0,
      true => MDISF_A::MDISF_1,
    }
  }
  #[doc = "Checks if the value of the field is `MDISF_0`"]
  #[inline(always)]
  pub fn is_mdisf_0(&self) -> bool {
    *self == MDISF_A::MDISF_0
  }
  #[doc = "Checks if the value of the field is `MDISF_1`"]
  #[inline(always)]
  pub fn is_mdisf_1(&self) -> bool {
    *self == MDISF_A::MDISF_1
  }
}
impl R {
  #[doc = "Bit 0 - Task completion with software error interrupt request"]
  #[inline(always)]
  pub fn tcseirq(&self) -> TCSEIRQ_R {
    TCSEIRQ_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Illegal instruction interrupt request"]
  #[inline(always)]
  pub fn illirq(&self) -> ILLIRQ_R {
    ILLIRQ_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 3 - AHB slave response error interrupt Request"]
  #[inline(always)]
  pub fn asreirq(&self) -> ASREIRQ_R {
    ASREIRQ_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - IMEM Illegal address interrupt request"]
  #[inline(always)]
  pub fn iiadirq(&self) -> IIADIRQ_R {
    IIADIRQ_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - DMEM illegal access interrupt request"]
  #[inline(always)]
  pub fn diadirq(&self) -> DIADIRQ_R {
    DIADIRQ_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Security violation interrupt request"]
  #[inline(always)]
  pub fn svirq(&self) -> SVIRQ_R {
    SVIRQ_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Task completion with no error interrupt request"]
  #[inline(always)]
  pub fn tcirq(&self) -> TCIRQ_R {
    TCIRQ_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bits 8:11 - Task completion status"]
  #[inline(always)]
  pub fn tkcs(&self) -> TKCS_R {
    TKCS_R::new(((self.bits >> 8) & 0x0f) as u8)
  }
  #[doc = "Bit 16 - Security violation flag"]
  #[inline(always)]
  pub fn svf(&self) -> SVF_R {
    SVF_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Debug mode"]
  #[inline(always)]
  pub fn dbg(&self) -> DBG_R {
    DBG_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bits 24:26 - Task completion configuration"]
  #[inline(always)]
  pub fn tccfg(&self) -> TCCFG_R {
    TCCFG_R::new(((self.bits >> 24) & 0x07) as u8)
  }
  #[doc = "Bit 31 - Module disable flag"]
  #[inline(always)]
  pub fn mdisf(&self) -> MDISF_R {
    MDISF_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Task completion with software error interrupt request"]
  #[inline(always)]
  pub fn tcseirq(&mut self) -> TCSEIRQ_W {
    TCSEIRQ_W { w: self }
  }
  #[doc = "Bit 1 - Illegal instruction interrupt request"]
  #[inline(always)]
  pub fn illirq(&mut self) -> ILLIRQ_W {
    ILLIRQ_W { w: self }
  }
  #[doc = "Bit 3 - AHB slave response error interrupt Request"]
  #[inline(always)]
  pub fn asreirq(&mut self) -> ASREIRQ_W {
    ASREIRQ_W { w: self }
  }
  #[doc = "Bit 4 - IMEM Illegal address interrupt request"]
  #[inline(always)]
  pub fn iiadirq(&mut self) -> IIADIRQ_W {
    IIADIRQ_W { w: self }
  }
  #[doc = "Bit 5 - DMEM illegal access interrupt request"]
  #[inline(always)]
  pub fn diadirq(&mut self) -> DIADIRQ_W {
    DIADIRQ_W { w: self }
  }
  #[doc = "Bit 6 - Security violation interrupt request"]
  #[inline(always)]
  pub fn svirq(&mut self) -> SVIRQ_W {
    SVIRQ_W { w: self }
  }
  #[doc = "Bit 7 - Task completion with no error interrupt request"]
  #[inline(always)]
  pub fn tcirq(&mut self) -> TCIRQ_W {
    TCIRQ_W { w: self }
  }
}
