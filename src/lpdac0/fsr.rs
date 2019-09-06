#[doc = "Reader of register FSR"]
pub type R = crate::R<u32, super::FSR>;
#[doc = "Writer for register FSR"]
pub type W = crate::W<u32, super::FSR>;
#[doc = "Register FSR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FSR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x02
  }
}
#[doc = "FIFO Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FULL_A {
  #[doc = "0: FIFO is not full"]
  FULL_0,
  #[doc = "1: FIFO is full"]
  FULL_1,
}
impl From<FULL_A> for bool {
  #[inline(always)]
  fn from(variant: FULL_A) -> Self {
    match variant {
      FULL_A::FULL_0 => false,
      FULL_A::FULL_1 => true,
    }
  }
}
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, FULL_A>;
impl FULL_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FULL_A {
    match self.bits {
      false => FULL_A::FULL_0,
      true => FULL_A::FULL_1,
    }
  }
  #[doc = "Checks if the value of the field is `FULL_0`"]
  #[inline(always)]
  pub fn is_full_0(&self) -> bool {
    *self == FULL_A::FULL_0
  }
  #[doc = "Checks if the value of the field is `FULL_1`"]
  #[inline(always)]
  pub fn is_full_1(&self) -> bool {
    *self == FULL_A::FULL_1
  }
}
#[doc = "FIFO Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPTY_A {
  #[doc = "0: FIFO is not empty"]
  EMPTY_0,
  #[doc = "1: FIFO is empty"]
  EMPTY_1,
}
impl From<EMPTY_A> for bool {
  #[inline(always)]
  fn from(variant: EMPTY_A) -> Self {
    match variant {
      EMPTY_A::EMPTY_0 => false,
      EMPTY_A::EMPTY_1 => true,
    }
  }
}
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, EMPTY_A>;
impl EMPTY_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> EMPTY_A {
    match self.bits {
      false => EMPTY_A::EMPTY_0,
      true => EMPTY_A::EMPTY_1,
    }
  }
  #[doc = "Checks if the value of the field is `EMPTY_0`"]
  #[inline(always)]
  pub fn is_empty_0(&self) -> bool {
    *self == EMPTY_A::EMPTY_0
  }
  #[doc = "Checks if the value of the field is `EMPTY_1`"]
  #[inline(always)]
  pub fn is_empty_1(&self) -> bool {
    *self == EMPTY_A::EMPTY_1
  }
}
#[doc = "FIFO Watermark Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WM_A {
  #[doc = "0: Data in FIFO is more than watermark level"]
  WM_0,
  #[doc = "1: Data in FIFO is less than or equal to watermark level"]
  WM_1,
}
impl From<WM_A> for bool {
  #[inline(always)]
  fn from(variant: WM_A) -> Self {
    match variant {
      WM_A::WM_0 => false,
      WM_A::WM_1 => true,
    }
  }
}
#[doc = "Reader of field `WM`"]
pub type WM_R = crate::R<bool, WM_A>;
impl WM_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WM_A {
    match self.bits {
      false => WM_A::WM_0,
      true => WM_A::WM_1,
    }
  }
  #[doc = "Checks if the value of the field is `WM_0`"]
  #[inline(always)]
  pub fn is_wm_0(&self) -> bool {
    *self == WM_A::WM_0
  }
  #[doc = "Checks if the value of the field is `WM_1`"]
  #[inline(always)]
  pub fn is_wm_1(&self) -> bool {
    *self == WM_A::WM_1
  }
}
#[doc = "Swing Back One Cycle Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWBK_A {
  #[doc = "0: No swing back cycle has completed since the last time the flag was cleared."]
  SWBK_0,
  #[doc = "1: At least one swing back cycle has occurred since the last time the flag was cleared."]
  SWBK_1,
}
impl From<SWBK_A> for bool {
  #[inline(always)]
  fn from(variant: SWBK_A) -> Self {
    match variant {
      SWBK_A::SWBK_0 => false,
      SWBK_A::SWBK_1 => true,
    }
  }
}
#[doc = "Reader of field `SWBK`"]
pub type SWBK_R = crate::R<bool, SWBK_A>;
impl SWBK_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SWBK_A {
    match self.bits {
      false => SWBK_A::SWBK_0,
      true => SWBK_A::SWBK_1,
    }
  }
  #[doc = "Checks if the value of the field is `SWBK_0`"]
  #[inline(always)]
  pub fn is_swbk_0(&self) -> bool {
    *self == SWBK_A::SWBK_0
  }
  #[doc = "Checks if the value of the field is `SWBK_1`"]
  #[inline(always)]
  pub fn is_swbk_1(&self) -> bool {
    *self == SWBK_A::SWBK_1
  }
}
#[doc = "Write proxy for field `SWBK`"]
pub struct SWBK_W<'a> {
  w: &'a mut W,
}
impl<'a> SWBK_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: SWBK_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No swing back cycle has completed since the last time the flag was cleared."]
  #[inline(always)]
  pub fn swbk_0(self) -> &'a mut W {
    self.variant(SWBK_A::SWBK_0)
  }
  #[doc = "At least one swing back cycle has occurred since the last time the flag was cleared."]
  #[inline(always)]
  pub fn swbk_1(self) -> &'a mut W {
    self.variant(SWBK_A::SWBK_1)
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
#[doc = "FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OF_A {
  #[doc = "0: No overflow has occurred since the last time the flag was cleared."]
  OF_0,
  #[doc = "1: At least one FIFO overflow has occurred since the last time the flag was cleared."]
  OF_1,
}
impl From<OF_A> for bool {
  #[inline(always)]
  fn from(variant: OF_A) -> Self {
    match variant {
      OF_A::OF_0 => false,
      OF_A::OF_1 => true,
    }
  }
}
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<bool, OF_A>;
impl OF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> OF_A {
    match self.bits {
      false => OF_A::OF_0,
      true => OF_A::OF_1,
    }
  }
  #[doc = "Checks if the value of the field is `OF_0`"]
  #[inline(always)]
  pub fn is_of_0(&self) -> bool {
    *self == OF_A::OF_0
  }
  #[doc = "Checks if the value of the field is `OF_1`"]
  #[inline(always)]
  pub fn is_of_1(&self) -> bool {
    *self == OF_A::OF_1
  }
}
#[doc = "Write proxy for field `OF`"]
pub struct OF_W<'a> {
  w: &'a mut W,
}
impl<'a> OF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: OF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No overflow has occurred since the last time the flag was cleared."]
  #[inline(always)]
  pub fn of_0(self) -> &'a mut W {
    self.variant(OF_A::OF_0)
  }
  #[doc = "At least one FIFO overflow has occurred since the last time the flag was cleared."]
  #[inline(always)]
  pub fn of_1(self) -> &'a mut W {
    self.variant(OF_A::OF_1)
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
#[doc = "FIFO Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UF_A {
  #[doc = "0: No underflow has occurred since the last time the flag was cleared."]
  UF_0,
  #[doc = "1: At least one trigger underflow has occurred since the last time the flag was cleared."]
  UF_1,
}
impl From<UF_A> for bool {
  #[inline(always)]
  fn from(variant: UF_A) -> Self {
    match variant {
      UF_A::UF_0 => false,
      UF_A::UF_1 => true,
    }
  }
}
#[doc = "Reader of field `UF`"]
pub type UF_R = crate::R<bool, UF_A>;
impl UF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> UF_A {
    match self.bits {
      false => UF_A::UF_0,
      true => UF_A::UF_1,
    }
  }
  #[doc = "Checks if the value of the field is `UF_0`"]
  #[inline(always)]
  pub fn is_uf_0(&self) -> bool {
    *self == UF_A::UF_0
  }
  #[doc = "Checks if the value of the field is `UF_1`"]
  #[inline(always)]
  pub fn is_uf_1(&self) -> bool {
    *self == UF_A::UF_1
  }
}
#[doc = "Write proxy for field `UF`"]
pub struct UF_W<'a> {
  w: &'a mut W,
}
impl<'a> UF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: UF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "No underflow has occurred since the last time the flag was cleared."]
  #[inline(always)]
  pub fn uf_0(self) -> &'a mut W {
    self.variant(UF_A::UF_0)
  }
  #[doc = "At least one trigger underflow has occurred since the last time the flag was cleared."]
  #[inline(always)]
  pub fn uf_1(self) -> &'a mut W {
    self.variant(UF_A::UF_1)
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
  #[doc = "Bit 0 - FIFO Full Flag"]
  #[inline(always)]
  pub fn full(&self) -> FULL_R {
    FULL_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - FIFO Empty Flag"]
  #[inline(always)]
  pub fn empty(&self) -> EMPTY_R {
    EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - FIFO Watermark Status Flag"]
  #[inline(always)]
  pub fn wm(&self) -> WM_R {
    WM_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Swing Back One Cycle Complete Flag"]
  #[inline(always)]
  pub fn swbk(&self) -> SWBK_R {
    SWBK_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 6 - FIFO Overflow Flag"]
  #[inline(always)]
  pub fn of(&self) -> OF_R {
    OF_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - FIFO Underflow Flag"]
  #[inline(always)]
  pub fn uf(&self) -> UF_R {
    UF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 3 - Swing Back One Cycle Complete Flag"]
  #[inline(always)]
  pub fn swbk(&mut self) -> SWBK_W {
    SWBK_W { w: self }
  }
  #[doc = "Bit 6 - FIFO Overflow Flag"]
  #[inline(always)]
  pub fn of(&mut self) -> OF_W {
    OF_W { w: self }
  }
  #[doc = "Bit 7 - FIFO Underflow Flag"]
  #[inline(always)]
  pub fn uf(&mut self) -> UF_W {
    UF_W { w: self }
  }
}
