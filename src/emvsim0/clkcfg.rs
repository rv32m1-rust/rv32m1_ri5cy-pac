#[doc = "Reader of register CLKCFG"]
pub type R = crate::R<u32, super::CLKCFG>;
#[doc = "Writer for register CLKCFG"]
pub type W = crate::W<u32, super::CLKCFG>;
#[doc = "Register CLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Clock Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_PRSC_A {
  #[doc = "2: Divide by 2"]
  CLK_PRSC_2,
}
impl From<CLK_PRSC_A> for u8 {
  #[inline(always)]
  fn from(variant: CLK_PRSC_A) -> Self {
    match variant {
      CLK_PRSC_A::CLK_PRSC_2 => 2,
    }
  }
}
#[doc = "Reader of field `CLK_PRSC`"]
pub type CLK_PRSC_R = crate::R<u8, CLK_PRSC_A>;
impl CLK_PRSC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CLK_PRSC_A> {
    use crate::Variant::*;
    match self.bits {
      2 => Val(CLK_PRSC_A::CLK_PRSC_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CLK_PRSC_2`"]
  #[inline(always)]
  pub fn is_clk_prsc_2(&self) -> bool {
    *self == CLK_PRSC_A::CLK_PRSC_2
  }
}
#[doc = "Write proxy for field `CLK_PRSC`"]
pub struct CLK_PRSC_W<'a> {
  w: &'a mut W,
}
impl<'a> CLK_PRSC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CLK_PRSC_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Divide by 2"]
  #[inline(always)]
  pub fn clk_prsc_2(self) -> &'a mut W {
    self.variant(CLK_PRSC_A::CLK_PRSC_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
    self.w
  }
}
#[doc = "General Purpose Counter 1 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT1_CLK_SEL_A {
  #[doc = "0: Disabled / Reset (default)"]
  GPCNT1_CLK_SEL_0,
  #[doc = "1: Card Clock"]
  GPCNT1_CLK_SEL_1,
  #[doc = "2: Receive Clock"]
  GPCNT1_CLK_SEL_2,
  #[doc = "3: ETU Clock (transmit clock)"]
  GPCNT1_CLK_SEL_3,
}
impl From<GPCNT1_CLK_SEL_A> for u8 {
  #[inline(always)]
  fn from(variant: GPCNT1_CLK_SEL_A) -> Self {
    match variant {
      GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_0 => 0,
      GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_1 => 1,
      GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_2 => 2,
      GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_3 => 3,
    }
  }
}
#[doc = "Reader of field `GPCNT1_CLK_SEL`"]
pub type GPCNT1_CLK_SEL_R = crate::R<u8, GPCNT1_CLK_SEL_A>;
impl GPCNT1_CLK_SEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GPCNT1_CLK_SEL_A {
    match self.bits {
      0 => GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_0,
      1 => GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_1,
      2 => GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_2,
      3 => GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `GPCNT1_CLK_SEL_0`"]
  #[inline(always)]
  pub fn is_gpcnt1_clk_sel_0(&self) -> bool {
    *self == GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_0
  }
  #[doc = "Checks if the value of the field is `GPCNT1_CLK_SEL_1`"]
  #[inline(always)]
  pub fn is_gpcnt1_clk_sel_1(&self) -> bool {
    *self == GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_1
  }
  #[doc = "Checks if the value of the field is `GPCNT1_CLK_SEL_2`"]
  #[inline(always)]
  pub fn is_gpcnt1_clk_sel_2(&self) -> bool {
    *self == GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_2
  }
  #[doc = "Checks if the value of the field is `GPCNT1_CLK_SEL_3`"]
  #[inline(always)]
  pub fn is_gpcnt1_clk_sel_3(&self) -> bool {
    *self == GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_3
  }
}
#[doc = "Write proxy for field `GPCNT1_CLK_SEL`"]
pub struct GPCNT1_CLK_SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> GPCNT1_CLK_SEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GPCNT1_CLK_SEL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Disabled / Reset (default)"]
  #[inline(always)]
  pub fn gpcnt1_clk_sel_0(self) -> &'a mut W {
    self.variant(GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_0)
  }
  #[doc = "Card Clock"]
  #[inline(always)]
  pub fn gpcnt1_clk_sel_1(self) -> &'a mut W {
    self.variant(GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_1)
  }
  #[doc = "Receive Clock"]
  #[inline(always)]
  pub fn gpcnt1_clk_sel_2(self) -> &'a mut W {
    self.variant(GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_2)
  }
  #[doc = "ETU Clock (transmit clock)"]
  #[inline(always)]
  pub fn gpcnt1_clk_sel_3(self) -> &'a mut W {
    self.variant(GPCNT1_CLK_SEL_A::GPCNT1_CLK_SEL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
    self.w
  }
}
#[doc = "General Purpose Counter 0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPCNT0_CLK_SEL_A {
  #[doc = "0: Disabled / Reset (default)"]
  GPCNT0_CLK_SEL_0,
  #[doc = "1: Card Clock"]
  GPCNT0_CLK_SEL_1,
  #[doc = "2: Receive Clock"]
  GPCNT0_CLK_SEL_2,
  #[doc = "3: ETU Clock (transmit clock)"]
  GPCNT0_CLK_SEL_3,
}
impl From<GPCNT0_CLK_SEL_A> for u8 {
  #[inline(always)]
  fn from(variant: GPCNT0_CLK_SEL_A) -> Self {
    match variant {
      GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_0 => 0,
      GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_1 => 1,
      GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_2 => 2,
      GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_3 => 3,
    }
  }
}
#[doc = "Reader of field `GPCNT0_CLK_SEL`"]
pub type GPCNT0_CLK_SEL_R = crate::R<u8, GPCNT0_CLK_SEL_A>;
impl GPCNT0_CLK_SEL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> GPCNT0_CLK_SEL_A {
    match self.bits {
      0 => GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_0,
      1 => GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_1,
      2 => GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_2,
      3 => GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `GPCNT0_CLK_SEL_0`"]
  #[inline(always)]
  pub fn is_gpcnt0_clk_sel_0(&self) -> bool {
    *self == GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_0
  }
  #[doc = "Checks if the value of the field is `GPCNT0_CLK_SEL_1`"]
  #[inline(always)]
  pub fn is_gpcnt0_clk_sel_1(&self) -> bool {
    *self == GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_1
  }
  #[doc = "Checks if the value of the field is `GPCNT0_CLK_SEL_2`"]
  #[inline(always)]
  pub fn is_gpcnt0_clk_sel_2(&self) -> bool {
    *self == GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_2
  }
  #[doc = "Checks if the value of the field is `GPCNT0_CLK_SEL_3`"]
  #[inline(always)]
  pub fn is_gpcnt0_clk_sel_3(&self) -> bool {
    *self == GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_3
  }
}
#[doc = "Write proxy for field `GPCNT0_CLK_SEL`"]
pub struct GPCNT0_CLK_SEL_W<'a> {
  w: &'a mut W,
}
impl<'a> GPCNT0_CLK_SEL_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: GPCNT0_CLK_SEL_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Disabled / Reset (default)"]
  #[inline(always)]
  pub fn gpcnt0_clk_sel_0(self) -> &'a mut W {
    self.variant(GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_0)
  }
  #[doc = "Card Clock"]
  #[inline(always)]
  pub fn gpcnt0_clk_sel_1(self) -> &'a mut W {
    self.variant(GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_1)
  }
  #[doc = "Receive Clock"]
  #[inline(always)]
  pub fn gpcnt0_clk_sel_2(self) -> &'a mut W {
    self.variant(GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_2)
  }
  #[doc = "ETU Clock (transmit clock)"]
  #[inline(always)]
  pub fn gpcnt0_clk_sel_3(self) -> &'a mut W {
    self.variant(GPCNT0_CLK_SEL_A::GPCNT0_CLK_SEL_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:7 - Clock Prescaler Value"]
  #[inline(always)]
  pub fn clk_prsc(&self) -> CLK_PRSC_R {
    CLK_PRSC_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:9 - General Purpose Counter 1 Clock Select"]
  #[inline(always)]
  pub fn gpcnt1_clk_sel(&self) -> GPCNT1_CLK_SEL_R {
    GPCNT1_CLK_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
  }
  #[doc = "Bits 10:11 - General Purpose Counter 0 Clock Select"]
  #[inline(always)]
  pub fn gpcnt0_clk_sel(&self) -> GPCNT0_CLK_SEL_R {
    GPCNT0_CLK_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:7 - Clock Prescaler Value"]
  #[inline(always)]
  pub fn clk_prsc(&mut self) -> CLK_PRSC_W {
    CLK_PRSC_W { w: self }
  }
  #[doc = "Bits 8:9 - General Purpose Counter 1 Clock Select"]
  #[inline(always)]
  pub fn gpcnt1_clk_sel(&mut self) -> GPCNT1_CLK_SEL_W {
    GPCNT1_CLK_SEL_W { w: self }
  }
  #[doc = "Bits 10:11 - General Purpose Counter 0 Clock Select"]
  #[inline(always)]
  pub fn gpcnt0_clk_sel(&mut self) -> GPCNT0_CLK_SEL_W {
    GPCNT0_CLK_SEL_W { w: self }
  }
}
