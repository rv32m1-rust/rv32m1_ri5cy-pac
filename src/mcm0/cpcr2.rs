#[doc = "Reader of register CPCR2"]
pub type R = crate::R<u32, super::CPCR2>;
#[doc = "Writer for register CPCR2"]
pub type W = crate::W<u32, super::CPCR2>;
#[doc = "Register CPCR2 `reset()`'s with value 0x30"]
impl crate::ResetValue for super::CPCR2 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x30
  }
}
#[doc = "Clear code bus cache, this field always reads as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCBC_AW {
  #[doc = "0: No effect"]
  CCBC_0,
  #[doc = "1: Clear code bus cache"]
  CCBC_1,
}
impl From<CCBC_AW> for bool {
  #[inline(always)]
  fn from(variant: CCBC_AW) -> Self {
    match variant {
      CCBC_AW::CCBC_0 => false,
      CCBC_AW::CCBC_1 => true,
    }
  }
}
#[doc = "Write proxy for field `CCBC`"]
pub struct CCBC_W<'a> {
  w: &'a mut W,
}
impl<'a> CCBC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CCBC_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No effect"]
  #[inline(always)]
  pub fn ccbc_0(self) -> &'a mut W {
    self.variant(CCBC_AW::CCBC_0)
  }
  #[doc = "Clear code bus cache"]
  #[inline(always)]
  pub fn ccbc_1(self) -> &'a mut W {
    self.variant(CCBC_AW::CCBC_1)
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
#[doc = "Disable code bus cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCBC_A {
  #[doc = "0: Enable code bus cache"]
  DCBC_0,
  #[doc = "1: Disable code bus cache"]
  DCBC_1,
}
impl From<DCBC_A> for bool {
  #[inline(always)]
  fn from(variant: DCBC_A) -> Self {
    match variant {
      DCBC_A::DCBC_0 => false,
      DCBC_A::DCBC_1 => true,
    }
  }
}
#[doc = "Reader of field `DCBC`"]
pub type DCBC_R = crate::R<bool, DCBC_A>;
impl DCBC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> DCBC_A {
    match self.bits {
      false => DCBC_A::DCBC_0,
      true => DCBC_A::DCBC_1,
    }
  }
  #[doc = "Checks if the value of the field is `DCBC_0`"]
  #[inline(always)]
  pub fn is_dcbc_0(&self) -> bool {
    *self == DCBC_A::DCBC_0
  }
  #[doc = "Checks if the value of the field is `DCBC_1`"]
  #[inline(always)]
  pub fn is_dcbc_1(&self) -> bool {
    *self == DCBC_A::DCBC_1
  }
}
#[doc = "Write proxy for field `DCBC`"]
pub struct DCBC_W<'a> {
  w: &'a mut W,
}
impl<'a> DCBC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: DCBC_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Enable code bus cache"]
  #[inline(always)]
  pub fn dcbc_0(self) -> &'a mut W {
    self.variant(DCBC_A::DCBC_0)
  }
  #[doc = "Disable code bus cache"]
  #[inline(always)]
  pub fn dcbc_1(self) -> &'a mut W {
    self.variant(DCBC_A::DCBC_1)
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
#[doc = "Code Bus Cache Size\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CBCS_A {
  #[doc = "0: 0 KB"]
  CBCS_0,
  #[doc = "1: 1 KB"]
  CBCS_1,
  #[doc = "2: 2 KB"]
  CBCS_2,
  #[doc = "3: 4 KB"]
  CBCS_3,
  #[doc = "4: 8 KB"]
  CBCS_4,
  #[doc = "5: 16 KB"]
  CBCS_5,
  #[doc = "6: 32 KB"]
  CBCS_6,
}
impl From<CBCS_A> for u8 {
  #[inline(always)]
  fn from(variant: CBCS_A) -> Self {
    match variant {
      CBCS_A::CBCS_0 => 0,
      CBCS_A::CBCS_1 => 1,
      CBCS_A::CBCS_2 => 2,
      CBCS_A::CBCS_3 => 3,
      CBCS_A::CBCS_4 => 4,
      CBCS_A::CBCS_5 => 5,
      CBCS_A::CBCS_6 => 6,
    }
  }
}
#[doc = "Reader of field `CBCS`"]
pub type CBCS_R = crate::R<u8, CBCS_A>;
impl CBCS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CBCS_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(CBCS_A::CBCS_0),
      1 => Val(CBCS_A::CBCS_1),
      2 => Val(CBCS_A::CBCS_2),
      3 => Val(CBCS_A::CBCS_3),
      4 => Val(CBCS_A::CBCS_4),
      5 => Val(CBCS_A::CBCS_5),
      6 => Val(CBCS_A::CBCS_6),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CBCS_0`"]
  #[inline(always)]
  pub fn is_cbcs_0(&self) -> bool {
    *self == CBCS_A::CBCS_0
  }
  #[doc = "Checks if the value of the field is `CBCS_1`"]
  #[inline(always)]
  pub fn is_cbcs_1(&self) -> bool {
    *self == CBCS_A::CBCS_1
  }
  #[doc = "Checks if the value of the field is `CBCS_2`"]
  #[inline(always)]
  pub fn is_cbcs_2(&self) -> bool {
    *self == CBCS_A::CBCS_2
  }
  #[doc = "Checks if the value of the field is `CBCS_3`"]
  #[inline(always)]
  pub fn is_cbcs_3(&self) -> bool {
    *self == CBCS_A::CBCS_3
  }
  #[doc = "Checks if the value of the field is `CBCS_4`"]
  #[inline(always)]
  pub fn is_cbcs_4(&self) -> bool {
    *self == CBCS_A::CBCS_4
  }
  #[doc = "Checks if the value of the field is `CBCS_5`"]
  #[inline(always)]
  pub fn is_cbcs_5(&self) -> bool {
    *self == CBCS_A::CBCS_5
  }
  #[doc = "Checks if the value of the field is `CBCS_6`"]
  #[inline(always)]
  pub fn is_cbcs_6(&self) -> bool {
    *self == CBCS_A::CBCS_6
  }
}
#[doc = "Bypass fixed code cache map\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCCMCTRL_A {
  #[doc = "0: The fixed code cache map is not bypassed"]
  PCCMCTRL_0,
  #[doc = "1: The fixed code cache map is bypassed"]
  PCCMCTRL_1,
}
impl From<PCCMCTRL_A> for bool {
  #[inline(always)]
  fn from(variant: PCCMCTRL_A) -> Self {
    match variant {
      PCCMCTRL_A::PCCMCTRL_0 => false,
      PCCMCTRL_A::PCCMCTRL_1 => true,
    }
  }
}
#[doc = "Reader of field `PCCMCTRL`"]
pub type PCCMCTRL_R = crate::R<bool, PCCMCTRL_A>;
impl PCCMCTRL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PCCMCTRL_A {
    match self.bits {
      false => PCCMCTRL_A::PCCMCTRL_0,
      true => PCCMCTRL_A::PCCMCTRL_1,
    }
  }
  #[doc = "Checks if the value of the field is `PCCMCTRL_0`"]
  #[inline(always)]
  pub fn is_pccmctrl_0(&self) -> bool {
    *self == PCCMCTRL_A::PCCMCTRL_0
  }
  #[doc = "Checks if the value of the field is `PCCMCTRL_1`"]
  #[inline(always)]
  pub fn is_pccmctrl_1(&self) -> bool {
    *self == PCCMCTRL_A::PCCMCTRL_1
  }
}
#[doc = "Write proxy for field `PCCMCTRL`"]
pub struct PCCMCTRL_W<'a> {
  w: &'a mut W,
}
impl<'a> PCCMCTRL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PCCMCTRL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The fixed code cache map is not bypassed"]
  #[inline(always)]
  pub fn pccmctrl_0(self) -> &'a mut W {
    self.variant(PCCMCTRL_A::PCCMCTRL_0)
  }
  #[doc = "The fixed code cache map is bypassed"]
  #[inline(always)]
  pub fn pccmctrl_1(self) -> &'a mut W {
    self.variant(PCCMCTRL_A::PCCMCTRL_1)
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
#[doc = "Limit code cache peripheral write buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCCPWB_A {
  #[doc = "0: Code cache peripheral write buffering is not limited"]
  LCCPWB_0,
  #[doc = "1: Code cache peripheral write buffering is limited"]
  LCCPWB_1,
}
impl From<LCCPWB_A> for bool {
  #[inline(always)]
  fn from(variant: LCCPWB_A) -> Self {
    match variant {
      LCCPWB_A::LCCPWB_0 => false,
      LCCPWB_A::LCCPWB_1 => true,
    }
  }
}
#[doc = "Reader of field `LCCPWB`"]
pub type LCCPWB_R = crate::R<bool, LCCPWB_A>;
impl LCCPWB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LCCPWB_A {
    match self.bits {
      false => LCCPWB_A::LCCPWB_0,
      true => LCCPWB_A::LCCPWB_1,
    }
  }
  #[doc = "Checks if the value of the field is `LCCPWB_0`"]
  #[inline(always)]
  pub fn is_lccpwb_0(&self) -> bool {
    *self == LCCPWB_A::LCCPWB_0
  }
  #[doc = "Checks if the value of the field is `LCCPWB_1`"]
  #[inline(always)]
  pub fn is_lccpwb_1(&self) -> bool {
    *self == LCCPWB_A::LCCPWB_1
  }
}
#[doc = "Write proxy for field `LCCPWB`"]
pub struct LCCPWB_W<'a> {
  w: &'a mut W,
}
impl<'a> LCCPWB_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LCCPWB_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Code cache peripheral write buffering is not limited"]
  #[inline(always)]
  pub fn lccpwb_0(self) -> &'a mut W {
    self.variant(LCCPWB_A::LCCPWB_0)
  }
  #[doc = "Code cache peripheral write buffering is limited"]
  #[inline(always)]
  pub fn lccpwb_1(self) -> &'a mut W {
    self.variant(LCCPWB_A::LCCPWB_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
    self.w
  }
}
impl R {
  #[doc = "Bit 3 - Disable code bus cache"]
  #[inline(always)]
  pub fn dcbc(&self) -> DCBC_R {
    DCBC_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 4:7 - Code Bus Cache Size"]
  #[inline(always)]
  pub fn cbcs(&self) -> CBCS_R {
    CBCS_R::new(((self.bits >> 4) & 0x0f) as u8)
  }
  #[doc = "Bit 16 - Bypass fixed code cache map"]
  #[inline(always)]
  pub fn pccmctrl(&self) -> PCCMCTRL_R {
    PCCMCTRL_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - Limit code cache peripheral write buffering"]
  #[inline(always)]
  pub fn lccpwb(&self) -> LCCPWB_R {
    LCCPWB_R::new(((self.bits >> 17) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Clear code bus cache, this field always reads as 0."]
  #[inline(always)]
  pub fn ccbc(&mut self) -> CCBC_W {
    CCBC_W { w: self }
  }
  #[doc = "Bit 3 - Disable code bus cache"]
  #[inline(always)]
  pub fn dcbc(&mut self) -> DCBC_W {
    DCBC_W { w: self }
  }
  #[doc = "Bit 16 - Bypass fixed code cache map"]
  #[inline(always)]
  pub fn pccmctrl(&mut self) -> PCCMCTRL_W {
    PCCMCTRL_W { w: self }
  }
  #[doc = "Bit 17 - Limit code cache peripheral write buffering"]
  #[inline(always)]
  pub fn lccpwb(&mut self) -> LCCPWB_W {
    LCCPWB_W { w: self }
  }
}
