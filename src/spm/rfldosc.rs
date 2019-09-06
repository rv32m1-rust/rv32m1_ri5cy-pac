#[doc = "Reader of register RFLDOSC"]
pub type R = crate::R<u32, super::RFLDOSC>;
#[doc = "Writer for register RFLDOSC"]
pub type W = crate::W<u32, super::RFLDOSC>;
#[doc = "Register RFLDOSC `reset()`'s with value 0x20"]
impl crate::ResetValue for super::RFLDOSC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x20
  }
}
#[doc = "IO Regulator Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOREGVSEL_A {
  #[doc = "0: Regulate to 1.8V."]
  IOREGVSEL_0,
  #[doc = "1: Regulate to 1.5V."]
  IOREGVSEL_1,
}
impl From<IOREGVSEL_A> for bool {
  #[inline(always)]
  fn from(variant: IOREGVSEL_A) -> Self {
    match variant {
      IOREGVSEL_A::IOREGVSEL_0 => false,
      IOREGVSEL_A::IOREGVSEL_1 => true,
    }
  }
}
#[doc = "Reader of field `IOREGVSEL`"]
pub type IOREGVSEL_R = crate::R<bool, IOREGVSEL_A>;
impl IOREGVSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IOREGVSEL_A {
    match self.bits {
      false => IOREGVSEL_A::IOREGVSEL_0,
      true => IOREGVSEL_A::IOREGVSEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `IOREGVSEL_0`"]
  #[inline(always)]
  pub fn is_ioregvsel_0(&self) -> bool {
    *self == IOREGVSEL_A::IOREGVSEL_0
  }
  #[doc = "Checks if the value of the field is `IOREGVSEL_1`"]
  #[inline(always)]
  pub fn is_ioregvsel_1(&self) -> bool {
    *self == IOREGVSEL_A::IOREGVSEL_1
  }
}
#[doc = "Write proxy for field `IOREGVSEL`"]
pub struct IOREGVSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> IOREGVSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IOREGVSEL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Regulate to 1.8V."]
  #[inline(always)]
  pub fn ioregvsel_0(self) -> &'a mut W {
    self.variant(IOREGVSEL_A::IOREGVSEL_0)
  }
  #[doc = "Regulate to 1.5V."]
  #[inline(always)]
  pub fn ioregvsel_1(self) -> &'a mut W {
    self.variant(IOREGVSEL_A::IOREGVSEL_1)
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
#[doc = "VDD 1p8 SNS Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD1P8SEL_A {
  #[doc = "0: VDD1p8_SNS0 selected."]
  VDD1P8SEL_0,
  #[doc = "1: VDD1p8_SNS1 selected."]
  VDD1P8SEL_1,
}
impl From<VDD1P8SEL_A> for bool {
  #[inline(always)]
  fn from(variant: VDD1P8SEL_A) -> Self {
    match variant {
      VDD1P8SEL_A::VDD1P8SEL_0 => false,
      VDD1P8SEL_A::VDD1P8SEL_1 => true,
    }
  }
}
#[doc = "Reader of field `VDD1P8SEL`"]
pub type VDD1P8SEL_R = crate::R<bool, VDD1P8SEL_A>;
impl VDD1P8SEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VDD1P8SEL_A {
    match self.bits {
      false => VDD1P8SEL_A::VDD1P8SEL_0,
      true => VDD1P8SEL_A::VDD1P8SEL_1,
    }
  }
  #[doc = "Checks if the value of the field is `VDD1P8SEL_0`"]
  #[inline(always)]
  pub fn is_vdd1p8sel_0(&self) -> bool {
    *self == VDD1P8SEL_A::VDD1P8SEL_0
  }
  #[doc = "Checks if the value of the field is `VDD1P8SEL_1`"]
  #[inline(always)]
  pub fn is_vdd1p8sel_1(&self) -> bool {
    *self == VDD1P8SEL_A::VDD1P8SEL_1
  }
}
#[doc = "Write proxy for field `VDD1P8SEL`"]
pub struct VDD1P8SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> VDD1P8SEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: VDD1P8SEL_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "VDD1p8_SNS0 selected."]
  #[inline(always)]
  pub fn vdd1p8sel_0(self) -> &'a mut W {
    self.variant(VDD1P8SEL_A::VDD1P8SEL_0)
  }
  #[doc = "VDD1p8_SNS1 selected."]
  #[inline(always)]
  pub fn vdd1p8sel_1(self) -> &'a mut W {
    self.variant(VDD1P8SEL_A::VDD1P8SEL_1)
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
#[doc = "ISINKEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISINKEN_A {
  #[doc = "0: Disable current sink feature of low power regulator."]
  ISINKEN_0,
  #[doc = "1: Enable current sink feature of low power regulator."]
  ISINKEN_1,
}
impl From<ISINKEN_A> for bool {
  #[inline(always)]
  fn from(variant: ISINKEN_A) -> Self {
    match variant {
      ISINKEN_A::ISINKEN_0 => false,
      ISINKEN_A::ISINKEN_1 => true,
    }
  }
}
#[doc = "Reader of field `ISINKEN`"]
pub type ISINKEN_R = crate::R<bool, ISINKEN_A>;
impl ISINKEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> ISINKEN_A {
    match self.bits {
      false => ISINKEN_A::ISINKEN_0,
      true => ISINKEN_A::ISINKEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `ISINKEN_0`"]
  #[inline(always)]
  pub fn is_isinken_0(&self) -> bool {
    *self == ISINKEN_A::ISINKEN_0
  }
  #[doc = "Checks if the value of the field is `ISINKEN_1`"]
  #[inline(always)]
  pub fn is_isinken_1(&self) -> bool {
    *self == ISINKEN_A::ISINKEN_1
  }
}
#[doc = "Write proxy for field `ISINKEN`"]
pub struct ISINKEN_W<'a> {
  w: &'a mut W,
}
impl<'a> ISINKEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: ISINKEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable current sink feature of low power regulator."]
  #[inline(always)]
  pub fn isinken_0(self) -> &'a mut W {
    self.variant(ISINKEN_A::ISINKEN_0)
  }
  #[doc = "Enable current sink feature of low power regulator."]
  #[inline(always)]
  pub fn isinken_1(self) -> &'a mut W {
    self.variant(ISINKEN_A::ISINKEN_1)
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
#[doc = "Reader of field `IOTRIM`"]
pub type IOTRIM_R = crate::R<u8, u8>;
#[doc = "IO 1.8 Reg Soft Start Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOSSSEL_A {
  #[doc = "0: Soft Start duration set to 110us."]
  IOSSSEL_0,
  #[doc = "1: Soft Start duration set to 95us."]
  IOSSSEL_1,
  #[doc = "2: Soft Start duration set to 60us."]
  IOSSSEL_2,
  #[doc = "3: Soft Start duration set to 48us."]
  IOSSSEL_3,
  #[doc = "4: Soft Start duration set to 38us."]
  IOSSSEL_4,
  #[doc = "5: Soft Start duration set to 30us."]
  IOSSSEL_5,
  #[doc = "6: Soft Start duration set to 24us."]
  IOSSSEL_6,
  #[doc = "7: Soft Start duration set to 17us."]
  IOSSSEL_7,
}
impl From<IOSSSEL_A> for u8 {
  #[inline(always)]
  fn from(variant: IOSSSEL_A) -> Self {
    match variant {
      IOSSSEL_A::IOSSSEL_0 => 0,
      IOSSSEL_A::IOSSSEL_1 => 1,
      IOSSSEL_A::IOSSSEL_2 => 2,
      IOSSSEL_A::IOSSSEL_3 => 3,
      IOSSSEL_A::IOSSSEL_4 => 4,
      IOSSSEL_A::IOSSSEL_5 => 5,
      IOSSSEL_A::IOSSSEL_6 => 6,
      IOSSSEL_A::IOSSSEL_7 => 7,
    }
  }
}
#[doc = "Reader of field `IOSSSEL`"]
pub type IOSSSEL_R = crate::R<u8, IOSSSEL_A>;
impl IOSSSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IOSSSEL_A {
    match self.bits {
      0 => IOSSSEL_A::IOSSSEL_0,
      1 => IOSSSEL_A::IOSSSEL_1,
      2 => IOSSSEL_A::IOSSSEL_2,
      3 => IOSSSEL_A::IOSSSEL_3,
      4 => IOSSSEL_A::IOSSSEL_4,
      5 => IOSSSEL_A::IOSSSEL_5,
      6 => IOSSSEL_A::IOSSSEL_6,
      7 => IOSSSEL_A::IOSSSEL_7,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `IOSSSEL_0`"]
  #[inline(always)]
  pub fn is_iosssel_0(&self) -> bool {
    *self == IOSSSEL_A::IOSSSEL_0
  }
  #[doc = "Checks if the value of the field is `IOSSSEL_1`"]
  #[inline(always)]
  pub fn is_iosssel_1(&self) -> bool {
    *self == IOSSSEL_A::IOSSSEL_1
  }
  #[doc = "Checks if the value of the field is `IOSSSEL_2`"]
  #[inline(always)]
  pub fn is_iosssel_2(&self) -> bool {
    *self == IOSSSEL_A::IOSSSEL_2
  }
  #[doc = "Checks if the value of the field is `IOSSSEL_3`"]
  #[inline(always)]
  pub fn is_iosssel_3(&self) -> bool {
    *self == IOSSSEL_A::IOSSSEL_3
  }
  #[doc = "Checks if the value of the field is `IOSSSEL_4`"]
  #[inline(always)]
  pub fn is_iosssel_4(&self) -> bool {
    *self == IOSSSEL_A::IOSSSEL_4
  }
  #[doc = "Checks if the value of the field is `IOSSSEL_5`"]
  #[inline(always)]
  pub fn is_iosssel_5(&self) -> bool {
    *self == IOSSSEL_A::IOSSSEL_5
  }
  #[doc = "Checks if the value of the field is `IOSSSEL_6`"]
  #[inline(always)]
  pub fn is_iosssel_6(&self) -> bool {
    *self == IOSSSEL_A::IOSSSEL_6
  }
  #[doc = "Checks if the value of the field is `IOSSSEL_7`"]
  #[inline(always)]
  pub fn is_iosssel_7(&self) -> bool {
    *self == IOSSSEL_A::IOSSSEL_7
  }
}
#[doc = "Write proxy for field `IOSSSEL`"]
pub struct IOSSSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> IOSSSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IOSSSEL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Soft Start duration set to 110us."]
  #[inline(always)]
  pub fn iosssel_0(self) -> &'a mut W {
    self.variant(IOSSSEL_A::IOSSSEL_0)
  }
  #[doc = "Soft Start duration set to 95us."]
  #[inline(always)]
  pub fn iosssel_1(self) -> &'a mut W {
    self.variant(IOSSSEL_A::IOSSSEL_1)
  }
  #[doc = "Soft Start duration set to 60us."]
  #[inline(always)]
  pub fn iosssel_2(self) -> &'a mut W {
    self.variant(IOSSSEL_A::IOSSSEL_2)
  }
  #[doc = "Soft Start duration set to 48us."]
  #[inline(always)]
  pub fn iosssel_3(self) -> &'a mut W {
    self.variant(IOSSSEL_A::IOSSSEL_3)
  }
  #[doc = "Soft Start duration set to 38us."]
  #[inline(always)]
  pub fn iosssel_4(self) -> &'a mut W {
    self.variant(IOSSSEL_A::IOSSSEL_4)
  }
  #[doc = "Soft Start duration set to 30us."]
  #[inline(always)]
  pub fn iosssel_5(self) -> &'a mut W {
    self.variant(IOSSSEL_A::IOSSSEL_5)
  }
  #[doc = "Soft Start duration set to 24us."]
  #[inline(always)]
  pub fn iosssel_6(self) -> &'a mut W {
    self.variant(IOSSSEL_A::IOSSSEL_6)
  }
  #[doc = "Soft Start duration set to 17us."]
  #[inline(always)]
  pub fn iosssel_7(self) -> &'a mut W {
    self.variant(IOSSSEL_A::IOSSSEL_7)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
    self.w
  }
}
#[doc = "Reader of field `SSDONE`"]
pub type SSDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `IOSPARE_OUT`"]
pub type IOSPARE_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOSPARE_OUT`"]
pub struct IOSPARE_OUT_W<'a> {
  w: &'a mut W,
}
impl<'a> IOSPARE_OUT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - IO Regulator Voltage Select"]
  #[inline(always)]
  pub fn ioregvsel(&self) -> IOREGVSEL_R {
    IOREGVSEL_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 4 - VDD 1p8 SNS Pin Select"]
  #[inline(always)]
  pub fn vdd1p8sel(&self) -> VDD1P8SEL_R {
    VDD1P8SEL_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - ISINKEN"]
  #[inline(always)]
  pub fn isinken(&self) -> ISINKEN_R {
    ISINKEN_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bits 8:12 - IO Regulator TRIM value"]
  #[inline(always)]
  pub fn iotrim(&self) -> IOTRIM_R {
    IOTRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
  }
  #[doc = "Bits 16:18 - IO 1.8 Reg Soft Start Select"]
  #[inline(always)]
  pub fn iosssel(&self) -> IOSSSEL_R {
    IOSSSEL_R::new(((self.bits >> 16) & 0x07) as u8)
  }
  #[doc = "Bit 24 - IO Soft Start Done Status Registers"]
  #[inline(always)]
  pub fn ssdone(&self) -> SSDONE_R {
    SSDONE_R::new(((self.bits >> 24) & 0x01) != 0)
  }
  #[doc = "Bits 26:27 - IO Spare Outputs"]
  #[inline(always)]
  pub fn iospare_out(&self) -> IOSPARE_OUT_R {
    IOSPARE_OUT_R::new(((self.bits >> 26) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - IO Regulator Voltage Select"]
  #[inline(always)]
  pub fn ioregvsel(&mut self) -> IOREGVSEL_W {
    IOREGVSEL_W { w: self }
  }
  #[doc = "Bit 4 - VDD 1p8 SNS Pin Select"]
  #[inline(always)]
  pub fn vdd1p8sel(&mut self) -> VDD1P8SEL_W {
    VDD1P8SEL_W { w: self }
  }
  #[doc = "Bit 5 - ISINKEN"]
  #[inline(always)]
  pub fn isinken(&mut self) -> ISINKEN_W {
    ISINKEN_W { w: self }
  }
  #[doc = "Bits 16:18 - IO 1.8 Reg Soft Start Select"]
  #[inline(always)]
  pub fn iosssel(&mut self) -> IOSSSEL_W {
    IOSSSEL_W { w: self }
  }
  #[doc = "Bits 26:27 - IO Spare Outputs"]
  #[inline(always)]
  pub fn iospare_out(&mut self) -> IOSPARE_OUT_W {
    IOSPARE_OUT_W { w: self }
  }
}
