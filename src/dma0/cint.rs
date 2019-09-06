#[doc = "Writer for register CINT"]
pub type W = crate::W<u8, super::CINT>;
#[doc = "Register CINT `reset()`'s with value 0"]
impl crate::ResetValue for super::CINT {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `CINT`"]
pub struct CINT_W<'a> {
  w: &'a mut W,
}
impl<'a> CINT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
    self.w
  }
}
#[doc = "Clear All Interrupt Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAIR_AW {
  #[doc = "0: no description available"]
  CAIR_0,
  #[doc = "1: no description available"]
  CAIR_1,
}
impl From<CAIR_AW> for bool {
  #[inline(always)]
  fn from(variant: CAIR_AW) -> Self {
    match variant {
      CAIR_AW::CAIR_0 => false,
      CAIR_AW::CAIR_1 => true,
    }
  }
}
#[doc = "Write proxy for field `CAIR`"]
pub struct CAIR_W<'a> {
  w: &'a mut W,
}
impl<'a> CAIR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CAIR_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn cair_0(self) -> &'a mut W {
    self.variant(CAIR_AW::CAIR_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn cair_1(self) -> &'a mut W {
    self.variant(CAIR_AW::CAIR_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
    self.w
  }
}
#[doc = "No Op enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOP_AW {
  #[doc = "0: Normal operation"]
  NOP_0,
  #[doc = "1: No operation, ignore the other bits in this register"]
  NOP_1,
}
impl From<NOP_AW> for bool {
  #[inline(always)]
  fn from(variant: NOP_AW) -> Self {
    match variant {
      NOP_AW::NOP_0 => false,
      NOP_AW::NOP_1 => true,
    }
  }
}
#[doc = "Write proxy for field `NOP`"]
pub struct NOP_W<'a> {
  w: &'a mut W,
}
impl<'a> NOP_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: NOP_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Normal operation"]
  #[inline(always)]
  pub fn nop_0(self) -> &'a mut W {
    self.variant(NOP_AW::NOP_0)
  }
  #[doc = "No operation, ignore the other bits in this register"]
  #[inline(always)]
  pub fn nop_1(self) -> &'a mut W {
    self.variant(NOP_AW::NOP_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl W {
  #[doc = "Bits 0:3 - Clear Interrupt Request"]
  #[inline(always)]
  pub fn cint(&mut self) -> CINT_W {
    CINT_W { w: self }
  }
  #[doc = "Bit 6 - Clear All Interrupt Requests"]
  #[inline(always)]
  pub fn cair(&mut self) -> CAIR_W {
    CAIR_W { w: self }
  }
  #[doc = "Bit 7 - No Op enable"]
  #[inline(always)]
  pub fn nop(&mut self) -> NOP_W {
    NOP_W { w: self }
  }
}
