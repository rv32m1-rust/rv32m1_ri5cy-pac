#[doc = "Reader of register FILT"]
pub type R = crate::R<u32, super::FILT>;
#[doc = "Writer for register FILT"]
pub type W = crate::W<u32, super::FILT>;
#[doc = "Register FILT `reset()`'s with value 0"]
impl crate::ResetValue for super::FILT {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Filter 1 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTSEL1_A {
  #[doc = "0: Select LLWU_P0 for filter"]
  FILTSEL1_0,
  #[doc = "31: Select LLWU_P31 for filter"]
  FILTSEL1_31,
}
impl From<FILTSEL1_A> for u8 {
  #[inline(always)]
  fn from(variant: FILTSEL1_A) -> Self {
    match variant {
      FILTSEL1_A::FILTSEL1_0 => 0,
      FILTSEL1_A::FILTSEL1_31 => 31,
    }
  }
}
#[doc = "Reader of field `FILTSEL1`"]
pub type FILTSEL1_R = crate::R<u8, FILTSEL1_A>;
impl FILTSEL1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FILTSEL1_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FILTSEL1_A::FILTSEL1_0),
      31 => Val(FILTSEL1_A::FILTSEL1_31),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FILTSEL1_0`"]
  #[inline(always)]
  pub fn is_filtsel1_0(&self) -> bool {
    *self == FILTSEL1_A::FILTSEL1_0
  }
  #[doc = "Checks if the value of the field is `FILTSEL1_31`"]
  #[inline(always)]
  pub fn is_filtsel1_31(&self) -> bool {
    *self == FILTSEL1_A::FILTSEL1_31
  }
}
#[doc = "Write proxy for field `FILTSEL1`"]
pub struct FILTSEL1_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTSEL1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTSEL1_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Select LLWU_P0 for filter"]
  #[inline(always)]
  pub fn filtsel1_0(self) -> &'a mut W {
    self.variant(FILTSEL1_A::FILTSEL1_0)
  }
  #[doc = "Select LLWU_P31 for filter"]
  #[inline(always)]
  pub fn filtsel1_31(self) -> &'a mut W {
    self.variant(FILTSEL1_A::FILTSEL1_31)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
    self.w
  }
}
#[doc = "Filter 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTE1_A {
  #[doc = "0: Filter disabled"]
  FILTE1_0,
  #[doc = "1: Filter posedge detect enabled when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  FILTE1_1,
  #[doc = "2: Filter negedge detect enabled when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  FILTE1_2,
  #[doc = "3: Filter any edge detect enabled when configured as interrupt/DMA request"]
  FILTE1_3,
}
impl From<FILTE1_A> for u8 {
  #[inline(always)]
  fn from(variant: FILTE1_A) -> Self {
    match variant {
      FILTE1_A::FILTE1_0 => 0,
      FILTE1_A::FILTE1_1 => 1,
      FILTE1_A::FILTE1_2 => 2,
      FILTE1_A::FILTE1_3 => 3,
    }
  }
}
#[doc = "Reader of field `FILTE1`"]
pub type FILTE1_R = crate::R<u8, FILTE1_A>;
impl FILTE1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FILTE1_A {
    match self.bits {
      0 => FILTE1_A::FILTE1_0,
      1 => FILTE1_A::FILTE1_1,
      2 => FILTE1_A::FILTE1_2,
      3 => FILTE1_A::FILTE1_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `FILTE1_0`"]
  #[inline(always)]
  pub fn is_filte1_0(&self) -> bool {
    *self == FILTE1_A::FILTE1_0
  }
  #[doc = "Checks if the value of the field is `FILTE1_1`"]
  #[inline(always)]
  pub fn is_filte1_1(&self) -> bool {
    *self == FILTE1_A::FILTE1_1
  }
  #[doc = "Checks if the value of the field is `FILTE1_2`"]
  #[inline(always)]
  pub fn is_filte1_2(&self) -> bool {
    *self == FILTE1_A::FILTE1_2
  }
  #[doc = "Checks if the value of the field is `FILTE1_3`"]
  #[inline(always)]
  pub fn is_filte1_3(&self) -> bool {
    *self == FILTE1_A::FILTE1_3
  }
}
#[doc = "Write proxy for field `FILTE1`"]
pub struct FILTE1_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTE1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTE1_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Filter disabled"]
  #[inline(always)]
  pub fn filte1_0(self) -> &'a mut W {
    self.variant(FILTE1_A::FILTE1_0)
  }
  #[doc = "Filter posedge detect enabled when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn filte1_1(self) -> &'a mut W {
    self.variant(FILTE1_A::FILTE1_1)
  }
  #[doc = "Filter negedge detect enabled when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn filte1_2(self) -> &'a mut W {
    self.variant(FILTE1_A::FILTE1_2)
  }
  #[doc = "Filter any edge detect enabled when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn filte1_3(self) -> &'a mut W {
    self.variant(FILTE1_A::FILTE1_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
    self.w
  }
}
#[doc = "Filter 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTF1_A {
  #[doc = "0: Pin Filter 1 was not a wakeup source"]
  FILTF1_0,
  #[doc = "1: Pin Filter 1 was a wakeup source"]
  FILTF1_1,
}
impl From<FILTF1_A> for bool {
  #[inline(always)]
  fn from(variant: FILTF1_A) -> Self {
    match variant {
      FILTF1_A::FILTF1_0 => false,
      FILTF1_A::FILTF1_1 => true,
    }
  }
}
#[doc = "Reader of field `FILTF1`"]
pub type FILTF1_R = crate::R<bool, FILTF1_A>;
impl FILTF1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FILTF1_A {
    match self.bits {
      false => FILTF1_A::FILTF1_0,
      true => FILTF1_A::FILTF1_1,
    }
  }
  #[doc = "Checks if the value of the field is `FILTF1_0`"]
  #[inline(always)]
  pub fn is_filtf1_0(&self) -> bool {
    *self == FILTF1_A::FILTF1_0
  }
  #[doc = "Checks if the value of the field is `FILTF1_1`"]
  #[inline(always)]
  pub fn is_filtf1_1(&self) -> bool {
    *self == FILTF1_A::FILTF1_1
  }
}
#[doc = "Write proxy for field `FILTF1`"]
pub struct FILTF1_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTF1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTF1_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Pin Filter 1 was not a wakeup source"]
  #[inline(always)]
  pub fn filtf1_0(self) -> &'a mut W {
    self.variant(FILTF1_A::FILTF1_0)
  }
  #[doc = "Pin Filter 1 was a wakeup source"]
  #[inline(always)]
  pub fn filtf1_1(self) -> &'a mut W {
    self.variant(FILTF1_A::FILTF1_1)
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
#[doc = "Filter 2 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTSEL2_A {
  #[doc = "0: Select LLWU_P0 for filter"]
  FILTSEL2_0,
  #[doc = "31: Select LLWU_P31 for filter"]
  FILTSEL2_31,
}
impl From<FILTSEL2_A> for u8 {
  #[inline(always)]
  fn from(variant: FILTSEL2_A) -> Self {
    match variant {
      FILTSEL2_A::FILTSEL2_0 => 0,
      FILTSEL2_A::FILTSEL2_31 => 31,
    }
  }
}
#[doc = "Reader of field `FILTSEL2`"]
pub type FILTSEL2_R = crate::R<u8, FILTSEL2_A>;
impl FILTSEL2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FILTSEL2_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FILTSEL2_A::FILTSEL2_0),
      31 => Val(FILTSEL2_A::FILTSEL2_31),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FILTSEL2_0`"]
  #[inline(always)]
  pub fn is_filtsel2_0(&self) -> bool {
    *self == FILTSEL2_A::FILTSEL2_0
  }
  #[doc = "Checks if the value of the field is `FILTSEL2_31`"]
  #[inline(always)]
  pub fn is_filtsel2_31(&self) -> bool {
    *self == FILTSEL2_A::FILTSEL2_31
  }
}
#[doc = "Write proxy for field `FILTSEL2`"]
pub struct FILTSEL2_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTSEL2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTSEL2_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Select LLWU_P0 for filter"]
  #[inline(always)]
  pub fn filtsel2_0(self) -> &'a mut W {
    self.variant(FILTSEL2_A::FILTSEL2_0)
  }
  #[doc = "Select LLWU_P31 for filter"]
  #[inline(always)]
  pub fn filtsel2_31(self) -> &'a mut W {
    self.variant(FILTSEL2_A::FILTSEL2_31)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
    self.w
  }
}
#[doc = "Filter 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTE2_A {
  #[doc = "0: Filter disabled"]
  FILTE2_0,
  #[doc = "1: Filter posedge detect enabled when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  FILTE2_1,
  #[doc = "2: Filter negedge detect enabled when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  FILTE2_2,
  #[doc = "3: Filter any edge detect enabled when configured as interrupt/DMA request"]
  FILTE2_3,
}
impl From<FILTE2_A> for u8 {
  #[inline(always)]
  fn from(variant: FILTE2_A) -> Self {
    match variant {
      FILTE2_A::FILTE2_0 => 0,
      FILTE2_A::FILTE2_1 => 1,
      FILTE2_A::FILTE2_2 => 2,
      FILTE2_A::FILTE2_3 => 3,
    }
  }
}
#[doc = "Reader of field `FILTE2`"]
pub type FILTE2_R = crate::R<u8, FILTE2_A>;
impl FILTE2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FILTE2_A {
    match self.bits {
      0 => FILTE2_A::FILTE2_0,
      1 => FILTE2_A::FILTE2_1,
      2 => FILTE2_A::FILTE2_2,
      3 => FILTE2_A::FILTE2_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `FILTE2_0`"]
  #[inline(always)]
  pub fn is_filte2_0(&self) -> bool {
    *self == FILTE2_A::FILTE2_0
  }
  #[doc = "Checks if the value of the field is `FILTE2_1`"]
  #[inline(always)]
  pub fn is_filte2_1(&self) -> bool {
    *self == FILTE2_A::FILTE2_1
  }
  #[doc = "Checks if the value of the field is `FILTE2_2`"]
  #[inline(always)]
  pub fn is_filte2_2(&self) -> bool {
    *self == FILTE2_A::FILTE2_2
  }
  #[doc = "Checks if the value of the field is `FILTE2_3`"]
  #[inline(always)]
  pub fn is_filte2_3(&self) -> bool {
    *self == FILTE2_A::FILTE2_3
  }
}
#[doc = "Write proxy for field `FILTE2`"]
pub struct FILTE2_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTE2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTE2_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "Filter disabled"]
  #[inline(always)]
  pub fn filte2_0(self) -> &'a mut W {
    self.variant(FILTE2_A::FILTE2_0)
  }
  #[doc = "Filter posedge detect enabled when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn filte2_1(self) -> &'a mut W {
    self.variant(FILTE2_A::FILTE2_1)
  }
  #[doc = "Filter negedge detect enabled when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn filte2_2(self) -> &'a mut W {
    self.variant(FILTE2_A::FILTE2_2)
  }
  #[doc = "Filter any edge detect enabled when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn filte2_3(self) -> &'a mut W {
    self.variant(FILTE2_A::FILTE2_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
    self.w
  }
}
#[doc = "Filter 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTF2_A {
  #[doc = "0: Pin Filter 2 was not a wakeup source"]
  FILTF2_0,
  #[doc = "1: Pin Filter 2 was a wakeup source"]
  FILTF2_1,
}
impl From<FILTF2_A> for bool {
  #[inline(always)]
  fn from(variant: FILTF2_A) -> Self {
    match variant {
      FILTF2_A::FILTF2_0 => false,
      FILTF2_A::FILTF2_1 => true,
    }
  }
}
#[doc = "Reader of field `FILTF2`"]
pub type FILTF2_R = crate::R<bool, FILTF2_A>;
impl FILTF2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FILTF2_A {
    match self.bits {
      false => FILTF2_A::FILTF2_0,
      true => FILTF2_A::FILTF2_1,
    }
  }
  #[doc = "Checks if the value of the field is `FILTF2_0`"]
  #[inline(always)]
  pub fn is_filtf2_0(&self) -> bool {
    *self == FILTF2_A::FILTF2_0
  }
  #[doc = "Checks if the value of the field is `FILTF2_1`"]
  #[inline(always)]
  pub fn is_filtf2_1(&self) -> bool {
    *self == FILTF2_A::FILTF2_1
  }
}
#[doc = "Write proxy for field `FILTF2`"]
pub struct FILTF2_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTF2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTF2_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Pin Filter 2 was not a wakeup source"]
  #[inline(always)]
  pub fn filtf2_0(self) -> &'a mut W {
    self.variant(FILTF2_A::FILTF2_0)
  }
  #[doc = "Pin Filter 2 was a wakeup source"]
  #[inline(always)]
  pub fn filtf2_1(self) -> &'a mut W {
    self.variant(FILTF2_A::FILTF2_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:4 - Filter 1 Pin Select"]
  #[inline(always)]
  pub fn filtsel1(&self) -> FILTSEL1_R {
    FILTSEL1_R::new((self.bits & 0x1f) as u8)
  }
  #[doc = "Bits 5:6 - Filter 1 Enable"]
  #[inline(always)]
  pub fn filte1(&self) -> FILTE1_R {
    FILTE1_R::new(((self.bits >> 5) & 0x03) as u8)
  }
  #[doc = "Bit 7 - Filter 1 Flag"]
  #[inline(always)]
  pub fn filtf1(&self) -> FILTF1_R {
    FILTF1_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bits 8:12 - Filter 2 Pin Select"]
  #[inline(always)]
  pub fn filtsel2(&self) -> FILTSEL2_R {
    FILTSEL2_R::new(((self.bits >> 8) & 0x1f) as u8)
  }
  #[doc = "Bits 13:14 - Filter 2 Enable"]
  #[inline(always)]
  pub fn filte2(&self) -> FILTE2_R {
    FILTE2_R::new(((self.bits >> 13) & 0x03) as u8)
  }
  #[doc = "Bit 15 - Filter 2 Flag"]
  #[inline(always)]
  pub fn filtf2(&self) -> FILTF2_R {
    FILTF2_R::new(((self.bits >> 15) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:4 - Filter 1 Pin Select"]
  #[inline(always)]
  pub fn filtsel1(&mut self) -> FILTSEL1_W {
    FILTSEL1_W { w: self }
  }
  #[doc = "Bits 5:6 - Filter 1 Enable"]
  #[inline(always)]
  pub fn filte1(&mut self) -> FILTE1_W {
    FILTE1_W { w: self }
  }
  #[doc = "Bit 7 - Filter 1 Flag"]
  #[inline(always)]
  pub fn filtf1(&mut self) -> FILTF1_W {
    FILTF1_W { w: self }
  }
  #[doc = "Bits 8:12 - Filter 2 Pin Select"]
  #[inline(always)]
  pub fn filtsel2(&mut self) -> FILTSEL2_W {
    FILTSEL2_W { w: self }
  }
  #[doc = "Bits 13:14 - Filter 2 Enable"]
  #[inline(always)]
  pub fn filte2(&mut self) -> FILTE2_W {
    FILTE2_W { w: self }
  }
  #[doc = "Bit 15 - Filter 2 Flag"]
  #[inline(always)]
  pub fn filtf2(&mut self) -> FILTF2_W {
    FILTF2_W { w: self }
  }
}
