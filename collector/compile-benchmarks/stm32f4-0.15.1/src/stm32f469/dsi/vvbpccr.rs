#[doc = "Register `VVBPCCR` reader"]
pub struct R(crate::R<VVBPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVBPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVBPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVBPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VVBPCCR` writer"]
pub struct W(crate::W<VVBPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVBPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<VVBPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVBPCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBP` reader - Vertical Back-Porch duration"]
pub type VBP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VBP` writer - Vertical Back-Porch duration"]
pub type VBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVBPCCR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Vertical Back-Porch duration"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Back-Porch duration"]
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W<0> {
        VBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video VBP Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvbpccr](index.html) module"]
pub struct VVBPCCR_SPEC;
impl crate::RegisterSpec for VVBPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vvbpccr::R](R) reader structure"]
impl crate::Readable for VVBPCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vvbpccr::W](W) writer structure"]
impl crate::Writable for VVBPCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VVBPCCR to value 0"]
impl crate::Resettable for VVBPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
