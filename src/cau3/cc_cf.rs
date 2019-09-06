#[doc = "Reader of register CC_CF"]
pub type R = crate::R<u32, super::CC_CF>;
#[doc = "Reader of field `C`"]
pub type C_R = crate::R<bool, bool>;
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, bool>;
#[doc = "Reader of field `Z`"]
pub type Z_R = crate::R<bool, bool>;
#[doc = "Reader of field `N`"]
pub type N_R = crate::R<bool, bool>;
impl R {
  #[doc = "Bit 0 - Carry flag"]
  #[inline(always)]
  pub fn c(&self) -> C_R {
    C_R::new((self.bits & 0x01) != 0)
  }
  #[doc = "Bit 1 - Overflow flag"]
  #[inline(always)]
  pub fn v(&self) -> V_R {
    V_R::new(((self.bits >> 1) & 0x01) != 0)
  }
  #[doc = "Bit 2 - Zero flag"]
  #[inline(always)]
  pub fn z(&self) -> Z_R {
    Z_R::new(((self.bits >> 2) & 0x01) != 0)
  }
  #[doc = "Bit 3 - Negative flag"]
  #[inline(always)]
  pub fn n(&self) -> N_R {
    N_R::new(((self.bits >> 3) & 0x01) != 0)
  }
}
