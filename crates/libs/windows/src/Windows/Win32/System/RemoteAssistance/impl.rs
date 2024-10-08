#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DRendezvousSessionEvents_Impl: Sized + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DRendezvousSessionEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DRendezvousSessionEvents_Vtbl {
    pub const fn new<Identity: DRendezvousSessionEvents_Impl, const OFFSET: isize>() -> DRendezvousSessionEvents_Vtbl {
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DRendezvousSessionEvents as windows_core::Interface>::IID || iid == &<super::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRendezvousApplication_Impl: Sized + windows_core::IUnknownImpl {
    fn SetRendezvousSession(&self, prendezvoussession: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRendezvousApplication {}
impl IRendezvousApplication_Vtbl {
    pub const fn new<Identity: IRendezvousApplication_Impl, const OFFSET: isize>() -> IRendezvousApplication_Vtbl {
        unsafe extern "system" fn SetRendezvousSession<Identity: IRendezvousApplication_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prendezvoussession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRendezvousApplication_Impl::SetRendezvousSession(this, windows_core::from_raw_borrowed(&prendezvoussession)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRendezvousSession: SetRendezvousSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRendezvousApplication as windows_core::Interface>::IID
    }
}
pub trait IRendezvousSession_Impl: Sized + windows_core::IUnknownImpl {
    fn State(&self) -> windows_core::Result<RENDEZVOUS_SESSION_STATE>;
    fn RemoteUser(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Flags(&self) -> windows_core::Result<i32>;
    fn SendContextData(&self, bstrdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Terminate(&self, hr: windows_core::HRESULT, bstrappdata: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRendezvousSession {}
impl IRendezvousSession_Vtbl {
    pub const fn new<Identity: IRendezvousSession_Impl, const OFFSET: isize>() -> IRendezvousSession_Vtbl {
        unsafe extern "system" fn State<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRendezvousSession_Impl::State(this) {
                Ok(ok__) => {
                    psessionstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteUser<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrusername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRendezvousSession_Impl::RemoteUser(this) {
                Ok(ok__) => {
                    bstrusername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flags<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRendezvousSession_Impl::Flags(this) {
                Ok(ok__) => {
                    pflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendContextData<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRendezvousSession_Impl::SendContextData(this, core::mem::transmute(&bstrdata)).into()
        }
        unsafe extern "system" fn Terminate<Identity: IRendezvousSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, bstrappdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRendezvousSession_Impl::Terminate(this, core::mem::transmute_copy(&hr), core::mem::transmute(&bstrappdata)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            State: State::<Identity, OFFSET>,
            RemoteUser: RemoteUser::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            SendContextData: SendContextData::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRendezvousSession as windows_core::Interface>::IID
    }
}
