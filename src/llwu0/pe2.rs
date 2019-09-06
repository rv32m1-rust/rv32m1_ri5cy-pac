#[doc = "Reader of register PE2"]
pub type R = crate::R<u32, super::PE2>;
#[doc = "Writer for register PE2"]
pub type W = crate::W<u32, super::PE2>;
#[doc = "Register PE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PE2 {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE16_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE16_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE16_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE16_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE16_3,
}
impl From<WUPE16_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE16_A) -> Self {
    match variant {
      WUPE16_A::WUPE16_0 => 0,
      WUPE16_A::WUPE16_1 => 1,
      WUPE16_A::WUPE16_2 => 2,
      WUPE16_A::WUPE16_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE16`"]
pub type WUPE16_R = crate::R<u8, WUPE16_A>;
impl WUPE16_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE16_A {
    match self.bits {
      0 => WUPE16_A::WUPE16_0,
      1 => WUPE16_A::WUPE16_1,
      2 => WUPE16_A::WUPE16_2,
      3 => WUPE16_A::WUPE16_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE16_0`"]
  #[inline(always)]
  pub fn is_wupe16_0(&self) -> bool {
    *self == WUPE16_A::WUPE16_0
  }
  #[doc = "Checks if the value of the field is `WUPE16_1`"]
  #[inline(always)]
  pub fn is_wupe16_1(&self) -> bool {
    *self == WUPE16_A::WUPE16_1
  }
  #[doc = "Checks if the value of the field is `WUPE16_2`"]
  #[inline(always)]
  pub fn is_wupe16_2(&self) -> bool {
    *self == WUPE16_A::WUPE16_2
  }
  #[doc = "Checks if the value of the field is `WUPE16_3`"]
  #[inline(always)]
  pub fn is_wupe16_3(&self) -> bool {
    *self == WUPE16_A::WUPE16_3
  }
}
#[doc = "Write proxy for field `WUPE16`"]
pub struct WUPE16_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE16_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE16_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe16_0(self) -> &'a mut W {
    self.variant(WUPE16_A::WUPE16_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe16_1(self) -> &'a mut W {
    self.variant(WUPE16_A::WUPE16_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe16_2(self) -> &'a mut W {
    self.variant(WUPE16_A::WUPE16_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe16_3(self) -> &'a mut W {
    self.variant(WUPE16_A::WUPE16_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE17_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE17_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE17_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE17_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE17_3,
}
impl From<WUPE17_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE17_A) -> Self {
    match variant {
      WUPE17_A::WUPE17_0 => 0,
      WUPE17_A::WUPE17_1 => 1,
      WUPE17_A::WUPE17_2 => 2,
      WUPE17_A::WUPE17_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE17`"]
pub type WUPE17_R = crate::R<u8, WUPE17_A>;
impl WUPE17_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE17_A {
    match self.bits {
      0 => WUPE17_A::WUPE17_0,
      1 => WUPE17_A::WUPE17_1,
      2 => WUPE17_A::WUPE17_2,
      3 => WUPE17_A::WUPE17_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE17_0`"]
  #[inline(always)]
  pub fn is_wupe17_0(&self) -> bool {
    *self == WUPE17_A::WUPE17_0
  }
  #[doc = "Checks if the value of the field is `WUPE17_1`"]
  #[inline(always)]
  pub fn is_wupe17_1(&self) -> bool {
    *self == WUPE17_A::WUPE17_1
  }
  #[doc = "Checks if the value of the field is `WUPE17_2`"]
  #[inline(always)]
  pub fn is_wupe17_2(&self) -> bool {
    *self == WUPE17_A::WUPE17_2
  }
  #[doc = "Checks if the value of the field is `WUPE17_3`"]
  #[inline(always)]
  pub fn is_wupe17_3(&self) -> bool {
    *self == WUPE17_A::WUPE17_3
  }
}
#[doc = "Write proxy for field `WUPE17`"]
pub struct WUPE17_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE17_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE17_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe17_0(self) -> &'a mut W {
    self.variant(WUPE17_A::WUPE17_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe17_1(self) -> &'a mut W {
    self.variant(WUPE17_A::WUPE17_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe17_2(self) -> &'a mut W {
    self.variant(WUPE17_A::WUPE17_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe17_3(self) -> &'a mut W {
    self.variant(WUPE17_A::WUPE17_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE18_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE18_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE18_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE18_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE18_3,
}
impl From<WUPE18_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE18_A) -> Self {
    match variant {
      WUPE18_A::WUPE18_0 => 0,
      WUPE18_A::WUPE18_1 => 1,
      WUPE18_A::WUPE18_2 => 2,
      WUPE18_A::WUPE18_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE18`"]
pub type WUPE18_R = crate::R<u8, WUPE18_A>;
impl WUPE18_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE18_A {
    match self.bits {
      0 => WUPE18_A::WUPE18_0,
      1 => WUPE18_A::WUPE18_1,
      2 => WUPE18_A::WUPE18_2,
      3 => WUPE18_A::WUPE18_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE18_0`"]
  #[inline(always)]
  pub fn is_wupe18_0(&self) -> bool {
    *self == WUPE18_A::WUPE18_0
  }
  #[doc = "Checks if the value of the field is `WUPE18_1`"]
  #[inline(always)]
  pub fn is_wupe18_1(&self) -> bool {
    *self == WUPE18_A::WUPE18_1
  }
  #[doc = "Checks if the value of the field is `WUPE18_2`"]
  #[inline(always)]
  pub fn is_wupe18_2(&self) -> bool {
    *self == WUPE18_A::WUPE18_2
  }
  #[doc = "Checks if the value of the field is `WUPE18_3`"]
  #[inline(always)]
  pub fn is_wupe18_3(&self) -> bool {
    *self == WUPE18_A::WUPE18_3
  }
}
#[doc = "Write proxy for field `WUPE18`"]
pub struct WUPE18_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE18_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE18_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe18_0(self) -> &'a mut W {
    self.variant(WUPE18_A::WUPE18_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe18_1(self) -> &'a mut W {
    self.variant(WUPE18_A::WUPE18_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe18_2(self) -> &'a mut W {
    self.variant(WUPE18_A::WUPE18_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe18_3(self) -> &'a mut W {
    self.variant(WUPE18_A::WUPE18_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE19_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE19_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE19_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE19_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE19_3,
}
impl From<WUPE19_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE19_A) -> Self {
    match variant {
      WUPE19_A::WUPE19_0 => 0,
      WUPE19_A::WUPE19_1 => 1,
      WUPE19_A::WUPE19_2 => 2,
      WUPE19_A::WUPE19_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE19`"]
pub type WUPE19_R = crate::R<u8, WUPE19_A>;
impl WUPE19_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE19_A {
    match self.bits {
      0 => WUPE19_A::WUPE19_0,
      1 => WUPE19_A::WUPE19_1,
      2 => WUPE19_A::WUPE19_2,
      3 => WUPE19_A::WUPE19_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE19_0`"]
  #[inline(always)]
  pub fn is_wupe19_0(&self) -> bool {
    *self == WUPE19_A::WUPE19_0
  }
  #[doc = "Checks if the value of the field is `WUPE19_1`"]
  #[inline(always)]
  pub fn is_wupe19_1(&self) -> bool {
    *self == WUPE19_A::WUPE19_1
  }
  #[doc = "Checks if the value of the field is `WUPE19_2`"]
  #[inline(always)]
  pub fn is_wupe19_2(&self) -> bool {
    *self == WUPE19_A::WUPE19_2
  }
  #[doc = "Checks if the value of the field is `WUPE19_3`"]
  #[inline(always)]
  pub fn is_wupe19_3(&self) -> bool {
    *self == WUPE19_A::WUPE19_3
  }
}
#[doc = "Write proxy for field `WUPE19`"]
pub struct WUPE19_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE19_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE19_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe19_0(self) -> &'a mut W {
    self.variant(WUPE19_A::WUPE19_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe19_1(self) -> &'a mut W {
    self.variant(WUPE19_A::WUPE19_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe19_2(self) -> &'a mut W {
    self.variant(WUPE19_A::WUPE19_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe19_3(self) -> &'a mut W {
    self.variant(WUPE19_A::WUPE19_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE20_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE20_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE20_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE20_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE20_3,
}
impl From<WUPE20_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE20_A) -> Self {
    match variant {
      WUPE20_A::WUPE20_0 => 0,
      WUPE20_A::WUPE20_1 => 1,
      WUPE20_A::WUPE20_2 => 2,
      WUPE20_A::WUPE20_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE20`"]
pub type WUPE20_R = crate::R<u8, WUPE20_A>;
impl WUPE20_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE20_A {
    match self.bits {
      0 => WUPE20_A::WUPE20_0,
      1 => WUPE20_A::WUPE20_1,
      2 => WUPE20_A::WUPE20_2,
      3 => WUPE20_A::WUPE20_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE20_0`"]
  #[inline(always)]
  pub fn is_wupe20_0(&self) -> bool {
    *self == WUPE20_A::WUPE20_0
  }
  #[doc = "Checks if the value of the field is `WUPE20_1`"]
  #[inline(always)]
  pub fn is_wupe20_1(&self) -> bool {
    *self == WUPE20_A::WUPE20_1
  }
  #[doc = "Checks if the value of the field is `WUPE20_2`"]
  #[inline(always)]
  pub fn is_wupe20_2(&self) -> bool {
    *self == WUPE20_A::WUPE20_2
  }
  #[doc = "Checks if the value of the field is `WUPE20_3`"]
  #[inline(always)]
  pub fn is_wupe20_3(&self) -> bool {
    *self == WUPE20_A::WUPE20_3
  }
}
#[doc = "Write proxy for field `WUPE20`"]
pub struct WUPE20_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE20_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE20_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe20_0(self) -> &'a mut W {
    self.variant(WUPE20_A::WUPE20_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe20_1(self) -> &'a mut W {
    self.variant(WUPE20_A::WUPE20_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe20_2(self) -> &'a mut W {
    self.variant(WUPE20_A::WUPE20_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe20_3(self) -> &'a mut W {
    self.variant(WUPE20_A::WUPE20_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE21_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE21_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE21_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE21_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE21_3,
}
impl From<WUPE21_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE21_A) -> Self {
    match variant {
      WUPE21_A::WUPE21_0 => 0,
      WUPE21_A::WUPE21_1 => 1,
      WUPE21_A::WUPE21_2 => 2,
      WUPE21_A::WUPE21_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE21`"]
pub type WUPE21_R = crate::R<u8, WUPE21_A>;
impl WUPE21_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE21_A {
    match self.bits {
      0 => WUPE21_A::WUPE21_0,
      1 => WUPE21_A::WUPE21_1,
      2 => WUPE21_A::WUPE21_2,
      3 => WUPE21_A::WUPE21_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE21_0`"]
  #[inline(always)]
  pub fn is_wupe21_0(&self) -> bool {
    *self == WUPE21_A::WUPE21_0
  }
  #[doc = "Checks if the value of the field is `WUPE21_1`"]
  #[inline(always)]
  pub fn is_wupe21_1(&self) -> bool {
    *self == WUPE21_A::WUPE21_1
  }
  #[doc = "Checks if the value of the field is `WUPE21_2`"]
  #[inline(always)]
  pub fn is_wupe21_2(&self) -> bool {
    *self == WUPE21_A::WUPE21_2
  }
  #[doc = "Checks if the value of the field is `WUPE21_3`"]
  #[inline(always)]
  pub fn is_wupe21_3(&self) -> bool {
    *self == WUPE21_A::WUPE21_3
  }
}
#[doc = "Write proxy for field `WUPE21`"]
pub struct WUPE21_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE21_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE21_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe21_0(self) -> &'a mut W {
    self.variant(WUPE21_A::WUPE21_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe21_1(self) -> &'a mut W {
    self.variant(WUPE21_A::WUPE21_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe21_2(self) -> &'a mut W {
    self.variant(WUPE21_A::WUPE21_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe21_3(self) -> &'a mut W {
    self.variant(WUPE21_A::WUPE21_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE22_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE22_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE22_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE22_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE22_3,
}
impl From<WUPE22_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE22_A) -> Self {
    match variant {
      WUPE22_A::WUPE22_0 => 0,
      WUPE22_A::WUPE22_1 => 1,
      WUPE22_A::WUPE22_2 => 2,
      WUPE22_A::WUPE22_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE22`"]
pub type WUPE22_R = crate::R<u8, WUPE22_A>;
impl WUPE22_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE22_A {
    match self.bits {
      0 => WUPE22_A::WUPE22_0,
      1 => WUPE22_A::WUPE22_1,
      2 => WUPE22_A::WUPE22_2,
      3 => WUPE22_A::WUPE22_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE22_0`"]
  #[inline(always)]
  pub fn is_wupe22_0(&self) -> bool {
    *self == WUPE22_A::WUPE22_0
  }
  #[doc = "Checks if the value of the field is `WUPE22_1`"]
  #[inline(always)]
  pub fn is_wupe22_1(&self) -> bool {
    *self == WUPE22_A::WUPE22_1
  }
  #[doc = "Checks if the value of the field is `WUPE22_2`"]
  #[inline(always)]
  pub fn is_wupe22_2(&self) -> bool {
    *self == WUPE22_A::WUPE22_2
  }
  #[doc = "Checks if the value of the field is `WUPE22_3`"]
  #[inline(always)]
  pub fn is_wupe22_3(&self) -> bool {
    *self == WUPE22_A::WUPE22_3
  }
}
#[doc = "Write proxy for field `WUPE22`"]
pub struct WUPE22_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE22_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE22_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe22_0(self) -> &'a mut W {
    self.variant(WUPE22_A::WUPE22_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe22_1(self) -> &'a mut W {
    self.variant(WUPE22_A::WUPE22_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe22_2(self) -> &'a mut W {
    self.variant(WUPE22_A::WUPE22_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe22_3(self) -> &'a mut W {
    self.variant(WUPE22_A::WUPE22_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE23_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE23_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE23_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE23_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE23_3,
}
impl From<WUPE23_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE23_A) -> Self {
    match variant {
      WUPE23_A::WUPE23_0 => 0,
      WUPE23_A::WUPE23_1 => 1,
      WUPE23_A::WUPE23_2 => 2,
      WUPE23_A::WUPE23_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE23`"]
pub type WUPE23_R = crate::R<u8, WUPE23_A>;
impl WUPE23_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE23_A {
    match self.bits {
      0 => WUPE23_A::WUPE23_0,
      1 => WUPE23_A::WUPE23_1,
      2 => WUPE23_A::WUPE23_2,
      3 => WUPE23_A::WUPE23_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE23_0`"]
  #[inline(always)]
  pub fn is_wupe23_0(&self) -> bool {
    *self == WUPE23_A::WUPE23_0
  }
  #[doc = "Checks if the value of the field is `WUPE23_1`"]
  #[inline(always)]
  pub fn is_wupe23_1(&self) -> bool {
    *self == WUPE23_A::WUPE23_1
  }
  #[doc = "Checks if the value of the field is `WUPE23_2`"]
  #[inline(always)]
  pub fn is_wupe23_2(&self) -> bool {
    *self == WUPE23_A::WUPE23_2
  }
  #[doc = "Checks if the value of the field is `WUPE23_3`"]
  #[inline(always)]
  pub fn is_wupe23_3(&self) -> bool {
    *self == WUPE23_A::WUPE23_3
  }
}
#[doc = "Write proxy for field `WUPE23`"]
pub struct WUPE23_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE23_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE23_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe23_0(self) -> &'a mut W {
    self.variant(WUPE23_A::WUPE23_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe23_1(self) -> &'a mut W {
    self.variant(WUPE23_A::WUPE23_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe23_2(self) -> &'a mut W {
    self.variant(WUPE23_A::WUPE23_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe23_3(self) -> &'a mut W {
    self.variant(WUPE23_A::WUPE23_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE24_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE24_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE24_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE24_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE24_3,
}
impl From<WUPE24_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE24_A) -> Self {
    match variant {
      WUPE24_A::WUPE24_0 => 0,
      WUPE24_A::WUPE24_1 => 1,
      WUPE24_A::WUPE24_2 => 2,
      WUPE24_A::WUPE24_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE24`"]
pub type WUPE24_R = crate::R<u8, WUPE24_A>;
impl WUPE24_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE24_A {
    match self.bits {
      0 => WUPE24_A::WUPE24_0,
      1 => WUPE24_A::WUPE24_1,
      2 => WUPE24_A::WUPE24_2,
      3 => WUPE24_A::WUPE24_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE24_0`"]
  #[inline(always)]
  pub fn is_wupe24_0(&self) -> bool {
    *self == WUPE24_A::WUPE24_0
  }
  #[doc = "Checks if the value of the field is `WUPE24_1`"]
  #[inline(always)]
  pub fn is_wupe24_1(&self) -> bool {
    *self == WUPE24_A::WUPE24_1
  }
  #[doc = "Checks if the value of the field is `WUPE24_2`"]
  #[inline(always)]
  pub fn is_wupe24_2(&self) -> bool {
    *self == WUPE24_A::WUPE24_2
  }
  #[doc = "Checks if the value of the field is `WUPE24_3`"]
  #[inline(always)]
  pub fn is_wupe24_3(&self) -> bool {
    *self == WUPE24_A::WUPE24_3
  }
}
#[doc = "Write proxy for field `WUPE24`"]
pub struct WUPE24_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE24_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE24_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe24_0(self) -> &'a mut W {
    self.variant(WUPE24_A::WUPE24_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe24_1(self) -> &'a mut W {
    self.variant(WUPE24_A::WUPE24_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe24_2(self) -> &'a mut W {
    self.variant(WUPE24_A::WUPE24_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe24_3(self) -> &'a mut W {
    self.variant(WUPE24_A::WUPE24_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE25_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE25_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE25_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE25_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE25_3,
}
impl From<WUPE25_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE25_A) -> Self {
    match variant {
      WUPE25_A::WUPE25_0 => 0,
      WUPE25_A::WUPE25_1 => 1,
      WUPE25_A::WUPE25_2 => 2,
      WUPE25_A::WUPE25_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE25`"]
pub type WUPE25_R = crate::R<u8, WUPE25_A>;
impl WUPE25_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE25_A {
    match self.bits {
      0 => WUPE25_A::WUPE25_0,
      1 => WUPE25_A::WUPE25_1,
      2 => WUPE25_A::WUPE25_2,
      3 => WUPE25_A::WUPE25_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE25_0`"]
  #[inline(always)]
  pub fn is_wupe25_0(&self) -> bool {
    *self == WUPE25_A::WUPE25_0
  }
  #[doc = "Checks if the value of the field is `WUPE25_1`"]
  #[inline(always)]
  pub fn is_wupe25_1(&self) -> bool {
    *self == WUPE25_A::WUPE25_1
  }
  #[doc = "Checks if the value of the field is `WUPE25_2`"]
  #[inline(always)]
  pub fn is_wupe25_2(&self) -> bool {
    *self == WUPE25_A::WUPE25_2
  }
  #[doc = "Checks if the value of the field is `WUPE25_3`"]
  #[inline(always)]
  pub fn is_wupe25_3(&self) -> bool {
    *self == WUPE25_A::WUPE25_3
  }
}
#[doc = "Write proxy for field `WUPE25`"]
pub struct WUPE25_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE25_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE25_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe25_0(self) -> &'a mut W {
    self.variant(WUPE25_A::WUPE25_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe25_1(self) -> &'a mut W {
    self.variant(WUPE25_A::WUPE25_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe25_2(self) -> &'a mut W {
    self.variant(WUPE25_A::WUPE25_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe25_3(self) -> &'a mut W {
    self.variant(WUPE25_A::WUPE25_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE26_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE26_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE26_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE26_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE26_3,
}
impl From<WUPE26_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE26_A) -> Self {
    match variant {
      WUPE26_A::WUPE26_0 => 0,
      WUPE26_A::WUPE26_1 => 1,
      WUPE26_A::WUPE26_2 => 2,
      WUPE26_A::WUPE26_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE26`"]
pub type WUPE26_R = crate::R<u8, WUPE26_A>;
impl WUPE26_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE26_A {
    match self.bits {
      0 => WUPE26_A::WUPE26_0,
      1 => WUPE26_A::WUPE26_1,
      2 => WUPE26_A::WUPE26_2,
      3 => WUPE26_A::WUPE26_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE26_0`"]
  #[inline(always)]
  pub fn is_wupe26_0(&self) -> bool {
    *self == WUPE26_A::WUPE26_0
  }
  #[doc = "Checks if the value of the field is `WUPE26_1`"]
  #[inline(always)]
  pub fn is_wupe26_1(&self) -> bool {
    *self == WUPE26_A::WUPE26_1
  }
  #[doc = "Checks if the value of the field is `WUPE26_2`"]
  #[inline(always)]
  pub fn is_wupe26_2(&self) -> bool {
    *self == WUPE26_A::WUPE26_2
  }
  #[doc = "Checks if the value of the field is `WUPE26_3`"]
  #[inline(always)]
  pub fn is_wupe26_3(&self) -> bool {
    *self == WUPE26_A::WUPE26_3
  }
}
#[doc = "Write proxy for field `WUPE26`"]
pub struct WUPE26_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE26_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE26_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe26_0(self) -> &'a mut W {
    self.variant(WUPE26_A::WUPE26_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe26_1(self) -> &'a mut W {
    self.variant(WUPE26_A::WUPE26_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe26_2(self) -> &'a mut W {
    self.variant(WUPE26_A::WUPE26_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe26_3(self) -> &'a mut W {
    self.variant(WUPE26_A::WUPE26_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED27_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  RESERVED27_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  RESERVED27_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  RESERVED27_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  RESERVED27_3,
}
impl From<RESERVED27_A> for u8 {
  #[inline(always)]
  fn from(variant: RESERVED27_A) -> Self {
    match variant {
      RESERVED27_A::RESERVED27_0 => 0,
      RESERVED27_A::RESERVED27_1 => 1,
      RESERVED27_A::RESERVED27_2 => 2,
      RESERVED27_A::RESERVED27_3 => 3,
    }
  }
}
#[doc = "Reader of field `Reserved27`"]
pub type RESERVED27_R = crate::R<u8, RESERVED27_A>;
impl RESERVED27_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED27_A {
    match self.bits {
      0 => RESERVED27_A::RESERVED27_0,
      1 => RESERVED27_A::RESERVED27_1,
      2 => RESERVED27_A::RESERVED27_2,
      3 => RESERVED27_A::RESERVED27_3,
      _ => unreachable!(),
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
  #[doc = "Checks if the value of the field is `RESERVED27_3`"]
  #[inline(always)]
  pub fn is_reserved27_3(&self) -> bool {
    *self == RESERVED27_A::RESERVED27_3
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESERVED28_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  RESERVED28_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  RESERVED28_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  RESERVED28_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  RESERVED28_3,
}
impl From<RESERVED28_A> for u8 {
  #[inline(always)]
  fn from(variant: RESERVED28_A) -> Self {
    match variant {
      RESERVED28_A::RESERVED28_0 => 0,
      RESERVED28_A::RESERVED28_1 => 1,
      RESERVED28_A::RESERVED28_2 => 2,
      RESERVED28_A::RESERVED28_3 => 3,
    }
  }
}
#[doc = "Reader of field `Reserved28`"]
pub type RESERVED28_R = crate::R<u8, RESERVED28_A>;
impl RESERVED28_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> RESERVED28_A {
    match self.bits {
      0 => RESERVED28_A::RESERVED28_0,
      1 => RESERVED28_A::RESERVED28_1,
      2 => RESERVED28_A::RESERVED28_2,
      3 => RESERVED28_A::RESERVED28_3,
      _ => unreachable!(),
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
  #[doc = "Checks if the value of the field is `RESERVED28_3`"]
  #[inline(always)]
  pub fn is_reserved28_3(&self) -> bool {
    *self == RESERVED28_A::RESERVED28_3
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE29_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE29_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE29_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE29_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE29_3,
}
impl From<WUPE29_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE29_A) -> Self {
    match variant {
      WUPE29_A::WUPE29_0 => 0,
      WUPE29_A::WUPE29_1 => 1,
      WUPE29_A::WUPE29_2 => 2,
      WUPE29_A::WUPE29_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE29`"]
pub type WUPE29_R = crate::R<u8, WUPE29_A>;
impl WUPE29_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE29_A {
    match self.bits {
      0 => WUPE29_A::WUPE29_0,
      1 => WUPE29_A::WUPE29_1,
      2 => WUPE29_A::WUPE29_2,
      3 => WUPE29_A::WUPE29_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE29_0`"]
  #[inline(always)]
  pub fn is_wupe29_0(&self) -> bool {
    *self == WUPE29_A::WUPE29_0
  }
  #[doc = "Checks if the value of the field is `WUPE29_1`"]
  #[inline(always)]
  pub fn is_wupe29_1(&self) -> bool {
    *self == WUPE29_A::WUPE29_1
  }
  #[doc = "Checks if the value of the field is `WUPE29_2`"]
  #[inline(always)]
  pub fn is_wupe29_2(&self) -> bool {
    *self == WUPE29_A::WUPE29_2
  }
  #[doc = "Checks if the value of the field is `WUPE29_3`"]
  #[inline(always)]
  pub fn is_wupe29_3(&self) -> bool {
    *self == WUPE29_A::WUPE29_3
  }
}
#[doc = "Write proxy for field `WUPE29`"]
pub struct WUPE29_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE29_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE29_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe29_0(self) -> &'a mut W {
    self.variant(WUPE29_A::WUPE29_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe29_1(self) -> &'a mut W {
    self.variant(WUPE29_A::WUPE29_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe29_2(self) -> &'a mut W {
    self.variant(WUPE29_A::WUPE29_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe29_3(self) -> &'a mut W {
    self.variant(WUPE29_A::WUPE29_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE30_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE30_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE30_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE30_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE30_3,
}
impl From<WUPE30_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE30_A) -> Self {
    match variant {
      WUPE30_A::WUPE30_0 => 0,
      WUPE30_A::WUPE30_1 => 1,
      WUPE30_A::WUPE30_2 => 2,
      WUPE30_A::WUPE30_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE30`"]
pub type WUPE30_R = crate::R<u8, WUPE30_A>;
impl WUPE30_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE30_A {
    match self.bits {
      0 => WUPE30_A::WUPE30_0,
      1 => WUPE30_A::WUPE30_1,
      2 => WUPE30_A::WUPE30_2,
      3 => WUPE30_A::WUPE30_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE30_0`"]
  #[inline(always)]
  pub fn is_wupe30_0(&self) -> bool {
    *self == WUPE30_A::WUPE30_0
  }
  #[doc = "Checks if the value of the field is `WUPE30_1`"]
  #[inline(always)]
  pub fn is_wupe30_1(&self) -> bool {
    *self == WUPE30_A::WUPE30_1
  }
  #[doc = "Checks if the value of the field is `WUPE30_2`"]
  #[inline(always)]
  pub fn is_wupe30_2(&self) -> bool {
    *self == WUPE30_A::WUPE30_2
  }
  #[doc = "Checks if the value of the field is `WUPE30_3`"]
  #[inline(always)]
  pub fn is_wupe30_3(&self) -> bool {
    *self == WUPE30_A::WUPE30_3
  }
}
#[doc = "Write proxy for field `WUPE30`"]
pub struct WUPE30_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE30_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE30_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe30_0(self) -> &'a mut W {
    self.variant(WUPE30_A::WUPE30_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe30_1(self) -> &'a mut W {
    self.variant(WUPE30_A::WUPE30_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe30_2(self) -> &'a mut W {
    self.variant(WUPE30_A::WUPE30_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe30_3(self) -> &'a mut W {
    self.variant(WUPE30_A::WUPE30_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
    self.w
  }
}
#[doc = "Wakeup pin enable for LLWU_Pn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPE31_A {
  #[doc = "0: External input pin disabled as wakeup input"]
  WUPE31_0,
  #[doc = "1: External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  WUPE31_1,
  #[doc = "2: External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  WUPE31_2,
  #[doc = "3: External input pin enabled with any change detection when configured as interrupt/DMA request"]
  WUPE31_3,
}
impl From<WUPE31_A> for u8 {
  #[inline(always)]
  fn from(variant: WUPE31_A) -> Self {
    match variant {
      WUPE31_A::WUPE31_0 => 0,
      WUPE31_A::WUPE31_1 => 1,
      WUPE31_A::WUPE31_2 => 2,
      WUPE31_A::WUPE31_3 => 3,
    }
  }
}
#[doc = "Reader of field `WUPE31`"]
pub type WUPE31_R = crate::R<u8, WUPE31_A>;
impl WUPE31_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> WUPE31_A {
    match self.bits {
      0 => WUPE31_A::WUPE31_0,
      1 => WUPE31_A::WUPE31_1,
      2 => WUPE31_A::WUPE31_2,
      3 => WUPE31_A::WUPE31_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `WUPE31_0`"]
  #[inline(always)]
  pub fn is_wupe31_0(&self) -> bool {
    *self == WUPE31_A::WUPE31_0
  }
  #[doc = "Checks if the value of the field is `WUPE31_1`"]
  #[inline(always)]
  pub fn is_wupe31_1(&self) -> bool {
    *self == WUPE31_A::WUPE31_1
  }
  #[doc = "Checks if the value of the field is `WUPE31_2`"]
  #[inline(always)]
  pub fn is_wupe31_2(&self) -> bool {
    *self == WUPE31_A::WUPE31_2
  }
  #[doc = "Checks if the value of the field is `WUPE31_3`"]
  #[inline(always)]
  pub fn is_wupe31_3(&self) -> bool {
    *self == WUPE31_A::WUPE31_3
  }
}
#[doc = "Write proxy for field `WUPE31`"]
pub struct WUPE31_W<'a> {
  w: &'a mut W,
}
impl<'a> WUPE31_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: WUPE31_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "External input pin disabled as wakeup input"]
  #[inline(always)]
  pub fn wupe31_0(self) -> &'a mut W {
    self.variant(WUPE31_A::WUPE31_0)
  }
  #[doc = "External input pin enabled with rising edge detection when configured as interrupt/DMA request or high level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe31_1(self) -> &'a mut W {
    self.variant(WUPE31_A::WUPE31_1)
  }
  #[doc = "External input pin enabled with falling edge detection when configured as interrupt/DMA request or low level detection when configured as trigger request"]
  #[inline(always)]
  pub fn wupe31_2(self) -> &'a mut W {
    self.variant(WUPE31_A::WUPE31_2)
  }
  #[doc = "External input pin enabled with any change detection when configured as interrupt/DMA request"]
  #[inline(always)]
  pub fn wupe31_3(self) -> &'a mut W {
    self.variant(WUPE31_A::WUPE31_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe16(&self) -> WUPE16_R {
    WUPE16_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bits 2:3 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe17(&self) -> WUPE17_R {
    WUPE17_R::new(((self.bits >> 2) & 0x03) as u8)
  }
  #[doc = "Bits 4:5 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe18(&self) -> WUPE18_R {
    WUPE18_R::new(((self.bits >> 4) & 0x03) as u8)
  }
  #[doc = "Bits 6:7 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe19(&self) -> WUPE19_R {
    WUPE19_R::new(((self.bits >> 6) & 0x03) as u8)
  }
  #[doc = "Bits 8:9 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe20(&self) -> WUPE20_R {
    WUPE20_R::new(((self.bits >> 8) & 0x03) as u8)
  }
  #[doc = "Bits 10:11 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe21(&self) -> WUPE21_R {
    WUPE21_R::new(((self.bits >> 10) & 0x03) as u8)
  }
  #[doc = "Bits 12:13 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe22(&self) -> WUPE22_R {
    WUPE22_R::new(((self.bits >> 12) & 0x03) as u8)
  }
  #[doc = "Bits 14:15 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe23(&self) -> WUPE23_R {
    WUPE23_R::new(((self.bits >> 14) & 0x03) as u8)
  }
  #[doc = "Bits 16:17 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe24(&self) -> WUPE24_R {
    WUPE24_R::new(((self.bits >> 16) & 0x03) as u8)
  }
  #[doc = "Bits 18:19 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe25(&self) -> WUPE25_R {
    WUPE25_R::new(((self.bits >> 18) & 0x03) as u8)
  }
  #[doc = "Bits 20:21 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe26(&self) -> WUPE26_R {
    WUPE26_R::new(((self.bits >> 20) & 0x03) as u8)
  }
  #[doc = "Bits 22:23 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn reserved27(&self) -> RESERVED27_R {
    RESERVED27_R::new(((self.bits >> 22) & 0x03) as u8)
  }
  #[doc = "Bits 24:25 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn reserved28(&self) -> RESERVED28_R {
    RESERVED28_R::new(((self.bits >> 24) & 0x03) as u8)
  }
  #[doc = "Bits 26:27 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe29(&self) -> WUPE29_R {
    WUPE29_R::new(((self.bits >> 26) & 0x03) as u8)
  }
  #[doc = "Bits 28:29 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe30(&self) -> WUPE30_R {
    WUPE30_R::new(((self.bits >> 28) & 0x03) as u8)
  }
  #[doc = "Bits 30:31 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe31(&self) -> WUPE31_R {
    WUPE31_R::new(((self.bits >> 30) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe16(&mut self) -> WUPE16_W {
    WUPE16_W { w: self }
  }
  #[doc = "Bits 2:3 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe17(&mut self) -> WUPE17_W {
    WUPE17_W { w: self }
  }
  #[doc = "Bits 4:5 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe18(&mut self) -> WUPE18_W {
    WUPE18_W { w: self }
  }
  #[doc = "Bits 6:7 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe19(&mut self) -> WUPE19_W {
    WUPE19_W { w: self }
  }
  #[doc = "Bits 8:9 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe20(&mut self) -> WUPE20_W {
    WUPE20_W { w: self }
  }
  #[doc = "Bits 10:11 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe21(&mut self) -> WUPE21_W {
    WUPE21_W { w: self }
  }
  #[doc = "Bits 12:13 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe22(&mut self) -> WUPE22_W {
    WUPE22_W { w: self }
  }
  #[doc = "Bits 14:15 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe23(&mut self) -> WUPE23_W {
    WUPE23_W { w: self }
  }
  #[doc = "Bits 16:17 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe24(&mut self) -> WUPE24_W {
    WUPE24_W { w: self }
  }
  #[doc = "Bits 18:19 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe25(&mut self) -> WUPE25_W {
    WUPE25_W { w: self }
  }
  #[doc = "Bits 20:21 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe26(&mut self) -> WUPE26_W {
    WUPE26_W { w: self }
  }
  #[doc = "Bits 26:27 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe29(&mut self) -> WUPE29_W {
    WUPE29_W { w: self }
  }
  #[doc = "Bits 28:29 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe30(&mut self) -> WUPE30_W {
    WUPE30_W { w: self }
  }
  #[doc = "Bits 30:31 - Wakeup pin enable for LLWU_Pn"]
  #[inline(always)]
  pub fn wupe31(&mut self) -> WUPE31_W {
    WUPE31_W { w: self }
  }
}
