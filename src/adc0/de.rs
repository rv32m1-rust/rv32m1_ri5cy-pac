#[doc = "Reader of register DE"]
pub type R = crate::R<u32, super::DE>;
#[doc = "Writer for register DE"]
pub type W = crate::W<u32, super::DE>;
#[doc = "Register DE `reset()`'s with value 0"]
impl crate::ResetValue for super::DE {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "FIFO Watermark DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWMDE_A {
  #[doc = "0: DMA request disabled."]
  FWMDE_0,
  #[doc = "1: DMA request enabled."]
  FWMDE_1,
}
impl From<FWMDE_A> for bool {
  #[inline(always)]
  fn from(variant: FWMDE_A) -> Self {
    match variant {
      FWMDE_A::FWMDE_0 => false,
      FWMDE_A::FWMDE_1 => true,
    }
  }
}
#[doc = "Reader of field `FWMDE`"]
pub type FWMDE_R = crate::R<bool, FWMDE_A>;
impl FWMDE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FWMDE_A {
    match self.bits {
      false => FWMDE_A::FWMDE_0,
      true => FWMDE_A::FWMDE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FWMDE_0`"]
  #[inline(always)]
  pub fn is_fwmde_0(&self) -> bool {
    *self == FWMDE_A::FWMDE_0
  }
  #[doc = "Checks if the value of the field is `FWMDE_1`"]
  #[inline(always)]
  pub fn is_fwmde_1(&self) -> bool {
    *self == FWMDE_A::FWMDE_1
  }
}
#[doc = "Write proxy for field `FWMDE`"]
pub struct FWMDE_W<'a> {
  w: &'a mut W,
}
impl<'a> FWMDE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FWMDE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "DMA request disabled."]
  #[inline(always)]
  pub fn fwmde_0(self) -> &'a mut W {
    self.variant(FWMDE_A::FWMDE_0)
  }
  #[doc = "DMA request enabled."]
  #[inline(always)]
  pub fn fwmde_1(self) -> &'a mut W {
    self.variant(FWMDE_A::FWMDE_1)
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
impl R {
  #[doc = "Bit 0 - FIFO Watermark DMA Enable"]
  #[inline(always)]
  pub fn fwmde(&self) -> FWMDE_R {
    FWMDE_R::new((self.bits & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - FIFO Watermark DMA Enable"]
  #[inline(always)]
  pub fn fwmde(&mut self) -> FWMDE_W {
    FWMDE_W { w: self }
  }
}
