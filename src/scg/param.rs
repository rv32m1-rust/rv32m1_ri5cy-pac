#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Clock Present\n\nValue on reset: 62"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKPRES_A {
  #[doc = "2: System OSC (SOSC) is present."]
  CLKPRES_2,
}
impl From<CLKPRES_A> for u8 {
  #[inline(always)]
  fn from(variant: CLKPRES_A) -> Self {
    match variant {
      CLKPRES_A::CLKPRES_2 => 2,
    }
  }
}
#[doc = "Reader of field `CLKPRES`"]
pub type CLKPRES_R = crate::R<u8, CLKPRES_A>;
impl CLKPRES_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, CLKPRES_A> {
    use crate::Variant::*;
    match self.bits {
      2 => Val(CLKPRES_A::CLKPRES_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `CLKPRES_2`"]
  #[inline(always)]
  pub fn is_clkpres_2(&self) -> bool {
    *self == CLKPRES_A::CLKPRES_2
  }
}
#[doc = "Divider Present\n\nValue on reset: 23"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVPRES_A {
  #[doc = "1: System DIVSLOW is present."]
  DIVPRES_1,
}
impl From<DIVPRES_A> for u8 {
  #[inline(always)]
  fn from(variant: DIVPRES_A) -> Self {
    match variant {
      DIVPRES_A::DIVPRES_1 => 1,
    }
  }
}
#[doc = "Reader of field `DIVPRES`"]
pub type DIVPRES_R = crate::R<u8, DIVPRES_A>;
impl DIVPRES_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, DIVPRES_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(DIVPRES_A::DIVPRES_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `DIVPRES_1`"]
  #[inline(always)]
  pub fn is_divpres_1(&self) -> bool {
    *self == DIVPRES_A::DIVPRES_1
  }
}
impl R {
  #[doc = "Bits 0:7 - Clock Present"]
  #[inline(always)]
  pub fn clkpres(&self) -> CLKPRES_R {
    CLKPRES_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 27:31 - Divider Present"]
  #[inline(always)]
  pub fn divpres(&self) -> DIVPRES_R {
    DIVPRES_R::new(((self.bits >> 27) & 0x1f) as u8)
  }
}
