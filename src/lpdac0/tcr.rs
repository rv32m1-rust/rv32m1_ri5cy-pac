#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Software Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRG_AW {
  #[doc = "0: The DAC soft trigger is not valid."]
  SWTRG_0,
  #[doc = "1: The DAC soft trigger is valid."]
  SWTRG_1,
}
impl From<SWTRG_AW> for bool {
  #[inline(always)]
  fn from(variant: SWTRG_AW) -> Self {
    match variant {
      SWTRG_AW::SWTRG_0 => false,
      SWTRG_AW::SWTRG_1 => true,
    }
  }
}
#[doc = "Write proxy for field `SWTRG`"]
pub struct SWTRG_W<'a> {
  w: &'a mut W,
}
impl<'a> SWTRG_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWTRG_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "The DAC soft trigger is not valid."]
  #[inline(always)]
  pub fn swtrg_0(self) -> &'a mut W {
    self.variant(SWTRG_AW::SWTRG_0)
  }
  #[doc = "The DAC soft trigger is valid."]
  #[inline(always)]
  pub fn swtrg_1(self) -> &'a mut W {
    self.variant(SWTRG_AW::SWTRG_1)
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
impl R {}
impl W {
  #[doc = "Bit 0 - Software Trigger"]
  #[inline(always)]
  pub fn swtrg(&mut self) -> SWTRG_W {
    SWTRG_W { w: self }
  }
}
