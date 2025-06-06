#[doc = "Register `MACA3LR` reader"]
pub struct R(crate::R<MACA3LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA3LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA3LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA3LR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACA3LR` writer"]
pub struct W(crate::W<MACA3LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA3LR_SPEC>;
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
impl From<crate::W<MACA3LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA3LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MBCA3L` reader - MBCA3L"]
pub type MBCA3L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MBCA3L` writer - MBCA3L"]
pub type MBCA3L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACA3LR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MBCA3L"]
    #[inline(always)]
    pub fn mbca3l(&self) -> MBCA3L_R {
        MBCA3L_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MBCA3L"]
    #[inline(always)]
    pub fn mbca3l(&mut self) -> MBCA3L_W<0> {
        MBCA3L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC address 3 low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maca3lr](index.html) module"]
pub struct MACA3LR_SPEC;
impl crate::RegisterSpec for MACA3LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maca3lr::R](R) reader structure"]
impl crate::Readable for MACA3LR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maca3lr::W](W) writer structure"]
impl crate::Writable for MACA3LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACA3LR to value 0xffff_ffff"]
impl crate::Resettable for MACA3LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
