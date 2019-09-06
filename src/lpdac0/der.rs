#[doc = "Reader of register DER"]
pub type R = crate::R<u32, super::DER>;
#[doc = "Writer for register DER"]
pub type W = crate::W<u32, super::DER>;
#[doc = "Register DER `reset()`'s with value 0"]
impl crate::ResetValue for super::DER {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "FIFO Empty DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPTY_DMAEN_A {
  #[doc = "0: FIFO Empty DMA request is disabled."]
  EMPTY_DMAEN_0,
  #[doc = "1: FIFO Empty DMA request is enabled."]
  EMPTY_DMAEN_1,
}
impl From<EMPTY_DMAEN_A> for bool {
  #[inline(always)]
  fn from(variant: EMPTY_DMAEN_A) -> Self {
    match variant {
      EMPTY_DMAEN_A::EMPTY_DMAEN_0 => false,
      EMPTY_DMAEN_A::EMPTY_DMAEN_1 => true,
    }
  }
}
#[doc = "Reader of field `EMPTY_DMAEN`"]
pub type EMPTY_DMAEN_R = crate::R<bool, EMPTY_DMAEN_A>;
impl EMPTY_DMAEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EMPTY_DMAEN_A {
    match self.bits {
      false => EMPTY_DMAEN_A::EMPTY_DMAEN_0,
      true => EMPTY_DMAEN_A::EMPTY_DMAEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `EMPTY_DMAEN_0`"]
  #[inline(always)]
  pub fn is_empty_dmaen_0(&self) -> bool {
    *self == EMPTY_DMAEN_A::EMPTY_DMAEN_0
  }
  #[doc = "Checks if the value of the field is `EMPTY_DMAEN_1`"]
  #[inline(always)]
  pub fn is_empty_dmaen_1(&self) -> bool {
    *self == EMPTY_DMAEN_A::EMPTY_DMAEN_1
  }
}
#[doc = "Write proxy for field `EMPTY_DMAEN`"]
pub struct EMPTY_DMAEN_W<'a> {
  w: &'a mut W,
}
impl<'a> EMPTY_DMAEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EMPTY_DMAEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "FIFO Empty DMA request is disabled."]
  #[inline(always)]
  pub fn empty_dmaen_0(self) -> &'a mut W {
    self.variant(EMPTY_DMAEN_A::EMPTY_DMAEN_0)
  }
  #[doc = "FIFO Empty DMA request is enabled."]
  #[inline(always)]
  pub fn empty_dmaen_1(self) -> &'a mut W {
    self.variant(EMPTY_DMAEN_A::EMPTY_DMAEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
    self.w
  }
}
#[doc = "FIFO Watermark DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WM_DMAEN_A {
  #[doc = "0: Watermark DMA request is disabled."]
  WM_DMAEN_0,
  #[doc = "1: Watermark DMA request is enabled."]
  WM_DMAEN_1,
}
impl From<WM_DMAEN_A> for bool {
  #[inline(always)]
  fn from(variant: WM_DMAEN_A) -> Self {
    match variant {
      WM_DMAEN_A::WM_DMAEN_0 => false,
      WM_DMAEN_A::WM_DMAEN_1 => true,
    }
  }
}
#[doc = "Reader of field `WM_DMAEN`"]
pub type WM_DMAEN_R = crate::R<bool, WM_DMAEN_A>;
impl WM_DMAEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WM_DMAEN_A {
    match self.bits {
      false => WM_DMAEN_A::WM_DMAEN_0,
      true => WM_DMAEN_A::WM_DMAEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `WM_DMAEN_0`"]
  #[inline(always)]
  pub fn is_wm_dmaen_0(&self) -> bool {
    *self == WM_DMAEN_A::WM_DMAEN_0
  }
  #[doc = "Checks if the value of the field is `WM_DMAEN_1`"]
  #[inline(always)]
  pub fn is_wm_dmaen_1(&self) -> bool {
    *self == WM_DMAEN_A::WM_DMAEN_1
  }
}
#[doc = "Write proxy for field `WM_DMAEN`"]
pub struct WM_DMAEN_W<'a> {
  w: &'a mut W,
}
impl<'a> WM_DMAEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WM_DMAEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Watermark DMA request is disabled."]
  #[inline(always)]
  pub fn wm_dmaen_0(self) -> &'a mut W {
    self.variant(WM_DMAEN_A::WM_DMAEN_0)
  }
  #[doc = "Watermark DMA request is enabled."]
  #[inline(always)]
  pub fn wm_dmaen_1(self) -> &'a mut W {
    self.variant(WM_DMAEN_A::WM_DMAEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
    self.w
  }
}
impl R {
  #[doc = "Bit 1 - FIFO Empty DMA Enable"]
  #[inline(always)]
  pub fn empty_dmaen(&self) -> EMPTY_DMAEN_R {
    EMPTY_DMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - FIFO Watermark DMA Enable"]
  #[inline(always)]
  pub fn wm_dmaen(&self) -> WM_DMAEN_R {
    WM_DMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 1 - FIFO Empty DMA Enable"]
  #[inline(always)]
  pub fn empty_dmaen(&mut self) -> EMPTY_DMAEN_W {
    EMPTY_DMAEN_W { w: self }
  }
  #[doc = "Bit 2 - FIFO Watermark DMA Enable"]
  #[inline(always)]
  pub fn wm_dmaen(&mut self) -> WM_DMAEN_W {
    WM_DMAEN_W { w: self }
  }
}
