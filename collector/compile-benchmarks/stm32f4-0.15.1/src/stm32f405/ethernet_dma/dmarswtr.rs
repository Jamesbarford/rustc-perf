#[doc = "Register `DMARSWTR` reader"]
pub struct R(crate::R<DMARSWTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMARSWTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMARSWTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMARSWTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMARSWTR` writer"]
pub struct W(crate::W<DMARSWTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMARSWTR_SPEC>;
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
impl From<crate::W<DMARSWTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMARSWTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSWTC` reader - RSWTC"]
pub type RSWTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSWTC` writer - RSWTC"]
pub type RSWTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMARSWTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - RSWTC"]
    #[inline(always)]
    pub fn rswtc(&self) -> RSWTC_R {
        RSWTC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RSWTC"]
    #[inline(always)]
    pub fn rswtc(&mut self) -> RSWTC_W<0> {
        RSWTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA receive status watchdog timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarswtr](index.html) module"]
pub struct DMARSWTR_SPEC;
impl crate::RegisterSpec for DMARSWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmarswtr::R](R) reader structure"]
impl crate::Readable for DMARSWTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmarswtr::W](W) writer structure"]
impl crate::Writable for DMARSWTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMARSWTR to value 0"]
impl crate::Resettable for DMARSWTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
