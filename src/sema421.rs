#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
  #[doc = "0x00 - Gate Register"]
  pub gate3: GATE,
  #[doc = "0x01 - Gate Register"]
  pub gate2: GATE,
  #[doc = "0x02 - Gate Register"]
  pub gate1: GATE,
  #[doc = "0x03 - Gate Register"]
  pub gate0: GATE,
  #[doc = "0x04 - Gate Register"]
  pub gate7: GATE,
  #[doc = "0x05 - Gate Register"]
  pub gate6: GATE,
  #[doc = "0x06 - Gate Register"]
  pub gate5: GATE,
  #[doc = "0x07 - Gate Register"]
  pub gate4: GATE,
  #[doc = "0x08 - Gate Register"]
  pub gate11: GATE,
  #[doc = "0x09 - Gate Register"]
  pub gate10: GATE,
  #[doc = "0x0a - Gate Register"]
  pub gate9: GATE,
  #[doc = "0x0b - Gate Register"]
  pub gate8: GATE,
  #[doc = "0x0c - Gate Register"]
  pub gate15: GATE,
  #[doc = "0x0d - Gate Register"]
  pub gate14: GATE,
  #[doc = "0x0e - Gate Register"]
  pub gate13: GATE,
  #[doc = "0x0f - Gate Register"]
  pub gate12: GATE,
  _reserved16: [u8; 50usize],
  _reserved_16_rstgt: [u8; 2usize],
}
impl RegisterBlock {
  #[doc = "0x42 - Reset Gate Write"]
  #[inline(always)]
  pub fn rstgt_w(&self) -> &RSTGT_W {
    unsafe { &*(((self as *const Self) as *const u8).add(66usize) as *const RSTGT_W) }
  }
  #[doc = "0x42 - Reset Gate Write"]
  #[inline(always)]
  pub fn rstgt_w_mut(&self) -> &mut RSTGT_W {
    unsafe { &mut *(((self as *const Self) as *mut u8).add(66usize) as *mut RSTGT_W) }
  }
  #[doc = "0x42 - Reset Gate Read"]
  #[inline(always)]
  pub fn rstgt_r(&self) -> &RSTGT_R {
    unsafe { &*(((self as *const Self) as *const u8).add(66usize) as *const RSTGT_R) }
  }
  #[doc = "0x42 - Reset Gate Read"]
  #[inline(always)]
  pub fn rstgt_r_mut(&self) -> &mut RSTGT_R {
    unsafe { &mut *(((self as *const Self) as *mut u8).add(66usize) as *mut RSTGT_R) }
  }
}
#[doc = "Gate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gate](gate) module"]
pub type GATE = crate::Reg<u8, _GATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GATE;
#[doc = "`read()` method returns [gate::R](gate::R) reader structure"]
impl crate::Readable for GATE {}
#[doc = "`write(|w| ..)` method takes [gate::W](gate::W) writer structure"]
impl crate::Writable for GATE {}
#[doc = "Gate Register"]
pub mod gate;
#[doc = "Reset Gate Read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rstgt_r](rstgt_r) module"]
pub type RSTGT_R = crate::Reg<u16, _RSTGT_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTGT_R;
#[doc = "`read()` method returns [rstgt_r::R](rstgt_r::R) reader structure"]
impl crate::Readable for RSTGT_R {}
#[doc = "Reset Gate Read"]
pub mod rstgt_r;
#[doc = "Reset Gate Write\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rstgt_w](rstgt_w) module"]
pub type RSTGT_W = crate::Reg<u16, _RSTGT_W>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTGT_W;
#[doc = "`write(|w| ..)` method takes [rstgt_w::W](rstgt_w::W) writer structure"]
impl crate::Writable for RSTGT_W {}
#[doc = "Reset Gate Write"]
pub mod rstgt_w;
