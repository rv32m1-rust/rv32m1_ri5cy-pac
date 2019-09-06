#[doc = "Reader of register DSM_WAKEUP"]
pub type R = crate::R<u32, super::DSM_WAKEUP>;
#[doc = "Writer for register DSM_WAKEUP"]
pub type W = crate::W<u32, super::DSM_WAKEUP>;
#[doc = "Register DSM_WAKEUP `reset()`'s with value 0"]
impl crate::ResetValue for super::DSM_WAKEUP {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Reader of field `DSM_POWER_OFFSET_TIME`"]
pub type DSM_POWER_OFFSET_TIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DSM_POWER_OFFSET_TIME`"]
pub struct DSM_POWER_OFFSET_TIME_W<'a> {
  w: &'a mut W,
}
impl<'a> DSM_POWER_OFFSET_TIME_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
    self.w
  }
}
#[doc = "Reader of field `ACTIVE_WARNING`"]
pub type ACTIVE_WARNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACTIVE_WARNING`"]
pub struct ACTIVE_WARNING_W<'a> {
  w: &'a mut W,
}
impl<'a> ACTIVE_WARNING_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
    self.w
  }
}
#[doc = "Reader of field `FINE_DELAY`"]
pub type FINE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FINE_DELAY`"]
pub struct FINE_DELAY_W<'a> {
  w: &'a mut W,
}
impl<'a> FINE_DELAY_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
    self.w
  }
}
#[doc = "Reader of field `COARSE_DELAY`"]
pub type COARSE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COARSE_DELAY`"]
pub struct COARSE_DELAY_W<'a> {
  w: &'a mut W,
}
impl<'a> COARSE_DELAY_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u8) -> &'a mut W {
    self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
    self.w
  }
}
impl R {
  #[doc = "Bits 0:9 - Deep Sleep Wakeup Power Offset Time"]
  #[inline(always)]
  pub fn dsm_power_offset_time(&self) -> DSM_POWER_OFFSET_TIME_R {
    DSM_POWER_OFFSET_TIME_R::new((self.bits & 0x03ff) as u16)
  }
  #[doc = "Bits 12:17 - Deep Sleep Wakeup RF Active Warning Time"]
  #[inline(always)]
  pub fn active_warning(&self) -> ACTIVE_WARNING_R {
    ACTIVE_WARNING_R::new(((self.bits >> 12) & 0x3f) as u8)
  }
  #[doc = "Bits 20:25 - Deep Sleep Wakeup Fine Delay Time"]
  #[inline(always)]
  pub fn fine_delay(&self) -> FINE_DELAY_R {
    FINE_DELAY_R::new(((self.bits >> 20) & 0x3f) as u8)
  }
  #[doc = "Bits 28:31 - Deep Sleep Wakeup Coarse Delay Time"]
  #[inline(always)]
  pub fn coarse_delay(&self) -> COARSE_DELAY_R {
    COARSE_DELAY_R::new(((self.bits >> 28) & 0x0f) as u8)
  }
}
impl W {
  #[doc = "Bits 0:9 - Deep Sleep Wakeup Power Offset Time"]
  #[inline(always)]
  pub fn dsm_power_offset_time(&mut self) -> DSM_POWER_OFFSET_TIME_W {
    DSM_POWER_OFFSET_TIME_W { w: self }
  }
  #[doc = "Bits 12:17 - Deep Sleep Wakeup RF Active Warning Time"]
  #[inline(always)]
  pub fn active_warning(&mut self) -> ACTIVE_WARNING_W {
    ACTIVE_WARNING_W { w: self }
  }
  #[doc = "Bits 20:25 - Deep Sleep Wakeup Fine Delay Time"]
  #[inline(always)]
  pub fn fine_delay(&mut self) -> FINE_DELAY_W {
    FINE_DELAY_W { w: self }
  }
  #[doc = "Bits 28:31 - Deep Sleep Wakeup Coarse Delay Time"]
  #[inline(always)]
  pub fn coarse_delay(&mut self) -> COARSE_DELAY_W {
    COARSE_DELAY_W { w: self }
  }
}
