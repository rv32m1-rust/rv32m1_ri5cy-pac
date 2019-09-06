#[doc = "Reader of register ERRSTAT"]
pub type R = crate::R<u8, super::ERRSTAT>;
#[doc = "Writer for register ERRSTAT"]
pub type W = crate::W<u8, super::ERRSTAT>;
#[doc = "Register ERRSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::ERRSTAT {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `PIDERR`"]
pub type PIDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIDERR`"]
pub struct PIDERR_W<'a> {
  w: &'a mut W,
}
impl<'a> PIDERR_W<'a> {
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
#[doc = "Reader of field `CRC5EOF`"]
pub type CRC5EOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC5EOF`"]
pub struct CRC5EOF_W<'a> {
  w: &'a mut W,
}
impl<'a> CRC5EOF_W<'a> {
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
#[doc = "Reader of field `CRC16`"]
pub type CRC16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC16`"]
pub struct CRC16_W<'a> {
  w: &'a mut W,
}
impl<'a> CRC16_W<'a> {
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
#[doc = "Reader of field `DFN8`"]
pub type DFN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFN8`"]
pub struct DFN8_W<'a> {
  w: &'a mut W,
}
impl<'a> DFN8_W<'a> {
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
#[doc = "Reader of field `BTOERR`"]
pub type BTOERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTOERR`"]
pub struct BTOERR_W<'a> {
  w: &'a mut W,
}
impl<'a> BTOERR_W<'a> {
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
#[doc = "Reader of field `DMAERR`"]
pub type DMAERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAERR`"]
pub struct DMAERR_W<'a> {
  w: &'a mut W,
}
impl<'a> DMAERR_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
    self.w
  }
}
#[doc = "Reader of field `OWNERR`"]
pub type OWNERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OWNERR`"]
pub struct OWNERR_W<'a> {
  w: &'a mut W,
}
impl<'a> OWNERR_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
    self.w
  }
}
#[doc = "Reader of field `BTSERR`"]
pub type BTSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTSERR`"]
pub struct BTSERR_W<'a> {
  w: &'a mut W,
}
impl<'a> BTSERR_W<'a> {
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - PID error"]
  #[inline(always)]
  pub fn piderr(&self) -> PIDERR_R {
    PIDERR_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - CRC5 error or end of frame error"]
  #[inline(always)]
  pub fn crc5eof(&self) -> CRC5EOF_R {
    CRC5EOF_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - CRC16 error"]
  #[inline(always)]
  pub fn crc16(&self) -> CRC16_R {
    CRC16_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Data field not 8 bits (in length)"]
  #[inline(always)]
  pub fn dfn8(&self) -> DFN8_R {
    DFN8_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - Bus turnaround timeout error"]
  #[inline(always)]
  pub fn btoerr(&self) -> BTOERR_R {
    BTOERR_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - DMAERR"]
  #[inline(always)]
  pub fn dmaerr(&self) -> DMAERR_R {
    DMAERR_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - OWNERR"]
  #[inline(always)]
  pub fn ownerr(&self) -> OWNERR_R {
    OWNERR_R::new(((self.bits >> 6) & 0x01) != 0)
  }
  #[doc = "Bit 7 - Bit stuff error"]
  #[inline(always)]
  pub fn btserr(&self) -> BTSERR_R {
    BTSERR_R::new(((self.bits >> 7) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 0 - PID error"]
  #[inline(always)]
  pub fn piderr(&mut self) -> PIDERR_W {
    PIDERR_W { w: self }
  }
  #[doc = "Bit 1 - CRC5 error or end of frame error"]
  #[inline(always)]
  pub fn crc5eof(&mut self) -> CRC5EOF_W {
    CRC5EOF_W { w: self }
  }
  #[doc = "Bit 2 - CRC16 error"]
  #[inline(always)]
  pub fn crc16(&mut self) -> CRC16_W {
    CRC16_W { w: self }
  }
  #[doc = "Bit 3 - Data field not 8 bits (in length)"]
  #[inline(always)]
  pub fn dfn8(&mut self) -> DFN8_W {
    DFN8_W { w: self }
  }
  #[doc = "Bit 4 - Bus turnaround timeout error"]
  #[inline(always)]
  pub fn btoerr(&mut self) -> BTOERR_W {
    BTOERR_W { w: self }
  }
  #[doc = "Bit 5 - DMAERR"]
  #[inline(always)]
  pub fn dmaerr(&mut self) -> DMAERR_W {
    DMAERR_W { w: self }
  }
  #[doc = "Bit 6 - OWNERR"]
  #[inline(always)]
  pub fn ownerr(&mut self) -> OWNERR_W {
    OWNERR_W { w: self }
  }
  #[doc = "Bit 7 - Bit stuff error"]
  #[inline(always)]
  pub fn btserr(&mut self) -> BTSERR_W {
    BTSERR_W { w: self }
  }
}
