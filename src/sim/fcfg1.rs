#[doc = "Reader of register FCFG1"]
pub type R = crate::R<u32, super::FCFG1>;
#[doc = "Writer for register FCFG1"]
pub type W = crate::W<u32, super::FCFG1>;
#[doc = "Register FCFG1 `reset()`'s with value 0xcaa9_0400"]
impl crate::ResetValue for super::FCFG1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0xcaa9_0400
  }
}
#[doc = "Flash disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHDIS_A {
  #[doc = "0: Flash is enabled"]
  FLASHDIS_0,
  #[doc = "1: Flash is disabled"]
  FLASHDIS_1,
}
impl From<FLASHDIS_A> for bool {
  #[inline(always)]
  fn from(variant: FLASHDIS_A) -> Self {
    match variant {
      FLASHDIS_A::FLASHDIS_0 => false,
      FLASHDIS_A::FLASHDIS_1 => true,
    }
  }
}
#[doc = "Reader of field `FLASHDIS`"]
pub type FLASHDIS_R = crate::R<bool, FLASHDIS_A>;
impl FLASHDIS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FLASHDIS_A {
    match self.bits {
      false => FLASHDIS_A::FLASHDIS_0,
      true => FLASHDIS_A::FLASHDIS_1,
    }
  }
  #[doc = "Checks if the value of the field is `FLASHDIS_0`"]
  #[inline(always)]
  pub fn is_flashdis_0(&self) -> bool {
    *self == FLASHDIS_A::FLASHDIS_0
  }
  #[doc = "Checks if the value of the field is `FLASHDIS_1`"]
  #[inline(always)]
  pub fn is_flashdis_1(&self) -> bool {
    *self == FLASHDIS_A::FLASHDIS_1
  }
}
#[doc = "Write proxy for field `FLASHDIS`"]
pub struct FLASHDIS_W<'a> {
  w: &'a mut W,
}
impl<'a> FLASHDIS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FLASHDIS_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Flash is enabled"]
  #[inline(always)]
  pub fn flashdis_0(self) -> &'a mut W {
    self.variant(FLASHDIS_A::FLASHDIS_0)
  }
  #[doc = "Flash is disabled"]
  #[inline(always)]
  pub fn flashdis_1(self) -> &'a mut W {
    self.variant(FLASHDIS_A::FLASHDIS_1)
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
#[doc = "Flash Doze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHDOZE_A {
  #[doc = "0: Flash remains enabled during Doze mode"]
  FLASHDOZE_0,
  #[doc = "1: Flash is disabled for the duration of Doze mode"]
  FLASHDOZE_1,
}
impl From<FLASHDOZE_A> for bool {
  #[inline(always)]
  fn from(variant: FLASHDOZE_A) -> Self {
    match variant {
      FLASHDOZE_A::FLASHDOZE_0 => false,
      FLASHDOZE_A::FLASHDOZE_1 => true,
    }
  }
}
#[doc = "Reader of field `FLASHDOZE`"]
pub type FLASHDOZE_R = crate::R<bool, FLASHDOZE_A>;
impl FLASHDOZE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FLASHDOZE_A {
    match self.bits {
      false => FLASHDOZE_A::FLASHDOZE_0,
      true => FLASHDOZE_A::FLASHDOZE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FLASHDOZE_0`"]
  #[inline(always)]
  pub fn is_flashdoze_0(&self) -> bool {
    *self == FLASHDOZE_A::FLASHDOZE_0
  }
  #[doc = "Checks if the value of the field is `FLASHDOZE_1`"]
  #[inline(always)]
  pub fn is_flashdoze_1(&self) -> bool {
    *self == FLASHDOZE_A::FLASHDOZE_1
  }
}
#[doc = "Write proxy for field `FLASHDOZE`"]
pub struct FLASHDOZE_W<'a> {
  w: &'a mut W,
}
impl<'a> FLASHDOZE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FLASHDOZE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Flash remains enabled during Doze mode"]
  #[inline(always)]
  pub fn flashdoze_0(self) -> &'a mut W {
    self.variant(FLASHDOZE_A::FLASHDOZE_0)
  }
  #[doc = "Flash is disabled for the duration of Doze mode"]
  #[inline(always)]
  pub fn flashdoze_1(self) -> &'a mut W {
    self.variant(FLASHDOZE_A::FLASHDOZE_1)
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
#[doc = "Flash auto disable enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLSAUTODISEN_A {
  #[doc = "0: Disable flash auto disable function"]
  FLSAUTODISEN_0,
  #[doc = "1: Enable flash auto disable function"]
  FLSAUTODISEN_1,
}
impl From<FLSAUTODISEN_A> for bool {
  #[inline(always)]
  fn from(variant: FLSAUTODISEN_A) -> Self {
    match variant {
      FLSAUTODISEN_A::FLSAUTODISEN_0 => false,
      FLSAUTODISEN_A::FLSAUTODISEN_1 => true,
    }
  }
}
#[doc = "Reader of field `FLSAUTODISEN`"]
pub type FLSAUTODISEN_R = crate::R<bool, FLSAUTODISEN_A>;
impl FLSAUTODISEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FLSAUTODISEN_A {
    match self.bits {
      false => FLSAUTODISEN_A::FLSAUTODISEN_0,
      true => FLSAUTODISEN_A::FLSAUTODISEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `FLSAUTODISEN_0`"]
  #[inline(always)]
  pub fn is_flsautodisen_0(&self) -> bool {
    *self == FLSAUTODISEN_A::FLSAUTODISEN_0
  }
  #[doc = "Checks if the value of the field is `FLSAUTODISEN_1`"]
  #[inline(always)]
  pub fn is_flsautodisen_1(&self) -> bool {
    *self == FLSAUTODISEN_A::FLSAUTODISEN_1
  }
}
#[doc = "Write proxy for field `FLSAUTODISEN`"]
pub struct FLSAUTODISEN_W<'a> {
  w: &'a mut W,
}
impl<'a> FLSAUTODISEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FLSAUTODISEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Disable flash auto disable function"]
  #[inline(always)]
  pub fn flsautodisen_0(self) -> &'a mut W {
    self.variant(FLSAUTODISEN_A::FLSAUTODISEN_0)
  }
  #[doc = "Enable flash auto disable function"]
  #[inline(always)]
  pub fn flsautodisen_1(self) -> &'a mut W {
    self.variant(FLSAUTODISEN_A::FLSAUTODISEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
#[doc = "Reader of field `FLSAUTODISWD`"]
pub type FLSAUTODISWD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLSAUTODISWD`"]
pub struct FLSAUTODISWD_W<'a> {
  w: &'a mut W,
}
impl<'a> FLSAUTODISWD_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x07ff << 3)) | (((value as u32) & 0x07ff) << 3);
    self.w
  }
}
#[doc = "The SRAM size for core1 (CM0+)\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE1_SRAMSIZE_A {
  #[doc = "9: CM0+ has 128 KB SRAM"]
  CORE1_SRAMSIZE_9,
}
impl From<CORE1_SRAMSIZE_A> for u8 {
  #[inline(always)]
  fn from(variant: CORE1_SRAMSIZE_A) -> Self {
    match variant {
      CORE1_SRAMSIZE_A::CORE1_SRAMSIZE_9 => 9,
    }
  }
}
#[doc = "Reader of field `CORE1_SRAMSIZE`"]
pub type CORE1_SRAMSIZE_R = crate::R<u8, CORE1_SRAMSIZE_A>;
impl CORE1_SRAMSIZE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CORE1_SRAMSIZE_A> {
    use crate::Variant::*;
    match self.bits {
      9 => Val(CORE1_SRAMSIZE_A::CORE1_SRAMSIZE_9),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CORE1_SRAMSIZE_9`"]
  #[inline(always)]
  pub fn is_core1_sramsize_9(&self) -> bool {
    *self == CORE1_SRAMSIZE_A::CORE1_SRAMSIZE_9
  }
}
#[doc = "The SRAM size for core0 (CM4)\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE0_SRAMSIZE_A {
  #[doc = "10: CM4 has 256 KB SRAM"]
  CORE0_SRAMSIZE_10,
}
impl From<CORE0_SRAMSIZE_A> for u8 {
  #[inline(always)]
  fn from(variant: CORE0_SRAMSIZE_A) -> Self {
    match variant {
      CORE0_SRAMSIZE_A::CORE0_SRAMSIZE_10 => 10,
    }
  }
}
#[doc = "Reader of field `CORE0_SRAMSIZE`"]
pub type CORE0_SRAMSIZE_R = crate::R<u8, CORE0_SRAMSIZE_A>;
impl CORE0_SRAMSIZE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CORE0_SRAMSIZE_A> {
    use crate::Variant::*;
    match self.bits {
      10 => Val(CORE0_SRAMSIZE_A::CORE0_SRAMSIZE_10),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CORE0_SRAMSIZE_10`"]
  #[inline(always)]
  pub fn is_core0_sramsize_10(&self) -> bool {
    *self == CORE0_SRAMSIZE_A::CORE0_SRAMSIZE_10
  }
}
#[doc = "The flash size for core1 (CM0+)\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE1_PFSIZE_A {
  #[doc = "10: CM0+ has 256 KB flash size."]
  CORE1_PFSIZE_10,
}
impl From<CORE1_PFSIZE_A> for u8 {
  #[inline(always)]
  fn from(variant: CORE1_PFSIZE_A) -> Self {
    match variant {
      CORE1_PFSIZE_A::CORE1_PFSIZE_10 => 10,
    }
  }
}
#[doc = "Reader of field `CORE1_PFSIZE`"]
pub type CORE1_PFSIZE_R = crate::R<u8, CORE1_PFSIZE_A>;
impl CORE1_PFSIZE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CORE1_PFSIZE_A> {
    use crate::Variant::*;
    match self.bits {
      10 => Val(CORE1_PFSIZE_A::CORE1_PFSIZE_10),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CORE1_PFSIZE_10`"]
  #[inline(always)]
  pub fn is_core1_pfsize_10(&self) -> bool {
    *self == CORE1_PFSIZE_A::CORE1_PFSIZE_10
  }
}
#[doc = "The flash size for core0 (CM4)\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE0_PFSIZE_A {
  #[doc = "12: CM4 has 1 MB flash size."]
  CORE0_PFSIZE_12,
}
impl From<CORE0_PFSIZE_A> for u8 {
  #[inline(always)]
  fn from(variant: CORE0_PFSIZE_A) -> Self {
    match variant {
      CORE0_PFSIZE_A::CORE0_PFSIZE_12 => 12,
    }
  }
}
#[doc = "Reader of field `CORE0_PFSIZE`"]
pub type CORE0_PFSIZE_R = crate::R<u8, CORE0_PFSIZE_A>;
impl CORE0_PFSIZE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CORE0_PFSIZE_A> {
    use crate::Variant::*;
    match self.bits {
      12 => Val(CORE0_PFSIZE_A::CORE0_PFSIZE_12),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CORE0_PFSIZE_12`"]
  #[inline(always)]
  pub fn is_core0_pfsize_12(&self) -> bool {
    *self == CORE0_PFSIZE_A::CORE0_PFSIZE_12
  }
}
impl R {
  #[doc = "Bit 0 - Flash disable"]
  #[inline(always)]
  pub fn flashdis(&self) -> FLASHDIS_R {
    FLASHDIS_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Flash Doze"]
  #[inline(always)]
  pub fn flashdoze(&self) -> FLASHDOZE_R {
    FLASHDOZE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Flash auto disable enabled."]
  #[inline(always)]
  pub fn flsautodisen(&self) -> FLSAUTODISEN_R {
    FLSAUTODISEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bits 3:13 - The clock counter for time period of flash auto disable."]
  #[inline(always)]
  pub fn flsautodiswd(&self) -> FLSAUTODISWD_R {
    FLSAUTODISWD_R::new(((self.bits >> 3) & 0x07ff) as u16)
  }
  #[doc = "Bits 16:19 - The SRAM size for core1 (CM0+)"]
  #[inline(always)]
  pub fn core1_sramsize(&self) -> CORE1_SRAMSIZE_R {
    CORE1_SRAMSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
  #[doc = "Bits 20:23 - The SRAM size for core0 (CM4)"]
  #[inline(always)]
  pub fn core0_sramsize(&self) -> CORE0_SRAMSIZE_R {
    CORE0_SRAMSIZE_R::new(((self.bits >> 20) & 0x0f) as u8)
  }
  #[doc = "Bits 24:27 - The flash size for core1 (CM0+)"]
  #[inline(always)]
  pub fn core1_pfsize(&self) -> CORE1_PFSIZE_R {
    CORE1_PFSIZE_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
  #[doc = "Bits 28:31 - The flash size for core0 (CM4)"]
  #[inline(always)]
  pub fn core0_pfsize(&self) -> CORE0_PFSIZE_R {
    CORE0_PFSIZE_R::new(((self.bits >> 28) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 0 - Flash disable"]
  #[inline(always)]
  pub fn flashdis(&mut self) -> FLASHDIS_W {
    FLASHDIS_W { w: self }
  }
  #[doc = "Bit 1 - Flash Doze"]
  #[inline(always)]
  pub fn flashdoze(&mut self) -> FLASHDOZE_W {
    FLASHDOZE_W { w: self }
  }
  #[doc = "Bit 2 - Flash auto disable enabled."]
  #[inline(always)]
  pub fn flsautodisen(&mut self) -> FLSAUTODISEN_W {
    FLSAUTODISEN_W { w: self }
  }
  #[doc = "Bits 3:13 - The clock counter for time period of flash auto disable."]
  #[inline(always)]
  pub fn flsautodiswd(&mut self) -> FLSAUTODISWD_W {
    FLSAUTODISWD_W { w: self }
  }
}
