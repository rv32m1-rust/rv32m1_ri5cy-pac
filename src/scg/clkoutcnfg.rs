#[doc = "Reader of register CLKOUTCNFG"]
pub type R = crate::R<u32, super::CLKOUTCNFG>;
#[doc = "Writer for register CLKOUTCNFG"]
pub type W = crate::W<u32, super::CLKOUTCNFG>;
#[doc = "Register CLKOUTCNFG `reset()`'s with value 0x0300_0000"]
impl crate::ResetValue for super::CLKOUTCNFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0300_0000
  }
}
#[doc = "SCG Clkout Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL_A {
  #[doc = "0: no description available"]
  CLKOUTSEL_0,
  #[doc = "1: no description available"]
  CLKOUTSEL_1,
  #[doc = "2: no description available"]
  CLKOUTSEL_2,
  #[doc = "3: no description available"]
  CLKOUTSEL_3,
  #[doc = "4: no description available"]
  CLKOUTSEL_4,
  #[doc = "5: no description available"]
  CLKOUTSEL_5,
}
impl From<CLKOUTSEL_A> for u8 {
  #[inline(always)]
  fn from(variant: CLKOUTSEL_A) -> Self {
    match variant {
      CLKOUTSEL_A::CLKOUTSEL_0 => 0,
      CLKOUTSEL_A::CLKOUTSEL_1 => 1,
      CLKOUTSEL_A::CLKOUTSEL_2 => 2,
      CLKOUTSEL_A::CLKOUTSEL_3 => 3,
      CLKOUTSEL_A::CLKOUTSEL_4 => 4,
      CLKOUTSEL_A::CLKOUTSEL_5 => 5,
    }
  }
}
#[doc = "Reader of field `CLKOUTSEL`"]
pub type CLKOUTSEL_R = crate::R<u8, CLKOUTSEL_A>;
impl CLKOUTSEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(CLKOUTSEL_A::CLKOUTSEL_0),
      1 => Val(CLKOUTSEL_A::CLKOUTSEL_1),
      2 => Val(CLKOUTSEL_A::CLKOUTSEL_2),
      3 => Val(CLKOUTSEL_A::CLKOUTSEL_3),
      4 => Val(CLKOUTSEL_A::CLKOUTSEL_4),
      5 => Val(CLKOUTSEL_A::CLKOUTSEL_5),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CLKOUTSEL_0`"]
  #[inline(always)]
  pub fn is_clkoutsel_0(&self) -> bool {
    *self == CLKOUTSEL_A::CLKOUTSEL_0
  }
  #[doc = "Checks if the value of the field is `CLKOUTSEL_1`"]
  #[inline(always)]
  pub fn is_clkoutsel_1(&self) -> bool {
    *self == CLKOUTSEL_A::CLKOUTSEL_1
  }
  #[doc = "Checks if the value of the field is `CLKOUTSEL_2`"]
  #[inline(always)]
  pub fn is_clkoutsel_2(&self) -> bool {
    *self == CLKOUTSEL_A::CLKOUTSEL_2
  }
  #[doc = "Checks if the value of the field is `CLKOUTSEL_3`"]
  #[inline(always)]
  pub fn is_clkoutsel_3(&self) -> bool {
    *self == CLKOUTSEL_A::CLKOUTSEL_3
  }
  #[doc = "Checks if the value of the field is `CLKOUTSEL_4`"]
  #[inline(always)]
  pub fn is_clkoutsel_4(&self) -> bool {
    *self == CLKOUTSEL_A::CLKOUTSEL_4
  }
  #[doc = "Checks if the value of the field is `CLKOUTSEL_5`"]
  #[inline(always)]
  pub fn is_clkoutsel_5(&self) -> bool {
    *self == CLKOUTSEL_A::CLKOUTSEL_5
  }
}
#[doc = "Write proxy for field `CLKOUTSEL`"]
pub struct CLKOUTSEL_W<'a> {
  w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn clkoutsel_0(self) -> &'a mut W {
    self.variant(CLKOUTSEL_A::CLKOUTSEL_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn clkoutsel_1(self) -> &'a mut W {
    self.variant(CLKOUTSEL_A::CLKOUTSEL_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn clkoutsel_2(self) -> &'a mut W {
    self.variant(CLKOUTSEL_A::CLKOUTSEL_2)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn clkoutsel_3(self) -> &'a mut W {
    self.variant(CLKOUTSEL_A::CLKOUTSEL_3)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn clkoutsel_4(self) -> &'a mut W {
    self.variant(CLKOUTSEL_A::CLKOUTSEL_4)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn clkoutsel_5(self) -> &'a mut W {
    self.variant(CLKOUTSEL_A::CLKOUTSEL_5)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
    self.w
  }
}
impl R {
  #[doc = "Bits 24:27 - SCG Clkout Select"]
  #[inline(always)]
  pub fn clkoutsel(&self) -> CLKOUTSEL_R {
    CLKOUTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 24:27 - SCG Clkout Select"]
  #[inline(always)]
  pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
    CLKOUTSEL_W { w: self }
  }
}
