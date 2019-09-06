#[doc = "Reader of register KEEP_ALIVE_WKCTRL"]
pub type R = crate::R<u8, super::KEEP_ALIVE_WKCTRL>;
#[doc = "Writer for register KEEP_ALIVE_WKCTRL"]
pub type W = crate::W<u8, super::KEEP_ALIVE_WKCTRL>;
#[doc = "Register KEEP_ALIVE_WKCTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::KEEP_ALIVE_WKCTRL {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x01
  }
}
#[doc = "WAKE_ON_THIS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_ON_THIS_A {
  #[doc = "1: Wake up after receiving OUT/SETUP token packet."]
  WAKE_ON_THIS_1,
  #[doc = "13: no description available"]
  WAKE_ON_THIS_13,
}
impl From<WAKE_ON_THIS_A> for u8 {
  #[inline(always)]
  fn from(variant: WAKE_ON_THIS_A) -> Self {
    match variant {
      WAKE_ON_THIS_A::WAKE_ON_THIS_1 => 1,
      WAKE_ON_THIS_A::WAKE_ON_THIS_13 => 13,
    }
  }
}
#[doc = "Reader of field `WAKE_ON_THIS`"]
pub type WAKE_ON_THIS_R = crate::R<u8, WAKE_ON_THIS_A>;
impl WAKE_ON_THIS_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WAKE_ON_THIS_A> {
    use crate::Variant::*;
    match self.bits {
      1 => Val(WAKE_ON_THIS_A::WAKE_ON_THIS_1),
      13 => Val(WAKE_ON_THIS_A::WAKE_ON_THIS_13),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WAKE_ON_THIS_1`"]
  #[inline(always)]
  pub fn is_wake_on_this_1(&self) -> bool {
    *self == WAKE_ON_THIS_A::WAKE_ON_THIS_1
  }
  #[doc = "Checks if the value of the field is `WAKE_ON_THIS_13`"]
  #[inline(always)]
  pub fn is_wake_on_this_13(&self) -> bool {
    *self == WAKE_ON_THIS_A::WAKE_ON_THIS_13
  }
}
#[doc = "Write proxy for field `WAKE_ON_THIS`"]
pub struct WAKE_ON_THIS_W<'a> {
  w: &'a mut W,
}
impl<'a> WAKE_ON_THIS_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WAKE_ON_THIS_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Wake up after receiving OUT/SETUP token packet."]
  #[inline(always)]
  pub fn wake_on_this_1(self) -> &'a mut W {
    self.variant(WAKE_ON_THIS_A::WAKE_ON_THIS_1)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn wake_on_this_13(self) -> &'a mut W {
    self.variant(WAKE_ON_THIS_A::WAKE_ON_THIS_13)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
    self.w
  }
}
#[doc = "Reader of field `WAKE_ENDPT`"]
pub type WAKE_ENDPT_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:3 - WAKE_ON_THIS"]
  #[inline(always)]
  pub fn wake_on_this(&self) -> WAKE_ON_THIS_R {
    WAKE_ON_THIS_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 4:7 - WAKE_ENDPT"]
  #[inline(always)]
  pub fn wake_endpt(&self) -> WAKE_ENDPT_R {
    WAKE_ENDPT_R::new(((self.bits >> 4) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:3 - WAKE_ON_THIS"]
  #[inline(always)]
  pub fn wake_on_this(&mut self) -> WAKE_ON_THIS_W {
    WAKE_ON_THIS_W { w: self }
  }
}
