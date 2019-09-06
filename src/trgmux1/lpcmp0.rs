#[doc = "Reader of register LPCMP0"]
pub type R = crate::R<u32, super::LPCMP0>;
#[doc = "Writer for register LPCMP0"]
pub type W = crate::W<u32, super::LPCMP0>;
#[doc = "Register LPCMP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LPCMP0 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `SEL0`"]
pub type SEL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL0`"]
pub struct SEL0_W<'a> {
  w: &'a mut W,
}
impl<'a> SEL0_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
    self.w
  }
}
#[doc = "TRGMUX register lock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LK_A {
  #[doc = "0: Register can be written."]
  UNLOCKED,
  #[doc = "1: Register cannot be written until the next system Reset."]
  LOCKED,
}
impl From<LK_A> for bool {
  #[inline(always)]
  fn from(variant: LK_A) -> Self {
    match variant {
      LK_A::UNLOCKED => false,
      LK_A::LOCKED => true,
    }
  }
}
#[doc = "Reader of field `LK`"]
pub type LK_R = crate::R<bool, LK_A>;
impl LK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LK_A {
    match self.bits {
      false => LK_A::UNLOCKED,
      true => LK_A::LOCKED,
    }
  }
  #[doc = "Checks if the value of the field is `UNLOCKED`"]
  #[inline(always)]
  pub fn is_unlocked(&self) -> bool {
    *self == LK_A::UNLOCKED
  }
  #[doc = "Checks if the value of the field is `LOCKED`"]
  #[inline(always)]
  pub fn is_locked(&self) -> bool {
    *self == LK_A::LOCKED
  }
}
#[doc = "Write proxy for field `LK`"]
pub struct LK_W<'a> {
  w: &'a mut W,
}
impl<'a> LK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Register can be written."]
  #[inline(always)]
  pub fn unlocked(self) -> &'a mut W {
    self.variant(LK_A::UNLOCKED)
  }
  #[doc = "Register cannot be written until the next system Reset."]
  #[inline(always)]
  pub fn locked(self) -> &'a mut W {
    self.variant(LK_A::LOCKED)
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
    self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:5 - Trigger MUX Input 0 Source Select"]
  #[inline(always)]
  pub fn sel0(&self) -> SEL0_R {
    SEL0_R::new((self.bits & 0x3f) as u8)
  }
  #[doc = "Bit 31 - TRGMUX register lock."]
  #[inline(always)]
  pub fn lk(&self) -> LK_R {
    LK_R::new(((self.bits >> 31) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:5 - Trigger MUX Input 0 Source Select"]
  #[inline(always)]
  pub fn sel0(&mut self) -> SEL0_W {
    SEL0_W { w: self }
  }
  #[doc = "Bit 31 - TRGMUX register lock."]
  #[inline(always)]
  pub fn lk(&mut self) -> LK_W {
    LK_W { w: self }
  }
}
