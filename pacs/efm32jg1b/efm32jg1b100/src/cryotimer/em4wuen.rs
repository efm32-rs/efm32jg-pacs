#[doc = "Register `EM4WUEN` reader"]
pub struct R(crate::R<EM4WUEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM4WUEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM4WUEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM4WUEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM4WUEN` writer"]
pub struct W(crate::W<EM4WUEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM4WUEN_SPEC>;
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
impl From<crate::W<EM4WUEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM4WUEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM4WU` reader - EM4 Wake-up Enable"]
pub type EM4WU_R = crate::BitReader<bool>;
#[doc = "Field `EM4WU` writer - EM4 Wake-up Enable"]
pub type EM4WU_W<'a> = crate::BitWriter<'a, u32, EM4WUEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - EM4 Wake-up Enable"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EM4 Wake-up Enable"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> EM4WU_W {
        EM4WU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake Up Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4wuen](index.html) module"]
pub struct EM4WUEN_SPEC;
impl crate::RegisterSpec for EM4WUEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em4wuen::R](R) reader structure"]
impl crate::Readable for EM4WUEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em4wuen::W](W) writer structure"]
impl crate::Writable for EM4WUEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for EM4WUEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
