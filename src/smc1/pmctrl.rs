#[doc = "Reader of register PMCTRL"]
pub type R = crate::R<u32, super::PMCTRL>;
#[doc = "Writer for register PMCTRL"]
pub type W = crate::W<u32, super::PMCTRL>;
#[doc = "Register PMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PMCTRL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Stop Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPM_A {
  #[doc = "0: Normal Stop (STOP)"]
  STOPM_0,
  #[doc = "2: Very-Low-Power Stop (VLPS)"]
  STOPM_2,
  #[doc = "3: Low-Leakage Stop (LLS)"]
  STOPM_3,
  #[doc = "4: Very-Low-Leakage Stop with SRAM retention(VLLS2/3)"]
  STOPM_4,
  #[doc = "6: Very-Low-Leakage Stop without SRAM retention (VLLS0/1)"]
  STOPM_6,
}
impl From<STOPM_A> for u8 {
  #[inline(always)]
  fn from(variant: STOPM_A) -> Self {
    match variant {
      STOPM_A::STOPM_0 => 0,
      STOPM_A::STOPM_2 => 2,
      STOPM_A::STOPM_3 => 3,
      STOPM_A::STOPM_4 => 4,
      STOPM_A::STOPM_6 => 6,
    }
  }
}
#[doc = "Reader of field `STOPM`"]
pub type STOPM_R = crate::R<u8, STOPM_A>;
impl STOPM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, STOPM_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(STOPM_A::STOPM_0),
      2 => Val(STOPM_A::STOPM_2),
      3 => Val(STOPM_A::STOPM_3),
      4 => Val(STOPM_A::STOPM_4),
      6 => Val(STOPM_A::STOPM_6),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `STOPM_0`"]
  #[inline(always)]
  pub fn is_stopm_0(&self) -> bool {
    *self == STOPM_A::STOPM_0
  }
  #[doc = "Checks if the value of the field is `STOPM_2`"]
  #[inline(always)]
  pub fn is_stopm_2(&self) -> bool {
    *self == STOPM_A::STOPM_2
  }
  #[doc = "Checks if the value of the field is `STOPM_3`"]
  #[inline(always)]
  pub fn is_stopm_3(&self) -> bool {
    *self == STOPM_A::STOPM_3
  }
  #[doc = "Checks if the value of the field is `STOPM_4`"]
  #[inline(always)]
  pub fn is_stopm_4(&self) -> bool {
    *self == STOPM_A::STOPM_4
  }
  #[doc = "Checks if the value of the field is `STOPM_6`"]
  #[inline(always)]
  pub fn is_stopm_6(&self) -> bool {
    *self == STOPM_A::STOPM_6
  }
}
#[doc = "Write proxy for field `STOPM`"]
pub struct STOPM_W<'a> {
  w: &'a mut W,
}
impl<'a> STOPM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STOPM_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Normal Stop (STOP)"]
  #[inline(always)]
  pub fn stopm_0(self) -> &'a mut W {
    self.variant(STOPM_A::STOPM_0)
  }
  #[doc = "Very-Low-Power Stop (VLPS)"]
  #[inline(always)]
  pub fn stopm_2(self) -> &'a mut W {
    self.variant(STOPM_A::STOPM_2)
  }
  #[doc = "Low-Leakage Stop (LLS)"]
  #[inline(always)]
  pub fn stopm_3(self) -> &'a mut W {
    self.variant(STOPM_A::STOPM_3)
  }
  #[doc = "Very-Low-Leakage Stop with SRAM retention(VLLS2/3)"]
  #[inline(always)]
  pub fn stopm_4(self) -> &'a mut W {
    self.variant(STOPM_A::STOPM_4)
  }
  #[doc = "Very-Low-Leakage Stop without SRAM retention (VLLS0/1)"]
  #[inline(always)]
  pub fn stopm_6(self) -> &'a mut W {
    self.variant(STOPM_A::STOPM_6)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
    self.w
  }
}
#[doc = "Run Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNM_A {
  #[doc = "0: Normal Run mode (RUN)"]
  RUNM_0,
  #[doc = "2: Very-Low-Power Run mode (VLPR)"]
  RUNM_2,
  #[doc = "3: High Speed Run mode (HSRUN)"]
  RUNM_3,
}
impl From<RUNM_A> for u8 {
  #[inline(always)]
  fn from(variant: RUNM_A) -> Self {
    match variant {
      RUNM_A::RUNM_0 => 0,
      RUNM_A::RUNM_2 => 2,
      RUNM_A::RUNM_3 => 3,
    }
  }
}
#[doc = "Reader of field `RUNM`"]
pub type RUNM_R = crate::R<u8, RUNM_A>;
impl RUNM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RUNM_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(RUNM_A::RUNM_0),
      2 => Val(RUNM_A::RUNM_2),
      3 => Val(RUNM_A::RUNM_3),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RUNM_0`"]
  #[inline(always)]
  pub fn is_runm_0(&self) -> bool {
    *self == RUNM_A::RUNM_0
  }
  #[doc = "Checks if the value of the field is `RUNM_2`"]
  #[inline(always)]
  pub fn is_runm_2(&self) -> bool {
    *self == RUNM_A::RUNM_2
  }
  #[doc = "Checks if the value of the field is `RUNM_3`"]
  #[inline(always)]
  pub fn is_runm_3(&self) -> bool {
    *self == RUNM_A::RUNM_3
  }
}
#[doc = "Write proxy for field `RUNM`"]
pub struct RUNM_W<'a> {
  w: &'a mut W,
}
impl<'a> RUNM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: RUNM_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Normal Run mode (RUN)"]
  #[inline(always)]
  pub fn runm_0(self) -> &'a mut W {
    self.variant(RUNM_A::RUNM_0)
  }
  #[doc = "Very-Low-Power Run mode (VLPR)"]
  #[inline(always)]
  pub fn runm_2(self) -> &'a mut W {
    self.variant(RUNM_A::RUNM_2)
  }
  #[doc = "High Speed Run mode (HSRUN)"]
  #[inline(always)]
  pub fn runm_3(self) -> &'a mut W {
    self.variant(RUNM_A::RUNM_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
    self.w
  }
}
#[doc = "Partial Stop Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSTOPO_A {
  #[doc = "0: STOP - Normal Stop mode"]
  PSTOPO_0,
  #[doc = "1: PSTOP1 - Partial Stop with system and bus clock disabled"]
  PSTOPO_1,
  #[doc = "2: PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
  PSTOPO_2,
  #[doc = "3: PSTOP3 - Partial Stop with system clock enabled and bus clock enabled"]
  PSTOPO_3,
}
impl From<PSTOPO_A> for u8 {
  #[inline(always)]
  fn from(variant: PSTOPO_A) -> Self {
    match variant {
      PSTOPO_A::PSTOPO_0 => 0,
      PSTOPO_A::PSTOPO_1 => 1,
      PSTOPO_A::PSTOPO_2 => 2,
      PSTOPO_A::PSTOPO_3 => 3,
    }
  }
}
#[doc = "Reader of field `PSTOPO`"]
pub type PSTOPO_R = crate::R<u8, PSTOPO_A>;
impl PSTOPO_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PSTOPO_A {
    match self.bits {
      0 => PSTOPO_A::PSTOPO_0,
      1 => PSTOPO_A::PSTOPO_1,
      2 => PSTOPO_A::PSTOPO_2,
      3 => PSTOPO_A::PSTOPO_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `PSTOPO_0`"]
  #[inline(always)]
  pub fn is_pstopo_0(&self) -> bool {
    *self == PSTOPO_A::PSTOPO_0
  }
  #[doc = "Checks if the value of the field is `PSTOPO_1`"]
  #[inline(always)]
  pub fn is_pstopo_1(&self) -> bool {
    *self == PSTOPO_A::PSTOPO_1
  }
  #[doc = "Checks if the value of the field is `PSTOPO_2`"]
  #[inline(always)]
  pub fn is_pstopo_2(&self) -> bool {
    *self == PSTOPO_A::PSTOPO_2
  }
  #[doc = "Checks if the value of the field is `PSTOPO_3`"]
  #[inline(always)]
  pub fn is_pstopo_3(&self) -> bool {
    *self == PSTOPO_A::PSTOPO_3
  }
}
#[doc = "Write proxy for field `PSTOPO`"]
pub struct PSTOPO_W<'a> {
  w: &'a mut W,
}
impl<'a> PSTOPO_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PSTOPO_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "STOP - Normal Stop mode"]
  #[inline(always)]
  pub fn pstopo_0(self) -> &'a mut W {
    self.variant(PSTOPO_A::PSTOPO_0)
  }
  #[doc = "PSTOP1 - Partial Stop with system and bus clock disabled"]
  #[inline(always)]
  pub fn pstopo_1(self) -> &'a mut W {
    self.variant(PSTOPO_A::PSTOPO_1)
  }
  #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
  #[inline(always)]
  pub fn pstopo_2(self) -> &'a mut W {
    self.variant(PSTOPO_A::PSTOPO_2)
  }
  #[doc = "PSTOP3 - Partial Stop with system clock enabled and bus clock enabled"]
  #[inline(always)]
  pub fn pstopo_3(self) -> &'a mut W {
    self.variant(PSTOPO_A::PSTOPO_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:2 - Stop Mode Control"]
  #[inline(always)]
  pub fn stopm(&self) -> STOPM_R {
    STOPM_R::new((self.bits & 0x07) as u8)
  }
  #[doc = "Bits 8:9 - Run Mode Control"]
  #[inline(always)]
  pub fn runm(&self) -> RUNM_R {
    RUNM_R::new(((self.bits >> 8) & 0x03) as u8)
  }
  #[doc = "Bits 16:17 - Partial Stop Option"]
  #[inline(always)]
  pub fn pstopo(&self) -> PSTOPO_R {
    PSTOPO_R::new(((self.bits >> 16) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:2 - Stop Mode Control"]
  #[inline(always)]
  pub fn stopm(&mut self) -> STOPM_W {
    STOPM_W { w: self }
  }
  #[doc = "Bits 8:9 - Run Mode Control"]
  #[inline(always)]
  pub fn runm(&mut self) -> RUNM_W {
    RUNM_W { w: self }
  }
  #[doc = "Bits 16:17 - Partial Stop Option"]
  #[inline(always)]
  pub fn pstopo(&mut self) -> PSTOPO_W {
    PSTOPO_W { w: self }
  }
}
