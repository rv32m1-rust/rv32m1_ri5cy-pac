#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Time Compensation Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR_A {
  #[doc = "0: Time Prescaler Register overflows every 32768 clock cycles."]
  TCR_0,
  #[doc = "1: Time Prescaler Register overflows every 32767 clock cycles."]
  TCR_1,
  #[doc = "126: Time Prescaler Register overflows every 32642 clock cycles."]
  TCR_126,
  #[doc = "127: Time Prescaler Register overflows every 32641 clock cycles."]
  TCR_127,
  #[doc = "128: Time Prescaler Register overflows every 32896 clock cycles."]
  TCR_128,
  #[doc = "129: Time Prescaler Register overflows every 32895 clock cycles."]
  TCR_129,
  #[doc = "255: Time Prescaler Register overflows every 32769 clock cycles."]
  TCR_255,
}
impl From<TCR_A> for u8 {
  #[inline(always)]
  fn from(variant: TCR_A) -> Self {
    match variant {
      TCR_A::TCR_0 => 0,
      TCR_A::TCR_1 => 1,
      TCR_A::TCR_126 => 126,
      TCR_A::TCR_127 => 127,
      TCR_A::TCR_128 => 128,
      TCR_A::TCR_129 => 129,
      TCR_A::TCR_255 => 255,
    }
  }
}
#[doc = "Reader of field `TCR`"]
pub type TCR_R = crate::R<u8, TCR_A>;
impl TCR_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TCR_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TCR_A::TCR_0),
      1 => Val(TCR_A::TCR_1),
      126 => Val(TCR_A::TCR_126),
      127 => Val(TCR_A::TCR_127),
      128 => Val(TCR_A::TCR_128),
      129 => Val(TCR_A::TCR_129),
      255 => Val(TCR_A::TCR_255),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TCR_0`"]
  #[inline(always)]
  pub fn is_tcr_0(&self) -> bool {
    *self == TCR_A::TCR_0
  }
  #[doc = "Checks if the value of the field is `TCR_1`"]
  #[inline(always)]
  pub fn is_tcr_1(&self) -> bool {
    *self == TCR_A::TCR_1
  }
  #[doc = "Checks if the value of the field is `TCR_126`"]
  #[inline(always)]
  pub fn is_tcr_126(&self) -> bool {
    *self == TCR_A::TCR_126
  }
  #[doc = "Checks if the value of the field is `TCR_127`"]
  #[inline(always)]
  pub fn is_tcr_127(&self) -> bool {
    *self == TCR_A::TCR_127
  }
  #[doc = "Checks if the value of the field is `TCR_128`"]
  #[inline(always)]
  pub fn is_tcr_128(&self) -> bool {
    *self == TCR_A::TCR_128
  }
  #[doc = "Checks if the value of the field is `TCR_129`"]
  #[inline(always)]
  pub fn is_tcr_129(&self) -> bool {
    *self == TCR_A::TCR_129
  }
  #[doc = "Checks if the value of the field is `TCR_255`"]
  #[inline(always)]
  pub fn is_tcr_255(&self) -> bool {
    *self == TCR_A::TCR_255
  }
}
#[doc = "Write proxy for field `TCR`"]
pub struct TCR_W<'a> {
  w: &'a mut W,
}
impl<'a> TCR_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TCR_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
  #[inline(always)]
  pub fn tcr_0(self) -> &'a mut W {
    self.variant(TCR_A::TCR_0)
  }
  #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
  #[inline(always)]
  pub fn tcr_1(self) -> &'a mut W {
    self.variant(TCR_A::TCR_1)
  }
  #[doc = "Time Prescaler Register overflows every 32642 clock cycles."]
  #[inline(always)]
  pub fn tcr_126(self) -> &'a mut W {
    self.variant(TCR_A::TCR_126)
  }
  #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
  #[inline(always)]
  pub fn tcr_127(self) -> &'a mut W {
    self.variant(TCR_A::TCR_127)
  }
  #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
  #[inline(always)]
  pub fn tcr_128(self) -> &'a mut W {
    self.variant(TCR_A::TCR_128)
  }
  #[doc = "Time Prescaler Register overflows every 32895 clock cycles."]
  #[inline(always)]
  pub fn tcr_129(self) -> &'a mut W {
    self.variant(TCR_A::TCR_129)
  }
  #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
  #[inline(always)]
  pub fn tcr_255(self) -> &'a mut W {
    self.variant(TCR_A::TCR_255)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
    self.w
  }
}
#[doc = "Reader of field `CIR`"]
pub type CIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CIR`"]
pub struct CIR_W<'a> {
  w: &'a mut W,
}
impl<'a> CIR_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
    self.w
  }
}
#[doc = "Reader of field `TCV`"]
pub type TCV_R = crate::R<u8, u8>;
#[doc = "Reader of field `CIC`"]
pub type CIC_R = crate::R<u8, u8>;
impl R {
  #[doc = "Bits 0:7 - Time Compensation Register"]
  #[inline(always)]
  pub fn tcr(&self) -> TCR_R {
    TCR_R::new((self.bits & 0xff) as u8)
  }
  #[doc = "Bits 8:15 - Compensation Interval Register"]
  #[inline(always)]
  pub fn cir(&self) -> CIR_R {
    CIR_R::new(((self.bits >> 8) & 0xff) as u8)
  }
  #[doc = "Bits 16:23 - Time Compensation Value"]
  #[inline(always)]
  pub fn tcv(&self) -> TCV_R {
    TCV_R::new(((self.bits >> 16) & 0xff) as u8)
  }
  #[doc = "Bits 24:31 - Compensation Interval Counter"]
  #[inline(always)]
  pub fn cic(&self) -> CIC_R {
    CIC_R::new(((self.bits >> 24) & 0xff) as u8)
  }
}
impl W {
  #[doc = "Bits 0:7 - Time Compensation Register"]
  #[inline(always)]
  pub fn tcr(&mut self) -> TCR_W {
    TCR_W { w: self }
  }
  #[doc = "Bits 8:15 - Compensation Interval Register"]
  #[inline(always)]
  pub fn cir(&mut self) -> CIR_W {
    CIR_W { w: self }
  }
}
