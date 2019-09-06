#[doc = "Reader of register LPFLLTCFG"]
pub type R = crate::R<u32, super::LPFLLTCFG>;
#[doc = "Writer for register LPFLLTCFG"]
pub type W = crate::W<u32, super::LPFLLTCFG>;
#[doc = "Register LPFLLTCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::LPFLLTCFG {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Trim Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIMSRC_A {
  #[doc = "0: SIRC"]
  TRIMSRC_0,
  #[doc = "1: FIRC"]
  TRIMSRC_1,
  #[doc = "2: System OSC"]
  TRIMSRC_2,
  #[doc = "3: RTC OSC"]
  TRIMSRC_3,
}
impl From<TRIMSRC_A> for u8 {
  #[inline(always)]
  fn from(variant: TRIMSRC_A) -> Self {
    match variant {
      TRIMSRC_A::TRIMSRC_0 => 0,
      TRIMSRC_A::TRIMSRC_1 => 1,
      TRIMSRC_A::TRIMSRC_2 => 2,
      TRIMSRC_A::TRIMSRC_3 => 3,
    }
  }
}
#[doc = "Reader of field `TRIMSRC`"]
pub type TRIMSRC_R = crate::R<u8, TRIMSRC_A>;
impl TRIMSRC_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> TRIMSRC_A {
    match self.bits {
      0 => TRIMSRC_A::TRIMSRC_0,
      1 => TRIMSRC_A::TRIMSRC_1,
      2 => TRIMSRC_A::TRIMSRC_2,
      3 => TRIMSRC_A::TRIMSRC_3,
      _ => unreachable!(),
    }
  }
  #[doc = "Checks if the value of the field is `TRIMSRC_0`"]
  #[inline(always)]
  pub fn is_trimsrc_0(&self) -> bool {
    *self == TRIMSRC_A::TRIMSRC_0
  }
  #[doc = "Checks if the value of the field is `TRIMSRC_1`"]
  #[inline(always)]
  pub fn is_trimsrc_1(&self) -> bool {
    *self == TRIMSRC_A::TRIMSRC_1
  }
  #[doc = "Checks if the value of the field is `TRIMSRC_2`"]
  #[inline(always)]
  pub fn is_trimsrc_2(&self) -> bool {
    *self == TRIMSRC_A::TRIMSRC_2
  }
  #[doc = "Checks if the value of the field is `TRIMSRC_3`"]
  #[inline(always)]
  pub fn is_trimsrc_3(&self) -> bool {
    *self == TRIMSRC_A::TRIMSRC_3
  }
}
#[doc = "Write proxy for field `TRIMSRC`"]
pub struct TRIMSRC_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIMSRC_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: TRIMSRC_A) -> &'a mut W {
    {
      self.bits(variant.into())
    }
  }
  #[doc = "SIRC"]
  #[inline(always)]
  pub fn trimsrc_0(self) -> &'a mut W {
    self.variant(TRIMSRC_A::TRIMSRC_0)
  }
  #[doc = "FIRC"]
  #[inline(always)]
  pub fn trimsrc_1(self) -> &'a mut W {
    self.variant(TRIMSRC_A::TRIMSRC_1)
  }
  #[doc = "System OSC"]
  #[inline(always)]
  pub fn trimsrc_2(self) -> &'a mut W {
    self.variant(TRIMSRC_A::TRIMSRC_2)
  }
  #[doc = "RTC OSC"]
  #[inline(always)]
  pub fn trimsrc_3(self) -> &'a mut W {
    self.variant(TRIMSRC_A::TRIMSRC_3)
  }
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
    self.w
  }
}
#[doc = "Reader of field `TRIMDIV`"]
pub type TRIMDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIMDIV`"]
pub struct TRIMDIV_W<'a> {
  w: &'a mut W,
}
impl<'a> TRIMDIV_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
    self.w
  }
}
#[doc = "Lock LPFLL with 2 LSBS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKW2LSB_A {
  #[doc = "0: LPFLL locks within 1LSB (0.4%)"]
  LOCKW2LSB_0,
  #[doc = "1: LPFLL locks within 2LSB (0.8%)"]
  LOCKW2LSB_1,
}
impl From<LOCKW2LSB_A> for bool {
  #[inline(always)]
  fn from(variant: LOCKW2LSB_A) -> Self {
    match variant {
      LOCKW2LSB_A::LOCKW2LSB_0 => false,
      LOCKW2LSB_A::LOCKW2LSB_1 => true,
    }
  }
}
#[doc = "Reader of field `LOCKW2LSB`"]
pub type LOCKW2LSB_R = crate::R<bool, LOCKW2LSB_A>;
impl LOCKW2LSB_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> LOCKW2LSB_A {
    match self.bits {
      false => LOCKW2LSB_A::LOCKW2LSB_0,
      true => LOCKW2LSB_A::LOCKW2LSB_1,
    }
  }
  #[doc = "Checks if the value of the field is `LOCKW2LSB_0`"]
  #[inline(always)]
  pub fn is_lockw2lsb_0(&self) -> bool {
    *self == LOCKW2LSB_A::LOCKW2LSB_0
  }
  #[doc = "Checks if the value of the field is `LOCKW2LSB_1`"]
  #[inline(always)]
  pub fn is_lockw2lsb_1(&self) -> bool {
    *self == LOCKW2LSB_A::LOCKW2LSB_1
  }
}
#[doc = "Write proxy for field `LOCKW2LSB`"]
pub struct LOCKW2LSB_W<'a> {
  w: &'a mut W,
}
impl<'a> LOCKW2LSB_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: LOCKW2LSB_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "LPFLL locks within 1LSB (0.4%)"]
  #[inline(always)]
  pub fn lockw2lsb_0(self) -> &'a mut W {
    self.variant(LOCKW2LSB_A::LOCKW2LSB_0)
  }
  #[doc = "LPFLL locks within 2LSB (0.8%)"]
  #[inline(always)]
  pub fn lockw2lsb_1(self) -> &'a mut W {
    self.variant(LOCKW2LSB_A::LOCKW2LSB_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:1 - Trim Source"]
  #[inline(always)]
  pub fn trimsrc(&self) -> TRIMSRC_R {
    TRIMSRC_R::new((self.bits & 0x03) as u8)
  }
  #[doc = "Bits 8:12 - LPFLL Trim Predivide"]
  #[inline(always)]
  pub fn trimdiv(&self) -> TRIMDIV_R {
    TRIMDIV_R::new(((self.bits >> 8) & 0x1f) as u8)
  }
  #[doc = "Bit 16 - Lock LPFLL with 2 LSBS"]
  #[inline(always)]
  pub fn lockw2lsb(&self) -> LOCKW2LSB_R {
    LOCKW2LSB_R::new(((self.bits >> 16) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bits 0:1 - Trim Source"]
  #[inline(always)]
  pub fn trimsrc(&mut self) -> TRIMSRC_W {
    TRIMSRC_W { w: self }
  }
  #[doc = "Bits 8:12 - LPFLL Trim Predivide"]
  #[inline(always)]
  pub fn trimdiv(&mut self) -> TRIMDIV_W {
    TRIMDIV_W { w: self }
  }
  #[doc = "Bit 16 - Lock LPFLL with 2 LSBS"]
  #[inline(always)]
  pub fn lockw2lsb(&mut self) -> LOCKW2LSB_W {
    LOCKW2LSB_W { w: self }
  }
}
