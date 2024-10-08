pub trait ITextCharacterFormat_Impl: Sized + windows_core::IUnknownImpl {
    fn AllCaps(&self) -> windows_core::Result<FormatEffect>;
    fn SetAllCaps(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn BackgroundColor(&self) -> windows_core::Result<super::Color>;
    fn SetBackgroundColor(&self, value: &super::Color) -> windows_core::Result<()>;
    fn Bold(&self) -> windows_core::Result<FormatEffect>;
    fn SetBold(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn FontStretch(&self) -> windows_core::Result<FontStretch>;
    fn SetFontStretch(&self, value: FontStretch) -> windows_core::Result<()>;
    fn FontStyle(&self) -> windows_core::Result<FontStyle>;
    fn SetFontStyle(&self, value: FontStyle) -> windows_core::Result<()>;
    fn ForegroundColor(&self) -> windows_core::Result<super::Color>;
    fn SetForegroundColor(&self, value: &super::Color) -> windows_core::Result<()>;
    fn Hidden(&self) -> windows_core::Result<FormatEffect>;
    fn SetHidden(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Italic(&self) -> windows_core::Result<FormatEffect>;
    fn SetItalic(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Kerning(&self) -> windows_core::Result<f32>;
    fn SetKerning(&self, value: f32) -> windows_core::Result<()>;
    fn LanguageTag(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetLanguageTag(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn LinkType(&self) -> windows_core::Result<LinkType>;
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Outline(&self) -> windows_core::Result<FormatEffect>;
    fn SetOutline(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Position(&self) -> windows_core::Result<f32>;
    fn SetPosition(&self, value: f32) -> windows_core::Result<()>;
    fn ProtectedText(&self) -> windows_core::Result<FormatEffect>;
    fn SetProtectedText(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<f32>;
    fn SetSize(&self, value: f32) -> windows_core::Result<()>;
    fn SmallCaps(&self) -> windows_core::Result<FormatEffect>;
    fn SetSmallCaps(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Spacing(&self) -> windows_core::Result<f32>;
    fn SetSpacing(&self, value: f32) -> windows_core::Result<()>;
    fn Strikethrough(&self) -> windows_core::Result<FormatEffect>;
    fn SetStrikethrough(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Subscript(&self) -> windows_core::Result<FormatEffect>;
    fn SetSubscript(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Superscript(&self) -> windows_core::Result<FormatEffect>;
    fn SetSuperscript(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn TextScript(&self) -> windows_core::Result<TextScript>;
    fn SetTextScript(&self, value: TextScript) -> windows_core::Result<()>;
    fn Underline(&self) -> windows_core::Result<UnderlineType>;
    fn SetUnderline(&self, value: UnderlineType) -> windows_core::Result<()>;
    fn Weight(&self) -> windows_core::Result<i32>;
    fn SetWeight(&self, value: i32) -> windows_core::Result<()>;
    fn SetClone(&self, value: Option<&ITextCharacterFormat>) -> windows_core::Result<()>;
    fn GetClone(&self) -> windows_core::Result<ITextCharacterFormat>;
    fn IsEqual(&self, format: Option<&ITextCharacterFormat>) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for ITextCharacterFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextCharacterFormat";
}
impl ITextCharacterFormat_Vtbl {
    pub const fn new<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>() -> ITextCharacterFormat_Vtbl {
        unsafe extern "system" fn AllCaps<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::AllCaps(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllCaps<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetAllCaps(this, value).into()
        }
        unsafe extern "system" fn BackgroundColor<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::Color) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::BackgroundColor(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Color) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetBackgroundColor(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Bold<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Bold(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBold<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetBold(this, value).into()
        }
        unsafe extern "system" fn FontStretch<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FontStretch) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::FontStretch(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStretch<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FontStretch) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetFontStretch(this, value).into()
        }
        unsafe extern "system" fn FontStyle<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FontStyle) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::FontStyle(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFontStyle<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FontStyle) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetFontStyle(this, value).into()
        }
        unsafe extern "system" fn ForegroundColor<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::Color) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::ForegroundColor(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Color) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetForegroundColor(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Hidden<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Hidden(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHidden<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetHidden(this, value).into()
        }
        unsafe extern "system" fn Italic<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Italic(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItalic<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetItalic(this, value).into()
        }
        unsafe extern "system" fn Kerning<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Kerning(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKerning<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetKerning(this, value).into()
        }
        unsafe extern "system" fn LanguageTag<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::LanguageTag(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguageTag<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetLanguageTag(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn LinkType<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LinkType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::LinkType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Name(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Outline<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Outline(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutline<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetOutline(this, value).into()
        }
        unsafe extern "system" fn Position<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Position(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetPosition(this, value).into()
        }
        unsafe extern "system" fn ProtectedText<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::ProtectedText(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtectedText<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetProtectedText(this, value).into()
        }
        unsafe extern "system" fn Size<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetSize(this, value).into()
        }
        unsafe extern "system" fn SmallCaps<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::SmallCaps(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallCaps<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetSmallCaps(this, value).into()
        }
        unsafe extern "system" fn Spacing<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Spacing(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpacing<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetSpacing(this, value).into()
        }
        unsafe extern "system" fn Strikethrough<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Strikethrough(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetStrikethrough(this, value).into()
        }
        unsafe extern "system" fn Subscript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Subscript(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetSubscript(this, value).into()
        }
        unsafe extern "system" fn Superscript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Superscript(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuperscript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetSuperscript(this, value).into()
        }
        unsafe extern "system" fn TextScript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut TextScript) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::TextScript(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextScript<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: TextScript) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetTextScript(this, value).into()
        }
        unsafe extern "system" fn Underline<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UnderlineType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Underline(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnderline<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: UnderlineType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetUnderline(this, value).into()
        }
        unsafe extern "system" fn Weight<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::Weight(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetWeight(this, value).into()
        }
        unsafe extern "system" fn SetClone<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextCharacterFormat_Impl::SetClone(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn GetClone<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::GetClone(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextCharacterFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextCharacterFormat_Impl::IsEqual(this, windows_core::from_raw_borrowed(&format)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextCharacterFormat, OFFSET>(),
            AllCaps: AllCaps::<Identity, OFFSET>,
            SetAllCaps: SetAllCaps::<Identity, OFFSET>,
            BackgroundColor: BackgroundColor::<Identity, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, OFFSET>,
            Bold: Bold::<Identity, OFFSET>,
            SetBold: SetBold::<Identity, OFFSET>,
            FontStretch: FontStretch::<Identity, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, OFFSET>,
            FontStyle: FontStyle::<Identity, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, OFFSET>,
            ForegroundColor: ForegroundColor::<Identity, OFFSET>,
            SetForegroundColor: SetForegroundColor::<Identity, OFFSET>,
            Hidden: Hidden::<Identity, OFFSET>,
            SetHidden: SetHidden::<Identity, OFFSET>,
            Italic: Italic::<Identity, OFFSET>,
            SetItalic: SetItalic::<Identity, OFFSET>,
            Kerning: Kerning::<Identity, OFFSET>,
            SetKerning: SetKerning::<Identity, OFFSET>,
            LanguageTag: LanguageTag::<Identity, OFFSET>,
            SetLanguageTag: SetLanguageTag::<Identity, OFFSET>,
            LinkType: LinkType::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Outline: Outline::<Identity, OFFSET>,
            SetOutline: SetOutline::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            ProtectedText: ProtectedText::<Identity, OFFSET>,
            SetProtectedText: SetProtectedText::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            SmallCaps: SmallCaps::<Identity, OFFSET>,
            SetSmallCaps: SetSmallCaps::<Identity, OFFSET>,
            Spacing: Spacing::<Identity, OFFSET>,
            SetSpacing: SetSpacing::<Identity, OFFSET>,
            Strikethrough: Strikethrough::<Identity, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, OFFSET>,
            Subscript: Subscript::<Identity, OFFSET>,
            SetSubscript: SetSubscript::<Identity, OFFSET>,
            Superscript: Superscript::<Identity, OFFSET>,
            SetSuperscript: SetSuperscript::<Identity, OFFSET>,
            TextScript: TextScript::<Identity, OFFSET>,
            SetTextScript: SetTextScript::<Identity, OFFSET>,
            Underline: Underline::<Identity, OFFSET>,
            SetUnderline: SetUnderline::<Identity, OFFSET>,
            Weight: Weight::<Identity, OFFSET>,
            SetWeight: SetWeight::<Identity, OFFSET>,
            SetClone: SetClone::<Identity, OFFSET>,
            GetClone: GetClone::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextCharacterFormat as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Streams")]
pub trait ITextDocument_Impl: Sized + windows_core::IUnknownImpl {
    fn CaretType(&self) -> windows_core::Result<CaretType>;
    fn SetCaretType(&self, value: CaretType) -> windows_core::Result<()>;
    fn DefaultTabStop(&self) -> windows_core::Result<f32>;
    fn SetDefaultTabStop(&self, value: f32) -> windows_core::Result<()>;
    fn Selection(&self) -> windows_core::Result<ITextSelection>;
    fn UndoLimit(&self) -> windows_core::Result<u32>;
    fn SetUndoLimit(&self, value: u32) -> windows_core::Result<()>;
    fn CanCopy(&self) -> windows_core::Result<bool>;
    fn CanPaste(&self) -> windows_core::Result<bool>;
    fn CanRedo(&self) -> windows_core::Result<bool>;
    fn CanUndo(&self) -> windows_core::Result<bool>;
    fn ApplyDisplayUpdates(&self) -> windows_core::Result<i32>;
    fn BatchDisplayUpdates(&self) -> windows_core::Result<i32>;
    fn BeginUndoGroup(&self) -> windows_core::Result<()>;
    fn EndUndoGroup(&self) -> windows_core::Result<()>;
    fn GetDefaultCharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat>;
    fn GetDefaultParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat>;
    fn GetRange(&self, startposition: i32, endposition: i32) -> windows_core::Result<ITextRange>;
    fn GetRangeFromPoint(&self, point: &super::super::Foundation::Point, options: PointOptions) -> windows_core::Result<ITextRange>;
    fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()>;
    fn LoadFromStream(&self, options: TextSetOptions, value: Option<&super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn Redo(&self) -> windows_core::Result<()>;
    fn SaveToStream(&self, options: TextGetOptions, value: Option<&super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn SetDefaultCharacterFormat(&self, value: Option<&ITextCharacterFormat>) -> windows_core::Result<()>;
    fn SetDefaultParagraphFormat(&self, value: Option<&ITextParagraphFormat>) -> windows_core::Result<()>;
    fn SetText(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Undo(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ITextDocument {
    const NAME: &'static str = "Windows.UI.Text.ITextDocument";
}
#[cfg(feature = "Storage_Streams")]
impl ITextDocument_Vtbl {
    pub const fn new<Identity: ITextDocument_Impl, const OFFSET: isize>() -> ITextDocument_Vtbl {
        unsafe extern "system" fn CaretType<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CaretType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::CaretType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaretType<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: CaretType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::SetCaretType(this, value).into()
        }
        unsafe extern "system" fn DefaultTabStop<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::DefaultTabStop(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::SetDefaultTabStop(this, value).into()
        }
        unsafe extern "system" fn Selection<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::Selection(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UndoLimit<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::UndoLimit(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUndoLimit<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::SetUndoLimit(this, value).into()
        }
        unsafe extern "system" fn CanCopy<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::CanCopy(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanPaste<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::CanPaste(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRedo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::CanRedo(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanUndo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::CanUndo(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyDisplayUpdates<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::ApplyDisplayUpdates(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatchDisplayUpdates<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::BatchDisplayUpdates(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUndoGroup<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::BeginUndoGroup(this).into()
        }
        unsafe extern "system" fn EndUndoGroup<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::EndUndoGroup(this).into()
        }
        unsafe extern "system" fn GetDefaultCharacterFormat<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::GetDefaultCharacterFormat(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultParagraphFormat<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::GetDefaultParagraphFormat(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRange<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startposition: i32, endposition: i32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::GetRange(this, startposition, endposition) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRangeFromPoint<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextDocument_Impl::GetRangeFromPoint(this, core::mem::transmute(&point), options) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextGetOptions, value: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::GetText(this, options, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn LoadFromStream<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextSetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::LoadFromStream(this, options, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Redo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::Redo(this).into()
        }
        unsafe extern "system" fn SaveToStream<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextGetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::SaveToStream(this, options, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetDefaultCharacterFormat<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::SetDefaultCharacterFormat(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetDefaultParagraphFormat<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::SetDefaultParagraphFormat(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetText<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextSetOptions, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::SetText(this, options, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Undo<Identity: ITextDocument_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextDocument_Impl::Undo(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextDocument, OFFSET>(),
            CaretType: CaretType::<Identity, OFFSET>,
            SetCaretType: SetCaretType::<Identity, OFFSET>,
            DefaultTabStop: DefaultTabStop::<Identity, OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Identity, OFFSET>,
            Selection: Selection::<Identity, OFFSET>,
            UndoLimit: UndoLimit::<Identity, OFFSET>,
            SetUndoLimit: SetUndoLimit::<Identity, OFFSET>,
            CanCopy: CanCopy::<Identity, OFFSET>,
            CanPaste: CanPaste::<Identity, OFFSET>,
            CanRedo: CanRedo::<Identity, OFFSET>,
            CanUndo: CanUndo::<Identity, OFFSET>,
            ApplyDisplayUpdates: ApplyDisplayUpdates::<Identity, OFFSET>,
            BatchDisplayUpdates: BatchDisplayUpdates::<Identity, OFFSET>,
            BeginUndoGroup: BeginUndoGroup::<Identity, OFFSET>,
            EndUndoGroup: EndUndoGroup::<Identity, OFFSET>,
            GetDefaultCharacterFormat: GetDefaultCharacterFormat::<Identity, OFFSET>,
            GetDefaultParagraphFormat: GetDefaultParagraphFormat::<Identity, OFFSET>,
            GetRange: GetRange::<Identity, OFFSET>,
            GetRangeFromPoint: GetRangeFromPoint::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            LoadFromStream: LoadFromStream::<Identity, OFFSET>,
            Redo: Redo::<Identity, OFFSET>,
            SaveToStream: SaveToStream::<Identity, OFFSET>,
            SetDefaultCharacterFormat: SetDefaultCharacterFormat::<Identity, OFFSET>,
            SetDefaultParagraphFormat: SetDefaultParagraphFormat::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            Undo: Undo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextDocument as windows_core::Interface>::IID
    }
}
pub trait ITextParagraphFormat_Impl: Sized + windows_core::IUnknownImpl {
    fn Alignment(&self) -> windows_core::Result<ParagraphAlignment>;
    fn SetAlignment(&self, value: ParagraphAlignment) -> windows_core::Result<()>;
    fn FirstLineIndent(&self) -> windows_core::Result<f32>;
    fn KeepTogether(&self) -> windows_core::Result<FormatEffect>;
    fn SetKeepTogether(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn KeepWithNext(&self) -> windows_core::Result<FormatEffect>;
    fn SetKeepWithNext(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn LeftIndent(&self) -> windows_core::Result<f32>;
    fn LineSpacing(&self) -> windows_core::Result<f32>;
    fn LineSpacingRule(&self) -> windows_core::Result<LineSpacingRule>;
    fn ListAlignment(&self) -> windows_core::Result<MarkerAlignment>;
    fn SetListAlignment(&self, value: MarkerAlignment) -> windows_core::Result<()>;
    fn ListLevelIndex(&self) -> windows_core::Result<i32>;
    fn SetListLevelIndex(&self, value: i32) -> windows_core::Result<()>;
    fn ListStart(&self) -> windows_core::Result<i32>;
    fn SetListStart(&self, value: i32) -> windows_core::Result<()>;
    fn ListStyle(&self) -> windows_core::Result<MarkerStyle>;
    fn SetListStyle(&self, value: MarkerStyle) -> windows_core::Result<()>;
    fn ListTab(&self) -> windows_core::Result<f32>;
    fn SetListTab(&self, value: f32) -> windows_core::Result<()>;
    fn ListType(&self) -> windows_core::Result<MarkerType>;
    fn SetListType(&self, value: MarkerType) -> windows_core::Result<()>;
    fn NoLineNumber(&self) -> windows_core::Result<FormatEffect>;
    fn SetNoLineNumber(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn PageBreakBefore(&self) -> windows_core::Result<FormatEffect>;
    fn SetPageBreakBefore(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn RightIndent(&self) -> windows_core::Result<f32>;
    fn SetRightIndent(&self, value: f32) -> windows_core::Result<()>;
    fn RightToLeft(&self) -> windows_core::Result<FormatEffect>;
    fn SetRightToLeft(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn Style(&self) -> windows_core::Result<ParagraphStyle>;
    fn SetStyle(&self, value: ParagraphStyle) -> windows_core::Result<()>;
    fn SpaceAfter(&self) -> windows_core::Result<f32>;
    fn SetSpaceAfter(&self, value: f32) -> windows_core::Result<()>;
    fn SpaceBefore(&self) -> windows_core::Result<f32>;
    fn SetSpaceBefore(&self, value: f32) -> windows_core::Result<()>;
    fn WidowControl(&self) -> windows_core::Result<FormatEffect>;
    fn SetWidowControl(&self, value: FormatEffect) -> windows_core::Result<()>;
    fn TabCount(&self) -> windows_core::Result<i32>;
    fn AddTab(&self, position: f32, align: TabAlignment, leader: TabLeader) -> windows_core::Result<()>;
    fn ClearAllTabs(&self) -> windows_core::Result<()>;
    fn DeleteTab(&self, position: f32) -> windows_core::Result<()>;
    fn GetClone(&self) -> windows_core::Result<ITextParagraphFormat>;
    fn GetTab(&self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> windows_core::Result<()>;
    fn IsEqual(&self, format: Option<&ITextParagraphFormat>) -> windows_core::Result<bool>;
    fn SetClone(&self, format: Option<&ITextParagraphFormat>) -> windows_core::Result<()>;
    fn SetIndents(&self, start: f32, left: f32, right: f32) -> windows_core::Result<()>;
    fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ITextParagraphFormat {
    const NAME: &'static str = "Windows.UI.Text.ITextParagraphFormat";
}
impl ITextParagraphFormat_Vtbl {
    pub const fn new<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>() -> ITextParagraphFormat_Vtbl {
        unsafe extern "system" fn Alignment<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ParagraphAlignment) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::Alignment(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlignment<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ParagraphAlignment) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetAlignment(this, value).into()
        }
        unsafe extern "system" fn FirstLineIndent<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::FirstLineIndent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeepTogether<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::KeepTogether(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetKeepTogether(this, value).into()
        }
        unsafe extern "system" fn KeepWithNext<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::KeepWithNext(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetKeepWithNext(this, value).into()
        }
        unsafe extern "system" fn LeftIndent<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::LeftIndent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineSpacing<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::LineSpacing(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LineSpacingRule<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LineSpacingRule) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::LineSpacingRule(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListAlignment<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MarkerAlignment) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::ListAlignment(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListAlignment<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: MarkerAlignment) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetListAlignment(this, value).into()
        }
        unsafe extern "system" fn ListLevelIndex<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::ListLevelIndex(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetListLevelIndex(this, value).into()
        }
        unsafe extern "system" fn ListStart<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::ListStart(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStart<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetListStart(this, value).into()
        }
        unsafe extern "system" fn ListStyle<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MarkerStyle) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::ListStyle(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListStyle<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: MarkerStyle) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetListStyle(this, value).into()
        }
        unsafe extern "system" fn ListTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::ListTab(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetListTab(this, value).into()
        }
        unsafe extern "system" fn ListType<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MarkerType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::ListType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListType<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: MarkerType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetListType(this, value).into()
        }
        unsafe extern "system" fn NoLineNumber<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::NoLineNumber(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetNoLineNumber(this, value).into()
        }
        unsafe extern "system" fn PageBreakBefore<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::PageBreakBefore(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetPageBreakBefore(this, value).into()
        }
        unsafe extern "system" fn RightIndent<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::RightIndent(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightIndent<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetRightIndent(this, value).into()
        }
        unsafe extern "system" fn RightToLeft<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::RightToLeft(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRightToLeft<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetRightToLeft(this, value).into()
        }
        unsafe extern "system" fn Style<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ParagraphStyle) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::Style(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ParagraphStyle) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetStyle(this, value).into()
        }
        unsafe extern "system" fn SpaceAfter<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::SpaceAfter(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetSpaceAfter(this, value).into()
        }
        unsafe extern "system" fn SpaceBefore<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::SpaceBefore(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetSpaceBefore(this, value).into()
        }
        unsafe extern "system" fn WidowControl<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::WidowControl(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidowControl<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FormatEffect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetWidowControl(this, value).into()
        }
        unsafe extern "system" fn TabCount<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::TabCount(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: f32, align: TabAlignment, leader: TabLeader) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::AddTab(this, position, align, leader).into()
        }
        unsafe extern "system" fn ClearAllTabs<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::ClearAllTabs(this).into()
        }
        unsafe extern "system" fn DeleteTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::DeleteTab(this, position).into()
        }
        unsafe extern "system" fn GetClone<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::GetClone(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTab<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::GetTab(this, index, core::mem::transmute_copy(&position), core::mem::transmute_copy(&align), core::mem::transmute_copy(&leader)).into()
        }
        unsafe extern "system" fn IsEqual<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextParagraphFormat_Impl::IsEqual(this, windows_core::from_raw_borrowed(&format)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClone<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetClone(this, windows_core::from_raw_borrowed(&format)).into()
        }
        unsafe extern "system" fn SetIndents<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: f32, left: f32, right: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetIndents(this, start, left, right).into()
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ITextParagraphFormat_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rule: LineSpacingRule, spacing: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextParagraphFormat_Impl::SetLineSpacing(this, rule, spacing).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextParagraphFormat, OFFSET>(),
            Alignment: Alignment::<Identity, OFFSET>,
            SetAlignment: SetAlignment::<Identity, OFFSET>,
            FirstLineIndent: FirstLineIndent::<Identity, OFFSET>,
            KeepTogether: KeepTogether::<Identity, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, OFFSET>,
            KeepWithNext: KeepWithNext::<Identity, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, OFFSET>,
            LeftIndent: LeftIndent::<Identity, OFFSET>,
            LineSpacing: LineSpacing::<Identity, OFFSET>,
            LineSpacingRule: LineSpacingRule::<Identity, OFFSET>,
            ListAlignment: ListAlignment::<Identity, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, OFFSET>,
            ListLevelIndex: ListLevelIndex::<Identity, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, OFFSET>,
            ListStart: ListStart::<Identity, OFFSET>,
            SetListStart: SetListStart::<Identity, OFFSET>,
            ListStyle: ListStyle::<Identity, OFFSET>,
            SetListStyle: SetListStyle::<Identity, OFFSET>,
            ListTab: ListTab::<Identity, OFFSET>,
            SetListTab: SetListTab::<Identity, OFFSET>,
            ListType: ListType::<Identity, OFFSET>,
            SetListType: SetListType::<Identity, OFFSET>,
            NoLineNumber: NoLineNumber::<Identity, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, OFFSET>,
            PageBreakBefore: PageBreakBefore::<Identity, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, OFFSET>,
            RightIndent: RightIndent::<Identity, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, OFFSET>,
            RightToLeft: RightToLeft::<Identity, OFFSET>,
            SetRightToLeft: SetRightToLeft::<Identity, OFFSET>,
            Style: Style::<Identity, OFFSET>,
            SetStyle: SetStyle::<Identity, OFFSET>,
            SpaceAfter: SpaceAfter::<Identity, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, OFFSET>,
            SpaceBefore: SpaceBefore::<Identity, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, OFFSET>,
            WidowControl: WidowControl::<Identity, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, OFFSET>,
            TabCount: TabCount::<Identity, OFFSET>,
            AddTab: AddTab::<Identity, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, OFFSET>,
            DeleteTab: DeleteTab::<Identity, OFFSET>,
            GetClone: GetClone::<Identity, OFFSET>,
            GetTab: GetTab::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            SetClone: SetClone::<Identity, OFFSET>,
            SetIndents: SetIndents::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextParagraphFormat as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Streams")]
pub trait ITextRange_Impl: Sized + windows_core::IUnknownImpl {
    fn Character(&self) -> windows_core::Result<u16>;
    fn SetCharacter(&self, value: u16) -> windows_core::Result<()>;
    fn CharacterFormat(&self) -> windows_core::Result<ITextCharacterFormat>;
    fn SetCharacterFormat(&self, value: Option<&ITextCharacterFormat>) -> windows_core::Result<()>;
    fn FormattedText(&self) -> windows_core::Result<ITextRange>;
    fn SetFormattedText(&self, value: Option<&ITextRange>) -> windows_core::Result<()>;
    fn EndPosition(&self) -> windows_core::Result<i32>;
    fn SetEndPosition(&self, value: i32) -> windows_core::Result<()>;
    fn Gravity(&self) -> windows_core::Result<RangeGravity>;
    fn SetGravity(&self, value: RangeGravity) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn Link(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetLink(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn ParagraphFormat(&self) -> windows_core::Result<ITextParagraphFormat>;
    fn SetParagraphFormat(&self, value: Option<&ITextParagraphFormat>) -> windows_core::Result<()>;
    fn StartPosition(&self) -> windows_core::Result<i32>;
    fn SetStartPosition(&self, value: i32) -> windows_core::Result<()>;
    fn StoryLength(&self) -> windows_core::Result<i32>;
    fn Text(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn CanPaste(&self, format: i32) -> windows_core::Result<bool>;
    fn ChangeCase(&self, value: LetterCase) -> windows_core::Result<()>;
    fn Collapse(&self, value: bool) -> windows_core::Result<()>;
    fn Copy(&self) -> windows_core::Result<()>;
    fn Cut(&self) -> windows_core::Result<()>;
    fn Delete(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32>;
    fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32>;
    fn Expand(&self, unit: TextRangeUnit) -> windows_core::Result<i32>;
    fn FindText(&self, value: &windows_core::HSTRING, scanlength: i32, options: FindOptions) -> windows_core::Result<i32>;
    fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> windows_core::Result<()>;
    fn GetClone(&self) -> windows_core::Result<ITextRange>;
    fn GetIndex(&self, unit: TextRangeUnit) -> windows_core::Result<i32>;
    fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> windows_core::Result<()>;
    fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> windows_core::Result<()>;
    fn GetText(&self, options: TextGetOptions, value: &mut windows_core::HSTRING) -> windows_core::Result<()>;
    fn GetTextViaStream(&self, options: TextGetOptions, value: Option<&super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn InRange(&self, range: Option<&ITextRange>) -> windows_core::Result<bool>;
    fn InsertImage(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &windows_core::HSTRING, value: Option<&super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn InStory(&self, range: Option<&ITextRange>) -> windows_core::Result<bool>;
    fn IsEqual(&self, range: Option<&ITextRange>) -> windows_core::Result<bool>;
    fn Move(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> windows_core::Result<i32>;
    fn Paste(&self, format: i32) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, value: PointOptions) -> windows_core::Result<()>;
    fn MatchSelection(&self) -> windows_core::Result<()>;
    fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> windows_core::Result<()>;
    fn SetPoint(&self, point: &super::super::Foundation::Point, options: PointOptions, extend: bool) -> windows_core::Result<()>;
    fn SetRange(&self, startposition: i32, endposition: i32) -> windows_core::Result<()>;
    fn SetText2(&self, options: TextSetOptions, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn SetTextViaStream(&self, options: TextSetOptions, value: Option<&super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ITextRange {
    const NAME: &'static str = "Windows.UI.Text.ITextRange";
}
#[cfg(feature = "Storage_Streams")]
impl ITextRange_Vtbl {
    pub const fn new<Identity: ITextRange_Impl, const OFFSET: isize>() -> ITextRange_Vtbl {
        unsafe extern "system" fn Character<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::Character(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacter<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetCharacter(this, value).into()
        }
        unsafe extern "system" fn CharacterFormat<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::CharacterFormat(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCharacterFormat<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetCharacterFormat(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn FormattedText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::FormattedText(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormattedText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetFormattedText(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn EndPosition<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::EndPosition(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndPosition<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetEndPosition(this, value).into()
        }
        unsafe extern "system" fn Gravity<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut RangeGravity) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::Gravity(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGravity<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: RangeGravity) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetGravity(this, value).into()
        }
        unsafe extern "system" fn Length<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::Length(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Link<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::Link(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLink<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetLink(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ParagraphFormat<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::ParagraphFormat(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParagraphFormat<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetParagraphFormat(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn StartPosition<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::StartPosition(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartPosition<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetStartPosition(this, value).into()
        }
        unsafe extern "system" fn StoryLength<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::StoryLength(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::Text(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetText(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CanPaste<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: i32, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::CanPaste(this, format) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangeCase<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: LetterCase) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::ChangeCase(this, value).into()
        }
        unsafe extern "system" fn Collapse<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::Collapse(this, value).into()
        }
        unsafe extern "system" fn Copy<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::Copy(this).into()
        }
        unsafe extern "system" fn Cut<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::Cut(this).into()
        }
        unsafe extern "system" fn Delete<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::Delete(this, unit, count) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndOf<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::EndOf(this, unit, extend) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Expand<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::Expand(this, unit) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>, scanlength: i32, options: FindOptions, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::FindText(this, core::mem::transmute(&value), scanlength, options) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacterUtf32<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32, offset: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::GetCharacterUtf32(this, core::mem::transmute_copy(&value), offset).into()
        }
        unsafe extern "system" fn GetClone<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::GetClone(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIndex<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::GetIndex(this, unit) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoint<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::GetPoint(this, horizontalalign, verticalalign, options, core::mem::transmute_copy(&point)).into()
        }
        unsafe extern "system" fn GetRect<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::GetRect(this, options, core::mem::transmute_copy(&rect), core::mem::transmute_copy(&hit)).into()
        }
        unsafe extern "system" fn GetText<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextGetOptions, value: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::GetText(this, options, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetTextViaStream<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextGetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::GetTextViaStream(this, options, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn InRange<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::InRange(this, windows_core::from_raw_borrowed(&range)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertImage<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: core::mem::MaybeUninit<windows_core::HSTRING>, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::InsertImage(this, width, height, ascent, verticalalign, core::mem::transmute(&alternatetext), windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn InStory<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::InStory(this, windows_core::from_raw_borrowed(&range)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::IsEqual(this, windows_core::from_raw_borrowed(&range)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Move<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::Move(this, unit, count) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveEnd<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::MoveEnd(this, unit, count) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveStart<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::MoveStart(this, unit, count) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Paste<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::Paste(this, format).into()
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PointOptions) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::ScrollIntoView(this, value).into()
        }
        unsafe extern "system" fn MatchSelection<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::MatchSelection(this).into()
        }
        unsafe extern "system" fn SetIndex<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, index: i32, extend: bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetIndex(this, unit, index, extend).into()
        }
        unsafe extern "system" fn SetPoint<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetPoint(this, core::mem::transmute(&point), options, extend).into()
        }
        unsafe extern "system" fn SetRange<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, startposition: i32, endposition: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetRange(this, startposition, endposition).into()
        }
        unsafe extern "system" fn SetText2<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextSetOptions, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetText2(this, options, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetTextViaStream<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: TextSetOptions, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextRange_Impl::SetTextViaStream(this, options, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn StartOf<Identity: ITextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextRange_Impl::StartOf(this, unit, extend) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextRange, OFFSET>(),
            Character: Character::<Identity, OFFSET>,
            SetCharacter: SetCharacter::<Identity, OFFSET>,
            CharacterFormat: CharacterFormat::<Identity, OFFSET>,
            SetCharacterFormat: SetCharacterFormat::<Identity, OFFSET>,
            FormattedText: FormattedText::<Identity, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, OFFSET>,
            EndPosition: EndPosition::<Identity, OFFSET>,
            SetEndPosition: SetEndPosition::<Identity, OFFSET>,
            Gravity: Gravity::<Identity, OFFSET>,
            SetGravity: SetGravity::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            SetLink: SetLink::<Identity, OFFSET>,
            ParagraphFormat: ParagraphFormat::<Identity, OFFSET>,
            SetParagraphFormat: SetParagraphFormat::<Identity, OFFSET>,
            StartPosition: StartPosition::<Identity, OFFSET>,
            SetStartPosition: SetStartPosition::<Identity, OFFSET>,
            StoryLength: StoryLength::<Identity, OFFSET>,
            Text: Text::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            CanPaste: CanPaste::<Identity, OFFSET>,
            ChangeCase: ChangeCase::<Identity, OFFSET>,
            Collapse: Collapse::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Cut: Cut::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            EndOf: EndOf::<Identity, OFFSET>,
            Expand: Expand::<Identity, OFFSET>,
            FindText: FindText::<Identity, OFFSET>,
            GetCharacterUtf32: GetCharacterUtf32::<Identity, OFFSET>,
            GetClone: GetClone::<Identity, OFFSET>,
            GetIndex: GetIndex::<Identity, OFFSET>,
            GetPoint: GetPoint::<Identity, OFFSET>,
            GetRect: GetRect::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetTextViaStream: GetTextViaStream::<Identity, OFFSET>,
            InRange: InRange::<Identity, OFFSET>,
            InsertImage: InsertImage::<Identity, OFFSET>,
            InStory: InStory::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            MoveEnd: MoveEnd::<Identity, OFFSET>,
            MoveStart: MoveStart::<Identity, OFFSET>,
            Paste: Paste::<Identity, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, OFFSET>,
            MatchSelection: MatchSelection::<Identity, OFFSET>,
            SetIndex: SetIndex::<Identity, OFFSET>,
            SetPoint: SetPoint::<Identity, OFFSET>,
            SetRange: SetRange::<Identity, OFFSET>,
            SetText2: SetText2::<Identity, OFFSET>,
            SetTextViaStream: SetTextViaStream::<Identity, OFFSET>,
            StartOf: StartOf::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRange as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Storage_Streams")]
pub trait ITextSelection_Impl: Sized + windows_core::IUnknownImpl + ITextRange_Impl {
    fn Options(&self) -> windows_core::Result<SelectionOptions>;
    fn SetOptions(&self, value: SelectionOptions) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<SelectionType>;
    fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32>;
    fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> windows_core::Result<i32>;
    fn MoveDown(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32>;
    fn MoveLeft(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32>;
    fn MoveRight(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32>;
    fn MoveUp(&self, unit: TextRangeUnit, count: i32, extend: bool) -> windows_core::Result<i32>;
    fn TypeText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ITextSelection {
    const NAME: &'static str = "Windows.UI.Text.ITextSelection";
}
#[cfg(feature = "Storage_Streams")]
impl ITextSelection_Vtbl {
    pub const fn new<Identity: ITextSelection_Impl, const OFFSET: isize>() -> ITextSelection_Vtbl {
        unsafe extern "system" fn Options<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SelectionOptions) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextSelection_Impl::Options(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOptions<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SelectionOptions) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextSelection_Impl::SetOptions(this, value).into()
        }
        unsafe extern "system" fn Type<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SelectionType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextSelection_Impl::Type(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndKey<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextSelection_Impl::EndKey(this, unit, extend) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HomeKey<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextSelection_Impl::HomeKey(this, unit, extend) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveDown<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextSelection_Impl::MoveDown(this, unit, count, extend) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveLeft<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextSelection_Impl::MoveLeft(this, unit, count, extend) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveRight<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextSelection_Impl::MoveRight(this, unit, count, extend) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveUp<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ITextSelection_Impl::MoveUp(this, unit, count, extend) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TypeText<Identity: ITextSelection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ITextSelection_Impl::TypeText(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextSelection, OFFSET>(),
            Options: Options::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            EndKey: EndKey::<Identity, OFFSET>,
            HomeKey: HomeKey::<Identity, OFFSET>,
            MoveDown: MoveDown::<Identity, OFFSET>,
            MoveLeft: MoveLeft::<Identity, OFFSET>,
            MoveRight: MoveRight::<Identity, OFFSET>,
            MoveUp: MoveUp::<Identity, OFFSET>,
            TypeText: TypeText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextSelection as windows_core::Interface>::IID
    }
}
