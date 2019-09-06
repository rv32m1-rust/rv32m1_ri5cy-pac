#[doc = "Reader of register ENDPT"]
pub type R = crate::R<u8, super::ENDPT>;
#[doc = "Writer for register ENDPT"]
pub type W = crate::W<u8, super::ENDPT>;
#[doc = "Register ENDPT `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDPT {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `EPHSHK`"]
pub type EPHSHK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPHSHK`"]
pub struct EPHSHK_W<'a> {
  w: &'a mut W,
}
impl<'a> EPHSHK_W<'a> {
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
    self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
    self.w
  }
}
#[doc = "Reader of field `EPSTALL`"]
pub type EPSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPSTALL`"]
pub struct EPSTALL_W<'a> {
  w: &'a mut W,
}
impl<'a> EPSTALL_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
    self.w
  }
}
#[doc = "Reader of field `EPTXEN`"]
pub type EPTXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPTXEN`"]
pub struct EPTXEN_W<'a> {
  w: &'a mut W,
}
impl<'a> EPTXEN_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
    self.w
  }
}
#[doc = "Reader of field `EPRXEN`"]
pub type EPRXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRXEN`"]
pub struct EPRXEN_W<'a> {
  w: &'a mut W,
}
impl<'a> EPRXEN_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
    self.w
  }
}
#[doc = "Reader of field `EPCTLDIS`"]
pub type EPCTLDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPCTLDIS`"]
pub struct EPCTLDIS_W<'a> {
  w: &'a mut W,
}
impl<'a> EPCTLDIS_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - Endpoint handshaking enable"]
  #[inline(always)]
  pub fn ephshk(&self) -> EPHSHK_R {
    EPHSHK_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Endpoint stalled"]
  #[inline(always)]
  pub fn epstall(&self) -> EPSTALL_R {
    EPSTALL_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Endpoint for TX transfers enable"]
  #[inline(always)]
  pub fn eptxen(&self) -> EPTXEN_R {
    EPTXEN_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Endpoint for RX transfers enable"]
  #[inline(always)]
  pub fn eprxen(&self) -> EPRXEN_R {
    EPRXEN_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Control (SETUP) transfer disable"]
  #[inline(always)]
  pub fn epctldis(&self) -> EPCTLDIS_R {
    EPCTLDIS_R::new(((self.bits >> 4) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - Endpoint handshaking enable"]
  #[inline(always)]
  pub fn ephshk(&mut self) -> EPHSHK_W {
    EPHSHK_W { w: self }
  }
  #[doc = "Bit 1 - Endpoint stalled"]
  #[inline(always)]
  pub fn epstall(&mut self) -> EPSTALL_W {
    EPSTALL_W { w: self }
  }
  #[doc = "Bit 2 - Endpoint for TX transfers enable"]
  #[inline(always)]
  pub fn eptxen(&mut self) -> EPTXEN_W {
    EPTXEN_W { w: self }
  }
  #[doc = "Bit 3 - Endpoint for RX transfers enable"]
  #[inline(always)]
  pub fn eprxen(&mut self) -> EPRXEN_W {
    EPRXEN_W { w: self }
  }
  #[doc = "Bit 4 - Control (SETUP) transfer disable"]
  #[inline(always)]
  pub fn epctldis(&mut self) -> EPCTLDIS_W {
    EPCTLDIS_W { w: self }
  }
}
