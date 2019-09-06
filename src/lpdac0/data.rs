#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
  type Type = u32;
  #[inline(always)]
  fn reset_value() -> Self::Type {
    0
  }
}
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
  w: &'a mut W,
}
impl<'a> DATA_W<'a> {
  #[doc = r"Writes raw bits to the field"]
  #[inline(always)]
  pub unsafe fn bits(self, value: u16) -> &'a mut W {
    self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
    self.w
  }
}
impl R {}
impl W {
  #[doc = "Bits 0:11 - In FIFO mode or swing back mode, this is the FIFO data entry. In buffer mode, write to this field will push the data to analog without trigger support. This field is write only and always read zero."]
  #[inline(always)]
  pub fn data(&mut self) -> DATA_W {
    DATA_W { w: self }
  }
}
