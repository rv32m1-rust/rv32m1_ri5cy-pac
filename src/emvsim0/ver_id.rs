#[doc = "Reader of register VER_ID"]
pub type R = crate::R<u32, super::VER_ID>;
#[doc = "Reader of field `VER`"]
pub type VER_R = crate::R<u32, u32>;
impl R {
  #[doc = "Bits 0:31 - Version ID of the module"]
  #[inline(always)]
  pub fn ver(&self) -> VER_R {
    VER_R::new((self.bits & 0xffff_ffff) as u32)
  }
}
