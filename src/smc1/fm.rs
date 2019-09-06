#[doc = "Reader of register FM"]
pub type R = crate::R<u32, super::FM>;
#[doc = "Writer for register FM"]
pub type W = crate::W<u32, super::FM>;
#[doc = "Register FM `reset()`'s with value 0"]
impl crate::ResetValue for super::FM {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Boot Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCECFG_A {
  #[doc = "0: No effect."]
  FORCECFG_0,
  #[doc = "1: Assert corresponding bit in Mode Register on next system reset."]
  FORCECFG_1,
}
impl From<FORCECFG_A> for u8 {
  #[inline(always)]
  fn from(variant: FORCECFG_A) -> Self {
    match variant {
      FORCECFG_A::FORCECFG_0 => 0,
      FORCECFG_A::FORCECFG_1 => 1,
    }
  }
}
#[doc = "Reader of field `FORCECFG`"]
pub type FORCECFG_R = crate::R<u8, FORCECFG_A>;
impl FORCECFG_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FORCECFG_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FORCECFG_A::FORCECFG_0),
      1 => Val(FORCECFG_A::FORCECFG_1),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FORCECFG_0`"]
  #[inline(always)]
  pub fn is_forcecfg_0(&self) -> bool {
    *self == FORCECFG_A::FORCECFG_0
  }
  #[doc = "Checks if the value of the field is `FORCECFG_1`"]
  #[inline(always)]
  pub fn is_forcecfg_1(&self) -> bool {
    *self == FORCECFG_A::FORCECFG_1
  }
}
#[doc = "Write proxy for field `FORCECFG`"]
pub struct FORCECFG_W<'a> {
  w: &'a mut W,
}
impl<'a> FORCECFG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FORCECFG_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "No effect."]
  #[inline(always)]
  pub fn forcecfg_0(self) -> &'a mut W {
    self.variant(FORCECFG_A::FORCECFG_0)
  }
  #[doc = "Assert corresponding bit in Mode Register on next system reset."]
  #[inline(always)]
  pub fn forcecfg_1(self) -> &'a mut W {
    self.variant(FORCECFG_A::FORCECFG_1)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Boot Configuration"]
  #[inline(always)]
  pub fn forcecfg(&self) -> FORCECFG_R {
    FORCECFG_R::new((self.bits & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Boot Configuration"]
  #[inline(always)]
  pub fn forcecfg(&mut self) -> FORCECFG_W {
    FORCECFG_W { w: self }
  }
}
