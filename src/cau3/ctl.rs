#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM_A {
  #[doc = "0: Interrupt not masked."]
  INT_NOT_MASKED,
  #[doc = "1: Interrupt masked"]
  INT_MASKED,
}
impl From<IM_A> for bool {
  #[inline(always)]
  fn from(variant: IM_A) -> Self {
    match variant {
      IM_A::INT_NOT_MASKED => false,
      IM_A::INT_MASKED => true,
    }
  }
}
#[doc = "Reader of field `IM`"]
pub type IM_R = crate::R<bool, IM_A>;
impl IM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> IM_A {
    match self.bits {
      false => IM_A::INT_NOT_MASKED,
      true => IM_A::INT_MASKED,
    }
  }
  #[doc = "Checks if the value of the field is `INT_NOT_MASKED`"]
  #[inline(always)]
  pub fn is_int_not_masked(&self) -> bool {
    *self == IM_A::INT_NOT_MASKED
  }
  #[doc = "Checks if the value of the field is `INT_MASKED`"]
  #[inline(always)]
  pub fn is_int_masked(&self) -> bool {
    *self == IM_A::INT_MASKED
  }
}
#[doc = "Write proxy for field `IM`"]
pub struct IM_W<'a> {
  w: &'a mut W,
}
impl<'a> IM_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: IM_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Interrupt not masked."]
  #[inline(always)]
  pub fn int_not_masked(self) -> &'a mut W {
    self.variant(IM_A::INT_NOT_MASKED)
  }
  #[doc = "Interrupt masked"]
  #[inline(always)]
  pub fn int_masked(self) -> &'a mut W {
    self.variant(IM_A::INT_MASKED)
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
#[doc = "PKHA Register DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDE_A {
  #[doc = "0: DMA Request and Done signals disabled for the PKHA Registers."]
  PDE_DISABLED,
  #[doc = "1: DMA Request and Done signals enabled for the PKHA Registers."]
  PDE_ENABLED,
}
impl From<PDE_A> for bool {
  #[inline(always)]
  fn from(variant: PDE_A) -> Self {
    match variant {
      PDE_A::PDE_DISABLED => false,
      PDE_A::PDE_ENABLED => true,
    }
  }
}
#[doc = "Reader of field `PDE`"]
pub type PDE_R = crate::R<bool, PDE_A>;
impl PDE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> PDE_A {
    match self.bits {
      false => PDE_A::PDE_DISABLED,
      true => PDE_A::PDE_ENABLED,
    }
  }
  #[doc = "Checks if the value of the field is `PDE_DISABLED`"]
  #[inline(always)]
  pub fn is_pde_disabled(&self) -> bool {
    *self == PDE_A::PDE_DISABLED
  }
  #[doc = "Checks if the value of the field is `PDE_ENABLED`"]
  #[inline(always)]
  pub fn is_pde_enabled(&self) -> bool {
    *self == PDE_A::PDE_ENABLED
  }
}
#[doc = "Write proxy for field `PDE`"]
pub struct PDE_W<'a> {
  w: &'a mut W,
}
impl<'a> PDE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: PDE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "DMA Request and Done signals disabled for the PKHA Registers."]
  #[inline(always)]
  pub fn pde_disabled(self) -> &'a mut W {
    self.variant(PDE_A::PDE_DISABLED)
  }
  #[doc = "DMA Request and Done signals enabled for the PKHA Registers."]
  #[inline(always)]
  pub fn pde_enabled(self) -> &'a mut W {
    self.variant(PDE_A::PDE_ENABLED)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Interrupt Mask"]
  #[inline(always)]
  pub fn im(&self) -> IM_R {
    IM_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 4 - PKHA Register DMA Enable"]
  #[inline(always)]
  pub fn pde(&self) -> PDE_R {
    PDE_R::new(((self.bits >> 4) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Interrupt Mask"]
  #[inline(always)]
  pub fn im(&mut self) -> IM_W {
    IM_W { w: self }
  }
  #[doc = "Bit 4 - PKHA Register DMA Enable"]
  #[inline(always)]
  pub fn pde(&mut self) -> PDE_W {
    PDE_W { w: self }
  }
}
