#[doc = "Reader of register PDC1"]
pub type R = crate::R<u32, super::PDC1>;
#[doc = "Writer for register PDC1"]
pub type W = crate::W<u32, super::PDC1>;
#[doc = "Register PDC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDC1 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC0_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC0_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC0_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC0_2,
}
impl From<WUPDC0_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC0_A) -> Self {
    match variant {
      WUPDC0_A::WUPDC0_0 => 0,
      WUPDC0_A::WUPDC0_1 => 1,
      WUPDC0_A::WUPDC0_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC0`"]
pub type WUPDC0_R = crate::R<u8, WUPDC0_A>;
impl WUPDC0_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC0_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC0_A::WUPDC0_0),
      1 => Val(WUPDC0_A::WUPDC0_1),
      2 => Val(WUPDC0_A::WUPDC0_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC0_0`"]
  #[inline(always)]
  pub fn is_wupdc0_0(&self) -> bool {
    *self == WUPDC0_A::WUPDC0_0
  }
  #[doc = "Checks if the value of the field is `WUPDC0_1`"]
  #[inline(always)]
  pub fn is_wupdc0_1(&self) -> bool {
    *self == WUPDC0_A::WUPDC0_1
  }
  #[doc = "Checks if the value of the field is `WUPDC0_2`"]
  #[inline(always)]
  pub fn is_wupdc0_2(&self) -> bool {
    *self == WUPDC0_A::WUPDC0_2
  }
}
#[doc = "Write proxy for field `WUPDC0`"]
pub struct WUPDC0_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC0_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC0_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc0_0(self) -> &'a mut W {
    self.variant(WUPDC0_A::WUPDC0_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc0_1(self) -> &'a mut W {
    self.variant(WUPDC0_A::WUPDC0_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc0_2(self) -> &'a mut W {
    self.variant(WUPDC0_A::WUPDC0_2)
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
pub enum WUPDC1_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC1_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC1_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC1_2,
}
impl From<WUPDC1_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC1_A) -> Self {
    match variant {
      WUPDC1_A::WUPDC1_0 => 0,
      WUPDC1_A::WUPDC1_1 => 1,
      WUPDC1_A::WUPDC1_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC1`"]
pub type WUPDC1_R = crate::R<u8, WUPDC1_A>;
impl WUPDC1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC1_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC1_A::WUPDC1_0),
      1 => Val(WUPDC1_A::WUPDC1_1),
      2 => Val(WUPDC1_A::WUPDC1_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC1_0`"]
  #[inline(always)]
  pub fn is_wupdc1_0(&self) -> bool {
    *self == WUPDC1_A::WUPDC1_0
  }
  #[doc = "Checks if the value of the field is `WUPDC1_1`"]
  #[inline(always)]
  pub fn is_wupdc1_1(&self) -> bool {
    *self == WUPDC1_A::WUPDC1_1
  }
  #[doc = "Checks if the value of the field is `WUPDC1_2`"]
  #[inline(always)]
  pub fn is_wupdc1_2(&self) -> bool {
    *self == WUPDC1_A::WUPDC1_2
  }
}
#[doc = "Write proxy for field `WUPDC1`"]
pub struct WUPDC1_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC1_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc1_0(self) -> &'a mut W {
    self.variant(WUPDC1_A::WUPDC1_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc1_1(self) -> &'a mut W {
    self.variant(WUPDC1_A::WUPDC1_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc1_2(self) -> &'a mut W {
    self.variant(WUPDC1_A::WUPDC1_2)
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
pub enum WUPDC2_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC2_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC2_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC2_2,
}
impl From<WUPDC2_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC2_A) -> Self {
    match variant {
      WUPDC2_A::WUPDC2_0 => 0,
      WUPDC2_A::WUPDC2_1 => 1,
      WUPDC2_A::WUPDC2_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC2`"]
pub type WUPDC2_R = crate::R<u8, WUPDC2_A>;
impl WUPDC2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC2_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC2_A::WUPDC2_0),
      1 => Val(WUPDC2_A::WUPDC2_1),
      2 => Val(WUPDC2_A::WUPDC2_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC2_0`"]
  #[inline(always)]
  pub fn is_wupdc2_0(&self) -> bool {
    *self == WUPDC2_A::WUPDC2_0
  }
  #[doc = "Checks if the value of the field is `WUPDC2_1`"]
  #[inline(always)]
  pub fn is_wupdc2_1(&self) -> bool {
    *self == WUPDC2_A::WUPDC2_1
  }
  #[doc = "Checks if the value of the field is `WUPDC2_2`"]
  #[inline(always)]
  pub fn is_wupdc2_2(&self) -> bool {
    *self == WUPDC2_A::WUPDC2_2
  }
}
#[doc = "Write proxy for field `WUPDC2`"]
pub struct WUPDC2_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC2_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc2_0(self) -> &'a mut W {
    self.variant(WUPDC2_A::WUPDC2_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc2_1(self) -> &'a mut W {
    self.variant(WUPDC2_A::WUPDC2_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc2_2(self) -> &'a mut W {
    self.variant(WUPDC2_A::WUPDC2_2)
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
pub enum WUPDC3_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC3_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC3_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC3_2,
}
impl From<WUPDC3_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC3_A) -> Self {
    match variant {
      WUPDC3_A::WUPDC3_0 => 0,
      WUPDC3_A::WUPDC3_1 => 1,
      WUPDC3_A::WUPDC3_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC3`"]
pub type WUPDC3_R = crate::R<u8, WUPDC3_A>;
impl WUPDC3_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC3_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC3_A::WUPDC3_0),
      1 => Val(WUPDC3_A::WUPDC3_1),
      2 => Val(WUPDC3_A::WUPDC3_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC3_0`"]
  #[inline(always)]
  pub fn is_wupdc3_0(&self) -> bool {
    *self == WUPDC3_A::WUPDC3_0
  }
  #[doc = "Checks if the value of the field is `WUPDC3_1`"]
  #[inline(always)]
  pub fn is_wupdc3_1(&self) -> bool {
    *self == WUPDC3_A::WUPDC3_1
  }
  #[doc = "Checks if the value of the field is `WUPDC3_2`"]
  #[inline(always)]
  pub fn is_wupdc3_2(&self) -> bool {
    *self == WUPDC3_A::WUPDC3_2
  }
}
#[doc = "Write proxy for field `WUPDC3`"]
pub struct WUPDC3_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC3_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC3_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc3_0(self) -> &'a mut W {
    self.variant(WUPDC3_A::WUPDC3_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc3_1(self) -> &'a mut W {
    self.variant(WUPDC3_A::WUPDC3_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc3_2(self) -> &'a mut W {
    self.variant(WUPDC3_A::WUPDC3_2)
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
pub enum WUPDC4_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC4_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC4_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC4_2,
}
impl From<WUPDC4_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC4_A) -> Self {
    match variant {
      WUPDC4_A::WUPDC4_0 => 0,
      WUPDC4_A::WUPDC4_1 => 1,
      WUPDC4_A::WUPDC4_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC4`"]
pub type WUPDC4_R = crate::R<u8, WUPDC4_A>;
impl WUPDC4_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC4_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC4_A::WUPDC4_0),
      1 => Val(WUPDC4_A::WUPDC4_1),
      2 => Val(WUPDC4_A::WUPDC4_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC4_0`"]
  #[inline(always)]
  pub fn is_wupdc4_0(&self) -> bool {
    *self == WUPDC4_A::WUPDC4_0
  }
  #[doc = "Checks if the value of the field is `WUPDC4_1`"]
  #[inline(always)]
  pub fn is_wupdc4_1(&self) -> bool {
    *self == WUPDC4_A::WUPDC4_1
  }
  #[doc = "Checks if the value of the field is `WUPDC4_2`"]
  #[inline(always)]
  pub fn is_wupdc4_2(&self) -> bool {
    *self == WUPDC4_A::WUPDC4_2
  }
}
#[doc = "Write proxy for field `WUPDC4`"]
pub struct WUPDC4_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC4_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC4_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc4_0(self) -> &'a mut W {
    self.variant(WUPDC4_A::WUPDC4_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc4_1(self) -> &'a mut W {
    self.variant(WUPDC4_A::WUPDC4_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc4_2(self) -> &'a mut W {
    self.variant(WUPDC4_A::WUPDC4_2)
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
pub enum WUPDC5_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC5_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC5_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC5_2,
}
impl From<WUPDC5_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC5_A) -> Self {
    match variant {
      WUPDC5_A::WUPDC5_0 => 0,
      WUPDC5_A::WUPDC5_1 => 1,
      WUPDC5_A::WUPDC5_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC5`"]
pub type WUPDC5_R = crate::R<u8, WUPDC5_A>;
impl WUPDC5_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC5_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC5_A::WUPDC5_0),
      1 => Val(WUPDC5_A::WUPDC5_1),
      2 => Val(WUPDC5_A::WUPDC5_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC5_0`"]
  #[inline(always)]
  pub fn is_wupdc5_0(&self) -> bool {
    *self == WUPDC5_A::WUPDC5_0
  }
  #[doc = "Checks if the value of the field is `WUPDC5_1`"]
  #[inline(always)]
  pub fn is_wupdc5_1(&self) -> bool {
    *self == WUPDC5_A::WUPDC5_1
  }
  #[doc = "Checks if the value of the field is `WUPDC5_2`"]
  #[inline(always)]
  pub fn is_wupdc5_2(&self) -> bool {
    *self == WUPDC5_A::WUPDC5_2
  }
}
#[doc = "Write proxy for field `WUPDC5`"]
pub struct WUPDC5_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC5_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC5_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc5_0(self) -> &'a mut W {
    self.variant(WUPDC5_A::WUPDC5_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc5_1(self) -> &'a mut W {
    self.variant(WUPDC5_A::WUPDC5_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc5_2(self) -> &'a mut W {
    self.variant(WUPDC5_A::WUPDC5_2)
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
pub enum WUPDC6_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC6_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC6_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC6_2,
}
impl From<WUPDC6_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC6_A) -> Self {
    match variant {
      WUPDC6_A::WUPDC6_0 => 0,
      WUPDC6_A::WUPDC6_1 => 1,
      WUPDC6_A::WUPDC6_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC6`"]
pub type WUPDC6_R = crate::R<u8, WUPDC6_A>;
impl WUPDC6_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC6_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC6_A::WUPDC6_0),
      1 => Val(WUPDC6_A::WUPDC6_1),
      2 => Val(WUPDC6_A::WUPDC6_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC6_0`"]
  #[inline(always)]
  pub fn is_wupdc6_0(&self) -> bool {
    *self == WUPDC6_A::WUPDC6_0
  }
  #[doc = "Checks if the value of the field is `WUPDC6_1`"]
  #[inline(always)]
  pub fn is_wupdc6_1(&self) -> bool {
    *self == WUPDC6_A::WUPDC6_1
  }
  #[doc = "Checks if the value of the field is `WUPDC6_2`"]
  #[inline(always)]
  pub fn is_wupdc6_2(&self) -> bool {
    *self == WUPDC6_A::WUPDC6_2
  }
}
#[doc = "Write proxy for field `WUPDC6`"]
pub struct WUPDC6_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC6_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC6_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc6_0(self) -> &'a mut W {
    self.variant(WUPDC6_A::WUPDC6_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc6_1(self) -> &'a mut W {
    self.variant(WUPDC6_A::WUPDC6_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc6_2(self) -> &'a mut W {
    self.variant(WUPDC6_A::WUPDC6_2)
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
pub enum WUPDC7_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC7_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC7_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC7_2,
}
impl From<WUPDC7_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC7_A) -> Self {
    match variant {
      WUPDC7_A::WUPDC7_0 => 0,
      WUPDC7_A::WUPDC7_1 => 1,
      WUPDC7_A::WUPDC7_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC7`"]
pub type WUPDC7_R = crate::R<u8, WUPDC7_A>;
impl WUPDC7_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC7_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC7_A::WUPDC7_0),
      1 => Val(WUPDC7_A::WUPDC7_1),
      2 => Val(WUPDC7_A::WUPDC7_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC7_0`"]
  #[inline(always)]
  pub fn is_wupdc7_0(&self) -> bool {
    *self == WUPDC7_A::WUPDC7_0
  }
  #[doc = "Checks if the value of the field is `WUPDC7_1`"]
  #[inline(always)]
  pub fn is_wupdc7_1(&self) -> bool {
    *self == WUPDC7_A::WUPDC7_1
  }
  #[doc = "Checks if the value of the field is `WUPDC7_2`"]
  #[inline(always)]
  pub fn is_wupdc7_2(&self) -> bool {
    *self == WUPDC7_A::WUPDC7_2
  }
}
#[doc = "Write proxy for field `WUPDC7`"]
pub struct WUPDC7_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC7_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC7_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc7_0(self) -> &'a mut W {
    self.variant(WUPDC7_A::WUPDC7_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc7_1(self) -> &'a mut W {
    self.variant(WUPDC7_A::WUPDC7_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc7_2(self) -> &'a mut W {
    self.variant(WUPDC7_A::WUPDC7_2)
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
pub enum WUPDC8_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC8_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC8_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC8_2,
}
impl From<WUPDC8_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC8_A) -> Self {
    match variant {
      WUPDC8_A::WUPDC8_0 => 0,
      WUPDC8_A::WUPDC8_1 => 1,
      WUPDC8_A::WUPDC8_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC8`"]
pub type WUPDC8_R = crate::R<u8, WUPDC8_A>;
impl WUPDC8_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC8_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC8_A::WUPDC8_0),
      1 => Val(WUPDC8_A::WUPDC8_1),
      2 => Val(WUPDC8_A::WUPDC8_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC8_0`"]
  #[inline(always)]
  pub fn is_wupdc8_0(&self) -> bool {
    *self == WUPDC8_A::WUPDC8_0
  }
  #[doc = "Checks if the value of the field is `WUPDC8_1`"]
  #[inline(always)]
  pub fn is_wupdc8_1(&self) -> bool {
    *self == WUPDC8_A::WUPDC8_1
  }
  #[doc = "Checks if the value of the field is `WUPDC8_2`"]
  #[inline(always)]
  pub fn is_wupdc8_2(&self) -> bool {
    *self == WUPDC8_A::WUPDC8_2
  }
}
#[doc = "Write proxy for field `WUPDC8`"]
pub struct WUPDC8_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC8_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC8_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc8_0(self) -> &'a mut W {
    self.variant(WUPDC8_A::WUPDC8_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc8_1(self) -> &'a mut W {
    self.variant(WUPDC8_A::WUPDC8_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc8_2(self) -> &'a mut W {
    self.variant(WUPDC8_A::WUPDC8_2)
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
pub enum WUPDC9_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC9_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC9_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC9_2,
}
impl From<WUPDC9_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC9_A) -> Self {
    match variant {
      WUPDC9_A::WUPDC9_0 => 0,
      WUPDC9_A::WUPDC9_1 => 1,
      WUPDC9_A::WUPDC9_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC9`"]
pub type WUPDC9_R = crate::R<u8, WUPDC9_A>;
impl WUPDC9_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC9_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC9_A::WUPDC9_0),
      1 => Val(WUPDC9_A::WUPDC9_1),
      2 => Val(WUPDC9_A::WUPDC9_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC9_0`"]
  #[inline(always)]
  pub fn is_wupdc9_0(&self) -> bool {
    *self == WUPDC9_A::WUPDC9_0
  }
  #[doc = "Checks if the value of the field is `WUPDC9_1`"]
  #[inline(always)]
  pub fn is_wupdc9_1(&self) -> bool {
    *self == WUPDC9_A::WUPDC9_1
  }
  #[doc = "Checks if the value of the field is `WUPDC9_2`"]
  #[inline(always)]
  pub fn is_wupdc9_2(&self) -> bool {
    *self == WUPDC9_A::WUPDC9_2
  }
}
#[doc = "Write proxy for field `WUPDC9`"]
pub struct WUPDC9_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC9_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC9_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc9_0(self) -> &'a mut W {
    self.variant(WUPDC9_A::WUPDC9_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc9_1(self) -> &'a mut W {
    self.variant(WUPDC9_A::WUPDC9_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc9_2(self) -> &'a mut W {
    self.variant(WUPDC9_A::WUPDC9_2)
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
pub enum WUPDC10_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC10_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC10_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC10_2,
}
impl From<WUPDC10_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC10_A) -> Self {
    match variant {
      WUPDC10_A::WUPDC10_0 => 0,
      WUPDC10_A::WUPDC10_1 => 1,
      WUPDC10_A::WUPDC10_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC10`"]
pub type WUPDC10_R = crate::R<u8, WUPDC10_A>;
impl WUPDC10_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC10_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC10_A::WUPDC10_0),
      1 => Val(WUPDC10_A::WUPDC10_1),
      2 => Val(WUPDC10_A::WUPDC10_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC10_0`"]
  #[inline(always)]
  pub fn is_wupdc10_0(&self) -> bool {
    *self == WUPDC10_A::WUPDC10_0
  }
  #[doc = "Checks if the value of the field is `WUPDC10_1`"]
  #[inline(always)]
  pub fn is_wupdc10_1(&self) -> bool {
    *self == WUPDC10_A::WUPDC10_1
  }
  #[doc = "Checks if the value of the field is `WUPDC10_2`"]
  #[inline(always)]
  pub fn is_wupdc10_2(&self) -> bool {
    *self == WUPDC10_A::WUPDC10_2
  }
}
#[doc = "Write proxy for field `WUPDC10`"]
pub struct WUPDC10_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC10_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC10_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc10_0(self) -> &'a mut W {
    self.variant(WUPDC10_A::WUPDC10_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc10_1(self) -> &'a mut W {
    self.variant(WUPDC10_A::WUPDC10_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc10_2(self) -> &'a mut W {
    self.variant(WUPDC10_A::WUPDC10_2)
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
pub enum WUPDC11_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC11_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC11_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC11_2,
}
impl From<WUPDC11_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC11_A) -> Self {
    match variant {
      WUPDC11_A::WUPDC11_0 => 0,
      WUPDC11_A::WUPDC11_1 => 1,
      WUPDC11_A::WUPDC11_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC11`"]
pub type WUPDC11_R = crate::R<u8, WUPDC11_A>;
impl WUPDC11_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC11_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC11_A::WUPDC11_0),
      1 => Val(WUPDC11_A::WUPDC11_1),
      2 => Val(WUPDC11_A::WUPDC11_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC11_0`"]
  #[inline(always)]
  pub fn is_wupdc11_0(&self) -> bool {
    *self == WUPDC11_A::WUPDC11_0
  }
  #[doc = "Checks if the value of the field is `WUPDC11_1`"]
  #[inline(always)]
  pub fn is_wupdc11_1(&self) -> bool {
    *self == WUPDC11_A::WUPDC11_1
  }
  #[doc = "Checks if the value of the field is `WUPDC11_2`"]
  #[inline(always)]
  pub fn is_wupdc11_2(&self) -> bool {
    *self == WUPDC11_A::WUPDC11_2
  }
}
#[doc = "Write proxy for field `WUPDC11`"]
pub struct WUPDC11_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC11_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC11_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc11_0(self) -> &'a mut W {
    self.variant(WUPDC11_A::WUPDC11_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc11_1(self) -> &'a mut W {
    self.variant(WUPDC11_A::WUPDC11_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc11_2(self) -> &'a mut W {
    self.variant(WUPDC11_A::WUPDC11_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC12_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC12_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC12_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC12_2,
}
impl From<WUPDC12_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC12_A) -> Self {
    match variant {
      WUPDC12_A::WUPDC12_0 => 0,
      WUPDC12_A::WUPDC12_1 => 1,
      WUPDC12_A::WUPDC12_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC12`"]
pub type WUPDC12_R = crate::R<u8, WUPDC12_A>;
impl WUPDC12_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC12_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC12_A::WUPDC12_0),
      1 => Val(WUPDC12_A::WUPDC12_1),
      2 => Val(WUPDC12_A::WUPDC12_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC12_0`"]
  #[inline(always)]
  pub fn is_wupdc12_0(&self) -> bool {
    *self == WUPDC12_A::WUPDC12_0
  }
  #[doc = "Checks if the value of the field is `WUPDC12_1`"]
  #[inline(always)]
  pub fn is_wupdc12_1(&self) -> bool {
    *self == WUPDC12_A::WUPDC12_1
  }
  #[doc = "Checks if the value of the field is `WUPDC12_2`"]
  #[inline(always)]
  pub fn is_wupdc12_2(&self) -> bool {
    *self == WUPDC12_A::WUPDC12_2
  }
}
#[doc = "Write proxy for field `WUPDC12`"]
pub struct WUPDC12_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC12_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC12_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc12_0(self) -> &'a mut W {
    self.variant(WUPDC12_A::WUPDC12_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc12_1(self) -> &'a mut W {
    self.variant(WUPDC12_A::WUPDC12_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc12_2(self) -> &'a mut W {
    self.variant(WUPDC12_A::WUPDC12_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
    self.w
  }
}
#[doc = "Wakeup pin configuration for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPDC13_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC13_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC13_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC13_2,
}
impl From<WUPDC13_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC13_A) -> Self {
    match variant {
      WUPDC13_A::WUPDC13_0 => 0,
      WUPDC13_A::WUPDC13_1 => 1,
      WUPDC13_A::WUPDC13_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC13`"]
pub type WUPDC13_R = crate::R<u8, WUPDC13_A>;
impl WUPDC13_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC13_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC13_A::WUPDC13_0),
      1 => Val(WUPDC13_A::WUPDC13_1),
      2 => Val(WUPDC13_A::WUPDC13_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC13_0`"]
  #[inline(always)]
  pub fn is_wupdc13_0(&self) -> bool {
    *self == WUPDC13_A::WUPDC13_0
  }
  #[doc = "Checks if the value of the field is `WUPDC13_1`"]
  #[inline(always)]
  pub fn is_wupdc13_1(&self) -> bool {
    *self == WUPDC13_A::WUPDC13_1
  }
  #[doc = "Checks if the value of the field is `WUPDC13_2`"]
  #[inline(always)]
  pub fn is_wupdc13_2(&self) -> bool {
    *self == WUPDC13_A::WUPDC13_2
  }
}
#[doc = "Write proxy for field `WUPDC13`"]
pub struct WUPDC13_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC13_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC13_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc13_0(self) -> &'a mut W {
    self.variant(WUPDC13_A::WUPDC13_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc13_1(self) -> &'a mut W {
    self.variant(WUPDC13_A::WUPDC13_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc13_2(self) -> &'a mut W {
    self.variant(WUPDC13_A::WUPDC13_2)
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
pub enum WUPDC14_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC14_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC14_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC14_2,
}
impl From<WUPDC14_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC14_A) -> Self {
    match variant {
      WUPDC14_A::WUPDC14_0 => 0,
      WUPDC14_A::WUPDC14_1 => 1,
      WUPDC14_A::WUPDC14_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC14`"]
pub type WUPDC14_R = crate::R<u8, WUPDC14_A>;
impl WUPDC14_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC14_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC14_A::WUPDC14_0),
      1 => Val(WUPDC14_A::WUPDC14_1),
      2 => Val(WUPDC14_A::WUPDC14_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC14_0`"]
  #[inline(always)]
  pub fn is_wupdc14_0(&self) -> bool {
    *self == WUPDC14_A::WUPDC14_0
  }
  #[doc = "Checks if the value of the field is `WUPDC14_1`"]
  #[inline(always)]
  pub fn is_wupdc14_1(&self) -> bool {
    *self == WUPDC14_A::WUPDC14_1
  }
  #[doc = "Checks if the value of the field is `WUPDC14_2`"]
  #[inline(always)]
  pub fn is_wupdc14_2(&self) -> bool {
    *self == WUPDC14_A::WUPDC14_2
  }
}
#[doc = "Write proxy for field `WUPDC14`"]
pub struct WUPDC14_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC14_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC14_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc14_0(self) -> &'a mut W {
    self.variant(WUPDC14_A::WUPDC14_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc14_1(self) -> &'a mut W {
    self.variant(WUPDC14_A::WUPDC14_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc14_2(self) -> &'a mut W {
    self.variant(WUPDC14_A::WUPDC14_2)
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
pub enum WUPDC15_A {
  #[doc = "0: External input pin configured as interrupt"]
  WUPDC15_0,
  #[doc = "1: External input pin configured as DMA request"]
  WUPDC15_1,
  #[doc = "2: External input pin configured as trigger event"]
  WUPDC15_2,
}
impl From<WUPDC15_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPDC15_A) -> Self {
    match variant {
      WUPDC15_A::WUPDC15_0 => 0,
      WUPDC15_A::WUPDC15_1 => 1,
      WUPDC15_A::WUPDC15_2 => 2,
    }
  }
}
#[doc = "Reader of field `WUPDC15`"]
pub type WUPDC15_R = crate::R<u8, WUPDC15_A>;
impl WUPDC15_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, WUPDC15_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(WUPDC15_A::WUPDC15_0),
      1 => Val(WUPDC15_A::WUPDC15_1),
      2 => Val(WUPDC15_A::WUPDC15_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `WUPDC15_0`"]
  #[inline(always)]
  pub fn is_wupdc15_0(&self) -> bool {
    *self == WUPDC15_A::WUPDC15_0
  }
  #[doc = "Checks if the value of the field is `WUPDC15_1`"]
  #[inline(always)]
  pub fn is_wupdc15_1(&self) -> bool {
    *self == WUPDC15_A::WUPDC15_1
  }
  #[doc = "Checks if the value of the field is `WUPDC15_2`"]
  #[inline(always)]
  pub fn is_wupdc15_2(&self) -> bool {
    *self == WUPDC15_A::WUPDC15_2
  }
}
#[doc = "Write proxy for field `WUPDC15`"]
pub struct WUPDC15_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPDC15_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPDC15_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "External input pin configured as interrupt"]
  #[inline(always)]
  pub fn wupdc15_0(self) -> &'a mut W {
    self.variant(WUPDC15_A::WUPDC15_0)
  }
  #[doc = "External input pin configured as DMA request"]
  #[inline(always)]
  pub fn wupdc15_1(self) -> &'a mut W {
    self.variant(WUPDC15_A::WUPDC15_1)
  }
  #[doc = "External input pin configured as trigger event"]
  #[inline(always)]
  pub fn wupdc15_2(self) -> &'a mut W {
    self.variant(WUPDC15_A::WUPDC15_2)
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
  pub fn wupdc0(&self) -> WUPDC0_R {
    WUPDC0_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bits 2:3 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc1(&self) -> WUPDC1_R {
    WUPDC1_R::new(((self.bits >> 2) & 0x03) as u8)
  }
  #[doc = "Bits 4:5 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc2(&self) -> WUPDC2_R {
    WUPDC2_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 6:7 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc3(&self) -> WUPDC3_R {
    WUPDC3_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bits 8:9 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc4(&self) -> WUPDC4_R {
    WUPDC4_R::new(((self.bits >> 8) & 0x03) as u8)
  }
  #[doc = "Bits 10:11 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc5(&self) -> WUPDC5_R {
    WUPDC5_R::new(((self.bits >> 10) & 0x03) as u8)
  }
  #[doc = "Bits 12:13 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc6(&self) -> WUPDC6_R {
    WUPDC6_R::new(((self.bits >> 12) & 0x03) as u8)
  }
  #[doc = "Bits 14:15 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc7(&self) -> WUPDC7_R {
    WUPDC7_R::new(((self.bits >> 14) & 0x03) as u8)
  }
  #[doc = "Bits 16:17 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc8(&self) -> WUPDC8_R {
    WUPDC8_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bits 18:19 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc9(&self) -> WUPDC9_R {
    WUPDC9_R::new(((self.bits >> 18) & 0x03) as u8)
  }
  #[doc = "Bits 20:21 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc10(&self) -> WUPDC10_R {
    WUPDC10_R::new(((self.bits >> 20) & 0x03) as u8)
  }
  #[doc = "Bits 22:23 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc11(&self) -> WUPDC11_R {
    WUPDC11_R::new(((self.bits >> 22) & 0x03) as u8)
  }
  #[doc = "Bits 24:25 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc12(&self) -> WUPDC12_R {
    WUPDC12_R::new(((self.bits >> 24) & 0x03) as u8)
  }
  #[doc = "Bits 26:27 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc13(&self) -> WUPDC13_R {
    WUPDC13_R::new(((self.bits >> 26) & 0x03) as u8)
  }
  #[doc = "Bits 28:29 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc14(&self) -> WUPDC14_R {
    WUPDC14_R::new(((self.bits >> 28) & 0x03) as u8)
  }
  #[doc = "Bits 30:31 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc15(&self) -> WUPDC15_R {
    WUPDC15_R::new(((self.bits >> 30) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc0(&mut self) -> WUPDC0_W {
    WUPDC0_W { w: self }
  }
  #[doc = "Bits 2:3 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc1(&mut self) -> WUPDC1_W {
    WUPDC1_W { w: self }
  }
  #[doc = "Bits 4:5 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc2(&mut self) -> WUPDC2_W {
    WUPDC2_W { w: self }
  }
  #[doc = "Bits 6:7 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc3(&mut self) -> WUPDC3_W {
    WUPDC3_W { w: self }
  }
  #[doc = "Bits 8:9 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc4(&mut self) -> WUPDC4_W {
    WUPDC4_W { w: self }
  }
  #[doc = "Bits 10:11 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc5(&mut self) -> WUPDC5_W {
    WUPDC5_W { w: self }
  }
  #[doc = "Bits 12:13 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc6(&mut self) -> WUPDC6_W {
    WUPDC6_W { w: self }
  }
  #[doc = "Bits 14:15 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc7(&mut self) -> WUPDC7_W {
    WUPDC7_W { w: self }
  }
  #[doc = "Bits 16:17 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc8(&mut self) -> WUPDC8_W {
    WUPDC8_W { w: self }
  }
  #[doc = "Bits 18:19 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc9(&mut self) -> WUPDC9_W {
    WUPDC9_W { w: self }
  }
  #[doc = "Bits 20:21 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc10(&mut self) -> WUPDC10_W {
    WUPDC10_W { w: self }
  }
  #[doc = "Bits 22:23 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc11(&mut self) -> WUPDC11_W {
    WUPDC11_W { w: self }
  }
  #[doc = "Bits 24:25 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc12(&mut self) -> WUPDC12_W {
    WUPDC12_W { w: self }
  }
  #[doc = "Bits 26:27 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc13(&mut self) -> WUPDC13_W {
    WUPDC13_W { w: self }
  }
  #[doc = "Bits 28:29 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc14(&mut self) -> WUPDC14_W {
    WUPDC14_W { w: self }
  }
  #[doc = "Bits 30:31 - Wakeup pin configuration for LLWU_Pn"]
  #[inline(always)]
  pub fn wupdc15(&mut self) -> WUPDC15_W {
    WUPDC15_W { w: self }
  }
}
