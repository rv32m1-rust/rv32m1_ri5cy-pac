#[doc = "Reader of register CW"]
pub type R = crate::R<u32, super::CW>;
#[doc = "Writer for register CW"]
pub type W = crate::W<u32, super::CW>;
#[doc = "Register CW `reset()`'s with value 0"]
impl crate::ResetValue for super::CW {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `CM`"]
pub struct CM_W<'a> {
  w: &'a mut W,
}
impl<'a> CM_W<'a> {
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
    self.w
  }
}
#[doc = "Write proxy for field `CPKA`"]
pub struct CPKA_W<'a> {
  w: &'a mut W,
}
impl<'a> CPKA_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
    self.w
  }
}
#[doc = "Write proxy for field `CPKB`"]
pub struct CPKB_W<'a> {
  w: &'a mut W,
}
impl<'a> CPKB_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
    self.w
  }
}
#[doc = "Write proxy for field `CPKN`"]
pub struct CPKN_W<'a> {
  w: &'a mut W,
}
impl<'a> CPKN_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
    self.w
  }
}
#[doc = "Write proxy for field `CPKE`"]
pub struct CPKE_W<'a> {
  w: &'a mut W,
}
impl<'a> CPKE_W<'a> {
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
impl R {}
impl W {
  #[doc = "Bit 0 - Clear the Mode Register"]
  #[inline(always)]
  pub fn cm(&mut self) -> CM_W {
    CM_W { w: self }
  }
  #[doc = "Bit 12 - Clear the PKHA A Size Register"]
  #[inline(always)]
  pub fn cpka(&mut self) -> CPKA_W {
    CPKA_W { w: self }
  }
  #[doc = "Bit 13 - Clear the PKHA B Size Register"]
  #[inline(always)]
  pub fn cpkb(&mut self) -> CPKB_W {
    CPKB_W { w: self }
  }
  #[doc = "Bit 14 - Clear the PKHA N Size Register"]
  #[inline(always)]
  pub fn cpkn(&mut self) -> CPKN_W {
    CPKN_W { w: self }
  }
  #[doc = "Bit 15 - Clear the PKHA E Size Register"]
  #[inline(always)]
  pub fn cpke(&mut self) -> CPKE_W {
    CPKE_W { w: self }
  }
}
