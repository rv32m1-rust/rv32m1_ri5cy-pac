#[doc = "Writer for register SEEI"]
pub type W = crate::W<u8, super::SEEI>;
#[doc = "Register SEEI `reset()`'s with value 0"]
impl crate::ResetValue for super::SEEI {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `SEEI`"]
pub struct SEEI_W<'a> {
  w: &'a mut W,
}
impl<'a> SEEI_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
    self.w
  }
}
#[doc = "Sets All Enable Error Interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAEE_AW {
  #[doc = "0: no description available"]
  SAEE_0,
  #[doc = "1: no description available"]
  SAEE_1,
}
impl From<SAEE_AW> for bool {
  #[inline(always)]
  fn from(variant: SAEE_AW) -> Self {
    match variant {
      SAEE_AW::SAEE_0 => false,
      SAEE_AW::SAEE_1 => true,
    }
  }
}
#[doc = "Write proxy for field `SAEE`"]
pub struct SAEE_W<'a> {
  w: &'a mut W,
}
impl<'a> SAEE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SAEE_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn saee_0(self) -> &'a mut W {
    self.variant(SAEE_AW::SAEE_0)
  }
  #[doc = "no description available"]
  #[inline(always)]
  pub fn saee_1(self) -> &'a mut W {
    self.variant(SAEE_AW::SAEE_1)
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
  #[doc = "Bits 0:3 - Set Enable Error Interrupt"]
  #[inline(always)]
  pub fn seei(&mut self) -> SEEI_W {
    SEEI_W { w: self }
  }
  #[doc = "Bit 6 - Sets All Enable Error Interrupts"]
  #[inline(always)]
  pub fn saee(&mut self) -> SAEE_W {
    SAEE_W { w: self }
  }
  #[doc = "Bit 7 - No Op enable"]
  #[inline(always)]
  pub fn nop(&mut self) -> NOP_W {
    NOP_W { w: self }
  }
}
