#[doc = "Writer for register CERQ"]
pub type W = crate::W<u8, super::CERQ>;
#[doc = "Register CERQ `reset()`'s with value 0"]
impl crate::ResetValue for super::CERQ {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `CERQ`"]
pub struct CERQ_W<'a> {
  w: &'a mut W,
}
impl<'a> CERQ_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
    self.w
  }
}
#[doc = "Clear All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAER_AW {
  #[doc = "0: no description available"]
  CAER_0,
  #[doc = "1: no description available"]
  CAER_1,
}
impl From<CAER_AW> for bool {
  #[inline(always)]
  fn from(variant: CAER_AW) -> Self {
    match variant {
      CAER_AW::CAER_0 => false,
      CAER_AW::CAER_1 => true,
    }
  }
}
#[doc = "Write proxy for field `CAER`"]
pub struct CAER_W<'a> {
  w: &'a mut W,
}
impl<'a> CAER_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: CAER_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn caer_0(self) -> &'a mut W {
    self.variant(CAER_AW::CAER_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn caer_1(self) -> &'a mut W {
    self.variant(CAER_AW::CAER_1)
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
  #[doc = "Bits 0:3 - Clear Enable Request"]
  #[inline(always)]
  pub fn cerq(&mut self) -> CERQ_W {
    CERQ_W { w: self }
  }
  #[doc = "Bit 6 - Clear All Enable Requests"]
  #[inline(always)]
  pub fn caer(&mut self) -> CAER_W {
    CAER_W { w: self }
  }
  #[doc = "Bit 7 - No Op enable"]
  #[inline(always)]
  pub fn nop(&mut self) -> NOP_W {
    NOP_W { w: self }
  }
}
