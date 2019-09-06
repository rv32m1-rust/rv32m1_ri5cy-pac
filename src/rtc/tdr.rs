#[doc = "Reader of register TDR"]
pub type R = crate::R<u32, super::TDR>;
#[doc = "Writer for register TDR"]
pub type W = crate::W<u32, super::TDR>;
#[doc = "Register TDR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::TDR {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0x01
  }
}
#[doc = "Loss of Clock Tamper Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCTF_A {
  #[doc = "0: Tamper not detected."]
  LCTF_0,
  #[doc = "1: Loss of Clock tamper detected."]
  LCTF_1,
}
impl From<LCTF_A> for bool {
  #[inline(always)]
  fn from(variant: LCTF_A) -> Self {
    match variant {
      LCTF_A::LCTF_0 => false,
      LCTF_A::LCTF_1 => true,
    }
  }
}
#[doc = "Reader of field `LCTF`"]
pub type LCTF_R = crate::R<bool, LCTF_A>;
impl LCTF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LCTF_A {
    match self.bits {
      false => LCTF_A::LCTF_0,
      true => LCTF_A::LCTF_1,
    }
  }
  #[doc = "Checks if the value of the field is `LCTF_0`"]
  #[inline(always)]
  pub fn is_lctf_0(&self) -> bool {
    *self == LCTF_A::LCTF_0
  }
  #[doc = "Checks if the value of the field is `LCTF_1`"]
  #[inline(always)]
  pub fn is_lctf_1(&self) -> bool {
    *self == LCTF_A::LCTF_1
  }
}
#[doc = "Write proxy for field `LCTF`"]
pub struct LCTF_W<'a> {
  w: &'a mut W,
}
impl<'a> LCTF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LCTF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Tamper not detected."]
  #[inline(always)]
  pub fn lctf_0(self) -> &'a mut W {
    self.variant(LCTF_A::LCTF_0)
  }
  #[doc = "Loss of Clock tamper detected."]
  #[inline(always)]
  pub fn lctf_1(self) -> &'a mut W {
    self.variant(LCTF_A::LCTF_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
    self.w
  }
}
#[doc = "Security Tamper Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STF_A {
  #[doc = "0: Tamper not detected."]
  STF_0,
  #[doc = "1: Security module tamper detected."]
  STF_1,
}
impl From<STF_A> for bool {
  #[inline(always)]
  fn from(variant: STF_A) -> Self {
    match variant {
      STF_A::STF_0 => false,
      STF_A::STF_1 => true,
    }
  }
}
#[doc = "Reader of field `STF`"]
pub type STF_R = crate::R<bool, STF_A>;
impl STF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> STF_A {
    match self.bits {
      false => STF_A::STF_0,
      true => STF_A::STF_1,
    }
  }
  #[doc = "Checks if the value of the field is `STF_0`"]
  #[inline(always)]
  pub fn is_stf_0(&self) -> bool {
    *self == STF_A::STF_0
  }
  #[doc = "Checks if the value of the field is `STF_1`"]
  #[inline(always)]
  pub fn is_stf_1(&self) -> bool {
    *self == STF_A::STF_1
  }
}
#[doc = "Write proxy for field `STF`"]
pub struct STF_W<'a> {
  w: &'a mut W,
}
impl<'a> STF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: STF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Tamper not detected."]
  #[inline(always)]
  pub fn stf_0(self) -> &'a mut W {
    self.variant(STF_A::STF_0)
  }
  #[doc = "Security module tamper detected."]
  #[inline(always)]
  pub fn stf_1(self) -> &'a mut W {
    self.variant(STF_A::STF_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
    self.w
  }
}
#[doc = "Flash Security Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSF_A {
  #[doc = "0: Tamper not detected."]
  FSF_0,
  #[doc = "1: Flash security tamper detected."]
  FSF_1,
}
impl From<FSF_A> for bool {
  #[inline(always)]
  fn from(variant: FSF_A) -> Self {
    match variant {
      FSF_A::FSF_0 => false,
      FSF_A::FSF_1 => true,
    }
  }
}
#[doc = "Reader of field `FSF`"]
pub type FSF_R = crate::R<bool, FSF_A>;
impl FSF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> FSF_A {
    match self.bits {
      false => FSF_A::FSF_0,
      true => FSF_A::FSF_1,
    }
  }
  #[doc = "Checks if the value of the field is `FSF_0`"]
  #[inline(always)]
  pub fn is_fsf_0(&self) -> bool {
    *self == FSF_A::FSF_0
  }
  #[doc = "Checks if the value of the field is `FSF_1`"]
  #[inline(always)]
  pub fn is_fsf_1(&self) -> bool {
    *self == FSF_A::FSF_1
  }
}
#[doc = "Write proxy for field `FSF`"]
pub struct FSF_W<'a> {
  w: &'a mut W,
}
impl<'a> FSF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: FSF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Tamper not detected."]
  #[inline(always)]
  pub fn fsf_0(self) -> &'a mut W {
    self.variant(FSF_A::FSF_0)
  }
  #[doc = "Flash security tamper detected."]
  #[inline(always)]
  pub fn fsf_1(self) -> &'a mut W {
    self.variant(FSF_A::FSF_1)
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
#[doc = "Test Mode Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMF_A {
  #[doc = "0: Tamper not detected."]
  TMF_0,
  #[doc = "1: Test mode tamper detected."]
  TMF_1,
}
impl From<TMF_A> for bool {
  #[inline(always)]
  fn from(variant: TMF_A) -> Self {
    match variant {
      TMF_A::TMF_0 => false,
      TMF_A::TMF_1 => true,
    }
  }
}
#[doc = "Reader of field `TMF`"]
pub type TMF_R = crate::R<bool, TMF_A>;
impl TMF_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TMF_A {
    match self.bits {
      false => TMF_A::TMF_0,
      true => TMF_A::TMF_1,
    }
  }
  #[doc = "Checks if the value of the field is `TMF_0`"]
  #[inline(always)]
  pub fn is_tmf_0(&self) -> bool {
    *self == TMF_A::TMF_0
  }
  #[doc = "Checks if the value of the field is `TMF_1`"]
  #[inline(always)]
  pub fn is_tmf_1(&self) -> bool {
    *self == TMF_A::TMF_1
  }
}
#[doc = "Write proxy for field `TMF`"]
pub struct TMF_W<'a> {
  w: &'a mut W,
}
impl<'a> TMF_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TMF_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Tamper not detected."]
  #[inline(always)]
  pub fn tmf_0(self) -> &'a mut W {
    self.variant(TMF_A::TMF_0)
  }
  #[doc = "Test mode tamper detected."]
  #[inline(always)]
  pub fn tmf_1(self) -> &'a mut W {
    self.variant(TMF_A::TMF_1)
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
#[doc = "Reader of field `TPF`"]
pub type TPF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TPF`"]
pub struct TPF_W<'a> {
  w: &'a mut W,
}
impl<'a> TPF_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bit 4 - Loss of Clock Tamper Flag"]
  #[inline(always)]
  pub fn lctf(&self) -> LCTF_R {
    LCTF_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Security Tamper Flag"]
  #[inline(always)]
  pub fn stf(&self) -> STF_R {
    STF_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - Flash Security Flag"]
  #[inline(always)]
  pub fn fsf(&self) -> FSF_R {
    FSF_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Test Mode Flag"]
  #[inline(always)]
  pub fn tmf(&self) -> TMF_R {
    TMF_R::new(((self.bits >> 7) & 0x01) != 0)
  }
  #[doc = "Bits 16:19 - Tamper Pin Flag"]
  #[inline(always)]
  pub fn tpf(&self) -> TPF_R {
    TPF_R::new(((self.bits >> 16) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bit 4 - Loss of Clock Tamper Flag"]
  #[inline(always)]
  pub fn lctf(&mut self) -> LCTF_W {
    LCTF_W { w: self }
  }
  #[doc = "Bit 5 - Security Tamper Flag"]
  #[inline(always)]
  pub fn stf(&mut self) -> STF_W {
    STF_W { w: self }
  }
  #[doc = "Bit 6 - Flash Security Flag"]
  #[inline(always)]
  pub fn fsf(&mut self) -> FSF_W {
    FSF_W { w: self }
  }
  #[doc = "Bit 7 - Test Mode Flag"]
  #[inline(always)]
  pub fn tmf(&mut self) -> TMF_W {
    TMF_W { w: self }
  }
  #[doc = "Bits 16:19 - Tamper Pin Flag"]
  #[inline(always)]
  pub fn tpf(&mut self) -> TPF_W {
    TPF_W { w: self }
  }
}
