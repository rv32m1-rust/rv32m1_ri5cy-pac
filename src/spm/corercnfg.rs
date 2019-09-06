#[doc = "Reader of register CORERCNFG"]
pub type R = crate::R<u32, super::CORERCNFG>;
#[doc = "Writer for register CORERCNFG"]
pub type W = crate::W<u32, super::CORERCNFG>;
#[doc = "Register CORERCNFG `reset()`'s with value 0x0007_0000"]
impl crate::ResetValue for super::CORERCNFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0007_0000
  }
}
#[doc = "VDDIOVDDMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDIOVDDMEN_A {
  #[doc = "0: VDDIO voltage monitor disabled in run modes."]
  VDDIOVDDMEN_0,
  #[doc = "1: VDDIO voltage monitor enabled in run modes."]
  VDDIOVDDMEN_1,
}
impl From<VDDIOVDDMEN_A> for bool {
  #[inline(always)]
  fn from(variant: VDDIOVDDMEN_A) -> Self {
    match variant {
      VDDIOVDDMEN_A::VDDIOVDDMEN_0 => false,
      VDDIOVDDMEN_A::VDDIOVDDMEN_1 => true,
    }
  }
}
#[doc = "Reader of field `VDDIOVDDMEN`"]
pub type VDDIOVDDMEN_R = crate::R<bool, VDDIOVDDMEN_A>;
impl VDDIOVDDMEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDDIOVDDMEN_A {
    match self.bits {
      false => VDDIOVDDMEN_A::VDDIOVDDMEN_0,
      true => VDDIOVDDMEN_A::VDDIOVDDMEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDDIOVDDMEN_0`"]
  #[inline(always)]
  pub fn is_vddiovddmen_0(&self) -> bool {
    *self == VDDIOVDDMEN_A::VDDIOVDDMEN_0
  }
  #[doc = "Checks if the value of the field is `VDDIOVDDMEN_1`"]
  #[inline(always)]
  pub fn is_vddiovddmen_1(&self) -> bool {
    *self == VDDIOVDDMEN_A::VDDIOVDDMEN_1
  }
}
#[doc = "Write proxy for field `VDDIOVDDMEN`"]
pub struct VDDIOVDDMEN_W<'a> {
  w: &'a mut W,
}
impl<'a> VDDIOVDDMEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDDIOVDDMEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VDDIO voltage monitor disabled in run modes."]
  #[inline(always)]
  pub fn vddiovddmen_0(self) -> &'a mut W {
    self.variant(VDDIOVDDMEN_A::VDDIOVDDMEN_0)
  }
  #[doc = "VDDIO voltage monitor enabled in run modes."]
  #[inline(always)]
  pub fn vddiovddmen_1(self) -> &'a mut W {
    self.variant(VDDIOVDDMEN_A::VDDIOVDDMEN_1)
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
#[doc = "USBVDDMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBVDDMEN_A {
  #[doc = "0: USB voltage monitor disabled in run modes."]
  USBVDDMEN_0,
  #[doc = "1: USB voltage monitor enabled in run modes."]
  USBVDDMEN_1,
}
impl From<USBVDDMEN_A> for bool {
  #[inline(always)]
  fn from(variant: USBVDDMEN_A) -> Self {
    match variant {
      USBVDDMEN_A::USBVDDMEN_0 => false,
      USBVDDMEN_A::USBVDDMEN_1 => true,
    }
  }
}
#[doc = "Reader of field `USBVDDMEN`"]
pub type USBVDDMEN_R = crate::R<bool, USBVDDMEN_A>;
impl USBVDDMEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> USBVDDMEN_A {
    match self.bits {
      false => USBVDDMEN_A::USBVDDMEN_0,
      true => USBVDDMEN_A::USBVDDMEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `USBVDDMEN_0`"]
  #[inline(always)]
  pub fn is_usbvddmen_0(&self) -> bool {
    *self == USBVDDMEN_A::USBVDDMEN_0
  }
  #[doc = "Checks if the value of the field is `USBVDDMEN_1`"]
  #[inline(always)]
  pub fn is_usbvddmen_1(&self) -> bool {
    *self == USBVDDMEN_A::USBVDDMEN_1
  }
}
#[doc = "Write proxy for field `USBVDDMEN`"]
pub struct USBVDDMEN_W<'a> {
  w: &'a mut W,
}
impl<'a> USBVDDMEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: USBVDDMEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "USB voltage monitor disabled in run modes."]
  #[inline(always)]
  pub fn usbvddmen_0(self) -> &'a mut W {
    self.variant(USBVDDMEN_A::USBVDDMEN_0)
  }
  #[doc = "USB voltage monitor enabled in run modes."]
  #[inline(always)]
  pub fn usbvddmen_1(self) -> &'a mut W {
    self.variant(USBVDDMEN_A::USBVDDMEN_1)
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
#[doc = "RTCVDDMEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCVDDMEN_A {
  #[doc = "0: RTC voltage monitor disabled in run modes."]
  RTCVDDMEN_0,
  #[doc = "1: RTC voltage monitor enabled in run modes."]
  RTCVDDMEN_1,
}
impl From<RTCVDDMEN_A> for bool {
  #[inline(always)]
  fn from(variant: RTCVDDMEN_A) -> Self {
    match variant {
      RTCVDDMEN_A::RTCVDDMEN_0 => false,
      RTCVDDMEN_A::RTCVDDMEN_1 => true,
    }
  }
}
#[doc = "Reader of field `RTCVDDMEN`"]
pub type RTCVDDMEN_R = crate::R<bool, RTCVDDMEN_A>;
impl RTCVDDMEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RTCVDDMEN_A {
    match self.bits {
      false => RTCVDDMEN_A::RTCVDDMEN_0,
      true => RTCVDDMEN_A::RTCVDDMEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `RTCVDDMEN_0`"]
  #[inline(always)]
  pub fn is_rtcvddmen_0(&self) -> bool {
    *self == RTCVDDMEN_A::RTCVDDMEN_0
  }
  #[doc = "Checks if the value of the field is `RTCVDDMEN_1`"]
  #[inline(always)]
  pub fn is_rtcvddmen_1(&self) -> bool {
    *self == RTCVDDMEN_A::RTCVDDMEN_1
  }
}
#[doc = "Write proxy for field `RTCVDDMEN`"]
pub struct RTCVDDMEN_W<'a> {
  w: &'a mut W,
}
impl<'a> RTCVDDMEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RTCVDDMEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "RTC voltage monitor disabled in run modes."]
  #[inline(always)]
  pub fn rtcvddmen_0(self) -> &'a mut W {
    self.variant(RTCVDDMEN_A::RTCVDDMEN_0)
  }
  #[doc = "RTC voltage monitor enabled in run modes."]
  #[inline(always)]
  pub fn rtcvddmen_1(self) -> &'a mut W {
    self.variant(RTCVDDMEN_A::RTCVDDMEN_1)
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
impl R {
  #[doc = "Bit 16 - VDDIOVDDMEN"]
  #[inline(always)]
  pub fn vddiovddmen(&self) -> VDDIOVDDMEN_R {
    VDDIOVDDMEN_R::new(((self.bits >> 16) & 0x01) != 0)
  }
  #[doc = "Bit 17 - USBVDDMEN"]
  #[inline(always)]
  pub fn usbvddmen(&self) -> USBVDDMEN_R {
    USBVDDMEN_R::new(((self.bits >> 17) & 0x01) != 0)
  }
  #[doc = "Bit 18 - RTCVDDMEN"]
  #[inline(always)]
  pub fn rtcvddmen(&self) -> RTCVDDMEN_R {
    RTCVDDMEN_R::new(((self.bits >> 18) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 16 - VDDIOVDDMEN"]
  #[inline(always)]
  pub fn vddiovddmen(&mut self) -> VDDIOVDDMEN_W {
    VDDIOVDDMEN_W { w: self }
  }
  #[doc = "Bit 17 - USBVDDMEN"]
  #[inline(always)]
  pub fn usbvddmen(&mut self) -> USBVDDMEN_W {
    USBVDDMEN_W { w: self }
  }
  #[doc = "Bit 18 - RTCVDDMEN"]
  #[inline(always)]
  pub fn rtcvddmen(&mut self) -> RTCVDDMEN_W {
    RTCVDDMEN_W { w: self }
  }
}
