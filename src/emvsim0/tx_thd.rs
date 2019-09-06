#[doc = "Reader of register TX_THD"]
pub type R = crate::R<u32, super::TX_THD>;
#[doc = "Writer for register TX_THD"]
pub type W = crate::W<u32, super::TX_THD>;
#[doc = "Register TX_THD `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::TX_THD {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x0f
  }
}
#[doc = "Reader of field `TDT`"]
pub type TDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDT`"]
pub struct TDT_W<'a> {
  w: &'a mut W,
}
impl<'a> TDT_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
    self.w
  }
}
#[doc = "Transmitter NACK Threshold Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TNCK_THD_A {
  #[doc = "0: TNTE will never be set; retransmission after NACK reception is disabled."]
  TNCK_THD_0,
  #[doc = "1: TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
  TNCK_THD_1,
  #[doc = "2: TNTE will be set after 2 nacks are received; at most 1 retransmission occurs."]
  TNCK_THD_2,
  #[doc = "3: TNTE will be set after 3 nacks are received; at most 2 retransmissions occurs."]
  TNCK_THD_3,
  #[doc = "15: TNTE will be set after 15 nacks are received; at most 14 retransmissions occurs."]
  TNCK_THD_15,
}
impl From<TNCK_THD_A> for u8 {
  #[inline(always)]
  fn from(variant: TNCK_THD_A) -> Self {
    match variant {
      TNCK_THD_A::TNCK_THD_0 => 0,
      TNCK_THD_A::TNCK_THD_1 => 1,
      TNCK_THD_A::TNCK_THD_2 => 2,
      TNCK_THD_A::TNCK_THD_3 => 3,
      TNCK_THD_A::TNCK_THD_15 => 15,
    }
  }
}
#[doc = "Reader of field `TNCK_THD`"]
pub type TNCK_THD_R = crate::R<u8, TNCK_THD_A>;
impl TNCK_THD_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> crate::Variant<u8, TNCK_THD_A> {
    use crate::Variant::*;
    match self.bits {
      0 => Val(TNCK_THD_A::TNCK_THD_0),
      1 => Val(TNCK_THD_A::TNCK_THD_1),
      2 => Val(TNCK_THD_A::TNCK_THD_2),
      3 => Val(TNCK_THD_A::TNCK_THD_3),
      15 => Val(TNCK_THD_A::TNCK_THD_15),
      i => Res(i),
    }
  }
  #[doc = "Checks if the value of the field is `TNCK_THD_0`"]
  #[inline(always)]
  pub fn is_tnck_thd_0(&self) -> bool {
    *self == TNCK_THD_A::TNCK_THD_0
  }
  #[doc = "Checks if the value of the field is `TNCK_THD_1`"]
  #[inline(always)]
  pub fn is_tnck_thd_1(&self) -> bool {
    *self == TNCK_THD_A::TNCK_THD_1
  }
  #[doc = "Checks if the value of the field is `TNCK_THD_2`"]
  #[inline(always)]
  pub fn is_tnck_thd_2(&self) -> bool {
    *self == TNCK_THD_A::TNCK_THD_2
  }
  #[doc = "Checks if the value of the field is `TNCK_THD_3`"]
  #[inline(always)]
  pub fn is_tnck_thd_3(&self) -> bool {
    *self == TNCK_THD_A::TNCK_THD_3
  }
  #[doc = "Checks if the value of the field is `TNCK_THD_15`"]
  #[inline(always)]
  pub fn is_tnck_thd_15(&self) -> bool {
    *self == TNCK_THD_A::TNCK_THD_15
  }
}
#[doc = "Write proxy for field `TNCK_THD`"]
pub struct TNCK_THD_W<'a> {
  w: &'a mut W,
}
impl<'a> TNCK_THD_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TNCK_THD_A) -> &'a mut W {
    unsafe { self.bits(variant.into()) }
  }
  #[doc = "TNTE will never be set; retransmission after NACK reception is disabled."]
  #[inline(always)]
  pub fn tnck_thd_0(self) -> &'a mut W {
    self.variant(TNCK_THD_A::TNCK_THD_0)
  }
  #[doc = "TNTE will be set after 1 nack is received; 0 retransmissions occurs."]
  #[inline(always)]
  pub fn tnck_thd_1(self) -> &'a mut W {
    self.variant(TNCK_THD_A::TNCK_THD_1)
  }
  #[doc = "TNTE will be set after 2 nacks are received; at most 1 retransmission occurs."]
  #[inline(always)]
  pub fn tnck_thd_2(self) -> &'a mut W {
    self.variant(TNCK_THD_A::TNCK_THD_2)
  }
  #[doc = "TNTE will be set after 3 nacks are received; at most 2 retransmissions occurs."]
  #[inline(always)]
  pub fn tnck_thd_3(self) -> &'a mut W {
    self.variant(TNCK_THD_A::TNCK_THD_3)
  }
  #[doc = "TNTE will be set after 15 nacks are received; at most 14 retransmissions occurs."]
  #[inline(always)]
  pub fn tnck_thd_15(self) -> &'a mut W {
    self.variant(TNCK_THD_A::TNCK_THD_15)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:3 - Transmitter Data Threshold Value"]
  #[inline(always)]
  pub fn tdt(&self) -> TDT_R {
    TDT_R::new((self.bits & 0x0f) as u8)
  }
  #[doc = "Bits 8:11 - Transmitter NACK Threshold Value"]
  #[inline(always)]
  pub fn tnck_thd(&self) -> TNCK_THD_R {
    TNCK_THD_R::new(((self.bits >> 8) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:3 - Transmitter Data Threshold Value"]
  #[inline(always)]
  pub fn tdt(&mut self) -> TDT_W {
    TDT_W { w: self }
  }
  #[doc = "Bits 8:11 - Transmitter NACK Threshold Value"]
  #[inline(always)]
  pub fn tnck_thd(&mut self) -> TNCK_THD_W {
    TNCK_THD_W { w: self }
  }
}
