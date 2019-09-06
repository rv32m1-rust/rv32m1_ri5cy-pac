#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "FIFO Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_IE_A {
  #[doc = "0: FIFO Full interrupt is disabled."]
  FULL_IE_0,
  #[doc = "1: FIFO Full interrupt is enabled."]
  FULL_IE_1,
}
impl From<FULL_IE_A> for bool {
  #[inline(always)]
  fn from(variant: FULL_IE_A) -> Self {
    match variant {
      FULL_IE_A::FULL_IE_0 => false,
      FULL_IE_A::FULL_IE_1 => true,
    }
  }
}
#[doc = "Reader of field `FULL_IE`"]
pub type FULL_IE_R = crate::R<bool, FULL_IE_A>;
impl FULL_IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FULL_IE_A {
    match self.bits {
      false => FULL_IE_A::FULL_IE_0,
      true => FULL_IE_A::FULL_IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `FULL_IE_0`"]
  #[inline(always)]
  pub fn is_full_ie_0(&self) -> bool {
    *self == FULL_IE_A::FULL_IE_0
  }
  #[doc = "Checks if the value of the field is `FULL_IE_1`"]
  #[inline(always)]
  pub fn is_full_ie_1(&self) -> bool {
    *self == FULL_IE_A::FULL_IE_1
  }
}
#[doc = "Write proxy for field `FULL_IE`"]
pub struct FULL_IE_W<'a> {
  w: &'a mut W,
}
impl<'a> FULL_IE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FULL_IE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "FIFO Full interrupt is disabled."]
  #[inline(always)]
  pub fn full_ie_0(self) -> &'a mut W {
    self.variant(FULL_IE_A::FULL_IE_0)
  }
  #[doc = "FIFO Full interrupt is enabled."]
  #[inline(always)]
  pub fn full_ie_1(self) -> &'a mut W {
    self.variant(FULL_IE_A::FULL_IE_1)
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
#[doc = "FIFO Empty Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPTY_IE_A {
  #[doc = "0: FIFO Empty interrupt is disabled."]
  EMPTY_IE_0,
  #[doc = "1: FIFO Empty interrupt is enabled."]
  EMPTY_IE_1,
}
impl From<EMPTY_IE_A> for bool {
  #[inline(always)]
  fn from(variant: EMPTY_IE_A) -> Self {
    match variant {
      EMPTY_IE_A::EMPTY_IE_0 => false,
      EMPTY_IE_A::EMPTY_IE_1 => true,
    }
  }
}
#[doc = "Reader of field `EMPTY_IE`"]
pub type EMPTY_IE_R = crate::R<bool, EMPTY_IE_A>;
impl EMPTY_IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EMPTY_IE_A {
    match self.bits {
      false => EMPTY_IE_A::EMPTY_IE_0,
      true => EMPTY_IE_A::EMPTY_IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `EMPTY_IE_0`"]
  #[inline(always)]
  pub fn is_empty_ie_0(&self) -> bool {
    *self == EMPTY_IE_A::EMPTY_IE_0
  }
  #[doc = "Checks if the value of the field is `EMPTY_IE_1`"]
  #[inline(always)]
  pub fn is_empty_ie_1(&self) -> bool {
    *self == EMPTY_IE_A::EMPTY_IE_1
  }
}
#[doc = "Write proxy for field `EMPTY_IE`"]
pub struct EMPTY_IE_W<'a> {
  w: &'a mut W,
}
impl<'a> EMPTY_IE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: EMPTY_IE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "FIFO Empty interrupt is disabled."]
  #[inline(always)]
  pub fn empty_ie_0(self) -> &'a mut W {
    self.variant(EMPTY_IE_A::EMPTY_IE_0)
  }
  #[doc = "FIFO Empty interrupt is enabled."]
  #[inline(always)]
  pub fn empty_ie_1(self) -> &'a mut W {
    self.variant(EMPTY_IE_A::EMPTY_IE_1)
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
#[doc = "FIFO Watermark Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WM_IE_A {
  #[doc = "0: Watermark interrupt is disabled."]
  WM_IE_0,
  #[doc = "1: Watermark interrupt is enabled."]
  WM_IE_1,
}
impl From<WM_IE_A> for bool {
  #[inline(always)]
  fn from(variant: WM_IE_A) -> Self {
    match variant {
      WM_IE_A::WM_IE_0 => false,
      WM_IE_A::WM_IE_1 => true,
    }
  }
}
#[doc = "Reader of field `WM_IE`"]
pub type WM_IE_R = crate::R<bool, WM_IE_A>;
impl WM_IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WM_IE_A {
    match self.bits {
      false => WM_IE_A::WM_IE_0,
      true => WM_IE_A::WM_IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `WM_IE_0`"]
  #[inline(always)]
  pub fn is_wm_ie_0(&self) -> bool {
    *self == WM_IE_A::WM_IE_0
  }
  #[doc = "Checks if the value of the field is `WM_IE_1`"]
  #[inline(always)]
  pub fn is_wm_ie_1(&self) -> bool {
    *self == WM_IE_A::WM_IE_1
  }
}
#[doc = "Write proxy for field `WM_IE`"]
pub struct WM_IE_W<'a> {
  w: &'a mut W,
}
impl<'a> WM_IE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WM_IE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Watermark interrupt is disabled."]
  #[inline(always)]
  pub fn wm_ie_0(self) -> &'a mut W {
    self.variant(WM_IE_A::WM_IE_0)
  }
  #[doc = "Watermark interrupt is enabled."]
  #[inline(always)]
  pub fn wm_ie_1(self) -> &'a mut W {
    self.variant(WM_IE_A::WM_IE_1)
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
#[doc = "Swing back One Cycle Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWBK_IE_A {
  #[doc = "0: Swing back one time complete interrupt is disabled."]
  SWBK_IE_0,
  #[doc = "1: Swing back one time complete interrupt is enabled."]
  SWBK_IE_1,
}
impl From<SWBK_IE_A> for bool {
  #[inline(always)]
  fn from(variant: SWBK_IE_A) -> Self {
    match variant {
      SWBK_IE_A::SWBK_IE_0 => false,
      SWBK_IE_A::SWBK_IE_1 => true,
    }
  }
}
#[doc = "Reader of field `SWBK_IE`"]
pub type SWBK_IE_R = crate::R<bool, SWBK_IE_A>;
impl SWBK_IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SWBK_IE_A {
    match self.bits {
      false => SWBK_IE_A::SWBK_IE_0,
      true => SWBK_IE_A::SWBK_IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `SWBK_IE_0`"]
  #[inline(always)]
  pub fn is_swbk_ie_0(&self) -> bool {
    *self == SWBK_IE_A::SWBK_IE_0
  }
  #[doc = "Checks if the value of the field is `SWBK_IE_1`"]
  #[inline(always)]
  pub fn is_swbk_ie_1(&self) -> bool {
    *self == SWBK_IE_A::SWBK_IE_1
  }
}
#[doc = "Write proxy for field `SWBK_IE`"]
pub struct SWBK_IE_W<'a> {
  w: &'a mut W,
}
impl<'a> SWBK_IE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWBK_IE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Swing back one time complete interrupt is disabled."]
  #[inline(always)]
  pub fn swbk_ie_0(self) -> &'a mut W {
    self.variant(SWBK_IE_A::SWBK_IE_0)
  }
  #[doc = "Swing back one time complete interrupt is enabled."]
  #[inline(always)]
  pub fn swbk_ie_1(self) -> &'a mut W {
    self.variant(SWBK_IE_A::SWBK_IE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
    self.w
  }
}
#[doc = "FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OF_IE_A {
  #[doc = "0: Overflow interrupt is disabled"]
  OF_IE_0,
  #[doc = "1: Overflow interrupt is enabled."]
  OF_IE_1,
}
impl From<OF_IE_A> for bool {
  #[inline(always)]
  fn from(variant: OF_IE_A) -> Self {
    match variant {
      OF_IE_A::OF_IE_0 => false,
      OF_IE_A::OF_IE_1 => true,
    }
  }
}
#[doc = "Reader of field `OF_IE`"]
pub type OF_IE_R = crate::R<bool, OF_IE_A>;
impl OF_IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OF_IE_A {
    match self.bits {
      false => OF_IE_A::OF_IE_0,
      true => OF_IE_A::OF_IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `OF_IE_0`"]
  #[inline(always)]
  pub fn is_of_ie_0(&self) -> bool {
    *self == OF_IE_A::OF_IE_0
  }
  #[doc = "Checks if the value of the field is `OF_IE_1`"]
  #[inline(always)]
  pub fn is_of_ie_1(&self) -> bool {
    *self == OF_IE_A::OF_IE_1
  }
}
#[doc = "Write proxy for field `OF_IE`"]
pub struct OF_IE_W<'a> {
  w: &'a mut W,
}
impl<'a> OF_IE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: OF_IE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Overflow interrupt is disabled"]
  #[inline(always)]
  pub fn of_ie_0(self) -> &'a mut W {
    self.variant(OF_IE_A::OF_IE_0)
  }
  #[doc = "Overflow interrupt is enabled."]
  #[inline(always)]
  pub fn of_ie_1(self) -> &'a mut W {
    self.variant(OF_IE_A::OF_IE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
    self.w
  }
}
#[doc = "FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UF_IE_A {
  #[doc = "0: Underflow interrupt is disabled."]
  UF_IE_0,
  #[doc = "1: Underflow interrupt is enabled."]
  UF_IE_1,
}
impl From<UF_IE_A> for bool {
  #[inline(always)]
  fn from(variant: UF_IE_A) -> Self {
    match variant {
      UF_IE_A::UF_IE_0 => false,
      UF_IE_A::UF_IE_1 => true,
    }
  }
}
#[doc = "Reader of field `UF_IE`"]
pub type UF_IE_R = crate::R<bool, UF_IE_A>;
impl UF_IE_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> UF_IE_A {
    match self.bits {
      false => UF_IE_A::UF_IE_0,
      true => UF_IE_A::UF_IE_1,
    }
  }
  #[doc = "Checks if the value of the field is `UF_IE_0`"]
  #[inline(always)]
  pub fn is_uf_ie_0(&self) -> bool {
    *self == UF_IE_A::UF_IE_0
  }
  #[doc = "Checks if the value of the field is `UF_IE_1`"]
  #[inline(always)]
  pub fn is_uf_ie_1(&self) -> bool {
    *self == UF_IE_A::UF_IE_1
  }
}
#[doc = "Write proxy for field `UF_IE`"]
pub struct UF_IE_W<'a> {
  w: &'a mut W,
}
impl<'a> UF_IE_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: UF_IE_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Underflow interrupt is disabled."]
  #[inline(always)]
  pub fn uf_ie_0(self) -> &'a mut W {
    self.variant(UF_IE_A::UF_IE_0)
  }
  #[doc = "Underflow interrupt is enabled."]
  #[inline(always)]
  pub fn uf_ie_1(self) -> &'a mut W {
    self.variant(UF_IE_A::UF_IE_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - FIFO Full Interrupt Enable"]
  #[inline(always)]
  pub fn full_ie(&self) -> FULL_IE_R {
    FULL_IE_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - FIFO Empty Interrupt Enable"]
  #[inline(always)]
  pub fn empty_ie(&self) -> EMPTY_IE_R {
    EMPTY_IE_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - FIFO Watermark Interrupt Enable"]
  #[inline(always)]
  pub fn wm_ie(&self) -> WM_IE_R {
    WM_IE_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Swing back One Cycle Complete Interrupt Enable"]
  #[inline(always)]
  pub fn swbk_ie(&self) -> SWBK_IE_R {
    SWBK_IE_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 6 - FIFO Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn of_ie(&self) -> OF_IE_R {
    OF_IE_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - FIFO Underflow Interrupt Enable"]
  #[inline(always)]
  pub fn uf_ie(&self) -> UF_IE_R {
    UF_IE_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - FIFO Full Interrupt Enable"]
  #[inline(always)]
  pub fn full_ie(&mut self) -> FULL_IE_W {
    FULL_IE_W { w: self }
  }
  #[doc = "Bit 1 - FIFO Empty Interrupt Enable"]
  #[inline(always)]
  pub fn empty_ie(&mut self) -> EMPTY_IE_W {
    EMPTY_IE_W { w: self }
  }
  #[doc = "Bit 2 - FIFO Watermark Interrupt Enable"]
  #[inline(always)]
  pub fn wm_ie(&mut self) -> WM_IE_W {
    WM_IE_W { w: self }
  }
  #[doc = "Bit 3 - Swing back One Cycle Complete Interrupt Enable"]
  #[inline(always)]
  pub fn swbk_ie(&mut self) -> SWBK_IE_W {
    SWBK_IE_W { w: self }
  }
  #[doc = "Bit 6 - FIFO Overflow Interrupt Enable"]
  #[inline(always)]
  pub fn of_ie(&mut self) -> OF_IE_W {
    OF_IE_W { w: self }
  }
  #[doc = "Bit 7 - FIFO Underflow Interrupt Enable"]
  #[inline(always)]
  pub fn uf_ie(&mut self) -> UF_IE_W {
    UF_IE_W { w: self }
  }
}
