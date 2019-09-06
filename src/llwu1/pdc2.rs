#[doc = "Reader of register PDC2"]
pub type R = crate::R<u32, super::PDC2>;
#[doc = "Writer for register PDC2"]
pub type W = crate::W<u32, super::PDC2>;
#[doc = "Register PDC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDC2 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC16_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC16_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC16_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC16_2,
}
impl From<WUPDC16_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC16_A) -> Self {
    match variant {
      WUPDC16_A::WUPDC16_0 => 0,
      WUPDC16_A::WUPDC16_1 => 1,
      WUPDC16_A::WUPDC16_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC16`"]
pub type WUPDC16_R = crate::R<u8, WUPDC16_A>;
impl WUPDC16_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC16_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC16_A::WUPDC16_0),
      1 => Val(WUPDC16_A::WUPDC16_1),
      2 => Val(WUPDC16_A::WUPDC16_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC16_0`"]
  #[inline(always)]
  pub fn is_wupdc16_0(&self) -> bool {
    *self == WUPDC16_A::WUPDC16_0
  }
  #[doc = "Checks if the value of the field is `WUPDC16_1`"]
  #[inline(always)]
  pub fn is_wupdc16_1(&self) -> bool {
    *self == WUPDC16_A::WUPDC16_1
  }
  #[doc = "Checks if the value of the field is `WUPDC16_2`"]
  #[inline(always)]
  pub fn is_wupdc16_2(&self) -> bool {
    *self == WUPDC16_A::WUPDC16_2
  }
}
#[doc = "Write proxy for field `WUPDC16`"]
pub struct WUPDC16_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC16_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC16_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc16_0(self) -> &'a mut W {
    self.variant(WUPDC16_A::WUPDC16_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc16_1(self) -> &'a mut W {
    self.variant(WUPDC16_A::WUPDC16_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc16_2(self) -> &'a mut W {
    self.variant(WUPDC16_A::WUPDC16_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC17_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC17_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC17_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC17_2,
}
impl From<WUPDC17_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC17_A) -> Self {
    match variant {
      WUPDC17_A::WUPDC17_0 => 0,
      WUPDC17_A::WUPDC17_1 => 1,
      WUPDC17_A::WUPDC17_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC17`"]
pub type WUPDC17_R = crate::R<u8, WUPDC17_A>;
impl WUPDC17_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC17_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC17_A::WUPDC17_0),
      1 => Val(WUPDC17_A::WUPDC17_1),
      2 => Val(WUPDC17_A::WUPDC17_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC17_0`"]
  #[inline(always)]
  pub fn is_wupdc17_0(&self) -> bool {
    *self == WUPDC17_A::WUPDC17_0
  }
  #[doc = "Checks if the value of the field is `WUPDC17_1`"]
  #[inline(always)]
  pub fn is_wupdc17_1(&self) -> bool {
    *self == WUPDC17_A::WUPDC17_1
  }
  #[doc = "Checks if the value of the field is `WUPDC17_2`"]
  #[inline(always)]
  pub fn is_wupdc17_2(&self) -> bool {
    *self == WUPDC17_A::WUPDC17_2
  }
}
#[doc = "Write proxy for field `WUPDC17`"]
pub struct WUPDC17_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC17_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC17_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc17_0(self) -> &'a mut W {
    self.variant(WUPDC17_A::WUPDC17_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc17_1(self) -> &'a mut W {
    self.variant(WUPDC17_A::WUPDC17_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc17_2(self) -> &'a mut W {
    self.variant(WUPDC17_A::WUPDC17_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC18_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC18_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC18_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC18_2,
}
impl From<WUPDC18_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC18_A) -> Self {
    match variant {
      WUPDC18_A::WUPDC18_0 => 0,
      WUPDC18_A::WUPDC18_1 => 1,
      WUPDC18_A::WUPDC18_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC18`"]
pub type WUPDC18_R = crate::R<u8, WUPDC18_A>;
impl WUPDC18_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC18_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC18_A::WUPDC18_0),
      1 => Val(WUPDC18_A::WUPDC18_1),
      2 => Val(WUPDC18_A::WUPDC18_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC18_0`"]
  #[inline(always)]
  pub fn is_wupdc18_0(&self) -> bool {
    *self == WUPDC18_A::WUPDC18_0
  }
  #[doc = "Checks if the value of the field is `WUPDC18_1`"]
  #[inline(always)]
  pub fn is_wupdc18_1(&self) -> bool {
    *self == WUPDC18_A::WUPDC18_1
  }
  #[doc = "Checks if the value of the field is `WUPDC18_2`"]
  #[inline(always)]
  pub fn is_wupdc18_2(&self) -> bool {
    *self == WUPDC18_A::WUPDC18_2
  }
}
#[doc = "Write proxy for field `WUPDC18`"]
pub struct WUPDC18_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC18_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC18_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc18_0(self) -> &'a mut W {
    self.variant(WUPDC18_A::WUPDC18_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc18_1(self) -> &'a mut W {
    self.variant(WUPDC18_A::WUPDC18_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc18_2(self) -> &'a mut W {
    self.variant(WUPDC18_A::WUPDC18_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC19_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC19_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC19_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC19_2,
}
impl From<WUPDC19_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC19_A) -> Self {
    match variant {
      WUPDC19_A::WUPDC19_0 => 0,
      WUPDC19_A::WUPDC19_1 => 1,
      WUPDC19_A::WUPDC19_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC19`"]
pub type WUPDC19_R = crate::R<u8, WUPDC19_A>;
impl WUPDC19_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC19_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC19_A::WUPDC19_0),
      1 => Val(WUPDC19_A::WUPDC19_1),
      2 => Val(WUPDC19_A::WUPDC19_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC19_0`"]
  #[inline(always)]
  pub fn is_wupdc19_0(&self) -> bool {
    *self == WUPDC19_A::WUPDC19_0
  }
  #[doc = "Checks if the value of the field is `WUPDC19_1`"]
  #[inline(always)]
  pub fn is_wupdc19_1(&self) -> bool {
    *self == WUPDC19_A::WUPDC19_1
  }
  #[doc = "Checks if the value of the field is `WUPDC19_2`"]
  #[inline(always)]
  pub fn is_wupdc19_2(&self) -> bool {
    *self == WUPDC19_A::WUPDC19_2
  }
}
#[doc = "Write proxy for field `WUPDC19`"]
pub struct WUPDC19_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC19_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC19_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc19_0(self) -> &'a mut W {
    self.variant(WUPDC19_A::WUPDC19_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc19_1(self) -> &'a mut W {
    self.variant(WUPDC19_A::WUPDC19_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc19_2(self) -> &'a mut W {
    self.variant(WUPDC19_A::WUPDC19_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC20_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC20_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC20_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC20_2,
}
impl From<WUPDC20_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC20_A) -> Self {
    match variant {
      WUPDC20_A::WUPDC20_0 => 0,
      WUPDC20_A::WUPDC20_1 => 1,
      WUPDC20_A::WUPDC20_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC20`"]
pub type WUPDC20_R = crate::R<u8, WUPDC20_A>;
impl WUPDC20_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC20_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC20_A::WUPDC20_0),
      1 => Val(WUPDC20_A::WUPDC20_1),
      2 => Val(WUPDC20_A::WUPDC20_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC20_0`"]
  #[inline(always)]
  pub fn is_wupdc20_0(&self) -> bool {
    *self == WUPDC20_A::WUPDC20_0
  }
  #[doc = "Checks if the value of the field is `WUPDC20_1`"]
  #[inline(always)]
  pub fn is_wupdc20_1(&self) -> bool {
    *self == WUPDC20_A::WUPDC20_1
  }
  #[doc = "Checks if the value of the field is `WUPDC20_2`"]
  #[inline(always)]
  pub fn is_wupdc20_2(&self) -> bool {
    *self == WUPDC20_A::WUPDC20_2
  }
}
#[doc = "Write proxy for field `WUPDC20`"]
pub struct WUPDC20_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC20_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC20_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc20_0(self) -> &'a mut W {
    self.variant(WUPDC20_A::WUPDC20_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc20_1(self) -> &'a mut W {
    self.variant(WUPDC20_A::WUPDC20_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc20_2(self) -> &'a mut W {
    self.variant(WUPDC20_A::WUPDC20_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC21_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC21_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC21_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC21_2,
}
impl From<WUPDC21_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC21_A) -> Self {
    match variant {
      WUPDC21_A::WUPDC21_0 => 0,
      WUPDC21_A::WUPDC21_1 => 1,
      WUPDC21_A::WUPDC21_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC21`"]
pub type WUPDC21_R = crate::R<u8, WUPDC21_A>;
impl WUPDC21_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC21_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC21_A::WUPDC21_0),
      1 => Val(WUPDC21_A::WUPDC21_1),
      2 => Val(WUPDC21_A::WUPDC21_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC21_0`"]
  #[inline(always)]
  pub fn is_wupdc21_0(&self) -> bool {
    *self == WUPDC21_A::WUPDC21_0
  }
  #[doc = "Checks if the value of the field is `WUPDC21_1`"]
  #[inline(always)]
  pub fn is_wupdc21_1(&self) -> bool {
    *self == WUPDC21_A::WUPDC21_1
  }
  #[doc = "Checks if the value of the field is `WUPDC21_2`"]
  #[inline(always)]
  pub fn is_wupdc21_2(&self) -> bool {
    *self == WUPDC21_A::WUPDC21_2
  }
}
#[doc = "Write proxy for field `WUPDC21`"]
pub struct WUPDC21_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC21_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC21_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc21_0(self) -> &'a mut W {
    self.variant(WUPDC21_A::WUPDC21_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc21_1(self) -> &'a mut W {
    self.variant(WUPDC21_A::WUPDC21_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc21_2(self) -> &'a mut W {
    self.variant(WUPDC21_A::WUPDC21_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC22_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC22_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC22_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC22_2,
}
impl From<WUPDC22_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC22_A) -> Self {
    match variant {
      WUPDC22_A::WUPDC22_0 => 0,
      WUPDC22_A::WUPDC22_1 => 1,
      WUPDC22_A::WUPDC22_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC22`"]
pub type WUPDC22_R = crate::R<u8, WUPDC22_A>;
impl WUPDC22_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC22_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC22_A::WUPDC22_0),
      1 => Val(WUPDC22_A::WUPDC22_1),
      2 => Val(WUPDC22_A::WUPDC22_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC22_0`"]
  #[inline(always)]
  pub fn is_wupdc22_0(&self) -> bool {
    *self == WUPDC22_A::WUPDC22_0
  }
  #[doc = "Checks if the value of the field is `WUPDC22_1`"]
  #[inline(always)]
  pub fn is_wupdc22_1(&self) -> bool {
    *self == WUPDC22_A::WUPDC22_1
  }
  #[doc = "Checks if the value of the field is `WUPDC22_2`"]
  #[inline(always)]
  pub fn is_wupdc22_2(&self) -> bool {
    *self == WUPDC22_A::WUPDC22_2
  }
}
#[doc = "Write proxy for field `WUPDC22`"]
pub struct WUPDC22_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC22_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC22_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc22_0(self) -> &'a mut W {
    self.variant(WUPDC22_A::WUPDC22_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc22_1(self) -> &'a mut W {
    self.variant(WUPDC22_A::WUPDC22_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc22_2(self) -> &'a mut W {
    self.variant(WUPDC22_A::WUPDC22_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC23_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC23_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC23_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC23_2,
}
impl From<WUPDC23_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC23_A) -> Self {
    match variant {
      WUPDC23_A::WUPDC23_0 => 0,
      WUPDC23_A::WUPDC23_1 => 1,
      WUPDC23_A::WUPDC23_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC23`"]
pub type WUPDC23_R = crate::R<u8, WUPDC23_A>;
impl WUPDC23_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC23_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC23_A::WUPDC23_0),
      1 => Val(WUPDC23_A::WUPDC23_1),
      2 => Val(WUPDC23_A::WUPDC23_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC23_0`"]
  #[inline(always)]
  pub fn is_wupdc23_0(&self) -> bool {
    *self == WUPDC23_A::WUPDC23_0
  }
  #[doc = "Checks if the value of the field is `WUPDC23_1`"]
  #[inline(always)]
  pub fn is_wupdc23_1(&self) -> bool {
    *self == WUPDC23_A::WUPDC23_1
  }
  #[doc = "Checks if the value of the field is `WUPDC23_2`"]
  #[inline(always)]
  pub fn is_wupdc23_2(&self) -> bool {
    *self == WUPDC23_A::WUPDC23_2
  }
}
#[doc = "Write proxy for field `WUPDC23`"]
pub struct WUPDC23_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC23_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC23_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc23_0(self) -> &'a mut W {
    self.variant(WUPDC23_A::WUPDC23_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc23_1(self) -> &'a mut W {
    self.variant(WUPDC23_A::WUPDC23_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc23_2(self) -> &'a mut W {
    self.variant(WUPDC23_A::WUPDC23_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC24_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC24_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC24_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC24_2,
}
impl From<WUPDC24_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC24_A) -> Self {
    match variant {
      WUPDC24_A::WUPDC24_0 => 0,
      WUPDC24_A::WUPDC24_1 => 1,
      WUPDC24_A::WUPDC24_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC24`"]
pub type WUPDC24_R = crate::R<u8, WUPDC24_A>;
impl WUPDC24_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC24_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC24_A::WUPDC24_0),
      1 => Val(WUPDC24_A::WUPDC24_1),
      2 => Val(WUPDC24_A::WUPDC24_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC24_0`"]
  #[inline(always)]
  pub fn is_wupdc24_0(&self) -> bool {
    *self == WUPDC24_A::WUPDC24_0
  }
  #[doc = "Checks if the value of the field is `WUPDC24_1`"]
  #[inline(always)]
  pub fn is_wupdc24_1(&self) -> bool {
    *self == WUPDC24_A::WUPDC24_1
  }
  #[doc = "Checks if the value of the field is `WUPDC24_2`"]
  #[inline(always)]
  pub fn is_wupdc24_2(&self) -> bool {
    *self == WUPDC24_A::WUPDC24_2
  }
}
#[doc = "Write proxy for field `WUPDC24`"]
pub struct WUPDC24_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC24_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC24_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc24_0(self) -> &'a mut W {
    self.variant(WUPDC24_A::WUPDC24_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc24_1(self) -> &'a mut W {
    self.variant(WUPDC24_A::WUPDC24_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc24_2(self) -> &'a mut W {
    self.variant(WUPDC24_A::WUPDC24_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC25_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC25_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC25_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC25_2,
}
impl From<WUPDC25_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC25_A) -> Self {
    match variant {
      WUPDC25_A::WUPDC25_0 => 0,
      WUPDC25_A::WUPDC25_1 => 1,
      WUPDC25_A::WUPDC25_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC25`"]
pub type WUPDC25_R = crate::R<u8, WUPDC25_A>;
impl WUPDC25_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC25_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC25_A::WUPDC25_0),
      1 => Val(WUPDC25_A::WUPDC25_1),
      2 => Val(WUPDC25_A::WUPDC25_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC25_0`"]
  #[inline(always)]
  pub fn is_wupdc25_0(&self) -> bool {
    *self == WUPDC25_A::WUPDC25_0
  }
  #[doc = "Checks if the value of the field is `WUPDC25_1`"]
  #[inline(always)]
  pub fn is_wupdc25_1(&self) -> bool {
    *self == WUPDC25_A::WUPDC25_1
  }
  #[doc = "Checks if the value of the field is `WUPDC25_2`"]
  #[inline(always)]
  pub fn is_wupdc25_2(&self) -> bool {
    *self == WUPDC25_A::WUPDC25_2
  }
}
#[doc = "Write proxy for field `WUPDC25`"]
pub struct WUPDC25_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC25_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC25_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc25_0(self) -> &'a mut W {
    self.variant(WUPDC25_A::WUPDC25_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc25_1(self) -> &'a mut W {
    self.variant(WUPDC25_A::WUPDC25_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc25_2(self) -> &'a mut W {
    self.variant(WUPDC25_A::WUPDC25_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC26_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC26_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC26_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC26_2,
}
impl From<WUPDC26_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC26_A) -> Self {
    match variant {
      WUPDC26_A::WUPDC26_0 => 0,
      WUPDC26_A::WUPDC26_1 => 1,
      WUPDC26_A::WUPDC26_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC26`"]
pub type WUPDC26_R = crate::R<u8, WUPDC26_A>;
impl WUPDC26_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC26_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC26_A::WUPDC26_0),
      1 => Val(WUPDC26_A::WUPDC26_1),
      2 => Val(WUPDC26_A::WUPDC26_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC26_0`"]
  #[inline(always)]
  pub fn is_wupdc26_0(&self) -> bool {
    *self == WUPDC26_A::WUPDC26_0
  }
  #[doc = "Checks if the value of the field is `WUPDC26_1`"]
  #[inline(always)]
  pub fn is_wupdc26_1(&self) -> bool {
    *self == WUPDC26_A::WUPDC26_1
  }
  #[doc = "Checks if the value of the field is `WUPDC26_2`"]
  #[inline(always)]
  pub fn is_wupdc26_2(&self) -> bool {
    *self == WUPDC26_A::WUPDC26_2
  }
}
#[doc = "Write proxy for field `WUPDC26`"]
pub struct WUPDC26_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC26_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC26_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc26_0(self) -> &'a mut W {
    self.variant(WUPDC26_A::WUPDC26_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc26_1(self) -> &'a mut W {
    self.variant(WUPDC26_A::WUPDC26_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc26_2(self) -> &'a mut W {
    self.variant(WUPDC26_A::WUPDC26_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED27_A {
  #[doc = "0: External input pin configured as interrupt"]
  RESERVED27_0,
  #[doc = "1: External input pin configured as DMA request"]
  RESERVED27_1,
  #[doc = "2: External input pin configured as trigger event"]
  RESERVED27_2,
}
impl From<RESERVED27_A> for u8 {
  #[inline(always)]
  fn from(variant: RESERVED27_A) -> Self {
    match variant {
      RESERVED27_A::RESERVED27_0 => 0,
      RESERVED27_A::RESERVED27_1 => 1,
      RESERVED27_A::RESERVED27_2 => 2,
    }
  }
}
#[doc = "Reader of field `Reserved27`"]
pub type RESERVED27_R = crate::R<u8, RESERVED27_A>;
impl RESERVED27_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RESERVED27_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(RESERVED27_A::RESERVED27_0),
      1 => Val(RESERVED27_A::RESERVED27_1),
      2 => Val(RESERVED27_A::RESERVED27_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED27_0`"]
  #[inline(always)]
  pub fn is_reserved27_0(&self) -> bool {
    *self == RESERVED27_A::RESERVED27_0
  }
  #[doc = "Checks if the value of the field is `RESERVED27_1`"]
  #[inline(always)]
  pub fn is_reserved27_1(&self) -> bool {
    *self == RESERVED27_A::RESERVED27_1
  }
  #[doc = "Checks if the value of the field is `RESERVED27_2`"]
  #[inline(always)]
  pub fn is_reserved27_2(&self) -> bool {
    *self == RESERVED27_A::RESERVED27_2
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED28_A {
  #[doc = "0: External input pin configured as interrupt"]
  RESERVED28_0,
  #[doc = "1: External input pin configured as DMA request"]
  RESERVED28_1,
  #[doc = "2: External input pin configured as trigger event"]
  RESERVED28_2,
}
impl From<RESERVED28_A> for u8 {
  #[inline(always)]
  fn from(variant: RESERVED28_A) -> Self {
    match variant {
      RESERVED28_A::RESERVED28_0 => 0,
      RESERVED28_A::RESERVED28_1 => 1,
      RESERVED28_A::RESERVED28_2 => 2,
    }
  }
}
#[doc = "Reader of field `Reserved28`"]
pub type RESERVED28_R = crate::R<u8, RESERVED28_A>;
impl RESERVED28_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, RESERVED28_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(RESERVED28_A::RESERVED28_0),
      1 => Val(RESERVED28_A::RESERVED28_1),
      2 => Val(RESERVED28_A::RESERVED28_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `RESERVED28_0`"]
  #[inline(always)]
  pub fn is_reserved28_0(&self) -> bool {
    *self == RESERVED28_A::RESERVED28_0
  }
  #[doc = "Checks if the value of the field is `RESERVED28_1`"]
  #[inline(always)]
  pub fn is_reserved28_1(&self) -> bool {
    *self == RESERVED28_A::RESERVED28_1
  }
  #[doc = "Checks if the value of the field is `RESERVED28_2`"]
  #[inline(always)]
  pub fn is_reserved28_2(&self) -> bool {
    *self == RESERVED28_A::RESERVED28_2
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC29_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC29_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC29_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC29_2,
}
impl From<WUPDC29_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC29_A) -> Self {
    match variant {
      WUPDC29_A::WUPDC29_0 => 0,
      WUPDC29_A::WUPDC29_1 => 1,
      WUPDC29_A::WUPDC29_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC29`"]
pub type WUPDC29_R = crate::R<u8, WUPDC29_A>;
impl WUPDC29_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC29_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC29_A::WUPDC29_0),
      1 => Val(WUPDC29_A::WUPDC29_1),
      2 => Val(WUPDC29_A::WUPDC29_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC29_0`"]
  #[inline(always)]
  pub fn is_wupdc29_0(&self) -> bool {
    *self == WUPDC29_A::WUPDC29_0
  }
  #[doc = "Checks if the value of the field is `WUPDC29_1`"]
  #[inline(always)]
  pub fn is_wupdc29_1(&self) -> bool {
    *self == WUPDC29_A::WUPDC29_1
  }
  #[doc = "Checks if the value of the field is `WUPDC29_2`"]
  #[inline(always)]
  pub fn is_wupdc29_2(&self) -> bool {
    *self == WUPDC29_A::WUPDC29_2
  }
}
#[doc = "Write proxy for field `WUPDC29`"]
pub struct WUPDC29_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC29_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC29_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc29_0(self) -> &'a mut W {
    self.variant(WUPDC29_A::WUPDC29_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc29_1(self) -> &'a mut W {
    self.variant(WUPDC29_A::WUPDC29_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc29_2(self) -> &'a mut W {
    self.variant(WUPDC29_A::WUPDC29_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC30_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC30_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC30_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC30_2,
}
impl From<WUPDC30_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC30_A) -> Self {
    match variant {
      WUPDC30_A::WUPDC30_0 => 0,
      WUPDC30_A::WUPDC30_1 => 1,
      WUPDC30_A::WUPDC30_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC30`"]
pub type WUPDC30_R = crate::R<u8, WUPDC30_A>;
impl WUPDC30_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC30_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC30_A::WUPDC30_0),
      1 => Val(WUPDC30_A::WUPDC30_1),
      2 => Val(WUPDC30_A::WUPDC30_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC30_0`"]
  #[inline(always)]
  pub fn is_wupdc30_0(&self) -> bool {
    *self == WUPDC30_A::WUPDC30_0
  }
  #[doc = "Checks if the value of the field is `WUPDC30_1`"]
  #[inline(always)]
  pub fn is_wupdc30_1(&self) -> bool {
    *self == WUPDC30_A::WUPDC30_1
  }
  #[doc = "Checks if the value of the field is `WUPDC30_2`"]
  #[inline(always)]
  pub fn is_wupdc30_2(&self) -> bool {
    *self == WUPDC30_A::WUPDC30_2
  }
}
#[doc = "Write proxy for field `WUPDC30`"]
pub struct WUPDC30_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC30_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC30_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc30_0(self) -> &'a mut W {
    self.variant(WUPDC30_A::WUPDC30_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc30_1(self) -> &'a mut W {
    self.variant(WUPDC30_A::WUPDC30_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc30_2(self) -> &'a mut W {
    self.variant(WUPDC30_A::WUPDC30_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC31_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC31_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC31_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC31_2,
}
impl From<WUPDC31_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC31_A) -> Self {
    match variant {
      WUPDC31_A::WUPDC31_0 => 0,
      WUPDC31_A::WUPDC31_1 => 1,
      WUPDC31_A::WUPDC31_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC31`"]
pub type WUPDC31_R = crate::R<u8, WUPDC31_A>;
impl WUPDC31_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC31_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC31_A::WUPDC31_0),
      1 => Val(WUPDC31_A::WUPDC31_1),
      2 => Val(WUPDC31_A::WUPDC31_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC31_0`"]
  #[inline(always)]
  pub fn is_wupdc31_0(&self) -> bool {
    *self == WUPDC31_A::WUPDC31_0
  }
  #[doc = "Checks if the value of the field is `WUPDC31_1`"]
  #[inline(always)]
  pub fn is_wupdc31_1(&self) -> bool {
    *self == WUPDC31_A::WUPDC31_1
  }
  #[doc = "Checks if the value of the field is `WUPDC31_2`"]
  #[inline(always)]
  pub fn is_wupdc31_2(&self) -> bool {
    *self == WUPDC31_A::WUPDC31_2
  }
}
#[doc = "Write proxy for field `WUPDC31`"]
pub struct WUPDC31_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC31_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC31_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc31_0(self) -> &'a mut W {
    self.variant(WUPDC31_A::WUPDC31_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc31_1(self) -> &'a mut W {
    self.variant(WUPDC31_A::WUPDC31_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc31_2(self) -> &'a mut W {
    self.variant(WUPDC31_A::WUPDC31_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc16(&self) -> WUPDC16_R {
    WUPDC16_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bits 2:3 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc17(&self) -> WUPDC17_R {
    WUPDC17_R::new(((self.bits >> 2) & 0x03) as u8)
  }
  #[doc = "Bits 4:5 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc18(&self) -> WUPDC18_R {
    WUPDC18_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 6:7 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc19(&self) -> WUPDC19_R {
    WUPDC19_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bits 8:9 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc20(&self) -> WUPDC20_R {
    WUPDC20_R::new(((self.bits >> 8) & 0x03) as u8)
  }
  #[doc = "Bits 10:11 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc21(&self) -> WUPDC21_R {
    WUPDC21_R::new(((self.bits >> 10) & 0x03) as u8)
  }
  #[doc = "Bits 12:13 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc22(&self) -> WUPDC22_R {
    WUPDC22_R::new(((self.bits >> 12) & 0x03) as u8)
  }
  #[doc = "Bits 14:15 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc23(&self) -> WUPDC23_R {
    WUPDC23_R::new(((self.bits >> 14) & 0x03) as u8)
  }
  #[doc = "Bits 16:17 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc24(&self) -> WUPDC24_R {
    WUPDC24_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bits 18:19 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc25(&self) -> WUPDC25_R {
    WUPDC25_R::new(((self.bits >> 18) & 0x03) as u8)
  }
  #[doc = "Bits 20:21 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc26(&self) -> WUPDC26_R {
    WUPDC26_R::new(((self.bits >> 20) & 0x03) as u8)
  }
  #[doc = "Bits 22:23 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn reserved27(&self) -> RESERVED27_R {
    RESERVED27_R::new(((self.bits >> 22) & 0x03) as u8)
  }
  #[doc = "Bits 24:25 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn reserved28(&self) -> RESERVED28_R {
    RESERVED28_R::new(((self.bits >> 24) & 0x03) as u8)
  }
  #[doc = "Bits 26:27 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc29(&self) -> WUPDC29_R {
    WUPDC29_R::new(((self.bits >> 26) & 0x03) as u8)
  }
  #[doc = "Bits 28:29 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc30(&self) -> WUPDC30_R {
    WUPDC30_R::new(((self.bits >> 28) & 0x03) as u8)
  }
  #[doc = "Bits 30:31 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc31(&self) -> WUPDC31_R {
    WUPDC31_R::new(((self.bits >> 30) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc16(&mut self) -> WUPDC16_W {
    WUPDC16_W { w: self }
  }
  #[doc = "Bits 2:3 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc17(&mut self) -> WUPDC17_W {
    WUPDC17_W { w: self }
  }
  #[doc = "Bits 4:5 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc18(&mut self) -> WUPDC18_W {
    WUPDC18_W { w: self }
  }
  #[doc = "Bits 6:7 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc19(&mut self) -> WUPDC19_W {
    WUPDC19_W { w: self }
  }
  #[doc = "Bits 8:9 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc20(&mut self) -> WUPDC20_W {
    WUPDC20_W { w: self }
  }
  #[doc = "Bits 10:11 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc21(&mut self) -> WUPDC21_W {
    WUPDC21_W { w: self }
  }
  #[doc = "Bits 12:13 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc22(&mut self) -> WUPDC22_W {
    WUPDC22_W { w: self }
  }
  #[doc = "Bits 14:15 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc23(&mut self) -> WUPDC23_W {
    WUPDC23_W { w: self }
  }
  #[doc = "Bits 16:17 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc24(&mut self) -> WUPDC24_W {
    WUPDC24_W { w: self }
  }
  #[doc = "Bits 18:19 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc25(&mut self) -> WUPDC25_W {
    WUPDC25_W { w: self }
  }
  #[doc = "Bits 20:21 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc26(&mut self) -> WUPDC26_W {
    WUPDC26_W { w: self }
  }
  #[doc = "Bits 26:27 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc29(&mut self) -> WUPDC29_W {
    WUPDC29_W { w: self }
  }
  #[doc = "Bits 28:29 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc30(&mut self) -> WUPDC30_W {
    WUPDC30_W { w: self }
  }
  #[doc = "Bits 30:31 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc31(&mut self) -> WUPDC31_W {
    WUPDC31_W { w: self }
  }
}
