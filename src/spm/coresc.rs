#[doc = "Reader of register CORESC"]
pub type R = crate::R<u32, super::CORESC>;
#[doc = "Writer for register CORESC"]
pub type W = crate::W<u32, super::CORESC>;
#[doc = "Register CORESC `reset()`'s with value 0x04"]
impl crate::ResetValue for super::CORESC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x04
  }
}
#[doc = "CORE LDO Regulator in Run Regulation Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGONS_A {
  #[doc = "0: Regulator is in low power state or in transition to/from it."]
  REGONS_0,
  #[doc = "1: Regulator is in high power state."]
  REGONS_1,
}
impl From<REGONS_A> for bool {
  #[inline(always)]
  fn from(variant: REGONS_A) -> Self {
    match variant {
      REGONS_A::REGONS_0 => false,
      REGONS_A::REGONS_1 => true,
    }
  }
}
#[doc = "Reader of field `REGONS`"]
pub type REGONS_R = crate::R<bool, REGONS_A>;
impl REGONS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> REGONS_A {
    match self.bits {
      false => REGONS_A::REGONS_0,
      true => REGONS_A::REGONS_1,
    }
  }
  #[doc = "Checks if the value of the field is `REGONS_0`"]
  #[inline(always)]
  pub fn is_regons_0(&self) -> bool {
    *self == REGONS_A::REGONS_0
  }
  #[doc = "Checks if the value of the field is `REGONS_1`"]
  #[inline(always)]
  pub fn is_regons_1(&self) -> bool {
    *self == REGONS_A::REGONS_1
  }
}
#[doc = "Acknowledge Isolation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKISO_A {
  #[doc = "0: Peripherals and I/O pads are in normal run state."]
  ACKISO_0,
  #[doc = "1: Certain peripherals and I/O pads are in a isolated and latched state."]
  ACKISO_1,
}
impl From<ACKISO_A> for bool {
  #[inline(always)]
  fn from(variant: ACKISO_A) -> Self {
    match variant {
      ACKISO_A::ACKISO_0 => false,
      ACKISO_A::ACKISO_1 => true,
    }
  }
}
#[doc = "Reader of field `ACKISO`"]
pub type ACKISO_R = crate::R<bool, ACKISO_A>;
impl ACKISO_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ACKISO_A {
    match self.bits {
      false => ACKISO_A::ACKISO_0,
      true => ACKISO_A::ACKISO_1,
    }
  }
  #[doc = "Checks if the value of the field is `ACKISO_0`"]
  #[inline(always)]
  pub fn is_ackiso_0(&self) -> bool {
    *self == ACKISO_A::ACKISO_0
  }
  #[doc = "Checks if the value of the field is `ACKISO_1`"]
  #[inline(always)]
  pub fn is_ackiso_1(&self) -> bool {
    *self == ACKISO_A::ACKISO_1
  }
}
#[doc = "Write proxy for field `ACKISO`"]
pub struct ACKISO_W<'a> {
  w: &'a mut W,
}
impl<'a> ACKISO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ACKISO_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Peripherals and I/O pads are in normal run state."]
  #[inline(always)]
  pub fn ackiso_0(self) -> &'a mut W {
    self.variant(ACKISO_A::ACKISO_0)
  }
  #[doc = "Certain peripherals and I/O pads are in a isolated and latched state."]
  #[inline(always)]
  pub fn ackiso_1(self) -> &'a mut W {
    self.variant(ACKISO_A::ACKISO_1)
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
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u8, u8>;
#[doc = "VDDIOOVRIDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDIOOVRIDE_A {
  #[doc = "0: VDDIOOK status set to 1'b0."]
  VDDIOOVRIDE_0,
  #[doc = "1: VDDIOOK status set to 1'b1."]
  VDDIOOVRIDE_1,
}
impl From<VDDIOOVRIDE_A> for bool {
  #[inline(always)]
  fn from(variant: VDDIOOVRIDE_A) -> Self {
    match variant {
      VDDIOOVRIDE_A::VDDIOOVRIDE_0 => false,
      VDDIOOVRIDE_A::VDDIOOVRIDE_1 => true,
    }
  }
}
#[doc = "Reader of field `VDDIOOVRIDE`"]
pub type VDDIOOVRIDE_R = crate::R<bool, VDDIOOVRIDE_A>;
impl VDDIOOVRIDE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDDIOOVRIDE_A {
    match self.bits {
      false => VDDIOOVRIDE_A::VDDIOOVRIDE_0,
      true => VDDIOOVRIDE_A::VDDIOOVRIDE_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDDIOOVRIDE_0`"]
  #[inline(always)]
  pub fn is_vddioovride_0(&self) -> bool {
    *self == VDDIOOVRIDE_A::VDDIOOVRIDE_0
  }
  #[doc = "Checks if the value of the field is `VDDIOOVRIDE_1`"]
  #[inline(always)]
  pub fn is_vddioovride_1(&self) -> bool {
    *self == VDDIOOVRIDE_A::VDDIOOVRIDE_1
  }
}
#[doc = "Write proxy for field `VDDIOOVRIDE`"]
pub struct VDDIOOVRIDE_W<'a> {
  w: &'a mut W,
}
impl<'a> VDDIOOVRIDE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDDIOOVRIDE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VDDIOOK status set to 1'b0."]
  #[inline(always)]
  pub fn vddioovride_0(self) -> &'a mut W {
    self.variant(VDDIOOVRIDE_A::VDDIOOVRIDE_0)
  }
  #[doc = "VDDIOOK status set to 1'b1."]
  #[inline(always)]
  pub fn vddioovride_1(self) -> &'a mut W {
    self.variant(VDDIOOVRIDE_A::VDDIOOVRIDE_1)
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
#[doc = "USBOVRIDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOVRIDE_A {
  #[doc = "0: USBVDDOK status set to 1'b0."]
  USBOVRIDE_0,
  #[doc = "1: USBVDDOK status set to 1'b1."]
  USBOVRIDE_1,
}
impl From<USBOVRIDE_A> for bool {
  #[inline(always)]
  fn from(variant: USBOVRIDE_A) -> Self {
    match variant {
      USBOVRIDE_A::USBOVRIDE_0 => false,
      USBOVRIDE_A::USBOVRIDE_1 => true,
    }
  }
}
#[doc = "Reader of field `USBOVRIDE`"]
pub type USBOVRIDE_R = crate::R<bool, USBOVRIDE_A>;
impl USBOVRIDE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> USBOVRIDE_A {
    match self.bits {
      false => USBOVRIDE_A::USBOVRIDE_0,
      true => USBOVRIDE_A::USBOVRIDE_1,
    }
  }
  #[doc = "Checks if the value of the field is `USBOVRIDE_0`"]
  #[inline(always)]
  pub fn is_usbovride_0(&self) -> bool {
    *self == USBOVRIDE_A::USBOVRIDE_0
  }
  #[doc = "Checks if the value of the field is `USBOVRIDE_1`"]
  #[inline(always)]
  pub fn is_usbovride_1(&self) -> bool {
    *self == USBOVRIDE_A::USBOVRIDE_1
  }
}
#[doc = "Write proxy for field `USBOVRIDE`"]
pub struct USBOVRIDE_W<'a> {
  w: &'a mut W,
}
impl<'a> USBOVRIDE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: USBOVRIDE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "USBVDDOK status set to 1'b0."]
  #[inline(always)]
  pub fn usbovride_0(self) -> &'a mut W {
    self.variant(USBOVRIDE_A::USBOVRIDE_0)
  }
  #[doc = "USBVDDOK status set to 1'b1."]
  #[inline(always)]
  pub fn usbovride_1(self) -> &'a mut W {
    self.variant(USBOVRIDE_A::USBOVRIDE_1)
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
#[doc = "RTCOVRIDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCOVRIDE_A {
  #[doc = "0: RTCVDDOK status set to 1'b0."]
  RTCOVRIDE_0,
  #[doc = "1: RTCVDDOK status set to 1'b1."]
  RTCOVRIDE_1,
}
impl From<RTCOVRIDE_A> for bool {
  #[inline(always)]
  fn from(variant: RTCOVRIDE_A) -> Self {
    match variant {
      RTCOVRIDE_A::RTCOVRIDE_0 => false,
      RTCOVRIDE_A::RTCOVRIDE_1 => true,
    }
  }
}
#[doc = "Reader of field `RTCOVRIDE`"]
pub type RTCOVRIDE_R = crate::R<bool, RTCOVRIDE_A>;
impl RTCOVRIDE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RTCOVRIDE_A {
    match self.bits {
      false => RTCOVRIDE_A::RTCOVRIDE_0,
      true => RTCOVRIDE_A::RTCOVRIDE_1,
    }
  }
  #[doc = "Checks if the value of the field is `RTCOVRIDE_0`"]
  #[inline(always)]
  pub fn is_rtcovride_0(&self) -> bool {
    *self == RTCOVRIDE_A::RTCOVRIDE_0
  }
  #[doc = "Checks if the value of the field is `RTCOVRIDE_1`"]
  #[inline(always)]
  pub fn is_rtcovride_1(&self) -> bool {
    *self == RTCOVRIDE_A::RTCOVRIDE_1
  }
}
#[doc = "Write proxy for field `RTCOVRIDE`"]
pub struct RTCOVRIDE_W<'a> {
  w: &'a mut W,
}
impl<'a> RTCOVRIDE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RTCOVRIDE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTCVDDOK status set to 1'b0."]
  #[inline(always)]
  pub fn rtcovride_0(self) -> &'a mut W {
    self.variant(RTCOVRIDE_A::RTCOVRIDE_0)
  }
  #[doc = "RTCVDDOK status set to 1'b1."]
  #[inline(always)]
  pub fn rtcovride_1(self) -> &'a mut W {
    self.variant(RTCOVRIDE_A::RTCOVRIDE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
    self.w
  }
}
#[doc = "Reader of field `VDDIOOK`"]
pub type VDDIOOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBVDDOK`"]
pub type USBVDDOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTCVDDOK`"]
pub type RTCVDDOK_R = crate::R<bool, bool>;
impl R {
  #[doc = "Bit 2 - CORE LDO Regulator in Run Regulation Status"]
  #[inline(always)]
  pub fn regons(&self) -> REGONS_R {
    REGONS_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Acknowledge Isolation"]
  #[inline(always)]
  pub fn ackiso(&self) -> ACKISO_R {
    ACKISO_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bits 8:13 - Core LDO Regulator TRIM value"]
  #[inline(always)]
  pub fn trim(&self) -> TRIM_R {
    TRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
  }
  #[doc = "Bit 16 - VDDIOOVRIDE"]
  #[inline(always)]
  pub fn vddioovride(&self) -> VDDIOOVRIDE_R {
    VDDIOOVRIDE_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - USBOVRIDE"]
  #[inline(always)]
  pub fn usbovride(&self) -> USBOVRIDE_R {
    USBOVRIDE_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - RTCOVRIDE"]
  #[inline(always)]
  pub fn rtcovride(&self) -> RTCOVRIDE_R {
    RTCOVRIDE_R::new(((self.bits >> 18) & 0x01) != 0)
  }
  #[doc = "Bit 24 - VDDIOOK"]
  #[inline(always)]
  pub fn vddiook(&self) -> VDDIOOK_R {
    VDDIOOK_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bit 25 - USBVDDOK"]
  #[inline(always)]
  pub fn usbvddok(&self) -> USBVDDOK_R {
    USBVDDOK_R::new(((self.bits >> 25) & 0x01) != 0)
  }
  #[doc = "Bit 26 - RTCVDDOK"]
  #[inline(always)]
  pub fn rtcvddok(&self) -> RTCVDDOK_R {
    RTCVDDOK_R::new(((self.bits >> 26) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 3 - Acknowledge Isolation"]
  #[inline(always)]
  pub fn ackiso(&mut self) -> ACKISO_W {
    ACKISO_W { w: self }
  }
  #[doc = "Bit 16 - VDDIOOVRIDE"]
  #[inline(always)]
  pub fn vddioovride(&mut self) -> VDDIOOVRIDE_W {
    VDDIOOVRIDE_W { w: self }
  }
  #[doc = "Bit 17 - USBOVRIDE"]
  #[inline(always)]
  pub fn usbovride(&mut self) -> USBOVRIDE_W {
    USBOVRIDE_W { w: self }
  }
  #[doc = "Bit 18 - RTCOVRIDE"]
  #[inline(always)]
  pub fn rtcovride(&mut self) -> RTCOVRIDE_W {
    RTCOVRIDE_W { w: self }
  }
}
