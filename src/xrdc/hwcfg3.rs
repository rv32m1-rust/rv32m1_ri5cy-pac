#[doc = "Reader of register HWCFG3"]
pub type R = crate::R<u32, super::HWCFG3>;
#[doc = "Reader of field `PIDPn`"]
pub type PIDPN_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:31 - Process identifier"]
  #[inline(always)]
  pub fn pidpn(&self) -> PIDPN_R {
    PIDPN_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
