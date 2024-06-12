#[cfg(feature = "bitdepth_16")]
use crate::include::common::bitdepth::BitDepth16;
#[cfg(feature = "bitdepth_8")]
use crate::include::common::bitdepth::BitDepth8;
use crate::include::common::validate::validate_input;
use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::common::Rav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
use crate::include::dav1d::data::Rav1dData;
use crate::include::dav1d::dav1d::Dav1dContext;
use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dSettings;
use crate::include::dav1d::dav1d::Rav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Rav1dInloopFilterType;
use crate::include::dav1d::dav1d::Rav1dSettings;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::Rav1dFilmGrainData;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::include::dav1d::picture::Rav1dPicture;
use crate::src::c_arc::RawArc;
use crate::src::c_box::FnFree;
use crate::src::cpu::rav1d_init_cpu;
use crate::src::cpu::rav1d_num_logical_processors;
use crate::src::decode::rav1d_decode_frame_exit;
use crate::src::error::Dav1dResult;
use crate::src::error::Rav1dError::EGeneric;
use crate::src::error::Rav1dError::EAGAIN;
use crate::src::error::Rav1dError::EINVAL;
use crate::src::error::Rav1dResult;
use crate::src::extensions::OptionError as _;
use crate::src::fg_apply;
use crate::src::internal::Rav1dBitDepthDSPContext;
use crate::src::internal::Rav1dContext;
use crate::src::internal::Rav1dContextTaskThread;
use crate::src::internal::Rav1dContextTaskType;
use crate::src::internal::Rav1dContext_frame_thread;
use crate::src::internal::Rav1dFrameContext;
use crate::src::internal::Rav1dState;
use crate::src::internal::Rav1dTaskContext;
use crate::src::internal::Rav1dTaskContext_task_thread;
use crate::src::internal::TaskThreadData;
use crate::src::iter::wrapping_iter;
use crate::src::log::Rav1dLog as _;
use crate::src::obu::rav1d_parse_obus;
use crate::src::obu::rav1d_parse_sequence_header;
use crate::src::picture::rav1d_picture_alloc_copy;
use crate::src::picture::PictureFlags;
use crate::src::picture::Rav1dThreadPicture;
use crate::src::thread_task::rav1d_task_delayed_fg;
use crate::src::thread_task::rav1d_worker_task;
use crate::src::thread_task::FRAME_ERROR;
use parking_lot::Mutex;
use std::cmp;
use std::ffi::c_char;
use std::ffi::c_uint;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem;
use std::process::abort;
use std::ptr;
use std::ptr::NonNull;
use std::slice;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Once;
use std::thread;
use to_method::To as _;

#[cold]
fn init_internal() {
    rav1d_init_cpu();
}

const DAV1D_VERSION: &CStr = c"966d63c1";
const RAV1D_VERSION: &str = match DAV1D_VERSION.to_str() {
    Ok(version) => version,
    Err(_) => unreachable!(),
};

pub const fn rav1d_version() -> &'static str {
    RAV1D_VERSION
}

#[no_mangle]
#[cold]
pub extern "C" fn dav1d_version() -> *const c_char {
    DAV1D_VERSION.as_ptr()
}

pub const DAV1D_API_VERSION_MAJOR: u8 = 7;
pub const DAV1D_API_VERSION_MINOR: u8 = 0;
pub const DAV1D_API_VERSION_PATCH: u8 = 0;

/// Get the `dav1d` library C API version.
///
/// Return a value in the format `0x00XXYYZZ`, where `XX` is the major version,
/// `YY` the minor version, and `ZZ` the patch version.
#[no_mangle]
#[cold]
pub extern "C" fn dav1d_version_api() -> c_uint {
    u32::from_be_bytes([
        0,
        DAV1D_API_VERSION_MAJOR,
        DAV1D_API_VERSION_MINOR,
        DAV1D_API_VERSION_PATCH,
    ])
}

impl Default for Rav1dSettings {
    fn default() -> Self {
        Self {
            n_threads: 0,
            max_frame_delay: 0,
            apply_grain: true,
            operating_point: 0,
            all_layers: true,
            frame_size_limit: 0,
            allocator: Default::default(),
            logger: Default::default(),
            strict_std_compliance: false,
            output_invisible_frames: false,
            inloop_filters: Rav1dInloopFilterType::all(),
            decode_frame_type: Rav1dDecodeFrameType::All,
        }
    }
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_default_settings(s: *mut Dav1dSettings) {
    s.write(Rav1dSettings::default().into());
}

struct NumThreads {
    n_tc: usize,
    n_fc: usize,
}

#[cold]
fn get_num_threads(s: &Rav1dSettings) -> NumThreads {
    let n_tc = if s.n_threads != 0 {
        s.n_threads as usize
    } else {
        rav1d_num_logical_processors().clamp(1, 256)
    };
    let n_fc = if s.max_frame_delay != 0 {
        cmp::min(s.max_frame_delay as usize, n_tc)
    } else {
        cmp::min((n_tc as f64).sqrt().ceil() as usize, 8)
    };
    NumThreads { n_fc, n_tc }
}

#[cold]
pub(crate) fn rav1d_get_frame_delay(s: &Rav1dSettings) -> Rav1dResult<usize> {
    validate_input!((s.n_threads >= 0 && s.n_threads <= 256, EINVAL))?;
    validate_input!((s.max_frame_delay >= 0 && s.max_frame_delay <= 256, EINVAL))?;
    let NumThreads { n_tc: _, n_fc } = get_num_threads(s);
    Ok(n_fc)
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_get_frame_delay(s: *const Dav1dSettings) -> Dav1dResult {
    (|| {
        validate_input!((!s.is_null(), EINVAL))?;
        rav1d_get_frame_delay(&s.read().try_into()?).map(|frame_delay| frame_delay as c_uint)
    })()
    .into()
}

#[cold]
pub(crate) fn rav1d_open(s: &Rav1dSettings) -> Rav1dResult<Arc<Rav1dContext>> {
    static initted: Once = Once::new();
    initted.call_once(|| init_internal());

    validate_input!((s.n_threads >= 0 && s.n_threads <= 256, EINVAL))?;
    validate_input!((s.max_frame_delay >= 0 && s.max_frame_delay <= 256, EINVAL))?;
    validate_input!((s.operating_point <= 31, EINVAL))?;
    validate_input!((
        !s.allocator.is_default() || s.allocator.cookie.is_null(),
        EINVAL
    ))?;

    // On 32-bit systems, extremely large frame sizes can cause overflows in
    // `rav1d_decode_frame` alloc size calculations. Prevent that from occuring
    // by enforcing a maximum frame size limit, chosen to roughly correspond to
    // the largest size possible to decode without exhausting virtual memory.
    let frame_size_limit;
    if mem::size_of::<usize>() < 8 && s.frame_size_limit.wrapping_sub(1) >= 8192 * 8192 {
        frame_size_limit = 8192 * 8192;
        if s.frame_size_limit != 0 {
            writeln!(
                s.logger,
                "Frame size limit reduced from {} to {}.",
                s.frame_size_limit, frame_size_limit,
            );
        }
    } else {
        frame_size_limit = s.frame_size_limit;
    }

    let NumThreads { n_tc, n_fc } = get_num_threads(s);

    let ttd = TaskThreadData {
        cur: AtomicU32::new(n_fc as u32),
        reset_task_cur: AtomicU32::new(u32::MAX),
        ..Default::default()
    };
    // TODO fallible allocation
    let task_thread = Arc::new(ttd);

    let fc = (0..n_fc)
        .map(|i| {
            let mut fc = Rav1dFrameContext::default(i);
            fc.task_thread.finished = AtomicBool::new(true);
            fc.task_thread.ttd = Arc::clone(&task_thread);
            let f = fc.data.get_mut();
            f.lf.last_sharpness = u8::MAX;
            fc
        })
        // TODO fallible allocation
        .collect();

    let state = Mutex::new(Rav1dState {
        frame_thread: Rav1dContext_frame_thread {
            out_delayed: if n_fc > 1 {
                (0..n_fc).map(|_| Default::default()).collect()
            } else {
                Box::new([])
            },
            ..Default::default()
        },
        ..Default::default()
    });

    let tc = (0..n_tc)
        .map(|n| {
            let task_thread = Arc::clone(&task_thread);
            let thread_data = Arc::new(Rav1dTaskContext_task_thread::new(task_thread));
            let thread_data_copy = Arc::clone(&thread_data);
            let task = if n_tc > 1 {
                let handle = thread::Builder::new()
                    // Don't set stack size like `dav1d` does.
                    // See <https://github.com/memorysafety/rav1d/issues/889>.
                    .name(format!("rav1d-worker-{n}"))
                    .spawn(|| rav1d_worker_task(thread_data_copy))
                    .unwrap();
                Rav1dContextTaskType::Worker(handle)
            } else {
                Rav1dContextTaskType::Single(Mutex::new(Box::new(Rav1dTaskContext::new(
                    thread_data_copy,
                ))))
            };
            Rav1dContextTaskThread { task, thread_data }
        })
        // TODO fallible allocation
        .collect();

    let c = Rav1dContext {
        allocator: s.allocator.clone(),
        logger: s.logger.clone(),
        apply_grain: s.apply_grain,
        operating_point: s.operating_point,
        all_layers: s.all_layers,
        frame_size_limit,
        strict_std_compliance: s.strict_std_compliance,
        output_invisible_frames: s.output_invisible_frames,
        inloop_filters: s.inloop_filters,
        decode_frame_type: s.decode_frame_type,
        fc,
        task_thread,
        state,
        tc,
        ..Default::default()
    };

    // TODO fallible allocation
    let mut c = Arc::new(c);

    if c.allocator.is_default() {
        let c = Arc::get_mut(&mut c).unwrap();
        // SAFETY: When `allocator.is_default()`, `allocator.cookie` should be a `&c.picture_pool`.
        // See `Rav1dPicAllocator::cookie` docs for more, including an analysis of the lifetime.
        // Note also that we must do this after we created the `Arc` so that `c` has a stable address.
        c.allocator.cookie = ptr::from_ref(&c.picture_pool).cast::<c_void>().cast_mut();
    }
    let c = c;

    for tc in c.tc.iter() {
        if let Rav1dContextTaskType::Worker(handle) = &tc.task {
            // Unpark each thread once we set its `thread_data.c`.
            *tc.thread_data.c.lock() = Some(Arc::clone(&c));
            handle.thread().unpark();
        }
    }

    Ok(c)
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_open(
    c_out: *mut Option<Dav1dContext>,
    s: *const Dav1dSettings,
) -> Dav1dResult {
    (|| {
        validate_input!((!c_out.is_null(), EINVAL))?;
        validate_input!((!s.is_null(), EINVAL))?;
        let s = s.read().try_into()?;
        let c = rav1d_open(&s).inspect_err(|_| {
            *c_out = None;
        })?;
        *c_out = Some(RawArc::from_arc(c));
        Ok(())
    })()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_parse_sequence_header(
    out: *mut Dav1dSequenceHeader,
    ptr: *const u8,
    sz: usize,
) -> Dav1dResult {
    (|| {
        validate_input!((!out.is_null(), EINVAL))?;
        validate_input!((!ptr.is_null(), EINVAL))?;
        validate_input!((sz > 0 && sz <= usize::MAX / 2, EINVAL))?;
        let seq_hdr = rav1d_parse_sequence_header(slice::from_raw_parts(ptr, sz))?;
        out.write(seq_hdr.dav1d);
        Ok(())
    })()
    .into()
}

impl Rav1dFilmGrainData {
    fn has_grain(&self) -> bool {
        self.num_y_points != 0
            || self.num_uv_points[0] != 0
            || self.num_uv_points[1] != 0
            || self.clip_to_restricted_range && self.chroma_scaling_from_luma
    }
}

impl Rav1dPicture {
    fn has_grain(&self) -> bool {
        self.frame_hdr.as_ref().unwrap().film_grain.data.has_grain()
    }
}

unsafe fn output_image(
    c: &Rav1dContext,
    state: &mut Rav1dState,
    out: &mut Rav1dPicture,
) -> Rav1dResult {
    let mut res = Ok(());

    let r#in: *mut Rav1dThreadPicture = if c.all_layers || state.max_spatial_id == 0 {
        &mut state.out
    } else {
        &mut state.cache
    };
    if !c.apply_grain || !(*r#in).p.has_grain() {
        *out = mem::take(&mut (*r#in).p);
    } else {
        res = rav1d_apply_grain(c, out, &(*r#in).p);
    }
    let _ = mem::take(&mut *r#in);

    if !c.all_layers && state.max_spatial_id != 0 && state.out.p.data.is_some() {
        *r#in = mem::take(&mut state.out);
    }
    res
}

fn output_picture_ready(c: &Rav1dContext, state: &mut Rav1dState, drain: bool) -> bool {
    if state.cached_error.is_some() {
        return true;
    }
    if !c.all_layers && state.max_spatial_id != 0 {
        if state.out.p.data.is_some() && state.cache.p.data.is_some() {
            if state.max_spatial_id == state.cache.p.frame_hdr.as_ref().unwrap().spatial_id
                || state.out.flags.contains(PictureFlags::NEW_TEMPORAL_UNIT)
            {
                return true;
            }
            state.cache = mem::take(&mut state.out);
            return false;
        } else {
            if state.cache.p.data.is_some() && drain {
                return true;
            } else {
                if state.out.p.data.is_some() {
                    state.cache = mem::take(&mut state.out);
                    return false;
                }
            }
        }
    }
    state.out.p.data.is_some()
}

unsafe fn drain_picture(
    c: &Rav1dContext,
    state: &mut Rav1dState,
    out: &mut Rav1dPicture,
) -> Rav1dResult {
    let mut drained = false;
    for _ in 0..c.fc.len() {
        let next = state.frame_thread.next;
        let fc = &c.fc[next as usize];
        let mut task_thread_lock = c.task_thread.lock.lock();
        while !fc.task_thread.finished.load(Ordering::SeqCst) {
            fc.task_thread.cond.wait(&mut task_thread_lock);
        }
        let out_delayed = &mut state.frame_thread.out_delayed[next as usize];
        if out_delayed.p.data.is_some() || fc.task_thread.error.load(Ordering::SeqCst) != 0 {
            let first = c.task_thread.first.load(Ordering::SeqCst);
            if first as usize + 1 < c.fc.len() {
                c.task_thread.first.fetch_add(1, Ordering::SeqCst);
            } else {
                c.task_thread.first.store(0, Ordering::SeqCst);
            }
            let _ = c.task_thread.reset_task_cur.compare_exchange(
                first,
                u32::MAX,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            let cur = c.task_thread.cur.load(Ordering::Relaxed);
            if cur != 0 && (cur as usize) < c.fc.len() {
                c.task_thread.cur.store(cur - 1, Ordering::Relaxed);
            }
            drained = true;
        } else if drained {
            break;
        }
        state.frame_thread.next = (state.frame_thread.next + 1) % c.fc.len() as u32;
        drop(task_thread_lock);
        mem::take(&mut *fc.task_thread.retval.try_lock().unwrap())
            .err_or(())
            .inspect_err(|_| {
                state.cached_error_props = out_delayed.p.m.clone();
                let _ = mem::take(out_delayed);
            })?;
        if out_delayed.p.data.is_some() {
            let progress = out_delayed.progress.as_ref().unwrap()[1].load(Ordering::Relaxed);
            if (out_delayed.visible || c.output_invisible_frames) && progress != FRAME_ERROR {
                state.out = out_delayed.clone();
                state.event_flags |= out_delayed.flags.into();
            }
            let _ = mem::take(out_delayed);
            if output_picture_ready(c, state, false) {
                return output_image(c, state, out);
            }
        }
    }
    if output_picture_ready(c, state, true) {
        return output_image(c, state, out);
    }
    Err(EAGAIN)
}

fn gen_picture(c: &Rav1dContext, state: &mut Rav1dState) -> Rav1dResult {
    if output_picture_ready(c, state, false) {
        return Ok(());
    }
    // Take so we don't have 2 `&mut`s.
    let Rav1dData {
        data: r#in,
        m: props,
    } = mem::take(&mut state.in_0);
    let Some(mut r#in) = r#in else { return Ok(()) };
    while !r#in.is_empty() {
        let len = rav1d_parse_obus(c, state, &r#in, &props);
        if let Ok(len) = len {
            r#in.slice_in_place(len..);
        }
        // Note that [`output_picture_ready`] doesn't read [`Rav1dContext::in_0`].
        if output_picture_ready(c, state, false) {
            // Restore into `c` when there's still data left.
            if !r#in.is_empty() {
                state.in_0 = Rav1dData {
                    data: Some(r#in),
                    m: props,
                }
            }
            break;
        }
        len?;
    }
    Ok(())
}

pub(crate) fn rav1d_send_data(c: &Rav1dContext, in_0: &mut Rav1dData) -> Rav1dResult {
    let state = &mut *c.state.try_lock().unwrap();
    if in_0.data.is_some() {
        let sz = in_0.data.as_ref().unwrap().len();
        validate_input!((sz > 0 && sz <= usize::MAX / 2, EINVAL))?;
        state.drain = false;
    }
    if state.in_0.data.is_some() {
        return Err(EAGAIN);
    }
    state.in_0 = in_0.clone();
    let res = gen_picture(c, state);
    if res.is_ok() {
        let _ = mem::take(in_0);
    }
    res
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_send_data(
    c: Option<Dav1dContext>,
    in_0: *mut Dav1dData,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        validate_input!((!in_0.is_null(), EINVAL))?;
        let c = c.as_ref();
        let mut in_rust = in_0.read().into();
        let result = rav1d_send_data(c, &mut in_rust);
        in_0.write(in_rust.into());
        result
    })()
    .into()
}

pub(crate) unsafe fn rav1d_get_picture(c: &Rav1dContext, out: &mut Rav1dPicture) -> Rav1dResult {
    let state = &mut *c.state.try_lock().unwrap();
    let drain = mem::replace(&mut state.drain, true);
    gen_picture(c, state)?;
    mem::take(&mut state.cached_error).err_or(())?;
    if output_picture_ready(c, state, c.fc.len() == 1) {
        return output_image(c, state, out);
    }
    if c.fc.len() > 1 && drain {
        return drain_picture(c, state, out);
    }
    Err(EAGAIN)
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_picture(
    c: Option<Dav1dContext>,
    out: *mut Dav1dPicture,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        validate_input!((!out.is_null(), EINVAL))?;
        let c = c.as_ref();
        let mut out_rust = Default::default(); // TODO(kkysen) Temporary until we return it directly.
        let result = rav1d_get_picture(c, &mut out_rust);
        out.write(out_rust.into());
        result
    })()
    .into()
}

pub(crate) fn rav1d_apply_grain(
    c: &Rav1dContext,
    out: &mut Rav1dPicture,
    in_0: &Rav1dPicture,
) -> Rav1dResult {
    if !in_0.has_grain() {
        *out = in_0.clone();
        return Ok(());
    }
    let res = rav1d_picture_alloc_copy(&c.logger, out, in_0.p.w, in_0);
    if res.is_err() {
        let _ = mem::take(out);
        return res;
    } else {
        if c.tc.len() > 1 {
            rav1d_task_delayed_fg(c, out, in_0);
        } else {
            match out.p.bpc {
                #[cfg(feature = "bitdepth_8")]
                bpc @ 8 => {
                    fg_apply::rav1d_apply_grain::<BitDepth8>(
                        &Rav1dBitDepthDSPContext::get(bpc).as_ref().unwrap().fg,
                        out,
                        in_0,
                    );
                }
                #[cfg(feature = "bitdepth_16")]
                bpc @ 10 | bpc @ 12 => {
                    fg_apply::rav1d_apply_grain::<BitDepth16>(
                        &Rav1dBitDepthDSPContext::get(bpc).as_ref().unwrap().fg,
                        out,
                        in_0,
                    );
                }
                _ => {
                    abort();
                }
            }
        }
        return Ok(());
    };
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_apply_grain(
    c: Option<Dav1dContext>,
    out: *mut Dav1dPicture,
    in_0: *const Dav1dPicture,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        validate_input!((!out.is_null(), EINVAL))?;
        validate_input!((!in_0.is_null(), EINVAL))?;
        let c = c.as_ref();
        let in_0 = in_0.read();
        // Don't `.update_rav1d()` [`Rav1dSequenceHeader`] because it's meant to be read-only.
        // Don't `.update_rav1d()` [`Rav1dFrameHeader`] because it's meant to be read-only.
        // Don't `.update_rav1d()` [`Rav1dITUTT35`] because we never read it.
        let mut out_rust = Default::default(); // TODO(kkysen) Temporary until we return it directly.
        let in_rust = in_0.into();
        let result = rav1d_apply_grain(c, &mut out_rust, &in_rust);
        out.write(out_rust.into());
        result
    })()
    .into()
}

pub(crate) fn rav1d_flush(c: &Rav1dContext) {
    let state = &mut *c.state.try_lock().unwrap();

    let old_state = mem::take(state);
    state.tiles = old_state.tiles;
    state.n_tiles = old_state.n_tiles;
    state.frame_thread = old_state.frame_thread;
    state.operating_point_idc = old_state.operating_point_idc;
    state.max_spatial_id = old_state.max_spatial_id;
    state.frame_flags = old_state.frame_flags;
    state.event_flags = old_state.event_flags;

    if c.fc.len() == 1 && c.tc.len() == 1 {
        return;
    }
    c.flush.store(true, Ordering::SeqCst);
    if c.tc.len() > 1 {
        let mut task_thread_lock = c.task_thread.lock.lock();
        for tc in c.tc.iter() {
            while !tc.flushed() {
                tc.thread_data.cond.wait(&mut task_thread_lock);
            }
        }
        for fc in c.fc.iter() {
            fc.task_thread.tasks.clear();
        }
        c.task_thread.first.store(0, Ordering::SeqCst);
        c.task_thread.cur.store(c.fc.len() as u32, Ordering::SeqCst);
        c.task_thread
            .reset_task_cur
            .store(u32::MAX, Ordering::SeqCst);
        c.task_thread.cond_signaled.store(0, Ordering::SeqCst);
    }
    if c.fc.len() > 1 {
        for fc in wrapping_iter(c.fc.iter(), state.frame_thread.next as usize) {
            let _ = rav1d_decode_frame_exit(c, fc, Err(EGeneric));
            *fc.task_thread.retval.try_lock().unwrap() = None;
            let out_delayed = &mut state.frame_thread.out_delayed[fc.index];
            if out_delayed.p.frame_hdr.is_some() {
                let _ = mem::take(out_delayed);
            }
        }
        state.frame_thread.next = 0;
    }
    c.flush.store(false, Ordering::SeqCst);
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_flush(c: Dav1dContext) {
    let c = c.as_ref();
    rav1d_flush(c)
}

#[cold]
pub(crate) fn rav1d_close(c: Arc<Rav1dContext>) {
    let c = &*c;
    rav1d_flush(c);
    c.tell_worker_threads_to_die();
}

#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_close(c_out: *mut Option<Dav1dContext>) {
    if validate_input!(!c_out.is_null()).is_err() {
        return;
    }
    let c_out = &mut *c_out;
    mem::take(c_out).map(|c| rav1d_close(c.into_arc()));
}

impl Rav1dContext {
    fn tell_worker_threads_to_die(&self) {
        if self.tc.is_empty() {
            return;
        }
        let ttd = &*self.task_thread;
        let _task_thread_lock = ttd.lock.lock();
        for tc in self.tc.iter() {
            tc.thread_data.die.store(true, Ordering::Relaxed);
        }
        ttd.cond.notify_all();
    }
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_event_flags(
    c: Option<Dav1dContext>,
    flags: *mut Dav1dEventFlags,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        validate_input!((!flags.is_null(), EINVAL))?;
        let c = c.as_ref();
        let state = &mut *c.state.try_lock().unwrap();
        flags.write(mem::take(&mut state.event_flags).into());
        Ok(())
    })()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_get_decode_error_data_props(
    c: Option<Dav1dContext>,
    out: *mut Dav1dDataProps,
) -> Dav1dResult {
    (|| {
        let c = validate_input!(c.ok_or(EINVAL))?;
        validate_input!((!out.is_null(), EINVAL))?;
        let c = c.as_ref();
        let state = &mut *c.state.try_lock().unwrap();
        out.write(mem::take(&mut state.cached_error_props).into());
        Ok(())
    })()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_picture_unref(p: *mut Dav1dPicture) {
    if validate_input!(!p.is_null()).is_err() {
        return;
    }
    let mut p_rust = p.read().to::<Rav1dPicture>();
    let _ = mem::take(&mut p_rust);
    p.write(p_rust.into());
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_create(buf: *mut Dav1dData, sz: usize) -> *mut u8 {
    || -> Rav1dResult<*mut u8> {
        let buf = validate_input!(NonNull::new(buf).ok_or(EINVAL))?;
        validate_input!((sz <= usize::MAX / 2, EINVAL))?;
        let data = Rav1dData::create(sz)?;
        let data = data.to::<Dav1dData>();
        let ptr = data
            .data
            .map(|ptr| ptr.as_ptr())
            .unwrap_or_else(ptr::null_mut);
        buf.as_ptr().write(data);
        Ok(ptr)
    }()
    .unwrap_or_else(|_| ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap(
    buf: *mut Dav1dData,
    ptr: *const u8,
    sz: usize,
    free_callback: Option<FnFree>,
    user_data: *mut c_void,
) -> Dav1dResult {
    || -> Rav1dResult {
        let buf = validate_input!(NonNull::new(buf).ok_or(EINVAL))?;
        let ptr = validate_input!(NonNull::new(ptr.cast_mut()).ok_or(EINVAL))?;
        validate_input!((sz <= usize::MAX / 2, EINVAL))?;
        let data = slice::from_raw_parts(ptr.as_ptr(), sz).into();
        let data = Rav1dData::wrap(data, free_callback, user_data)?;
        buf.as_ptr().write(data.into());
        Ok(())
    }()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_wrap_user_data(
    buf: *mut Dav1dData,
    user_data: *const u8,
    free_callback: Option<FnFree>,
    cookie: *mut c_void,
) -> Dav1dResult {
    || -> Rav1dResult {
        let buf = validate_input!(NonNull::new(buf).ok_or(EINVAL))?;
        // Note that `dav1d` doesn't do this check, but they do for the similar [`dav1d_data_wrap`].
        let user_data = validate_input!(NonNull::new(user_data.cast_mut()).ok_or(EINVAL))?;
        let mut data = buf.as_ptr().read().to::<Rav1dData>();
        data.wrap_user_data(user_data, free_callback, cookie)?;
        buf.as_ptr().write(data.into());
        Ok(())
    }()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_unref(buf: *mut Dav1dData) {
    let buf = validate_input!(NonNull::new(buf).ok_or(()));
    let Ok(mut buf) = buf else { return };
    let _ = mem::take(buf.as_mut()).to::<Rav1dData>();
}

#[no_mangle]
pub unsafe extern "C" fn dav1d_data_props_unref(props: *mut Dav1dDataProps) {
    let props = validate_input!(NonNull::new(props).ok_or(()));
    let Ok(mut props) = props else { return };
    let _ = mem::take(props.as_mut()).to::<Rav1dDataProps>();
}
