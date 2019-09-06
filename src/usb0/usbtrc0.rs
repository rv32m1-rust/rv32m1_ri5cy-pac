#[doc = "Reader of register USBTRC0"]
pub type R = crate::R<u8, super::USBTRC0>;
#[doc = "Writer for register USBTRC0"]
pub type W = crate::W<u8, super::USBTRC0>;
#[doc = "Register USBTRC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::USBTRC0 {
  type Type = u8;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "USB Asynchronous Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RESUME_INT_A {
  #[doc = "0: No interrupt was generated."]
  USB_RESUME_INT_0,
  #[doc = "1: Interrupt was generated because of the USB asynchronous interrupt."]
  USB_RESUME_INT_1,
}
impl From<USB_RESUME_INT_A> for bool {
  #[inline(always)]
  fn from(variant: USB_RESUME_INT_A) -> Self {
    match variant {
      USB_RESUME_INT_A::USB_RESUME_INT_0 => false,
      USB_RESUME_INT_A::USB_RESUME_INT_1 => true,
    }
  }
}
#[doc = "Reader of field `USB_RESUME_INT`"]
pub type USB_RESUME_INT_R = crate::R<bool, USB_RESUME_INT_A>;
impl USB_RESUME_INT_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> USB_RESUME_INT_A {
    match self.bits {
      false => USB_RESUME_INT_A::USB_RESUME_INT_0,
      true => USB_RESUME_INT_A::USB_RESUME_INT_1,
    }
  }
  #[doc = "Checks if the value of the field is `USB_RESUME_INT_0`"]
  #[inline(always)]
  pub fn is_usb_resume_int_0(&self) -> bool {
    *self == USB_RESUME_INT_A::USB_RESUME_INT_0
  }
  #[doc = "Checks if the value of the field is `USB_RESUME_INT_1`"]
  #[inline(always)]
  pub fn is_usb_resume_int_1(&self) -> bool {
    *self == USB_RESUME_INT_A::USB_RESUME_INT_1
  }
}
#[doc = "Synchronous USB Interrupt Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNC_DET_A {
  #[doc = "0: Synchronous interrupt has not been detected."]
  SYNC_DET_0,
  #[doc = "1: Synchronous interrupt has been detected."]
  SYNC_DET_1,
}
impl From<SYNC_DET_A> for bool {
  #[inline(always)]
  fn from(variant: SYNC_DET_A) -> Self {
    match variant {
      SYNC_DET_A::SYNC_DET_0 => false,
      SYNC_DET_A::SYNC_DET_1 => true,
    }
  }
}
#[doc = "Reader of field `SYNC_DET`"]
pub type SYNC_DET_R = crate::R<bool, SYNC_DET_A>;
impl SYNC_DET_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> SYNC_DET_A {
    match self.bits {
      false => SYNC_DET_A::SYNC_DET_0,
      true => SYNC_DET_A::SYNC_DET_1,
    }
  }
  #[doc = "Checks if the value of the field is `SYNC_DET_0`"]
  #[inline(always)]
  pub fn is_sync_det_0(&self) -> bool {
    *self == SYNC_DET_A::SYNC_DET_0
  }
  #[doc = "Checks if the value of the field is `SYNC_DET_1`"]
  #[inline(always)]
  pub fn is_sync_det_1(&self) -> bool {
    *self == SYNC_DET_A::SYNC_DET_1
  }
}
#[doc = "Reader of field `USB_CLK_RECOVERY_INT`"]
pub type USB_CLK_RECOVERY_INT_R = crate::R<bool, bool>;
#[doc = "VREGIN Rising Edge Interrupt Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREDG_DET_A {
  #[doc = "0: VREGIN rising edge interrupt has not been detected."]
  VREDG_DET_0,
  #[doc = "1: VREGIN rising edge interrupt has been detected."]
  VREDG_DET_1,
}
impl From<VREDG_DET_A> for bool {
  #[inline(always)]
  fn from(variant: VREDG_DET_A) -> Self {
    match variant {
      VREDG_DET_A::VREDG_DET_0 => false,
      VREDG_DET_A::VREDG_DET_1 => true,
    }
  }
}
#[doc = "Reader of field `VREDG_DET`"]
pub type VREDG_DET_R = crate::R<bool, VREDG_DET_A>;
impl VREDG_DET_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VREDG_DET_A {
    match self.bits {
      false => VREDG_DET_A::VREDG_DET_0,
      true => VREDG_DET_A::VREDG_DET_1,
    }
  }
  #[doc = "Checks if the value of the field is `VREDG_DET_0`"]
  #[inline(always)]
  pub fn is_vredg_det_0(&self) -> bool {
    *self == VREDG_DET_A::VREDG_DET_0
  }
  #[doc = "Checks if the value of the field is `VREDG_DET_1`"]
  #[inline(always)]
  pub fn is_vredg_det_1(&self) -> bool {
    *self == VREDG_DET_A::VREDG_DET_1
  }
}
#[doc = "VREGIN Falling Edge Interrupt Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VFEDG_DET_A {
  #[doc = "0: VREGIN falling edge interrupt has not been detected."]
  VFEDG_DET_0,
  #[doc = "1: VREGIN falling edge interrupt has been detected."]
  VFEDG_DET_1,
}
impl From<VFEDG_DET_A> for bool {
  #[inline(always)]
  fn from(variant: VFEDG_DET_A) -> Self {
    match variant {
      VFEDG_DET_A::VFEDG_DET_0 => false,
      VFEDG_DET_A::VFEDG_DET_1 => true,
    }
  }
}
#[doc = "Reader of field `VFEDG_DET`"]
pub type VFEDG_DET_R = crate::R<bool, VFEDG_DET_A>;
impl VFEDG_DET_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> VFEDG_DET_A {
    match self.bits {
      false => VFEDG_DET_A::VFEDG_DET_0,
      true => VFEDG_DET_A::VFEDG_DET_1,
    }
  }
  #[doc = "Checks if the value of the field is `VFEDG_DET_0`"]
  #[inline(always)]
  pub fn is_vfedg_det_0(&self) -> bool {
    *self == VFEDG_DET_A::VFEDG_DET_0
  }
  #[doc = "Checks if the value of the field is `VFEDG_DET_1`"]
  #[inline(always)]
  pub fn is_vfedg_det_1(&self) -> bool {
    *self == VFEDG_DET_A::VFEDG_DET_1
  }
}
#[doc = "Asynchronous Resume Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESMEN_A {
  #[doc = "0: USB asynchronous wakeup from Suspend mode is disabled."]
  USBRESMEN_0,
  #[doc = "1: USB asynchronous wakeup from Suspend mode is enabled."]
  USBRESMEN_1,
}
impl From<USBRESMEN_A> for bool {
  #[inline(always)]
  fn from(variant: USBRESMEN_A) -> Self {
    match variant {
      USBRESMEN_A::USBRESMEN_0 => false,
      USBRESMEN_A::USBRESMEN_1 => true,
    }
  }
}
#[doc = "Reader of field `USBRESMEN`"]
pub type USBRESMEN_R = crate::R<bool, USBRESMEN_A>;
impl USBRESMEN_R {
  #[doc = r"Get enumerated values variant"]
  #[inline(always)]
  pub fn variant(&self) -> USBRESMEN_A {
    match self.bits {
      false => USBRESMEN_A::USBRESMEN_0,
      true => USBRESMEN_A::USBRESMEN_1,
    }
  }
  #[doc = "Checks if the value of the field is `USBRESMEN_0`"]
  #[inline(always)]
  pub fn is_usbresmen_0(&self) -> bool {
    *self == USBRESMEN_A::USBRESMEN_0
  }
  #[doc = "Checks if the value of the field is `USBRESMEN_1`"]
  #[inline(always)]
  pub fn is_usbresmen_1(&self) -> bool {
    *self == USBRESMEN_A::USBRESMEN_1
  }
}
#[doc = "Write proxy for field `USBRESMEN`"]
pub struct USBRESMEN_W<'a> {
  w: &'a mut W,
}
impl<'a> USBRESMEN_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: USBRESMEN_A) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "USB asynchronous wakeup from Suspend mode is disabled."]
  #[inline(always)]
  pub fn usbresmen_0(self) -> &'a mut W {
    self.variant(USBRESMEN_A::USBRESMEN_0)
  }
  #[doc = "USB asynchronous wakeup from Suspend mode is enabled."]
  #[inline(always)]
  pub fn usbresmen_1(self) -> &'a mut W {
    self.variant(USBRESMEN_A::USBRESMEN_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
    self.w
  }
}
#[doc = "Reader of field `VREGIN_STS`"]
pub type VREGIN_STS_R = crate::R<bool, bool>;
#[doc = "USB Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESET_AW {
  #[doc = "0: Normal USB module operation."]
  USBRESET_0,
  #[doc = "1: Returns the USB module to its reset state."]
  USBRESET_1,
}
impl From<USBRESET_AW> for bool {
  #[inline(always)]
  fn from(variant: USBRESET_AW) -> Self {
    match variant {
      USBRESET_AW::USBRESET_0 => false,
      USBRESET_AW::USBRESET_1 => true,
    }
  }
}
#[doc = "Write proxy for field `USBRESET`"]
pub struct USBRESET_W<'a> {
  w: &'a mut W,
}
impl<'a> USBRESET_W<'a> {
  #[doc = r"Writes `variant` to the field"]
  #[inline(always)]
  pub fn variant(self, variant: USBRESET_AW) -> &'a mut W {
    {
      self.bit(variant.into())
    }
  }
  #[doc = "Normal USB module operation."]
  #[inline(always)]
  pub fn usbreset_0(self) -> &'a mut W {
    self.variant(USBRESET_AW::USBRESET_0)
  }
  #[doc = "Returns the USB module to its reset state."]
  #[inline(always)]
  pub fn usbreset_1(self) -> &'a mut W {
    self.variant(USBRESET_AW::USBRESET_1)
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
    self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
    self.w
  }
}
impl R {
  #[doc = "Bit 0 - USB Asynchronous Interrupt"]
  #[inline(always)]
  pub fn usb_resume_int(&self) -> USB_RESUME_INT_R {
    USB_RESUME_INT_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Synchronous USB Interrupt Detect"]
  #[inline(always)]
  pub fn sync_det(&self) -> SYNC_DET_R {
    SYNC_DET_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Combined USB Clock Recovery interrupt status"]
  #[inline(always)]
  pub fn usb_clk_recovery_int(&self) -> USB_CLK_RECOVERY_INT_R {
    USB_CLK_RECOVERY_INT_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - VREGIN Rising Edge Interrupt Detect"]
  #[inline(always)]
  pub fn vredg_det(&self) -> VREDG_DET_R {
    VREDG_DET_R::new(((self.bits >> 3) & 0x01) != 0)
  }
  #[doc = "Bit 4 - VREGIN Falling Edge Interrupt Detect"]
  #[inline(always)]
  pub fn vfedg_det(&self) -> VFEDG_DET_R {
    VFEDG_DET_R::new(((self.bits >> 4) & 0x01) != 0)
  }
  #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
  #[inline(always)]
  pub fn usbresmen(&self) -> USBRESMEN_R {
    USBRESMEN_R::new(((self.bits >> 5) & 0x01) != 0)
  }
  #[doc = "Bit 6 - VREGIN_STS"]
  #[inline(always)]
  pub fn vregin_sts(&self) -> VREGIN_STS_R {
    VREGIN_STS_R::new(((self.bits >> 6) & 0x01) != 0)
  }
}
impl W {
  #[doc = "Bit 5 - Asynchronous Resume Interrupt Enable"]
  #[inline(always)]
  pub fn usbresmen(&mut self) -> USBRESMEN_W {
    USBRESMEN_W { w: self }
  }
  #[doc = "Bit 7 - USB Reset"]
  #[inline(always)]
  pub fn usbreset(&mut self) -> USBRESET_W {
    USBRESET_W { w: self }
  }
}
