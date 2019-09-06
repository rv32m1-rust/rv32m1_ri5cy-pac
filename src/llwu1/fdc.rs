#[doc = "Reader of register FDC"]
pub type R = crate::R<u32, super::FDC>;
#[doc = "Writer for register FDC"]
pub type W = crate::W<u32, super::FDC>;
#[doc = "Register FDC `reset()`'s with value 0"]
impl crate::ResetValue for super::FDC {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Filter configuration for FILT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTC1_A {
  #[doc = "0: Filter output configured as interrupt"]
  FILTC1_0,
  #[doc = "1: Filter output configured as DMA request"]
  FILTC1_1,
  #[doc = "2: Filter output configured as trigger event"]
  FILTC1_2,
}
impl From<FILTC1_A> for u8 {
  #[inline(always)]
  fn from(variant: FILTC1_A) -> Self {
    match variant {
      FILTC1_A::FILTC1_0 => 0,
      FILTC1_A::FILTC1_1 => 1,
      FILTC1_A::FILTC1_2 => 2,
    }
  }
}
#[doc = "Reader of field `FILTC1`"]
pub type FILTC1_R = crate::R<u8, FILTC1_A>;
impl FILTC1_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FILTC1_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FILTC1_A::FILTC1_0),
      1 => Val(FILTC1_A::FILTC1_1),
      2 => Val(FILTC1_A::FILTC1_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FILTC1_0`"]
  #[inline(always)]
  pub fn is_filtc1_0(&self) -> bool {
    *self == FILTC1_A::FILTC1_0
  }
  #[doc = "Checks if the value of the field is `FILTC1_1`"]
  #[inline(always)]
  pub fn is_filtc1_1(&self) -> bool {
    *self == FILTC1_A::FILTC1_1
  }
  #[doc = "Checks if the value of the field is `FILTC1_2`"]
  #[inline(always)]
  pub fn is_filtc1_2(&self) -> bool {
    *self == FILTC1_A::FILTC1_2
  }
}
#[doc = "Write proxy for field `FILTC1`"]
pub struct FILTC1_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTC1_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTC1_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Filter output configured as interrupt"]
  #[inline(always)]
  pub fn filtc1_0(self) -> &'a mut W {
    self.variant(FILTC1_A::FILTC1_0)
  }
  #[doc = "Filter output configured as DMA request"]
  #[inline(always)]
  pub fn filtc1_1(self) -> &'a mut W {
    self.variant(FILTC1_A::FILTC1_1)
  }
  #[doc = "Filter output configured as trigger event"]
  #[inline(always)]
  pub fn filtc1_2(self) -> &'a mut W {
    self.variant(FILTC1_A::FILTC1_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "Filter configuration for FILT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTC2_A {
  #[doc = "0: Filter output configured as interrupt"]
  FILTC2_0,
  #[doc = "1: Filter output configured as DMA request"]
  FILTC2_1,
  #[doc = "2: Filter output configured as trigger event"]
  FILTC2_2,
}
impl From<FILTC2_A> for u8 {
  #[inline(always)]
  fn from(variant: FILTC2_A) -> Self {
    match variant {
      FILTC2_A::FILTC2_0 => 0,
      FILTC2_A::FILTC2_1 => 1,
      FILTC2_A::FILTC2_2 => 2,
    }
  }
}
#[doc = "Reader of field `FILTC2`"]
pub type FILTC2_R = crate::R<u8, FILTC2_A>;
impl FILTC2_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, FILTC2_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(FILTC2_A::FILTC2_0),
      1 => Val(FILTC2_A::FILTC2_1),
      2 => Val(FILTC2_A::FILTC2_2),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `FILTC2_0`"]
  #[inline(always)]
  pub fn is_filtc2_0(&self) -> bool {
    *self == FILTC2_A::FILTC2_0
  }
  #[doc = "Checks if the value of the field is `FILTC2_1`"]
  #[inline(always)]
  pub fn is_filtc2_1(&self) -> bool {
    *self == FILTC2_A::FILTC2_1
  }
  #[doc = "Checks if the value of the field is `FILTC2_2`"]
  #[inline(always)]
  pub fn is_filtc2_2(&self) -> bool {
    *self == FILTC2_A::FILTC2_2
  }
}
#[doc = "Write proxy for field `FILTC2`"]
pub struct FILTC2_W<'a> {
  w: &'a mut W,
}
impl<'a> FILTC2_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FILTC2_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Filter output configured as interrupt"]
  #[inline(always)]
  pub fn filtc2_0(self) -> &'a mut W {
    self.variant(FILTC2_A::FILTC2_0)
  }
  #[doc = "Filter output configured as DMA request"]
  #[inline(always)]
  pub fn filtc2_1(self) -> &'a mut W {
    self.variant(FILTC2_A::FILTC2_1)
  }
  #[doc = "Filter output configured as trigger event"]
  #[inline(always)]
  pub fn filtc2_2(self) -> &'a mut W {
    self.variant(FILTC2_A::FILTC2_2)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Filter configuration for FILT1"]
  #[inline(always)]
  pub fn filtc1(&self) -> FILTC1_R {
    FILTC1_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bits 2:3 - Filter configuration for FILT2"]
  #[inline(always)]
  pub fn filtc2(&self) -> FILTC2_R {
    FILTC2_R::new(((self.bits >> 2) & 0x03) as u8)
  }
}
impl W {
  #[doc = "Bits 0:1 - Filter configuration for FILT1"]
  #[inline(always)]
  pub fn filtc1(&mut self) -> FILTC1_W {
    FILTC1_W { w: self }
  }
  #[doc = "Bits 2:3 - Filter configuration for FILT2"]
  #[inline(always)]
  pub fn filtc2(&mut self) -> FILTC2_W {
    FILTC2_W { w: self }
  }
}
