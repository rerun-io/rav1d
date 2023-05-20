use crate::include::common::frame::{is_inter_or_switch, is_key_or_intra};
use crate::include::dav1d::headers::DAV1D_MAX_SEGMENTS;
use crate::include::stddef::*;
use crate::include::stdint::*;
use crate::src::align::Align16;
use crate::src::cdf::{CdfContext, CdfMvComponent, CdfMvContext};
use crate::src::ctx::{case_set, SetCtxFn};
use crate::src::msac::MsacContext;
use libc;

extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: size_t) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_cdef_dsp_init_8bpc(c: *mut Dav1dCdefDSPContext);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_cdef_dsp_init_16bpc(c: *mut Dav1dCdefDSPContext);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
    fn dav1d_ref_create_using_pool(pool: *mut Dav1dMemPool, size: size_t) -> *mut Dav1dRef;
    fn dav1d_ref_dec(r#ref: *mut *mut Dav1dRef);
    fn dav1d_cdf_thread_init_static(cdf: *mut CdfThreadContext, qidx: libc::c_int);
    fn dav1d_cdf_thread_alloc(
        c: *mut Dav1dContext,
        cdf: *mut CdfThreadContext,
        have_frame_mt: libc::c_int,
    ) -> libc::c_int;
    fn dav1d_cdf_thread_copy(dst: *mut CdfContext, src: *const CdfThreadContext);
    fn dav1d_cdf_thread_ref(dst: *mut CdfThreadContext, src: *mut CdfThreadContext);
    fn dav1d_cdf_thread_unref(cdf: *mut CdfThreadContext);
    fn dav1d_cdf_thread_update(
        hdr: *const Dav1dFrameHeader,
        dst: *mut CdfContext,
        src: *const CdfContext,
    );
    fn dav1d_data_props_copy(dst: *mut Dav1dDataProps, src: *const Dav1dDataProps);
    fn dav1d_data_unref_internal(buf: *mut Dav1dData);
    fn dav1d_refmvs_init_frame(
        rf: *mut refmvs_frame,
        seq_hdr: *const Dav1dSequenceHeader,
        frm_hdr: *const Dav1dFrameHeader,
        ref_poc: *const libc::c_uint,
        rp: *mut refmvs_temporal_block,
        ref_ref_poc: *const [libc::c_uint; 7],
        rp_ref: *const *mut refmvs_temporal_block,
        n_tile_threads: libc::c_int,
        n_frame_threads: libc::c_int,
    ) -> libc::c_int;
    fn dav1d_refmvs_save_tmvs(
        dsp: *const Dav1dRefmvsDSPContext,
        rt: *const refmvs_tile,
        col_start8: libc::c_int,
        col_end8: libc::c_int,
        row_start8: libc::c_int,
        row_end8: libc::c_int,
    );
    fn dav1d_refmvs_tile_sbrow_init(
        rt: *mut refmvs_tile,
        rf: *const refmvs_frame,
        tile_col_start4: libc::c_int,
        tile_col_end4: libc::c_int,
        tile_row_start4: libc::c_int,
        tile_row_end4: libc::c_int,
        sby: libc::c_int,
        tile_row_idx: libc::c_int,
        pass: libc::c_int,
    );
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_film_grain_dsp_init_8bpc(c: *mut Dav1dFilmGrainDSPContext);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_film_grain_dsp_init_16bpc(c: *mut Dav1dFilmGrainDSPContext);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_intra_pred_dsp_init_8bpc(c: *mut Dav1dIntraPredDSPContext);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_intra_pred_dsp_init_16bpc(c: *mut Dav1dIntraPredDSPContext);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_itx_dsp_init_8bpc(c: *mut Dav1dInvTxfmDSPContext, bpc: libc::c_int);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_itx_dsp_init_16bpc(c: *mut Dav1dInvTxfmDSPContext, bpc: libc::c_int);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_loop_filter_dsp_init_8bpc(c: *mut Dav1dLoopFilterDSPContext);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_loop_filter_dsp_init_16bpc(c: *mut Dav1dLoopFilterDSPContext);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_loop_restoration_dsp_init_8bpc(
        c: *mut Dav1dLoopRestorationDSPContext,
        bpc: libc::c_int,
    );
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_loop_restoration_dsp_init_16bpc(
        c: *mut Dav1dLoopRestorationDSPContext,
        bpc: libc::c_int,
    );
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_mc_dsp_init_8bpc(c: *mut Dav1dMCDSPContext);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_mc_dsp_init_16bpc(c: *mut Dav1dMCDSPContext);
    fn dav1d_thread_picture_alloc(
        c: *mut Dav1dContext,
        f: *mut Dav1dFrameContext,
        bpc: libc::c_int,
    ) -> libc::c_int;
    fn dav1d_picture_alloc_copy(
        c: *mut Dav1dContext,
        dst: *mut Dav1dPicture,
        w: libc::c_int,
        src: *const Dav1dPicture,
    ) -> libc::c_int;
    fn dav1d_picture_ref(dst: *mut Dav1dPicture, src: *const Dav1dPicture);
    fn dav1d_thread_picture_ref(dst: *mut Dav1dThreadPicture, src: *const Dav1dThreadPicture);
    fn dav1d_thread_picture_unref(p: *mut Dav1dThreadPicture);
    fn dav1d_picture_unref_internal(p: *mut Dav1dPicture);
    fn dav1d_picture_get_event_flags(p: *const Dav1dThreadPicture) -> Dav1dEventFlags;
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_recon_b_intra_8bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        intra_edge_flags: EdgeFlags,
        b: *const Av1Block,
    );
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_recon_b_intra_16bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        intra_edge_flags: EdgeFlags,
        b: *const Av1Block,
    );
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_recon_b_inter_8bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        b: *const Av1Block,
    ) -> libc::c_int;
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_recon_b_inter_16bpc(
        t: *mut Dav1dTaskContext,
        bs: BlockSize,
        b: *const Av1Block,
    ) -> libc::c_int;
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_filter_sbrow_8bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_filter_sbrow_16bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_filter_sbrow_deblock_cols_8bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_filter_sbrow_deblock_cols_16bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_filter_sbrow_deblock_rows_8bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_filter_sbrow_deblock_rows_16bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_filter_sbrow_cdef_8bpc(tc: *mut Dav1dTaskContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_filter_sbrow_cdef_16bpc(tc: *mut Dav1dTaskContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_filter_sbrow_resize_8bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_filter_sbrow_resize_16bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_filter_sbrow_lr_8bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_filter_sbrow_lr_16bpc(f: *mut Dav1dFrameContext, sby: libc::c_int);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_backup_ipred_edge_8bpc(t: *mut Dav1dTaskContext);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_backup_ipred_edge_16bpc(t: *mut Dav1dTaskContext);
    #[cfg(feature = "bitdepth_8")]
    fn dav1d_read_coef_blocks_8bpc(t: *mut Dav1dTaskContext, bs: BlockSize, b: *const Av1Block);
    #[cfg(feature = "bitdepth_16")]
    fn dav1d_read_coef_blocks_16bpc(t: *mut Dav1dTaskContext, bs: BlockSize, b: *const Av1Block);
    fn dav1d_log(c: *mut Dav1dContext, format: *const libc::c_char, _: ...);
    static mut dav1d_qm_tbl: [[[*const uint8_t; 19]; 2]; 16];
    fn dav1d_task_create_tile_sbrow(
        f: *mut Dav1dFrameContext,
        pass: libc::c_int,
        cond_signal: libc::c_int,
    ) -> libc::c_int;
    fn dav1d_task_frame_init(f: *mut Dav1dFrameContext);
}

use crate::src::dequant_tables::dav1d_dq_tbl;
use crate::src::lf_mask::dav1d_calc_eih;
use crate::src::lf_mask::dav1d_calc_lf_values;
use crate::src::lf_mask::dav1d_create_lf_mask_inter;
use crate::src::lf_mask::dav1d_create_lf_mask_intra;
use crate::src::msac::dav1d_msac_decode_bool;
use crate::src::msac::dav1d_msac_decode_bool_adapt;
use crate::src::msac::dav1d_msac_decode_bool_equi;
use crate::src::msac::dav1d_msac_decode_subexp;
use crate::src::msac::dav1d_msac_decode_symbol_adapt16;
use crate::src::msac::dav1d_msac_decode_symbol_adapt4;
use crate::src::msac::dav1d_msac_decode_symbol_adapt8;
use crate::src::msac::dav1d_msac_init;
use crate::src::refmvs::dav1d_refmvs_find;
use crate::src::tables::dav1d_al_part_ctx;
use crate::src::tables::dav1d_block_dimensions;
use crate::src::tables::dav1d_block_sizes;
use crate::src::tables::dav1d_comp_inter_pred_modes;
use crate::src::tables::dav1d_filter_2d;
use crate::src::tables::dav1d_filter_dir;
use crate::src::tables::dav1d_intra_mode_context;
use crate::src::tables::dav1d_max_txfm_size_for_bs;
use crate::src::tables::dav1d_partition_type_count;
use crate::src::tables::dav1d_sgr_params;
use crate::src::tables::dav1d_txfm_dimensions;
use crate::src::tables::dav1d_wedge_ctx_lut;
use crate::src::tables::dav1d_ymode_size_context;
use crate::src::warpmv::dav1d_find_affine_int;
use crate::src::warpmv::dav1d_get_shear_params;
use crate::src::warpmv::dav1d_set_affine_mv2d;

use crate::include::dav1d::common::Dav1dDataProps;
use crate::include::dav1d::data::Dav1dData;
use crate::include::stdatomic::atomic_int;
use crate::src::ctx::alias16;
use crate::src::ctx::alias32;
use crate::src::ctx::alias64;
use crate::src::ctx::alias8;
use crate::src::r#ref::Dav1dRef;

use crate::include::stdatomic::atomic_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext {
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub refp: [Dav1dThreadPicture; 7],
    pub cur: Dav1dPicture,
    pub sr_cur: Dav1dThreadPicture,
    pub mvs_ref: *mut Dav1dRef,
    pub mvs: *mut refmvs_temporal_block,
    pub ref_mvs: [*mut refmvs_temporal_block; 7],
    pub ref_mvs_ref: [*mut Dav1dRef; 7],
    pub cur_segmap_ref: *mut Dav1dRef,
    pub prev_segmap_ref: *mut Dav1dRef,
    pub cur_segmap: *mut uint8_t,
    pub prev_segmap: *const uint8_t,
    pub refpoc: [libc::c_uint; 7],
    pub refrefpoc: [[libc::c_uint; 7]; 7],
    pub gmv_warp_allowed: [uint8_t; 7],
    pub in_cdf: CdfThreadContext,
    pub out_cdf: CdfThreadContext,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: libc::c_int,
    pub n_tile_data: libc::c_int,
    pub svc: [[ScalableMotionParams; 2]; 7],
    pub resize_step: [libc::c_int; 2],
    pub resize_start: [libc::c_int; 2],
    pub c: *const Dav1dContext,
    pub ts: *mut Dav1dTileState,
    pub n_ts: libc::c_int,
    pub dsp: *const Dav1dDSPContext,
    pub bd_fn: Dav1dFrameContext_bd_fn,
    pub ipred_edge_sz: libc::c_int,
    pub ipred_edge: [*mut libc::c_void; 3],
    pub b4_stride: ptrdiff_t,
    pub w4: libc::c_int,
    pub h4: libc::c_int,
    pub bw: libc::c_int,
    pub bh: libc::c_int,
    pub sb128w: libc::c_int,
    pub sb128h: libc::c_int,
    pub sbh: libc::c_int,
    pub sb_shift: libc::c_int,
    pub sb_step: libc::c_int,
    pub sr_sb128w: libc::c_int,
    pub dq: [[[uint16_t; 2]; 3]; 8],
    pub qm: [[*const uint8_t; 3]; 19],
    pub a: *mut BlockContext,
    pub a_sz: libc::c_int,
    pub rf: refmvs_frame,
    pub jnt_weights: [[uint8_t; 7]; 7],
    pub bitdepth_max: libc::c_int,
    pub frame_thread: Dav1dFrameContext_frame_thread,
    pub lf: Dav1dFrameContext_lf,
    pub task_thread: Dav1dFrameContext_task_thread,
    pub tile_thread: FrameTileThreadData,
}
use crate::include::pthread::pthread_mutex_t;
use crate::src::internal::Dav1dFrameContext_task_thread;
use crate::src::internal::FrameTileThreadData;

use crate::include::dav1d::headers::Dav1dContentLightLevel;
use crate::include::dav1d::headers::Dav1dITUTT35;
use crate::include::dav1d::headers::Dav1dMasteringDisplay;
use crate::include::dav1d::picture::Dav1dPicture;
use crate::src::internal::TaskThreadData;

use crate::include::dav1d::headers::Dav1dFrameHeader;
use crate::include::dav1d::headers::Dav1dWarpedMotionParams;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I400;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I420;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I422;
use crate::include::dav1d::headers::DAV1D_PIXEL_LAYOUT_I444;

use crate::include::dav1d::headers::DAV1D_WM_TYPE_AFFINE;

use crate::include::dav1d::headers::DAV1D_TX_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_IDENTITY;
use crate::include::dav1d::headers::DAV1D_WM_TYPE_TRANSLATION;

use crate::include::dav1d::headers::Dav1dFilmGrainData;
use crate::include::dav1d::headers::Dav1dFilterMode;
use crate::include::dav1d::headers::Dav1dRestorationType;
use crate::include::dav1d::headers::Dav1dSequenceHeader;
use crate::include::dav1d::headers::DAV1D_FILTER_8TAP_REGULAR;
use crate::include::dav1d::headers::DAV1D_FILTER_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_N_SWITCHABLE_FILTERS;
use crate::include::dav1d::headers::DAV1D_RESTORATION_NONE;
use crate::include::dav1d::headers::DAV1D_RESTORATION_SGRPROJ;
use crate::include::dav1d::headers::DAV1D_RESTORATION_SWITCHABLE;
use crate::include::dav1d::headers::DAV1D_RESTORATION_WIENER;

use crate::include::pthread::pthread_cond_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_lf {
    pub level: *mut [uint8_t; 4],
    pub mask: *mut Av1Filter,
    pub lr_mask: *mut Av1Restoration,
    pub mask_sz: libc::c_int,
    pub lr_mask_sz: libc::c_int,
    pub cdef_buf_plane_sz: [libc::c_int; 2],
    pub cdef_buf_sbh: libc::c_int,
    pub lr_buf_plane_sz: [libc::c_int; 2],
    pub re_sz: libc::c_int,
    pub lim_lut: Align16<Av1FilterLUT>,
    pub last_sharpness: libc::c_int,
    pub lvl: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub tx_lpf_right_edge: [*mut uint8_t; 2],
    pub cdef_line_buf: *mut uint8_t,
    pub lr_line_buf: *mut uint8_t,
    pub cdef_line: [[*mut libc::c_void; 3]; 2],
    pub cdef_lpf_line: [*mut libc::c_void; 3],
    pub lr_lpf_line: [*mut libc::c_void; 3],
    pub start_of_tile_row: *mut uint8_t,
    pub start_of_tile_row_sz: libc::c_int,
    pub need_cdef_lpf_copy: libc::c_int,
    pub p: [*mut libc::c_void; 3],
    pub sr_p: [*mut libc::c_void; 3],
    pub mask_ptr: *mut Av1Filter,
    pub prev_mask_ptr: *mut Av1Filter,
    pub restore_planes: libc::c_int,
}
use crate::src::lf_mask::Av1Filter;
pub type pixel = ();
use crate::src::lf_mask::Av1FilterLUT;
use crate::src::lf_mask::Av1Restoration;
use crate::src::lf_mask::Av1RestorationUnit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_frame_thread {
    pub next_tile_row: [libc::c_int; 2],
    pub entropy_progress: atomic_int,
    pub deblock_progress: atomic_int,
    pub frame_progress: *mut atomic_uint,
    pub copy_lpf_progress: *mut atomic_uint,
    pub b: *mut Av1Block,
    pub cbi: *mut CodedBlockInfo,
    pub pal: *mut [[uint16_t; 8]; 3],
    pub pal_idx: *mut uint8_t,
    pub cf: *mut libc::c_void,
    pub prog_sz: libc::c_int,
    pub pal_sz: libc::c_int,
    pub pal_idx_sz: libc::c_int,
    pub cf_sz: libc::c_int,
    pub tile_start_off: *mut libc::c_int,
}
pub type coef = ();
use crate::src::internal::CodedBlockInfo;
use crate::src::levels::Av1Block;

use crate::src::levels::mv;

use crate::src::env::BlockContext;
use crate::src::refmvs::refmvs_block;
use crate::src::refmvs::refmvs_block_unaligned;
use crate::src::refmvs::refmvs_frame;
use crate::src::refmvs::refmvs_mvpair;
use crate::src::refmvs::refmvs_refpair;
use crate::src::refmvs::refmvs_temporal_block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFrameContext_bd_fn {
    pub recon_b_intra: recon_b_intra_fn,
    pub recon_b_inter: recon_b_inter_fn,
    pub filter_sbrow: filter_sbrow_fn,
    pub filter_sbrow_deblock_cols: filter_sbrow_fn,
    pub filter_sbrow_deblock_rows: filter_sbrow_fn,
    pub filter_sbrow_cdef: Option<unsafe extern "C" fn(*mut Dav1dTaskContext, libc::c_int) -> ()>,
    pub filter_sbrow_resize: filter_sbrow_fn,
    pub filter_sbrow_lr: filter_sbrow_fn,
    pub backup_ipred_edge: backup_ipred_edge_fn,
    pub read_coef_blocks: read_coef_blocks_fn,
}

impl Dav1dFrameContext_bd_fn {
    pub unsafe fn recon_b_intra(
        &self,
        context: *mut Dav1dTaskContext,
        block_size: BlockSize,
        flags: EdgeFlags,
        block: *const Av1Block,
    ) {
        self.recon_b_intra.expect("non-null function pointer")(context, block_size, flags, block);
    }

    pub unsafe fn recon_b_inter(
        &self,
        context: *mut Dav1dTaskContext,
        block_size: BlockSize,
        block: *const Av1Block,
    ) -> libc::c_int {
        self.recon_b_inter.expect("non-null function pointer")(context, block_size, block)
    }

    pub unsafe fn read_coef_blocks(
        &self,
        context: *mut Dav1dTaskContext,
        block_size: BlockSize,
        block: *const Av1Block,
    ) {
        self.read_coef_blocks.expect("non-null function pointer")(context, block_size, block);
    }
}

pub type read_coef_blocks_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> ()>;
use crate::src::levels::BlockSize;

use crate::src::levels::BS_128x128;
use crate::src::levels::BS_4x4;
use crate::src::levels::BS_64x64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTaskContext {
    pub c: *const Dav1dContext,
    pub f: *const Dav1dFrameContext,
    pub ts: *mut Dav1dTileState,
    pub bx: libc::c_int,
    pub by: libc::c_int,
    pub l: BlockContext,
    pub a: *mut BlockContext,
    pub rt: refmvs_tile,
    pub c2rust_unnamed: Dav1dTaskContext_cf,
    pub al_pal: [[[[uint16_t; 8]; 3]; 32]; 2],
    pub pal_sz_uv: [[uint8_t; 32]; 2],
    pub txtp_map: [uint8_t; 1024],
    pub scratch: Dav1dTaskContext_scratch,
    pub warpmv: Dav1dWarpedMotionParams,
    pub lf_mask: *mut Av1Filter,
    pub top_pre_cdef_toggle: libc::c_int,
    pub cur_sb_cdef_idx_ptr: *mut int8_t,
    pub tl_4x4_filter: Filter2d,
    pub frame_thread: Dav1dTaskContext_frame_thread,
    pub task_thread: Dav1dTaskContext_task_thread,
}
use crate::src::internal::Dav1dTaskContext_frame_thread;
use crate::src::internal::Dav1dTaskContext_task_thread;
use crate::src::levels::Filter2d;

use crate::src::levels::FILTER_2D_BILINEAR;

use crate::src::internal::Dav1dTaskContext_cf;
use crate::src::internal::Dav1dTaskContext_scratch;
use crate::src::refmvs::refmvs_tile;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState {
    pub cdf: CdfContext,
    pub msac: MsacContext,
    pub tiling: Dav1dTileState_tiling,
    pub progress: [atomic_int; 2],
    pub frame_thread: [Dav1dTileState_frame_thread; 2],
    pub lowest_pixel: *mut [[libc::c_int; 2]; 7],
    pub dqmem: [[[uint16_t; 2]; 3]; 8],
    pub dq: *const [[uint16_t; 2]; 3],
    pub last_qidx: libc::c_int,
    pub last_delta_lf: [int8_t; 4],
    pub lflvlmem: [[[[uint8_t; 2]; 8]; 4]; 8],
    pub lflvl: *const [[[uint8_t; 2]; 8]; 4],
    pub lr_ref: [*mut Av1RestorationUnit; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dTileState_frame_thread {
    pub pal_idx: *mut uint8_t,
    pub cf: *mut libc::c_void,
}
use crate::src::internal::Dav1dTileState_tiling;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dContext {
    pub fc: *mut Dav1dFrameContext,
    pub n_fc: libc::c_uint,
    pub tc: *mut Dav1dTaskContext,
    pub n_tc: libc::c_uint,
    pub tile: *mut Dav1dTileGroup,
    pub n_tile_data_alloc: libc::c_int,
    pub n_tile_data: libc::c_int,
    pub n_tiles: libc::c_int,
    pub seq_hdr_pool: *mut Dav1dMemPool,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr_pool: *mut Dav1dMemPool,
    pub frame_hdr_ref: *mut Dav1dRef,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub content_light_ref: *mut Dav1dRef,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display_ref: *mut Dav1dRef,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35_ref: *mut Dav1dRef,
    pub itut_t35: *mut Dav1dITUTT35,
    pub in_0: Dav1dData,
    pub out: Dav1dThreadPicture,
    pub cache: Dav1dThreadPicture,
    pub flush_mem: atomic_int,
    pub flush: *mut atomic_int,
    pub frame_thread: Dav1dContext_frame_thread,
    pub task_thread: TaskThreadData,
    pub segmap_pool: *mut Dav1dMemPool,
    pub refmvs_pool: *mut Dav1dMemPool,
    pub refs: [Dav1dContext_refs; 8],
    pub cdf_pool: *mut Dav1dMemPool,
    pub cdf: [CdfThreadContext; 8],
    pub dsp: [Dav1dDSPContext; 3],
    pub refmvs_dsp: Dav1dRefmvsDSPContext,
    pub intra_edge: Dav1dContext_intra_edge,
    pub allocator: Dav1dPicAllocator,
    pub apply_grain: libc::c_int,
    pub operating_point: libc::c_int,
    pub operating_point_idc: libc::c_uint,
    pub all_layers: libc::c_int,
    pub max_spatial_id: libc::c_int,
    pub frame_size_limit: libc::c_uint,
    pub strict_std_compliance: libc::c_int,
    pub output_invisible_frames: libc::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub decode_frame_type: Dav1dDecodeFrameType,
    pub drain: libc::c_int,
    pub frame_flags: PictureFlags,
    pub event_flags: Dav1dEventFlags,
    pub cached_error_props: Dav1dDataProps,
    pub cached_error: libc::c_int,
    pub logger: Dav1dLogger,
    pub picture_pool: *mut Dav1dMemPool,
}
use crate::src::mem::Dav1dMemPool;

use crate::include::dav1d::dav1d::Dav1dEventFlags;
use crate::include::dav1d::dav1d::Dav1dLogger;
use crate::src::picture::PictureFlags;

use crate::include::dav1d::dav1d::Dav1dDecodeFrameType;
use crate::include::dav1d::dav1d::Dav1dInloopFilterType;

use crate::include::dav1d::picture::Dav1dPicAllocator;
use crate::src::internal::Dav1dContext_intra_edge;
use crate::src::intra_edge::EdgeFlags;
use crate::src::intra_edge::EdgeTip;

use crate::src::intra_edge::EdgeBranch;
use crate::src::intra_edge::EdgeNode;
use crate::src::intra_edge::EDGE_I444_TOP_HAS_RIGHT;
use crate::src::refmvs::Dav1dRefmvsDSPContext;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dDSPContext {
    pub fg: Dav1dFilmGrainDSPContext,
    pub ipred: Dav1dIntraPredDSPContext,
    pub mc: Dav1dMCDSPContext,
    pub itx: Dav1dInvTxfmDSPContext,
    pub lf: Dav1dLoopFilterDSPContext,
    pub cdef: Dav1dCdefDSPContext,
    pub lr: Dav1dLoopRestorationDSPContext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopRestorationDSPContext {
    pub wiener: [looprestorationfilter_fn; 2],
    pub sgr: [looprestorationfilter_fn; 3],
}
pub type looprestorationfilter_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        const_left_pixel_row,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        *const LooprestorationParams,
        LrEdgeFlags,
    ) -> (),
>;
use crate::src::looprestoration::LooprestorationParams;
use crate::src::looprestoration::LrEdgeFlags;

pub type const_left_pixel_row = *const libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: cdef_dir_fn,
    pub fb: [cdef_fn; 3],
}
pub type cdef_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        const_left_pixel_row_2px,
        *const libc::c_void,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        CdefEdgeFlags,
    ) -> (),
>;
use crate::src::cdef::CdefEdgeFlags;
pub type const_left_pixel_row_2px = *const libc::c_void;
pub type cdef_dir_fn =
    Option<unsafe extern "C" fn(*const libc::c_void, ptrdiff_t, *mut libc::c_uint) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[loopfilter_sb_fn; 2]; 2],
}
pub type loopfilter_sb_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const uint32_t,
        *const [uint8_t; 4],
        ptrdiff_t,
        *const Av1FilterLUT,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
pub type itxfm_fn = Option<
    unsafe extern "C" fn(*mut libc::c_void, ptrdiff_t, *mut libc::c_void, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMCDSPContext {
    pub mc: [mc_fn; 10],
    pub mc_scaled: [mc_scaled_fn; 10],
    pub mct: [mct_fn; 10],
    pub mct_scaled: [mct_scaled_fn; 10],
    pub avg: avg_fn,
    pub w_avg: w_avg_fn,
    pub mask: mask_fn,
    pub w_mask: [w_mask_fn; 3],
    pub blend: blend_fn,
    pub blend_v: blend_dir_fn,
    pub blend_h: blend_dir_fn,
    pub warp8x8: warp8x8_fn,
    pub warp8x8t: warp8x8t_fn,
    pub emu_edge: emu_edge_fn,
    pub resize: resize_fn,
}
pub type resize_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type emu_edge_fn = Option<
    unsafe extern "C" fn(
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        intptr_t,
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
    ) -> (),
>;
pub type warp8x8t_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type warp8x8_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_dir_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type blend_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_mask_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *mut uint8_t,
        libc::c_int,
    ) -> (),
>;
pub type mask_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        *const uint8_t,
    ) -> (),
>;
pub type w_avg_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type avg_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const int16_t,
        *const int16_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mct_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_scaled_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type mc_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dIntraPredDSPContext {
    pub intra_pred: [angular_ipred_fn; 14],
    pub cfl_ac: [cfl_ac_fn; 3],
    pub cfl_pred: [cfl_pred_fn; 6],
    pub pal_pred: pal_pred_fn,
}
pub type pal_pred_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const uint16_t,
        *const uint8_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type cfl_pred_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        *const int16_t,
        libc::c_int,
    ) -> (),
>;
pub type cfl_ac_fn = Option<
    unsafe extern "C" fn(
        *mut int16_t,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type angular_ipred_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        ptrdiff_t,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dFilmGrainDSPContext {
    pub generate_grain_y: generate_grain_y_fn,
    pub generate_grain_uv: [generate_grain_uv_fn; 3],
    pub fgy_32x32xn: fgy_32x32xn_fn,
    pub fguv_32x32xn: [fguv_32x32xn_fn; 3],
}
pub type fguv_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_void,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
        *const libc::c_void,
        ptrdiff_t,
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type entry = int8_t;
pub type fgy_32x32xn_fn = Option<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_void,
        ptrdiff_t,
        *const Dav1dFilmGrainData,
        size_t,
        *const uint8_t,
        *const [entry; 82],
        libc::c_int,
        libc::c_int,
    ) -> (),
>;
pub type generate_grain_uv_fn = Option<
    unsafe extern "C" fn(
        *mut [entry; 82],
        *const [entry; 82],
        *const Dav1dFilmGrainData,
        intptr_t,
    ) -> (),
>;
pub type generate_grain_y_fn =
    Option<unsafe extern "C" fn(*mut [entry; 82], *const Dav1dFilmGrainData) -> ()>;
use crate::src::cdf::CdfThreadContext;

use crate::src::internal::Dav1dContext_frame_thread;
use crate::src::internal::Dav1dContext_refs;
use crate::src::internal::Dav1dTileGroup;
use crate::src::picture::Dav1dThreadPicture;
pub type backup_ipred_edge_fn = Option<unsafe extern "C" fn(*mut Dav1dTaskContext) -> ()>;
pub type filter_sbrow_fn = Option<unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> ()>;
pub type recon_b_inter_fn =
    Option<unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, *const Av1Block) -> libc::c_int>;
pub type recon_b_intra_fn = Option<
    unsafe extern "C" fn(*mut Dav1dTaskContext, BlockSize, EdgeFlags, *const Av1Block) -> (),
>;
use crate::src::internal::ScalableMotionParams;
use crate::src::levels::BlockLevel;
use crate::src::levels::TX_4X4;
use crate::src::levels::TX_64X64;
use crate::src::levels::TX_8X8;

use crate::src::levels::IntraPredMode;
use crate::src::levels::RectTxfmSize;
use crate::src::levels::BL_128X128;
use crate::src::levels::BL_64X64;
use crate::src::levels::BL_8X8;
use crate::src::levels::FILTER_PRED;
use crate::src::levels::N_RECT_TX_SIZES;

use crate::src::levels::CFL_PRED;
use crate::src::levels::DC_PRED;
use crate::src::levels::N_INTRA_PRED_MODES;
use crate::src::levels::N_UV_INTRA_PRED_MODES;
use crate::src::levels::VERT_LEFT_PRED;
use crate::src::levels::VERT_PRED;

use crate::src::levels::BlockPartition;
use crate::src::levels::N_INTER_INTRA_PRED_MODES;
use crate::src::levels::PARTITION_H;
use crate::src::levels::PARTITION_H4;
use crate::src::levels::PARTITION_NONE;
use crate::src::levels::PARTITION_SPLIT;
use crate::src::levels::PARTITION_T_BOTTOM_SPLIT;
use crate::src::levels::PARTITION_T_LEFT_SPLIT;
use crate::src::levels::PARTITION_T_RIGHT_SPLIT;
use crate::src::levels::PARTITION_T_TOP_SPLIT;
use crate::src::levels::PARTITION_V;
use crate::src::levels::PARTITION_V4;

use crate::src::levels::InterPredMode;
use crate::src::levels::MV_JOINT_H;
use crate::src::levels::MV_JOINT_HV;
use crate::src::levels::MV_JOINT_V;
use crate::src::levels::N_MV_JOINTS;

use crate::src::levels::GLOBALMV;
use crate::src::levels::NEARESTMV;
use crate::src::levels::NEARMV;
use crate::src::levels::NEWMV;

use crate::src::levels::CompInterPredMode;
use crate::src::levels::GLOBALMV_GLOBALMV;
use crate::src::levels::NEARER_DRL;
use crate::src::levels::NEAREST_DRL;
use crate::src::levels::NEARISH_DRL;
use crate::src::levels::NEAR_DRL;
use crate::src::levels::NEWMV_NEWMV;
use crate::src::levels::N_COMP_INTER_PRED_MODES;

use crate::src::levels::NEARESTMV_NEARESTMV;

use crate::src::levels::COMP_INTER_AVG;
use crate::src::levels::COMP_INTER_NONE;
use crate::src::levels::COMP_INTER_SEG;
use crate::src::levels::COMP_INTER_WEDGE;
use crate::src::levels::COMP_INTER_WEIGHTED_AVG;

use crate::src::levels::INTER_INTRA_BLEND;
use crate::src::levels::INTER_INTRA_NONE;
use crate::src::levels::INTER_INTRA_WEDGE;

use crate::include::common::attributes::ctz;
use crate::src::levels::MM_OBMC;
use crate::src::levels::MM_TRANSLATION;
use crate::src::levels::MM_WARP;
use crate::src::refmvs::refmvs_candidate;
use crate::src::tables::TxfmInfo;

use crate::include::common::intops::iclip;
use crate::include::common::intops::iclip_u8;
use crate::include::common::intops::imax;
use crate::include::common::intops::imin;

use crate::include::common::intops::apply_sign64;
use crate::include::common::intops::ulog2;
use crate::src::mem::dav1d_alloc_aligned;
use crate::src::mem::dav1d_free_aligned;
use crate::src::mem::dav1d_freep_aligned;
use crate::src::mem::freep;
use crate::src::r#ref::dav1d_ref_inc;

use crate::src::tables::cfl_allowed_mask;
use crate::src::tables::interintra_allowed_mask;
use crate::src::tables::wedge_allowed_mask;

use crate::src::env::av1_get_bwd_ref_1_ctx;
use crate::src::env::av1_get_bwd_ref_ctx;
use crate::src::env::av1_get_fwd_ref_1_ctx;
use crate::src::env::av1_get_fwd_ref_2_ctx;
use crate::src::env::av1_get_fwd_ref_ctx;
use crate::src::env::av1_get_ref_ctx;
use crate::src::env::av1_get_uni_p1_ctx;
use crate::src::env::fix_mv_precision;
use crate::src::env::gather_left_partition_prob;
use crate::src::env::gather_top_partition_prob;
use crate::src::env::get_comp_ctx;
use crate::src::env::get_comp_dir_ctx;
use crate::src::env::get_cur_frame_segid;
use crate::src::env::get_drl_context;
use crate::src::env::get_filter_ctx;
use crate::src::env::get_gmv_2d;
use crate::src::env::get_intra_ctx;
use crate::src::env::get_jnt_comp_ctx;
use crate::src::env::get_mask_comp_ctx;
use crate::src::env::get_partition_ctx;
use crate::src::env::get_poc_diff;
use crate::src::env::get_tx_ctx;

use crate::src::msac::dav1d_msac_decode_bools;
use crate::src::msac::dav1d_msac_decode_uniform;

use crate::src::recon::define_DEBUG_BLOCK_INFO;

define_DEBUG_BLOCK_INFO!();

fn init_quant_tables(
    seq_hdr: &Dav1dSequenceHeader,
    frame_hdr: &Dav1dFrameHeader,
    qidx: libc::c_int,
    mut dq: &mut [[[uint16_t; 2]; 3]],
) {
    let tbl = &dav1d_dq_tbl;

    let segmentation_is_enabled = frame_hdr.segmentation.enabled != 0;
    let len = if segmentation_is_enabled { 8 } else { 1 };
    for i in 0..len {
        let yac = if segmentation_is_enabled {
            iclip_u8(qidx + frame_hdr.segmentation.seg_data.d[i].delta_q)
        } else {
            qidx
        };
        let ydc = iclip_u8(yac + frame_hdr.quant.ydc_delta);
        let uac = iclip_u8(yac + frame_hdr.quant.uac_delta);
        let udc = iclip_u8(yac + frame_hdr.quant.udc_delta);
        let vac = iclip_u8(yac + frame_hdr.quant.vac_delta);
        let vdc = iclip_u8(yac + frame_hdr.quant.vdc_delta);
        dq[i][0][0] = tbl[seq_hdr.hbd as usize][ydc as usize][0];
        dq[i][0][1] = tbl[seq_hdr.hbd as usize][yac as usize][1];
        dq[i][1][0] = tbl[seq_hdr.hbd as usize][udc as usize][0];
        dq[i][1][1] = tbl[seq_hdr.hbd as usize][uac as usize][1];
        dq[i][2][0] = tbl[seq_hdr.hbd as usize][vdc as usize][0];
        dq[i][2][1] = tbl[seq_hdr.hbd as usize][vac as usize][1];
    }
}

unsafe fn read_mv_component_diff(
    t: &mut Dav1dTaskContext,
    mv_comp: &mut CdfMvComponent,
    have_fp: bool,
) -> libc::c_int {
    let ts = &mut *t.ts;
    let f = &*t.f;
    let have_hp = (*f.frame_hdr).hp != 0;
    let sign = dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.sign.0);
    let cl = dav1d_msac_decode_symbol_adapt16(&mut ts.msac, &mut mv_comp.classes.0, 10);
    let mut up;
    let mut fp;
    let mut hp;

    if cl == 0 {
        up = dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.class0.0) as libc::c_uint;
        if have_fp {
            fp = dav1d_msac_decode_symbol_adapt4(
                &mut ts.msac,
                &mut mv_comp.class0_fp[up as usize],
                3,
            );
            hp = if have_hp {
                dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.class0_hp.0)
            } else {
                true
            };
        } else {
            fp = 3;
            hp = true;
        }
    } else {
        up = 1 << cl;
        for n in 0..cl as usize {
            up |= (dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.classN[n])
                as libc::c_uint)
                << n;
        }
        if have_fp {
            fp = dav1d_msac_decode_symbol_adapt4(&mut ts.msac, &mut mv_comp.classN_fp.0, 3);
            hp = if have_hp {
                dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut mv_comp.classN_hp.0)
            } else {
                true
            };
        } else {
            fp = 3;
            hp = true;
        }
    }
    let hp = hp as libc::c_uint;

    let diff = ((up << 3 | fp << 1 | hp) + 1) as libc::c_int;

    if sign {
        -diff
    } else {
        diff
    }
}

unsafe fn read_mv_residual(
    t: &mut Dav1dTaskContext,
    ref_mv: &mut mv,
    mv_cdf: &mut CdfMvContext,
    have_fp: bool,
) {
    let ts = &mut *t.ts;
    match dav1d_msac_decode_symbol_adapt4(
        &mut ts.msac,
        &mut ts.cdf.mv.joint.0,
        N_MV_JOINTS as size_t - 1,
    ) {
        MV_JOINT_HV => {
            ref_mv.y += read_mv_component_diff(t, &mut mv_cdf.comp[0], have_fp) as i16;
            ref_mv.x += read_mv_component_diff(t, &mut mv_cdf.comp[1], have_fp) as i16;
        }
        MV_JOINT_H => {
            ref_mv.x += read_mv_component_diff(t, &mut mv_cdf.comp[1], have_fp) as i16;
        }
        MV_JOINT_V => {
            ref_mv.y += read_mv_component_diff(t, &mut mv_cdf.comp[0], have_fp) as i16;
        }
        _ => {}
    };
}

unsafe extern "C" fn read_tx_tree(
    t: *mut Dav1dTaskContext,
    from: RectTxfmSize,
    depth: libc::c_int,
    masks: *mut uint16_t,
    x_off: libc::c_int,
    y_off: libc::c_int,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    let bx4 = (*t).bx & 31;
    let by4 = (*t).by & 31;
    let t_dim: *const TxfmInfo =
        &*dav1d_txfm_dimensions.as_ptr().offset(from as isize) as *const TxfmInfo;
    let txw = (*t_dim).lw as libc::c_int;
    let txh = (*t_dim).lh as libc::c_int;
    let mut is_split = 0;
    if depth < 2 && from as libc::c_uint > TX_4X4 as libc::c_int as libc::c_uint {
        let cat =
            2 as libc::c_int * (TX_64X64 as libc::c_int - (*t_dim).max as libc::c_int) - depth;
        let a = (((*(*t).a).tx.0[bx4 as usize] as libc::c_int) < txw) as libc::c_int;
        let l = (((*t).l.tx.0[by4 as usize] as libc::c_int) < txh) as libc::c_int;
        is_split = dav1d_msac_decode_bool_adapt(
            &mut (*(*t).ts).msac,
            &mut (*(*t).ts).cdf.m.txpart[cat as usize][(a + l) as usize],
        ) as libc::c_int;
        if is_split != 0 {
            let ref mut fresh1 = *masks.offset(depth as isize);
            *fresh1 =
                (*fresh1 as libc::c_int | (1 as libc::c_int) << y_off * 4 + x_off) as uint16_t;
        }
    } else {
        is_split = 0 as libc::c_int;
    }
    if is_split != 0 && (*t_dim).max as libc::c_int > TX_8X8 as libc::c_int {
        let sub: RectTxfmSize = (*t_dim).sub as RectTxfmSize;
        let sub_t_dim: *const TxfmInfo =
            &*dav1d_txfm_dimensions.as_ptr().offset(sub as isize) as *const TxfmInfo;
        let txsw = (*sub_t_dim).w as libc::c_int;
        let txsh = (*sub_t_dim).h as libc::c_int;
        read_tx_tree(t, sub, depth + 1, masks, x_off * 2 + 0, y_off * 2 + 0);
        (*t).bx += txsw;
        if txw >= txh && (*t).bx < (*f).bw {
            read_tx_tree(t, sub, depth + 1, masks, x_off * 2 + 1, y_off * 2 + 0);
        }
        (*t).bx -= txsw;
        (*t).by += txsh;
        if txh >= txw && (*t).by < (*f).bh {
            read_tx_tree(t, sub, depth + 1, masks, x_off * 2 + 0, y_off * 2 + 1);
            (*t).bx += txsw;
            if txw >= txh && (*t).bx < (*f).bw {
                read_tx_tree(t, sub, depth + 1, masks, x_off * 2 + 1, y_off * 2 + 1);
            }
            (*t).bx -= txsw;
        }
        (*t).by -= txsh;
    } else {
        match (*t_dim).h as libc::c_int {
            1 => {
                (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut alias8))
                    .u8_0 = (if is_split != 0 {
                    TX_4X4 as libc::c_int
                } else {
                    0x1 * txh
                }) as uint8_t;
            }
            2 => {
                (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut alias16))
                    .u16_0 = (if is_split != 0 {
                    TX_4X4 as libc::c_int
                } else {
                    0x101 * txh
                }) as uint16_t;
            }
            4 => {
                (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut alias32))
                    .u32_0 = if is_split != 0 {
                    TX_4X4 as libc::c_int as libc::c_uint
                } else {
                    (0x1010101 as libc::c_uint).wrapping_mul(txh as libc::c_uint)
                };
            }
            8 => {
                (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = (if is_split != 0 {
                    TX_4X4 as libc::c_int as libc::c_ulonglong
                } else {
                    (0x101010101010101 as libc::c_ulonglong).wrapping_mul(txh as libc::c_ulonglong)
                }) as uint64_t;
            }
            16 => {
                let const_val: uint64_t = (if is_split != 0 {
                    TX_4X4 as libc::c_int as libc::c_ulonglong
                } else {
                    (0x101010101010101 as libc::c_ulonglong).wrapping_mul(txh as libc::c_ulonglong)
                }) as uint64_t;
                (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val;
                (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val;
            }
            _ => {}
        }
        match (*t_dim).w as libc::c_int {
            1 => {
                (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut alias8))
                    .u8_0 = (if is_split != 0 {
                    TX_4X4 as libc::c_int
                } else {
                    0x1 * txw
                }) as uint8_t;
            }
            2 => {
                (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut alias16))
                    .u16_0 = (if is_split != 0 {
                    TX_4X4 as libc::c_int
                } else {
                    0x101 * txw
                }) as uint16_t;
            }
            4 => {
                (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut alias32))
                    .u32_0 = if is_split != 0 {
                    TX_4X4 as libc::c_int as libc::c_uint
                } else {
                    (0x1010101 as libc::c_uint).wrapping_mul(txw as libc::c_uint)
                };
            }
            8 => {
                (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = (if is_split != 0 {
                    TX_4X4 as libc::c_int as libc::c_ulonglong
                } else {
                    (0x101010101010101 as libc::c_ulonglong).wrapping_mul(txw as libc::c_ulonglong)
                }) as uint64_t;
            }
            16 => {
                let const_val_0: uint64_t = (if is_split != 0 {
                    TX_4X4 as libc::c_int as libc::c_ulonglong
                } else {
                    (0x101010101010101 as libc::c_ulonglong).wrapping_mul(txw as libc::c_ulonglong)
                }) as uint64_t;
                (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
                (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_0;
            }
            _ => {}
        }
    };
}

fn neg_deinterleave(
    mut diff: libc::c_int,
    mut r#ref: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    if r#ref == 0 {
        diff
    } else if r#ref >= max - 1 {
        max - diff - 1
    } else if 2 * r#ref < max {
        if diff <= 2 * r#ref {
            if diff & 1 != 0 {
                r#ref + (diff + 1 >> 1)
            } else {
                r#ref - (diff >> 1)
            }
        } else {
            diff
        }
    } else {
        if diff <= 2 * (max - r#ref - 1) {
            if diff & 1 != 0 {
                r#ref + (diff + 1 >> 1)
            } else {
                r#ref - (diff >> 1)
            }
        } else {
            max - (diff + 1)
        }
    }
}

unsafe extern "C" fn find_matching_ref(
    t: *const Dav1dTaskContext,
    intra_edge_flags: EdgeFlags,
    bw4: libc::c_int,
    bh4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    have_left: bool,
    have_top: bool,
    r#ref: libc::c_int,
    mut masks: *mut uint64_t,
) {
    let mut r: *const *mut refmvs_block =
        &*((*t).rt.r).as_ptr().offset((((*t).by & 31) + 5) as isize) as *const *mut refmvs_block;
    let mut count = 0;
    let mut have_topleft = (have_top && have_left) as libc::c_int;
    let mut have_topright = (imax(bw4, bh4) < 32
        && have_top
        && (*t).bx + bw4 < (*(*t).ts).tiling.col_end
        && intra_edge_flags as libc::c_uint
            & EDGE_I444_TOP_HAS_RIGHT as libc::c_int as libc::c_uint
            != 0) as libc::c_int;
    if have_top {
        let mut r2: *const refmvs_block = &mut *(*r.offset(-(1 as libc::c_int) as isize))
            .offset((*t).bx as isize)
            as *mut refmvs_block;
        if (*r2).0.r#ref.r#ref[0] as libc::c_int == r#ref + 1
            && (*r2).0.r#ref.r#ref[1] as libc::c_int == -(1 as libc::c_int)
        {
            let ref mut fresh2 = *masks.offset(0);
            *fresh2 |= 1;
            count = 1 as libc::c_int;
        }
        let mut aw4 = dav1d_block_dimensions[(*r2).0.bs as usize][0] as libc::c_int;
        if aw4 >= bw4 {
            let off = (*t).bx & aw4 - 1;
            if off != 0 {
                have_topleft = 0 as libc::c_int;
            }
            if aw4 - off > bw4 {
                have_topright = 0 as libc::c_int;
            }
        } else {
            let mut mask: libc::c_uint = ((1 as libc::c_int) << aw4) as libc::c_uint;
            let mut x = aw4;
            while x < w4 {
                r2 = r2.offset(aw4 as isize);
                if (*r2).0.r#ref.r#ref[0] as libc::c_int == r#ref + 1
                    && (*r2).0.r#ref.r#ref[1] as libc::c_int == -(1 as libc::c_int)
                {
                    let ref mut fresh3 = *masks.offset(0);
                    *fresh3 |= mask as uint64_t;
                    count += 1;
                    if count >= 8 {
                        return;
                    }
                }
                aw4 = dav1d_block_dimensions[(*r2).0.bs as usize][0] as libc::c_int;
                mask <<= aw4;
                x += aw4;
            }
        }
    }
    if have_left {
        let mut r2_0: *const *mut refmvs_block = r;
        if (*(*r2_0.offset(0)).offset(((*t).bx - 1) as isize))
            .0
            .r#ref
            .r#ref[0] as libc::c_int
            == r#ref + 1
            && (*(*r2_0.offset(0)).offset(((*t).bx - 1) as isize))
                .0
                .r#ref
                .r#ref[1] as libc::c_int
                == -(1 as libc::c_int)
        {
            let ref mut fresh4 = *masks.offset(1);
            *fresh4 |= 1;
            count += 1;
            if count >= 8 {
                return;
            }
        }
        let mut lh4 = dav1d_block_dimensions
            [(*(*r2_0.offset(0)).offset(((*t).bx - 1) as isize)).0.bs as usize][1]
            as libc::c_int;
        if lh4 >= bh4 {
            if (*t).by & lh4 - 1 != 0 {
                have_topleft = 0 as libc::c_int;
            }
        } else {
            let mut mask_0: libc::c_uint = ((1 as libc::c_int) << lh4) as libc::c_uint;
            let mut y = lh4;
            while y < h4 {
                r2_0 = r2_0.offset(lh4 as isize);
                if (*(*r2_0.offset(0)).offset(((*t).bx - 1) as isize))
                    .0
                    .r#ref
                    .r#ref[0] as libc::c_int
                    == r#ref + 1
                    && (*(*r2_0.offset(0)).offset(((*t).bx - 1) as isize))
                        .0
                        .r#ref
                        .r#ref[1] as libc::c_int
                        == -(1 as libc::c_int)
                {
                    let ref mut fresh5 = *masks.offset(1);
                    *fresh5 |= mask_0 as uint64_t;
                    count += 1;
                    if count >= 8 {
                        return;
                    }
                }
                lh4 = dav1d_block_dimensions
                    [(*(*r2_0.offset(0)).offset(((*t).bx - 1) as isize)).0.bs as usize][1]
                    as libc::c_int;
                mask_0 <<= lh4;
                y += lh4;
            }
        }
    }
    if have_topleft != 0
        && ((*(*r.offset(-(1 as libc::c_int) as isize)).offset(((*t).bx - 1) as isize))
            .0
            .r#ref
            .r#ref[0] as libc::c_int
            == r#ref + 1
            && (*(*r.offset(-(1 as libc::c_int) as isize)).offset(((*t).bx - 1) as isize))
                .0
                .r#ref
                .r#ref[1] as libc::c_int
                == -(1 as libc::c_int))
    {
        let ref mut fresh6 = *masks.offset(1);
        *fresh6 = (*fresh6 as libc::c_ulonglong | (1 as libc::c_ulonglong) << 32) as uint64_t;
        count += 1;
        if count >= 8 {
            return;
        }
    }
    if have_topright != 0
        && ((*(*r.offset(-(1 as libc::c_int) as isize)).offset(((*t).bx + bw4) as isize))
            .0
            .r#ref
            .r#ref[0] as libc::c_int
            == r#ref + 1
            && (*(*r.offset(-(1 as libc::c_int) as isize)).offset(((*t).bx + bw4) as isize))
                .0
                .r#ref
                .r#ref[1] as libc::c_int
                == -(1 as libc::c_int))
    {
        let ref mut fresh7 = *masks.offset(0);
        *fresh7 = (*fresh7 as libc::c_ulonglong | (1 as libc::c_ulonglong) << 32) as uint64_t;
    }
}

unsafe fn derive_warpmv(
    t: &Dav1dTaskContext,
    bw4: libc::c_int,
    bh4: libc::c_int,
    masks: &[u64; 2],
    mv: mv,
    mut wmp: Dav1dWarpedMotionParams,
) -> Dav1dWarpedMotionParams {
    let mut pts = [[[0; 2 /* x, y */]; 2 /* in, out */]; 8];
    let mut np = 0;
    let r = |i: isize| {
        // Need to use a closure here vs. a slice because `i` can be negative
        // (and not just by a constant -1).
        // See `-off` below.
        let offset = (t.by & 31) + 5;
        t.rt.r[(offset as isize + i) as usize]
    };

    let rp = |i: i32, j: i32| &*r(i as isize).offset(j as isize);

    let bs = |rp: &refmvs_block| dav1d_block_dimensions[(*rp).0.bs as usize];

    let mut add_sample = |np: usize, dx: i32, dy: i32, sx: i32, sy: i32, rp: &refmvs_block| {
        pts[np][0][0] = 16 * (2 * dx + sx * bs(rp)[0] as i32) - 8;
        pts[np][0][1] = 16 * (2 * dy + sy * bs(rp)[1] as i32) - 8;
        pts[np][1][0] = pts[np][0][0] + (*rp).0.mv.mv[0].x as i32;
        pts[np][1][1] = pts[np][0][1] + (*rp).0.mv.mv[0].y as i32;
        np + 1
    };

    // use masks[] to find the projectable motion vectors in the edges
    if masks[0] as u32 == 1 && masks[1] >> 32 == 0 {
        let off = t.bx & bs(rp(-1, t.bx))[0] as i32 - 1;
        np = add_sample(np, -off, 0, 1, -1, rp(-1, t.bx));
    } else {
        let mut off = 0;
        let mut xmask = masks[0] as u32;
        while np < 8 && xmask != 0 {
            let tz = ctz(xmask);
            off += tz;
            xmask >>= tz;
            np = add_sample(np, off, 0, 1, -1, rp(-1, t.bx + off));
            xmask &= !1;
        }
    }
    if np < 8 && masks[1] as u32 == 1 {
        let off = t.by & bs(rp(0, t.bx - 1))[1] as i32 - 1;
        np = add_sample(np, 0, -off, -1, 1, rp(-off, t.bx - 1));
    } else {
        let mut off = 0;
        let mut ymask = masks[1] as u32;
        while np < 8 && ymask != 0 {
            let tz = ctz(ymask);
            off += tz;
            ymask >>= tz;
            np = add_sample(np, 0, off, -1, 1, rp(off, t.bx - 1));
            ymask &= !1;
        }
    }
    if np < 8 && masks[1] >> 32 != 0 {
        // top/left
        np = add_sample(np, 0, 0, -1, -1, rp(-1, t.bx - 1));
    }
    if np < 8 && masks[0] >> 32 != 0 {
        // top/right
        np = add_sample(np, bw4, 0, 1, -1, rp(-1, t.bx + bw4));
    }
    assert!(np > 0 && np <= 8);

    // select according to motion vector difference against a threshold
    let mut mvd = [0; 8];
    let mut ret = 0;
    let thresh = 4 * iclip(imax(bw4, bh4), 4, 28);
    for (mvd, pts) in std::iter::zip(&mut mvd[..np], &pts[..np]) {
        *mvd = (pts[1][0] - pts[0][0] - mv.x as i32).abs()
            + (pts[1][1] - pts[0][1] - mv.y as i32).abs();
        if *mvd > thresh {
            *mvd = -1;
        } else {
            ret += 1;
        }
    }
    if ret == 0 {
        ret = 1;
    } else {
        let mut i = 0;
        let mut j = np - 1;
        for _ in 0..np - ret {
            while mvd[i] != -1 {
                i += 1;
            }
            while mvd[j] == -1 {
                j -= 1;
            }
            assert!(i != j);
            if i > j {
                break;
            }
            // replace the discarded samples;
            mvd[i] = mvd[j];
            pts[i] = pts[j];
            i += 1;
            j -= 1;
        }
    }

    wmp.type_0 = if !dav1d_find_affine_int(&pts, ret, bw4, bh4, mv, &mut wmp, t.bx, t.by)
        && !dav1d_get_shear_params(&mut wmp)
    {
        DAV1D_WM_TYPE_AFFINE
    } else {
        DAV1D_WM_TYPE_IDENTITY
    };
    wmp
}

#[inline]
fn findoddzero(buf: &[u8]) -> bool {
    buf.iter()
        .enumerate()
        .find(|(i, &e)| i & 1 == 1 && e == 0)
        .is_some()
}

unsafe extern "C" fn read_pal_plane(
    t: *mut Dav1dTaskContext,
    b: *mut Av1Block,
    pl: libc::c_int,
    sz_ctx: libc::c_int,
    bx4: libc::c_int,
    by4: libc::c_int,
) {
    let ts: *mut Dav1dTileState = (*t).ts;
    let f: *const Dav1dFrameContext = (*t).f;
    (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[pl as usize] = (dav1d_msac_decode_symbol_adapt8(
        &mut (*ts).msac,
        &mut (*ts).cdf.m.pal_sz[pl as usize][sz_ctx as usize],
        6 as libc::c_int as size_t,
    ))
    .wrapping_add(2 as libc::c_int as libc::c_uint)
        as uint8_t;
    let pal_sz = (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[pl as usize] as libc::c_int;
    let mut cache: [uint16_t; 16] = [0; 16];
    let mut used_cache: [uint16_t; 8] = [0; 8];
    let mut l_cache = if pl != 0 {
        (*t).pal_sz_uv[1][by4 as usize] as libc::c_int
    } else {
        (*t).l.pal_sz.0[by4 as usize] as libc::c_int
    };
    let mut n_cache = 0;
    let mut a_cache = if by4 & 15 != 0 {
        if pl != 0 {
            (*t).pal_sz_uv[0][bx4 as usize] as libc::c_int
        } else {
            (*(*t).a).pal_sz.0[bx4 as usize] as libc::c_int
        }
    } else {
        0 as libc::c_int
    };
    let mut l: *const uint16_t = ((*t).al_pal[1][by4 as usize][pl as usize]).as_mut_ptr();
    let mut a: *const uint16_t = ((*t).al_pal[0][bx4 as usize][pl as usize]).as_mut_ptr();
    while l_cache != 0 && a_cache != 0 {
        if (*l as libc::c_int) < *a as libc::c_int {
            if n_cache == 0 || cache[(n_cache - 1) as usize] as libc::c_int != *l as libc::c_int {
                let fresh8 = n_cache;
                n_cache = n_cache + 1;
                cache[fresh8 as usize] = *l;
            }
            l = l.offset(1);
            l_cache -= 1;
        } else {
            if *a as libc::c_int == *l as libc::c_int {
                l = l.offset(1);
                l_cache -= 1;
            }
            if n_cache == 0 || cache[(n_cache - 1) as usize] as libc::c_int != *a as libc::c_int {
                let fresh9 = n_cache;
                n_cache = n_cache + 1;
                cache[fresh9 as usize] = *a;
            }
            a = a.offset(1);
            a_cache -= 1;
        }
    }
    if l_cache != 0 {
        loop {
            if n_cache == 0 || cache[(n_cache - 1) as usize] as libc::c_int != *l as libc::c_int {
                let fresh10 = n_cache;
                n_cache = n_cache + 1;
                cache[fresh10 as usize] = *l;
            }
            l = l.offset(1);
            l_cache -= 1;
            if !(l_cache > 0) {
                break;
            }
        }
    } else if a_cache != 0 {
        loop {
            if n_cache == 0 || cache[(n_cache - 1) as usize] as libc::c_int != *a as libc::c_int {
                let fresh11 = n_cache;
                n_cache = n_cache + 1;
                cache[fresh11 as usize] = *a;
            }
            a = a.offset(1);
            a_cache -= 1;
            if !(a_cache > 0) {
                break;
            }
        }
    }
    let mut i = 0;
    let mut n = 0;
    while n < n_cache && i < pal_sz {
        if dav1d_msac_decode_bool_equi(&mut (*ts).msac) {
            let fresh12 = i;
            i = i + 1;
            used_cache[fresh12 as usize] = cache[n as usize];
        }
        n += 1;
    }
    let n_used_cache = i;
    let pal: *mut uint16_t = if (*t).frame_thread.pass != 0 {
        ((*((*f).frame_thread.pal).offset(
            ((((*t).by >> 1) + ((*t).bx & 1)) as isize * ((*f).b4_stride >> 1)
                + (((*t).bx >> 1) + ((*t).by & 1)) as isize) as isize,
        ))[pl as usize])
            .as_mut_ptr()
    } else {
        ((*t).scratch.c2rust_unnamed_0.pal[pl as usize]).as_mut_ptr()
    };
    if i < pal_sz {
        let fresh13 = i;
        i = i + 1;
        let ref mut fresh14 = *pal.offset(fresh13 as isize);
        *fresh14 =
            dav1d_msac_decode_bools(&mut (*ts).msac, (*f).cur.p.bpc as libc::c_uint) as uint16_t;
        let mut prev = *fresh14 as libc::c_int;
        if i < pal_sz {
            let mut bits = (((*f).cur.p.bpc - 3) as libc::c_uint).wrapping_add(
                dav1d_msac_decode_bools(&mut (*ts).msac, 2 as libc::c_int as libc::c_uint),
            ) as libc::c_int;
            let max = ((1 as libc::c_int) << (*f).cur.p.bpc) - 1;
            loop {
                let delta =
                    dav1d_msac_decode_bools(&mut (*ts).msac, bits as libc::c_uint) as libc::c_int;
                let fresh15 = i;
                i = i + 1;
                let ref mut fresh16 = *pal.offset(fresh15 as isize);
                *fresh16 = imin(prev + delta + (pl == 0) as libc::c_int, max) as uint16_t;
                prev = *fresh16 as libc::c_int;
                if prev + (pl == 0) as libc::c_int >= max {
                    while i < pal_sz {
                        *pal.offset(i as isize) = max as uint16_t;
                        i += 1;
                    }
                    break;
                } else {
                    bits = imin(
                        bits,
                        1 as libc::c_int
                            + ulog2((max - prev - (pl == 0) as libc::c_int) as libc::c_uint),
                    );
                    if !(i < pal_sz) {
                        break;
                    }
                }
            }
        }
        let mut n_0 = 0;
        let mut m = n_used_cache;
        i = 0 as libc::c_int;
        while i < pal_sz {
            if n_0 < n_used_cache
                && (m >= pal_sz
                    || used_cache[n_0 as usize] as libc::c_int
                        <= *pal.offset(m as isize) as libc::c_int)
            {
                let fresh17 = n_0;
                n_0 = n_0 + 1;
                *pal.offset(i as isize) = used_cache[fresh17 as usize];
            } else {
                if !(m < pal_sz) {
                    unreachable!();
                }
                let fresh18 = m;
                m = m + 1;
                *pal.offset(i as isize) = *pal.offset(fresh18 as isize);
            }
            i += 1;
        }
    } else {
        memcpy(
            pal as *mut libc::c_void,
            used_cache.as_mut_ptr() as *const libc::c_void,
            (n_used_cache as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
        );
    }
    if DEBUG_BLOCK_INFO(&*f, &*t) {
        printf(
            b"Post-pal[pl=%d,sz=%d,cache_size=%d,used_cache=%d]: r=%d, cache=\0" as *const u8
                as *const libc::c_char,
            pl,
            pal_sz,
            n_cache,
            n_used_cache,
            (*ts).msac.rng,
        );
        let mut n_1 = 0;
        while n_1 < n_cache {
            printf(
                b"%c%02x\0" as *const u8 as *const libc::c_char,
                if n_1 != 0 { ' ' as i32 } else { '[' as i32 },
                cache[n_1 as usize] as libc::c_int,
            );
            n_1 += 1;
        }
        printf(
            b"%s, pal=\0" as *const u8 as *const libc::c_char,
            if n_cache != 0 {
                b"]\0" as *const u8 as *const libc::c_char
            } else {
                b"[]\0" as *const u8 as *const libc::c_char
            },
        );
        let mut n_2 = 0;
        while n_2 < pal_sz {
            printf(
                b"%c%02x\0" as *const u8 as *const libc::c_char,
                if n_2 != 0 { ' ' as i32 } else { '[' as i32 },
                *pal.offset(n_2 as isize) as libc::c_int,
            );
            n_2 += 1;
        }
        printf(b"]\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn read_pal_uv(
    t: *mut Dav1dTaskContext,
    b: *mut Av1Block,
    sz_ctx: libc::c_int,
    bx4: libc::c_int,
    by4: libc::c_int,
) {
    read_pal_plane(t, b, 1 as libc::c_int, sz_ctx, bx4, by4);
    let ts: *mut Dav1dTileState = (*t).ts;
    let f: *const Dav1dFrameContext = (*t).f;
    let pal: *mut uint16_t = if (*t).frame_thread.pass != 0 {
        ((*((*f).frame_thread.pal).offset(
            ((((*t).by >> 1) + ((*t).bx & 1)) as isize * ((*f).b4_stride >> 1)
                + (((*t).bx >> 1) + ((*t).by & 1)) as isize) as isize,
        ))[2])
            .as_mut_ptr()
    } else {
        ((*t).scratch.c2rust_unnamed_0.pal[2]).as_mut_ptr()
    };
    if dav1d_msac_decode_bool_equi(&mut (*ts).msac) {
        let bits = (((*f).cur.p.bpc - 4) as libc::c_uint).wrapping_add(dav1d_msac_decode_bools(
            &mut (*ts).msac,
            2 as libc::c_int as libc::c_uint,
        )) as libc::c_int;
        let ref mut fresh19 = *pal.offset(0);
        *fresh19 =
            dav1d_msac_decode_bools(&mut (*ts).msac, (*f).cur.p.bpc as libc::c_uint) as uint16_t;
        let mut prev = *fresh19 as libc::c_int;
        let max = ((1 as libc::c_int) << (*f).cur.p.bpc) - 1;
        let mut i = 1;
        while i < (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1] as libc::c_int {
            let mut delta =
                dav1d_msac_decode_bools(&mut (*ts).msac, bits as libc::c_uint) as libc::c_int;
            if delta != 0 && dav1d_msac_decode_bool_equi(&mut (*ts).msac) {
                delta = -delta;
            }
            let ref mut fresh20 = *pal.offset(i as isize);
            *fresh20 = (prev + delta & max) as uint16_t;
            prev = *fresh20 as libc::c_int;
            i += 1;
        }
    } else {
        let mut i_0 = 0;
        while i_0 < (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1] as libc::c_int {
            *pal.offset(i_0 as isize) =
                dav1d_msac_decode_bools(&mut (*ts).msac, (*f).cur.p.bpc as libc::c_uint)
                    as uint16_t;
            i_0 += 1;
        }
    }
    if DEBUG_BLOCK_INFO(&*f, &*t) {
        printf(
            b"Post-pal[pl=2]: r=%d \0" as *const u8 as *const libc::c_char,
            (*ts).msac.rng,
        );
        let mut n = 0;
        while n < (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[1] as libc::c_int {
            printf(
                b"%c%02x\0" as *const u8 as *const libc::c_char,
                if n != 0 { ' ' as i32 } else { '[' as i32 },
                *pal.offset(n as isize) as libc::c_int,
            );
            n += 1;
        }
        printf(b"]\n\0" as *const u8 as *const libc::c_char);
    }
}

unsafe fn order_palette(
    mut pal_idx: *const uint8_t,
    stride: ptrdiff_t,
    i: libc::c_int,
    first: libc::c_int,
    last: libc::c_int,
    order: &mut [[u8; u8::BITS as usize]; 64],
    ctx: &mut [u8; 64],
) {
    let mut have_top = i > first;

    assert!(!pal_idx.is_null());
    pal_idx = pal_idx.offset(first as isize + (i - first) as isize * stride);
    for ((ctx, order), j) in ctx
        .iter_mut()
        .zip(order.iter_mut())
        .zip((last..=first).rev())
    {
        let have_left = j > 0;

        assert!(have_left || have_top);

        let mut mask = 0u8;
        let mut o_idx = 0;
        let mut add = |v: u8| {
            assert!(v < u8::BITS as u8);
            order[o_idx] = v;
            o_idx += 1;
            mask |= 1 << v;
        };

        if !have_left {
            *ctx = 0;
            add(*pal_idx.offset(-stride));
        } else if !have_top {
            *ctx = 0;
            add(*pal_idx.offset(-1));
        } else {
            let l = *pal_idx.offset(-1);
            let t = *pal_idx.offset(-stride);
            let tl = *pal_idx.offset(-(stride + 1));
            let same_t_l = t == l;
            let same_t_tl = t == tl;
            let same_l_tl = l == tl;
            let same_all = same_t_l & same_t_tl & same_l_tl;

            if same_all {
                *ctx = 4;
                add(t);
            } else if same_t_l {
                *ctx = 3;
                add(t);
                add(tl);
            } else if same_t_tl | same_l_tl {
                *ctx = 2;
                add(tl);
                add(if same_t_tl { l } else { t });
            } else {
                *ctx = 1;
                add(std::cmp::min(t, l));
                add(std::cmp::max(t, l));
                add(tl);
            }
        }
        for bit in 0..u8::BITS as u8 {
            if mask & (1 << bit) == 0 {
                order[o_idx] = bit;
                o_idx += 1;
            }
        }
        assert!(o_idx == u8::BITS as usize);
        have_top = true;
        pal_idx = pal_idx.offset(stride - 1);
    }
}

unsafe extern "C" fn read_pal_indices(
    t: *mut Dav1dTaskContext,
    pal_idx: *mut uint8_t,
    b: *const Av1Block,
    pl: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    bw4: libc::c_int,
    bh4: libc::c_int,
) {
    let ts: *mut Dav1dTileState = (*t).ts;
    let stride: ptrdiff_t = (bw4 * 4) as ptrdiff_t;
    if pal_idx.is_null() {
        unreachable!();
    }
    *pal_idx.offset(0) = dav1d_msac_decode_uniform(
        &mut (*ts).msac,
        (*b).c2rust_unnamed.c2rust_unnamed.pal_sz[pl as usize] as libc::c_uint,
    ) as uint8_t;
    let color_map_cdf: *mut [uint16_t; 8] = ((*ts).cdf.m.color_map[pl as usize]
        [((*b).c2rust_unnamed.c2rust_unnamed.pal_sz[pl as usize] as libc::c_int - 2) as usize])
        .as_mut_ptr();
    let order = &mut (*t)
        .scratch
        .c2rust_unnamed_0
        .c2rust_unnamed
        .c2rust_unnamed
        .pal_order;
    let ctx = &mut (*t)
        .scratch
        .c2rust_unnamed_0
        .c2rust_unnamed
        .c2rust_unnamed
        .pal_ctx;
    let mut i = 1;
    while i < 4 * (w4 + h4) - 1 {
        let first = imin(i, w4 * 4 - 1);
        let last = imax(0 as libc::c_int, i - h4 * 4 + 1);
        order_palette(pal_idx, stride, i, first, last, order, ctx);
        let mut j = first;
        let mut m = 0;
        while j >= last {
            let color_idx = dav1d_msac_decode_symbol_adapt8(
                &mut (*ts).msac,
                &mut *color_map_cdf.offset(ctx[m as usize] as isize),
                ((*b).c2rust_unnamed.c2rust_unnamed.pal_sz[pl as usize] as libc::c_int - 1)
                    as size_t,
            ) as libc::c_int;
            *pal_idx.offset(((i - j) as isize * stride + j as isize) as isize) =
                order[m as usize][color_idx as usize];
            j -= 1;
            m += 1;
        }
        i += 1;
    }
    if bw4 > w4 {
        let mut y = 0;
        while y < 4 * h4 {
            memset(
                &mut *pal_idx.offset((y as isize * stride + (4 * w4) as isize) as isize)
                    as *mut uint8_t as *mut libc::c_void,
                *pal_idx.offset((y as isize * stride + (4 * w4) as isize - 1) as isize)
                    as libc::c_int,
                (4 * (bw4 - w4)) as size_t,
            );
            y += 1;
        }
    }
    if h4 < bh4 {
        let src: *const uint8_t =
            &mut *pal_idx.offset(stride * (4 * h4 as isize - 1)) as *mut uint8_t;
        let mut y_0 = h4 * 4;
        while y_0 < bh4 * 4 {
            memcpy(
                &mut *pal_idx.offset((y_0 as isize * stride) as isize) as *mut uint8_t
                    as *mut libc::c_void,
                src as *const libc::c_void,
                (bw4 * 4) as libc::c_ulong,
            );
            y_0 += 1;
        }
    }
}
unsafe extern "C" fn read_vartx_tree(
    t: *mut Dav1dTaskContext,
    b: *mut Av1Block,
    bs: BlockSize,
    bx4: libc::c_int,
    by4: libc::c_int,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    let b_dim: *const uint8_t = (dav1d_block_dimensions[bs as usize]).as_ptr();
    let bw4 = *b_dim.offset(0) as libc::c_int;
    let bh4 = *b_dim.offset(1) as libc::c_int;
    let mut tx_split: [uint16_t; 2] = [0 as libc::c_int as uint16_t, 0];
    (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx = dav1d_max_txfm_size_for_bs[bs as usize][0];
    if (*b).skip == 0
        && ((*(*f).frame_hdr).segmentation.lossless[(*b).seg_id as usize] != 0
            || (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as libc::c_int == TX_4X4 as libc::c_int)
    {
        (*b).uvtx = TX_4X4 as libc::c_int as uint8_t;
        (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx = (*b).uvtx;
        if (*(*f).frame_hdr).txfm_mode as libc::c_uint
            == DAV1D_TX_SWITCHABLE as libc::c_int as libc::c_uint
        {
            match bh4 {
                1 => {
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                        as *mut alias8))
                        .u8_0 = TX_4X4 as libc::c_int as uint8_t;
                }
                2 => {
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                        as *mut alias16))
                        .u16_0 = TX_4X4 as libc::c_int as uint16_t;
                }
                4 => {
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                        as *mut alias32))
                        .u32_0 = TX_4X4 as libc::c_int as uint32_t;
                }
                8 => {
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = TX_4X4 as libc::c_int as uint64_t;
                }
                16 => {
                    let const_val: uint64_t = TX_4X4 as libc::c_int as uint64_t;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val;
                }
                32 => {
                    let const_val_0: uint64_t = TX_4X4 as libc::c_int as uint64_t;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_0;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_0;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_0;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_0;
                }
                _ => {}
            }
            match bw4 {
                1 => {
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                        as *mut alias8))
                        .u8_0 = TX_4X4 as libc::c_int as uint8_t;
                }
                2 => {
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                        as *mut alias16))
                        .u16_0 = TX_4X4 as libc::c_int as uint16_t;
                }
                4 => {
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                        as *mut alias32))
                        .u32_0 = TX_4X4 as libc::c_int as uint32_t;
                }
                8 => {
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = TX_4X4 as libc::c_int as uint64_t;
                }
                16 => {
                    let const_val_1: uint64_t = TX_4X4 as libc::c_int as uint64_t;
                    (*(&mut *((*(*t).a).tx.0)
                        .as_mut_ptr()
                        .offset((bx4 + 0) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_1;
                    (*(&mut *((*(*t).a).tx.0)
                        .as_mut_ptr()
                        .offset((bx4 + 8) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_1;
                }
                32 => {
                    let const_val_2: uint64_t = TX_4X4 as libc::c_int as uint64_t;
                    (*(&mut *((*(*t).a).tx.0)
                        .as_mut_ptr()
                        .offset((bx4 + 0) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_2;
                    (*(&mut *((*(*t).a).tx.0)
                        .as_mut_ptr()
                        .offset((bx4 + 8) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_2;
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset((bx4 + 16) as isize)
                        as *mut int8_t as *mut alias64))
                        .u64_0 = const_val_2;
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset((bx4 + 24) as isize)
                        as *mut int8_t as *mut alias64))
                        .u64_0 = const_val_2;
                }
                _ => {}
            }
        }
    } else if (*(*f).frame_hdr).txfm_mode as libc::c_uint
        != DAV1D_TX_SWITCHABLE as libc::c_int as libc::c_uint
        || (*b).skip as libc::c_int != 0
    {
        if (*(*f).frame_hdr).txfm_mode as libc::c_uint
            == DAV1D_TX_SWITCHABLE as libc::c_int as libc::c_uint
        {
            match bh4 {
                1 => {
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                        as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int
                        * *b_dim.offset((2 + 1) as isize) as libc::c_int)
                        as uint8_t;
                }
                2 => {
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                        as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int
                        * *b_dim.offset((2 + 1) as isize) as libc::c_int)
                        as uint16_t;
                }
                4 => {
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                        as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(*b_dim.offset((2 + 1) as isize) as libc::c_uint);
                }
                8 => {
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(*b_dim.offset((2 + 1) as isize) as libc::c_ulonglong)
                        as uint64_t;
                }
                16 => {
                    let const_val_3: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(*b_dim.offset((2 + 1) as isize) as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_3;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_3;
                }
                32 => {
                    let const_val_4: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(*b_dim.offset((2 + 1) as isize) as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_4;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_4;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_4;
                    (*(&mut *((*t).l.tx.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_4;
                }
                _ => {}
            }
            match bw4 {
                1 => {
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                        as *mut alias8))
                        .u8_0 = (0x1 as libc::c_int
                        * *b_dim.offset((2 + 0) as isize) as libc::c_int)
                        as uint8_t;
                }
                2 => {
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                        as *mut alias16))
                        .u16_0 = (0x101 as libc::c_int
                        * *b_dim.offset((2 + 0) as isize) as libc::c_int)
                        as uint16_t;
                }
                4 => {
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                        as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(*b_dim.offset((2 + 0) as isize) as libc::c_uint);
                }
                8 => {
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(*b_dim.offset((2 + 0) as isize) as libc::c_ulonglong)
                        as uint64_t;
                }
                16 => {
                    let const_val_5: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(*b_dim.offset((2 + 0) as isize) as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *((*(*t).a).tx.0)
                        .as_mut_ptr()
                        .offset((bx4 + 0) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                    (*(&mut *((*(*t).a).tx.0)
                        .as_mut_ptr()
                        .offset((bx4 + 8) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_5;
                }
                32 => {
                    let const_val_6: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(*b_dim.offset((2 + 0) as isize) as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *((*(*t).a).tx.0)
                        .as_mut_ptr()
                        .offset((bx4 + 0) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *((*(*t).a).tx.0)
                        .as_mut_ptr()
                        .offset((bx4 + 8) as isize) as *mut int8_t
                        as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset((bx4 + 16) as isize)
                        as *mut int8_t as *mut alias64))
                        .u64_0 = const_val_6;
                    (*(&mut *((*(*t).a).tx.0).as_mut_ptr().offset((bx4 + 24) as isize)
                        as *mut int8_t as *mut alias64))
                        .u64_0 = const_val_6;
                }
                _ => {}
            }
        }
        (*b).uvtx = dav1d_max_txfm_size_for_bs[bs as usize][(*f).cur.p.layout as usize];
    } else {
        if !(bw4 <= 16
            || bh4 <= 16
            || (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as libc::c_int
                == TX_64X64 as libc::c_int)
        {
            unreachable!();
        }
        let mut y = 0;
        let mut x = 0;
        let mut y_off = 0;
        let mut x_off = 0;
        let ytx: *const TxfmInfo = &*dav1d_txfm_dimensions
            .as_ptr()
            .offset((*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as isize)
            as *const TxfmInfo;
        y = 0 as libc::c_int;
        y_off = 0 as libc::c_int;
        while y < bh4 {
            x = 0 as libc::c_int;
            x_off = 0 as libc::c_int;
            while x < bw4 {
                read_tx_tree(
                    t,
                    (*b).c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize,
                    0 as libc::c_int,
                    tx_split.as_mut_ptr(),
                    x_off,
                    y_off,
                );
                (*t).bx += (*ytx).w as libc::c_int;
                x += (*ytx).w as libc::c_int;
                x_off += 1;
            }
            (*t).bx -= x;
            (*t).by += (*ytx).h as libc::c_int;
            y += (*ytx).h as libc::c_int;
            y_off += 1;
        }
        (*t).by -= y;
        if DEBUG_BLOCK_INFO(&*f, &*t) {
            printf(
                b"Post-vartxtree[%x/%x]: r=%d\n\0" as *const u8 as *const libc::c_char,
                tx_split[0] as libc::c_int,
                tx_split[1] as libc::c_int,
                (*(*t).ts).msac.rng,
            );
        }
        (*b).uvtx = dav1d_max_txfm_size_for_bs[bs as usize][(*f).cur.p.layout as usize];
    }
    if tx_split[0] as libc::c_int & !(0x33 as libc::c_int) != 0 {
        unreachable!();
    }
    (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split0 = tx_split[0] as uint8_t;
    (*b).c2rust_unnamed.c2rust_unnamed_0.tx_split1 = tx_split[1];
}

#[inline]
unsafe fn get_prev_frame_segid(
    f: &Dav1dFrameContext,
    by: libc::c_int,
    bx: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
    // It's very difficult to make this safe (a slice),
    // as it comes from [`Dav1dFrameContext::prev_segmap`],
    // which is set to [`Dav1dFrameContext::prev_segmap_ref`],
    // which is a [`Dav1dRef`], which has no size and is refcounted.
    ref_seg_map: *const u8,
    stride: ptrdiff_t,
) -> u8 {
    assert!((*f.frame_hdr).primary_ref_frame != 7);

    // Need checked casts here because an overflowing cast
    // would give a too large `len` to [`std::slice::from_raw_parts`], which would UB.
    let w4 = usize::try_from(w4).unwrap();
    let h4 = usize::try_from(h4).unwrap();
    let stride = usize::try_from(stride).unwrap();

    let mut prev_seg_id = 8;
    let ref_seg_map = std::slice::from_raw_parts(
        ref_seg_map.offset(by as isize * stride as isize + bx as isize),
        h4 * stride,
    );

    assert!(w4 <= stride);
    for ref_seg_map in ref_seg_map.chunks_exact(stride) {
        prev_seg_id = ref_seg_map[..w4]
            .iter()
            .copied()
            .fold(prev_seg_id, std::cmp::min);
        if prev_seg_id == 0 {
            break;
        }
    }
    assert!(prev_seg_id < 8);

    prev_seg_id
}

#[inline]
unsafe extern "C" fn splat_oneref_mv(
    c: *const Dav1dContext,
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
    bw4: libc::c_int,
    bh4: libc::c_int,
) {
    let mode: InterPredMode = (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as InterPredMode;
    let tmpl: Align16<refmvs_block> = {
        let mut init = refmvs_block(refmvs_block_unaligned {
            mv: refmvs_mvpair {
                mv: [
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0],
                    mv::ZERO,
                ],
            },
            r#ref: refmvs_refpair {
                r#ref: [
                    ((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int + 1) as int8_t,
                    (if (*b).c2rust_unnamed.c2rust_unnamed_0.interintra_type as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    }) as int8_t,
                ],
            },
            bs: bs as uint8_t,
            mf: ((mode as libc::c_uint == GLOBALMV as libc::c_int as libc::c_uint
                && imin(bw4, bh4) >= 2) as libc::c_int
                | (mode as libc::c_uint == NEWMV as libc::c_int as libc::c_uint) as libc::c_int * 2)
                as uint8_t,
        });
        Align16(init)
    };
    ((*c).refmvs_dsp.splat_mv).expect("non-null function pointer")(
        &mut *((*t).rt.r)
            .as_mut_ptr()
            .offset((((*t).by & 31) + 5) as isize),
        &tmpl.0,
        (*t).bx,
        bw4,
        bh4,
    );
}
#[inline]
unsafe extern "C" fn splat_intrabc_mv(
    c: *const Dav1dContext,
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
    bw4: libc::c_int,
    bh4: libc::c_int,
) {
    let tmpl: Align16<refmvs_block> = {
        let mut init = refmvs_block(refmvs_block_unaligned {
            mv: refmvs_mvpair {
                mv: [
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0],
                    mv::ZERO,
                ],
            },
            r#ref: refmvs_refpair {
                r#ref: [0 as libc::c_int as int8_t, -(1 as libc::c_int) as int8_t],
            },
            bs: bs as uint8_t,
            mf: 0 as libc::c_int as uint8_t,
        });
        Align16(init)
    };
    ((*c).refmvs_dsp.splat_mv).expect("non-null function pointer")(
        &mut *((*t).rt.r)
            .as_mut_ptr()
            .offset((((*t).by & 31) + 5) as isize),
        &tmpl.0,
        (*t).bx,
        bw4,
        bh4,
    );
}
#[inline]
unsafe extern "C" fn splat_tworef_mv(
    c: *const Dav1dContext,
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    b: *const Av1Block,
    bw4: libc::c_int,
    bh4: libc::c_int,
) {
    if !(bw4 >= 2 && bh4 >= 2) {
        unreachable!();
    }
    let mode: CompInterPredMode =
        (*b).c2rust_unnamed.c2rust_unnamed_0.inter_mode as CompInterPredMode;
    let tmpl: Align16<refmvs_block> = {
        let mut init = refmvs_block(refmvs_block_unaligned {
            mv: refmvs_mvpair {
                mv: [
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0],
                    (*b).c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[1],
                ],
            },
            r#ref: refmvs_refpair {
                r#ref: [
                    ((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int + 1) as int8_t,
                    ((*b).c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as libc::c_int + 1) as int8_t,
                ],
            },
            bs: bs as uint8_t,
            mf: ((mode as libc::c_uint == GLOBALMV_GLOBALMV as libc::c_int as libc::c_uint)
                as libc::c_int
                | ((1 as libc::c_int) << mode as libc::c_uint & 0xbc as libc::c_int != 0)
                    as libc::c_int
                    * 2) as uint8_t,
        });
        Align16(init)
    };
    ((*c).refmvs_dsp.splat_mv).expect("non-null function pointer")(
        &mut *((*t).rt.r)
            .as_mut_ptr()
            .offset((((*t).by & 31) + 5) as isize),
        &tmpl.0,
        (*t).bx,
        bw4,
        bh4,
    );
}
#[inline]
unsafe extern "C" fn splat_intraref(
    c: *const Dav1dContext,
    t: *mut Dav1dTaskContext,
    bs: BlockSize,
    bw4: libc::c_int,
    bh4: libc::c_int,
) {
    let tmpl: Align16<refmvs_block> = {
        let mut init = refmvs_block(refmvs_block_unaligned {
            mv: refmvs_mvpair {
                mv: [mv::INVALID, mv::ZERO],
            },
            r#ref: refmvs_refpair {
                r#ref: [0 as libc::c_int as int8_t, -(1 as libc::c_int) as int8_t],
            },
            bs: bs as uint8_t,
            mf: 0 as libc::c_int as uint8_t,
        });
        Align16(init)
    };
    ((*c).refmvs_dsp.splat_mv).expect("non-null function pointer")(
        &mut *((*t).rt.r)
            .as_mut_ptr()
            .offset((((*t).by & 31) + 5) as isize),
        &tmpl.0,
        (*t).bx,
        bw4,
        bh4,
    );
}

fn mc_lowest_px(
    dst: &mut libc::c_int,
    by4: libc::c_int,
    bh4: libc::c_int,
    mvy: libc::c_int,
    ss_ver: libc::c_int,
    smp: &ScalableMotionParams,
) {
    let v_mul = 4 >> ss_ver;
    if smp.scale == 0 {
        let my = mvy >> 3 + ss_ver;
        let dy = mvy & 15 >> (ss_ver == 0) as libc::c_int;
        *dst = imax(
            *dst,
            (by4 + bh4) * v_mul + my + 4 * (dy != 0) as libc::c_int,
        );
    } else {
        let mut y = (by4 * v_mul << 4) + mvy * (1 << (ss_ver == 0) as libc::c_int);
        let tmp = y as int64_t * smp.scale as int64_t + ((smp.scale - 0x4000) * 8) as int64_t;
        y = apply_sign64((tmp.abs() + 128 >> 8) as libc::c_int, tmp) + 32;
        let bottom = (y + (bh4 * v_mul - 1) * smp.step >> 10) + 1 + 4;
        *dst = imax(*dst, bottom);
    };
}

#[inline(always)]
fn affine_lowest_px(
    t: &Dav1dTaskContext,
    dst: &mut libc::c_int,
    b_dim: &[u8; 4],
    wmp: &Dav1dWarpedMotionParams,
    ss_ver: libc::c_int,
    ss_hor: libc::c_int,
) {
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    assert!(b_dim[0] as libc::c_int * h_mul & 7 == 0 && b_dim[1] as libc::c_int * v_mul & 7 == 0);
    let mat = &wmp.matrix;
    let y = b_dim[1] as libc::c_int * v_mul - 8;
    let src_y = t.by * 4 + ((y + 4) << ss_ver);
    let mat5_y = mat[5] as int64_t * src_y as int64_t + mat[1] as int64_t;
    let mut x = 0;
    while x < b_dim[0] as libc::c_int * h_mul {
        let src_x = t.bx * 4 + ((x + 4) << ss_hor);
        let mvy = mat[4] as int64_t * src_x as int64_t + mat5_y >> ss_ver;
        let dy = (mvy >> 16) as libc::c_int - 4;
        *dst = imax(*dst, dy + 4 + 8);
        x += imax(8, b_dim[0] as libc::c_int * h_mul - 8);
    }
}

#[inline(never)]
fn affine_lowest_px_luma(
    t: &Dav1dTaskContext,
    dst: &mut libc::c_int,
    b_dim: &[u8; 4],
    wmp: &Dav1dWarpedMotionParams,
) {
    affine_lowest_px(t, dst, b_dim, wmp, 0, 0);
}

#[inline(never)]
unsafe fn affine_lowest_px_chroma(
    t: &Dav1dTaskContext,
    dst: &mut libc::c_int,
    b_dim: &[u8; 4],
    wmp: &Dav1dWarpedMotionParams,
) {
    let f = &*t.f;
    assert!(f.cur.p.layout != DAV1D_PIXEL_LAYOUT_I400);
    if f.cur.p.layout == DAV1D_PIXEL_LAYOUT_I444 {
        affine_lowest_px_luma(t, dst, b_dim, wmp);
    } else {
        affine_lowest_px(
            t,
            dst,
            b_dim,
            wmp,
            (f.cur.p.layout & DAV1D_PIXEL_LAYOUT_I420) as libc::c_int,
            1,
        );
    };
}

unsafe fn obmc_lowest_px(
    t: &mut Dav1dTaskContext,
    dst: &mut [[libc::c_int; 2]; 7],
    is_chroma: bool,
    b_dim: &[u8; 4],
    _bx4: libc::c_int,
    _by4: libc::c_int,
    w4: libc::c_int,
    h4: libc::c_int,
) {
    assert!(t.bx & 1 == 0 && t.by & 1 == 0);
    let f = &*t.f;
    let r = &t.rt.r[(t.by as usize & 31) + 5 - 1..];
    let ss_ver = (is_chroma && f.cur.p.layout == DAV1D_PIXEL_LAYOUT_I420) as libc::c_int;
    let ss_hor = (is_chroma && f.cur.p.layout != DAV1D_PIXEL_LAYOUT_I444) as libc::c_int;
    let h_mul = 4 >> ss_hor;
    let v_mul = 4 >> ss_ver;
    if t.by > (*t.ts).tiling.row_start
        && (!is_chroma || b_dim[0] as libc::c_int * h_mul + b_dim[1] as libc::c_int * v_mul >= 16)
    {
        let mut i = 0;
        let mut x = 0;
        while x < w4 && i < imin(b_dim[2] as libc::c_int, 4) {
            let a_r = &*r[0].offset((t.bx + x + 1) as isize);
            let a_b_dim = &dav1d_block_dimensions[a_r.0.bs as usize];
            if a_r.0.r#ref.r#ref[0] as libc::c_int > 0 {
                let oh4 = imin(b_dim[1] as libc::c_int, 16) >> 1;
                mc_lowest_px(
                    &mut dst[a_r.0.r#ref.r#ref[0] as usize - 1][is_chroma as usize],
                    t.by,
                    oh4 * 3 + 3 >> 2,
                    a_r.0.mv.mv[0].y as libc::c_int,
                    ss_ver,
                    &f.svc[a_r.0.r#ref.r#ref[0] as usize - 1][1],
                );
                i += 1;
            }
            x += imax(a_b_dim[0] as libc::c_int, 2);
        }
    }
    if t.bx > (*t.ts).tiling.col_start {
        let mut i = 0;
        let mut y = 0;
        while y < h4 && i < imin(b_dim[3] as libc::c_int, 4) {
            let l_r = &*r[y as usize + 1 + 1].offset((t.bx - 1) as isize);
            let l_b_dim = &dav1d_block_dimensions[l_r.0.bs as usize];
            if l_r.0.r#ref.r#ref[0] as libc::c_int > 0 {
                let oh4 = iclip(l_b_dim[1] as libc::c_int, 2, b_dim[1] as libc::c_int);
                mc_lowest_px(
                    &mut dst[l_r.0.r#ref.r#ref[0] as usize - 1][is_chroma as usize],
                    t.by + y,
                    oh4,
                    l_r.0.mv.mv[0].y as libc::c_int,
                    ss_ver,
                    &f.svc[l_r.0.r#ref.r#ref[0] as usize - 1][1],
                );
                i += 1;
            }
            y += imax(l_b_dim[1] as libc::c_int, 2);
        }
    }
}

unsafe fn decode_b(
    t: &mut Dav1dTaskContext,
    bl: BlockLevel,
    bs: BlockSize,
    bp: BlockPartition,
    intra_edge_flags: EdgeFlags,
) -> libc::c_int {
    use std::fmt;

    /// Helper struct for printing a number as a signed hexidecimal value.
    struct SignAbs(i32);

    impl fmt::Display for SignAbs {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let sign = if self.0 < 0 { "-" } else { " " };
            write!(f, "{}{:x}", sign, self.0.abs())
        }
    }

    let ts = &mut *t.ts;
    let f = &*t.f;
    let frame_hdr = &mut *f.frame_hdr;
    let mut b_mem = Default::default();

    let b = if t.frame_thread.pass != 0 {
        &mut *f
            .frame_thread
            .b
            .offset(t.by as isize * f.b4_stride + t.bx as isize)
    } else {
        &mut b_mem
    };

    let b_dim = &dav1d_block_dimensions[bs as usize];
    let bx4 = t.bx & 31;
    let by4 = t.by & 31;
    let ss_ver = (f.cur.p.layout == DAV1D_PIXEL_LAYOUT_I420) as libc::c_int;
    let ss_hor = (f.cur.p.layout != DAV1D_PIXEL_LAYOUT_I444) as libc::c_int;
    let cbx4 = bx4 >> ss_hor;
    let cby4 = by4 >> ss_ver;
    let bw4 = b_dim[0] as libc::c_int;
    let bh4 = b_dim[1] as libc::c_int;
    let w4 = imin(bw4, f.bw - t.bx);
    let h4 = imin(bh4, f.bh - t.by);
    let cbw4 = bw4 + ss_hor >> ss_hor;
    let cbh4 = bh4 + ss_ver >> ss_ver;
    let have_left = t.bx > ts.tiling.col_start;
    let have_top = t.by > ts.tiling.row_start;
    let has_chroma = f.cur.p.layout != DAV1D_PIXEL_LAYOUT_I400
        && (bw4 > ss_hor || t.bx & 1 != 0)
        && (bh4 > ss_ver || t.by & 1 != 0);

    if t.frame_thread.pass == 2 {
        if b.intra != 0 {
            f.bd_fn.recon_b_intra(t, bs, intra_edge_flags, b);

            let y_mode = b.y_mode() as IntraPredMode;
            let y_mode_nofilt = if y_mode == FILTER_PRED {
                DC_PRED
            } else {
                y_mode
            };

            let mut set_ctx = |dir: &mut BlockContext, _diridx, off, mul, rep_macro: SetCtxFn| {
                rep_macro(dir.mode.0.as_mut_ptr(), off, mul * y_mode_nofilt as u64);
                rep_macro(dir.intra.0.as_mut_ptr(), off, mul);
            };
            case_set(bh4, &mut t.l, 1, by4 as isize, &mut set_ctx);
            case_set(bw4, &mut *t.a, 0, bx4 as isize, &mut set_ctx);

            if is_inter_or_switch(frame_hdr) {
                let r: *mut refmvs_block =
                    t.rt.r[((t.by & 31) + 5 + bh4 - 1) as usize].offset(t.bx as isize);
                for x in 0..bw4 {
                    let block = &mut *r.offset(x as isize);
                    block.0.r#ref.r#ref[0] = 0;
                    block.0.bs = bs as uint8_t;
                }

                let mut rr = &t.rt.r[((t.by & 31) + 5) as usize..];
                for y in 0..bh4 - 1 {
                    let block = &mut *rr[y as usize].offset((t.bx + bw4 - 1) as isize);
                    block.0.r#ref.r#ref[0] = 0;
                    block.0.bs = bs as uint8_t;
                }
            }

            if has_chroma {
                let mut set_ctx =
                    |dir: &mut BlockContext, _diridx, off, mul, rep_macro: SetCtxFn| {
                        rep_macro(
                            dir.uvmode.0.as_mut_ptr(),
                            off,
                            mul * b.c2rust_unnamed.c2rust_unnamed.uv_mode as u64,
                        );
                    };
                case_set(cbh4, &mut t.l, 1, cby4 as isize, &mut set_ctx);
                case_set(cbw4, &mut *t.a, 0, cbx4 as isize, &mut set_ctx);
            }
        } else {
            if is_inter_or_switch(frame_hdr) {
                if b.matrix()[0] == i16::MIN {
                    t.warpmv.type_0 = DAV1D_WM_TYPE_IDENTITY;
                } else {
                    t.warpmv.type_0 = DAV1D_WM_TYPE_AFFINE;
                    t.warpmv.matrix[2] = b.matrix()[0] as int32_t + 0x10000;
                    t.warpmv.matrix[3] = b.matrix()[1] as int32_t;
                    t.warpmv.matrix[4] = b.matrix()[2] as int32_t;
                    t.warpmv.matrix[5] = b.matrix()[3] as int32_t + 0x10000;
                    dav1d_set_affine_mv2d(bw4, bh4, *b.mv2d(), &mut t.warpmv, t.bx, t.by);
                    dav1d_get_shear_params(&mut t.warpmv);

                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "[ {} {} {}\n  {} {} {} ]\n\
                            alpha={}, beta={}, gamma={}, deta={}, mv=y:{},x:{}",
                            SignAbs(t.warpmv.matrix[0]),
                            SignAbs(t.warpmv.matrix[1]),
                            SignAbs(t.warpmv.matrix[2]),
                            SignAbs(t.warpmv.matrix[3]),
                            SignAbs(t.warpmv.matrix[4]),
                            SignAbs(t.warpmv.matrix[5]),
                            SignAbs(t.warpmv.alpha().into()),
                            SignAbs(t.warpmv.beta().into()),
                            SignAbs(t.warpmv.gamma().into()),
                            SignAbs(t.warpmv.delta().into()),
                            b.mv2d().y,
                            b.mv2d().x,
                        );
                    }
                }
            }

            if f.bd_fn.recon_b_inter(t, bs, b) != 0 {
                return -1;
            }

            let filter = &dav1d_filter_dir[b.filter2d() as usize];

            let mut set_ctx = |dir: &mut BlockContext, _diridx, off, mul, rep_macro: SetCtxFn| {
                rep_macro(dir.filter.0[0].as_mut_ptr(), off, mul * filter[0] as u64);
                rep_macro(dir.filter.0[1].as_mut_ptr(), off, mul * filter[1] as u64);
                rep_macro(dir.intra.0.as_mut_ptr(), off, 0);
            };
            case_set(bh4, &mut t.l, 1, by4 as isize, &mut set_ctx);
            case_set(bw4, &mut *t.a, 0, bx4 as isize, &mut set_ctx);

            if is_inter_or_switch(frame_hdr) {
                let r: *mut refmvs_block =
                    t.rt.r[((t.by & 31) + 5 + bh4 - 1) as usize].offset(t.bx as isize);
                for x in 0..bw4 as isize {
                    (*r.offset(x)).0.r#ref.r#ref[0] = b.r#ref()[0] + 1;
                    (*r.offset(x)).0.mv.mv[0] = b.mv()[0];
                    (*r.offset(x)).0.bs = bs as uint8_t;
                }

                let mut rr: &[*mut refmvs_block] = &t.rt.r[((t.by & 31) + 5) as usize..];
                for y in 0..bh4 as usize - 1 {
                    let r = &mut *rr[y].offset((t.bx + bw4 - 1) as isize);
                    r.0.r#ref.r#ref[0] = b.r#ref()[0] + 1;
                    r.0.mv.mv[0] = b.mv()[0];
                    r.0.bs = bs as uint8_t;
                }
            }

            if has_chroma {
                let mut set_ctx =
                    |dir: &mut BlockContext, _diridx, off, mul, rep_macro: SetCtxFn| {
                        rep_macro(dir.uvmode.0.as_mut_ptr(), off, mul * DC_PRED as u64);
                    };
                case_set(cbh4, &mut t.l, 1, cby4 as isize, &mut set_ctx);
                case_set(cbw4, &mut *t.a, 0, cbx4 as isize, &mut set_ctx);
            }
        }

        return 0;
    }

    let cw4 = w4 + ss_hor >> ss_hor;
    let ch4 = h4 + ss_ver >> ss_ver;

    b.bl = bl as uint8_t;
    b.bp = bp as uint8_t;
    b.bs = bs as uint8_t;

    let mut seg = None;

    // segment_id (if seg_feature for skip/ref/gmv is enabled)
    let mut seg_pred = 0;
    if frame_hdr.segmentation.enabled != 0 {
        if frame_hdr.segmentation.update_map == 0 {
            if !(f.prev_segmap).is_null() {
                let mut seg_id =
                    get_prev_frame_segid(f, t.by, t.bx, w4, h4, f.prev_segmap, f.b4_stride);

                if seg_id >= DAV1D_MAX_SEGMENTS.into() {
                    return -1;
                }

                b.seg_id = seg_id;
            } else {
                b.seg_id = 0;
            }

            seg = Some(&frame_hdr.segmentation.seg_data.d[b.seg_id as usize]);
        } else if frame_hdr.segmentation.seg_data.preskip != 0 {
            if frame_hdr.segmentation.temporal != 0 && {
                let index = (*t.a).seg_pred.0[bx4 as usize] + t.l.seg_pred.0[by4 as usize];
                seg_pred = dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.seg_pred.0[index as usize],
                ) as libc::c_int;
                seg_pred != 0
            } {
                if !(f.prev_segmap).is_null() {
                    let mut seg_id =
                        get_prev_frame_segid(f, t.by, t.bx, w4, h4, f.prev_segmap, f.b4_stride);

                    if seg_id >= DAV1D_MAX_SEGMENTS.into() {
                        return -1;
                    }

                    b.seg_id = seg_id;
                } else {
                    b.seg_id = 0;
                }
            } else {
                let (pred_seg_id, seg_ctx) =
                    get_cur_frame_segid(t.by, t.bx, have_top, have_left, f.cur_segmap, f.b4_stride);
                let diff = dav1d_msac_decode_symbol_adapt8(
                    &mut ts.msac,
                    &mut ts.cdf.m.seg_id[seg_ctx as usize],
                    (DAV1D_MAX_SEGMENTS - 1) as size_t,
                );
                let last_active_seg_id = frame_hdr.segmentation.seg_data.last_active_segid;

                b.seg_id = neg_deinterleave(
                    diff as libc::c_int,
                    pred_seg_id as libc::c_int,
                    last_active_seg_id + 1,
                ) as uint8_t;

                if b.seg_id as libc::c_int > last_active_seg_id {
                    b.seg_id = 0;
                }

                if b.seg_id >= DAV1D_MAX_SEGMENTS {
                    b.seg_id = 0;
                }
            }

            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-segid[preskip;{}]: r={}", b.seg_id, ts.msac.rng);
            }

            seg = Some(&frame_hdr.segmentation.seg_data.d[b.seg_id as usize]);
        }
    } else {
        b.seg_id = 0;
    }

    // skip_mode
    if seg
        .map(|seg| seg.globalmv == 0 && seg.r#ref == -1 && seg.skip == 0)
        .unwrap_or(true)
        && (*f.frame_hdr).skip_mode_enabled != 0
        && imin(bw4, bh4) > 1
    {
        let smctx = (*t.a).skip_mode.0[bx4 as usize] + t.l.skip_mode.0[by4 as usize];
        b.skip_mode =
            dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.skip_mode.0[smctx as usize])
                as uint8_t;

        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-skipmode[{}]: r={}", b.skip_mode, ts.msac.rng);
        }
    } else {
        b.skip_mode = 0;
    }

    // skip
    if b.skip_mode != 0 || seg.map(|seg| seg.skip != 0).unwrap_or(false) {
        b.skip = 1;
    } else {
        let sctx = (*t.a).skip[bx4 as usize] + t.l.skip[by4 as usize];
        b.skip = dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.skip[sctx as usize])
            as uint8_t;

        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-skip[{}]: r={}", b.skip, ts.msac.rng);
        }
    }

    // segment_id
    if (*f.frame_hdr).segmentation.enabled != 0
        && (*f.frame_hdr).segmentation.update_map != 0
        && (*f.frame_hdr).segmentation.seg_data.preskip == 0
    {
        if b.skip == 0 && (*f.frame_hdr).segmentation.temporal != 0 && {
            let index = (*t.a).seg_pred.0[bx4 as usize] + t.l.seg_pred.0[by4 as usize];
            seg_pred = dav1d_msac_decode_bool_adapt(
                &mut ts.msac,
                &mut ts.cdf.m.seg_pred.0[index as usize],
            ) as libc::c_int;
            seg_pred != 0
        } {
            // temporal predicted seg_id
            if !(f.prev_segmap).is_null() {
                let mut seg_id =
                    get_prev_frame_segid(f, t.by, t.bx, w4, h4, f.prev_segmap, f.b4_stride);

                if seg_id >= DAV1D_MAX_SEGMENTS.into() {
                    return -1;
                }

                b.seg_id = seg_id;
            } else {
                b.seg_id = 0;
            }
        } else {
            let (pred_seg_id, seg_ctx) =
                get_cur_frame_segid(t.by, t.bx, have_top, have_left, f.cur_segmap, f.b4_stride);

            if b.skip != 0 {
                b.seg_id = pred_seg_id as uint8_t;
            } else {
                let diff = dav1d_msac_decode_symbol_adapt8(
                    &mut ts.msac,
                    &mut ts.cdf.m.seg_id[seg_ctx as usize],
                    (DAV1D_MAX_SEGMENTS - 1) as size_t,
                );
                let last_active_seg_id = (*f.frame_hdr).segmentation.seg_data.last_active_segid;

                b.seg_id = neg_deinterleave(
                    diff as libc::c_int,
                    pred_seg_id as libc::c_int,
                    last_active_seg_id + 1,
                ) as uint8_t;

                if b.seg_id as i32 > last_active_seg_id {
                    b.seg_id = 0;
                }
            }

            if b.seg_id >= DAV1D_MAX_SEGMENTS {
                b.seg_id = 0;
            }
        }

        seg = Some(&(*f.frame_hdr).segmentation.seg_data.d[b.seg_id as usize]);

        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-segid[postskip;{}]: r={}", b.seg_id, ts.msac.rng);
        }
    }

    if b.skip == 0 {
        let idx = if (*f.seq_hdr).sb128 != 0 {
            ((t.bx & 16) >> 4) + ((t.by & 16) >> 3)
        } else {
            0
        } as isize;

        if *(t.cur_sb_cdef_idx_ptr).offset(idx) == -1 {
            let v =
                dav1d_msac_decode_bools(&mut ts.msac, frame_hdr.cdef.n_bits as libc::c_uint) as i8;

            *(t.cur_sb_cdef_idx_ptr).offset(idx) = v;

            if bw4 > 16 {
                *(t.cur_sb_cdef_idx_ptr).offset(idx + 1) = v;
            }

            if bh4 > 16 {
                *(t.cur_sb_cdef_idx_ptr).offset(idx + 2) = v;
            }

            if bw4 == 32 && bh4 == 32 {
                *(t.cur_sb_cdef_idx_ptr).offset(idx + 3) = v;
            }

            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-cdef_idx[{}]: r={}",
                    *t.cur_sb_cdef_idx_ptr, ts.msac.rng
                );
            }
        }
    }

    // delta-q/lf
    let not_sb128 = ((*f.seq_hdr).sb128 == 0) as libc::c_int;
    if t.bx & (31 >> not_sb128) == 0 && t.by & (31 >> not_sb128) == 0 {
        let prev_qidx = ts.last_qidx;
        let have_delta_q = frame_hdr.delta.q.present != 0
            && (bs
                != (if (*f.seq_hdr).sb128 != 0 {
                    BS_128x128
                } else {
                    BS_64x64
                })
                || b.skip == 0);
        let prev_delta_lf = ts.last_delta_lf;

        if have_delta_q {
            let mut delta_q =
                dav1d_msac_decode_symbol_adapt4(&mut ts.msac, &mut ts.cdf.m.delta_q.0, 3)
                    as libc::c_int;

            if delta_q == 3 {
                let n_bits = 1 + dav1d_msac_decode_bools(&mut ts.msac, 3);
                delta_q = (dav1d_msac_decode_bools(&mut ts.msac, n_bits) + 1 + (1 << n_bits))
                    as libc::c_int;
            }

            if delta_q != 0 {
                if dav1d_msac_decode_bool_equi(&mut ts.msac) {
                    delta_q = -delta_q;
                }
                delta_q *= 1 << frame_hdr.delta.q.res_log2;
            }

            ts.last_qidx = iclip(ts.last_qidx + delta_q, 1, 255);

            if have_delta_q && DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-delta_q[{}->{}]: r={}",
                    delta_q, ts.last_qidx, ts.msac.rng
                );
            }

            if frame_hdr.delta.lf.present != 0 {
                let n_lfs = if frame_hdr.delta.lf.multi != 0 {
                    if f.cur.p.layout != DAV1D_PIXEL_LAYOUT_I400 {
                        4
                    } else {
                        2
                    }
                } else {
                    1
                };

                for i in 0..n_lfs as usize {
                    let delta_lf_index = i + frame_hdr.delta.lf.multi as usize;
                    let mut delta_lf = dav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.delta_lf[delta_lf_index],
                        3,
                    ) as libc::c_int;

                    if delta_lf == 3 {
                        let n_bits = 1 + dav1d_msac_decode_bools(&mut ts.msac, 3);
                        delta_lf = (dav1d_msac_decode_bools(&mut ts.msac, n_bits as libc::c_uint)
                            + 1
                            + (1 << n_bits)) as libc::c_int;
                    }

                    if delta_lf != 0 {
                        if dav1d_msac_decode_bool_equi(&mut ts.msac) {
                            delta_lf = -delta_lf;
                        }

                        delta_lf *= 1 << frame_hdr.delta.lf.res_log2;
                    }

                    ts.last_delta_lf[i] =
                        iclip(ts.last_delta_lf[i] as libc::c_int + delta_lf, -63, 63) as int8_t;

                    if have_delta_q && DEBUG_BLOCK_INFO(f, t) {
                        println!("Post-delta_lf[{}:{}]: r={}", i, delta_lf, ts.msac.rng);
                    }
                }
            }
        }

        if ts.last_qidx == frame_hdr.quant.yac {
            // assign frame-wide q values to this sb
            ts.dq = f.dq.as_ptr();
        } else if ts.last_qidx != prev_qidx {
            // find sb-specific quant parameters
            init_quant_tables(&*f.seq_hdr, frame_hdr, ts.last_qidx, &mut ts.dqmem);
            ts.dq = ts.dqmem.as_ptr();
        }

        if ts.last_delta_lf == [0, 0, 0, 0] {
            // assign frame-wide lf values to this sb
            ts.lflvl = f.lf.lvl.as_ptr();
        } else if ts.last_delta_lf != prev_delta_lf {
            // find sb-specific lf lvl parameters
            dav1d_calc_lf_values(&mut ts.lflvlmem, frame_hdr, &ts.last_delta_lf);
            ts.lflvl = ts.lflvlmem.as_ptr();
        }
    }

    if b.skip_mode != 0 {
        b.intra = 0;
    } else if is_inter_or_switch(frame_hdr) {
        if let Some(seg) = seg.filter(|seg| seg.r#ref >= 0 || seg.globalmv != 0) {
            b.intra = (seg.r#ref == 0) as uint8_t;
        } else {
            let ictx = get_intra_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
            b.intra =
                (!dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.intra[ictx.into()]))
                    as uint8_t;

            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-intra[{}]: r={}", b.intra, ts.msac.rng);
            }
        }
    } else if frame_hdr.allow_intrabc != 0 {
        b.intra = (!dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.intrabc.0)) as uint8_t;

        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-intrabcflag[{}]: r={}", b.intra, ts.msac.rng);
        }
    } else {
        b.intra = 1;
    }

    // intra/inter-specific stuff
    if b.intra != 0 {
        let ymode_cdf = if frame_hdr.frame_type & 1 != 0 {
            &mut ts.cdf.m.y_mode[dav1d_ymode_size_context[bs as usize] as usize]
        } else {
            &mut ts.cdf.kfym
                [dav1d_intra_mode_context[(*t.a).mode.0[bx4 as usize] as usize] as usize]
                [dav1d_intra_mode_context[t.l.mode.0[by4 as usize] as usize] as usize]
        };

        *b.y_mode_mut() = dav1d_msac_decode_symbol_adapt16(
            &mut ts.msac,
            ymode_cdf,
            (N_INTRA_PRED_MODES - 1) as size_t,
        ) as uint8_t;

        if DEBUG_BLOCK_INFO(f, t) {
            println!("Post-ymode[{}]: r={}", b.y_mode(), ts.msac.rng);
        }

        // angle delta
        if b_dim[2] + b_dim[3] >= 2
            && b.y_mode() as IntraPredMode >= VERT_PRED
            && b.y_mode() as IntraPredMode <= VERT_LEFT_PRED
        {
            let acdf = &mut ts.cdf.m.angle_delta[b.y_mode() as usize - VERT_PRED as usize];
            let angle = dav1d_msac_decode_symbol_adapt8(&mut ts.msac, acdf, 6);
            *b.y_angle_mut() = angle as int8_t - 3;
        } else {
            *b.y_angle_mut() = 0;
        }

        if has_chroma {
            let cfl_allowed = if frame_hdr.segmentation.lossless[b.seg_id as usize] != 0 {
                cbw4 == 1 && cbh4 == 1
            } else {
                (cfl_allowed_mask & (1 << bs)) != 0
            };

            let uvmode_cdf = &mut ts.cdf.m.uv_mode[cfl_allowed as usize][b.y_mode() as usize];
            *b.uv_mode_mut() = dav1d_msac_decode_symbol_adapt16(
                &mut ts.msac,
                uvmode_cdf,
                N_UV_INTRA_PRED_MODES as size_t - 1 - (!cfl_allowed) as size_t,
            ) as uint8_t;

            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-uvmode[{}]: r={}", b.uv_mode(), ts.msac.rng);
            }

            *b.uv_angle_mut() = 0;
            if b.uv_mode() == CFL_PRED as u8 {
                let sign =
                    dav1d_msac_decode_symbol_adapt8(&mut ts.msac, &mut ts.cdf.m.cfl_sign.0, 7) + 1;
                let sign_u = sign * 0x56 >> 8;
                let sign_v = sign - sign_u * 3;

                assert!(sign_u == sign / 3);

                if sign_u != 0 {
                    let ctx = (sign_u == 2) as usize * 3 + sign_v as usize;
                    b.cfl_alpha_mut()[0] = dav1d_msac_decode_symbol_adapt16(
                        &mut ts.msac,
                        &mut ts.cdf.m.cfl_alpha[ctx],
                        15,
                    ) as i8
                        + 1;

                    if sign_u == 1 {
                        b.cfl_alpha_mut()[0] = -b.cfl_alpha()[0];
                    }
                } else {
                    b.cfl_alpha_mut()[0] = 0;
                }

                if sign_v != 0 {
                    let ctx = (sign_v == 2) as usize * 3 + sign_u as usize;
                    b.cfl_alpha_mut()[1] = dav1d_msac_decode_symbol_adapt16(
                        &mut ts.msac,
                        &mut ts.cdf.m.cfl_alpha[ctx],
                        15,
                    ) as i8
                        + 1;

                    if sign_v == 1 {
                        b.cfl_alpha_mut()[1] = -b.cfl_alpha()[1];
                    }
                } else {
                    b.cfl_alpha_mut()[1] = 0;
                }

                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-uvalphas[{}/{}]: r={}",
                        b.cfl_alpha()[0],
                        b.cfl_alpha()[1],
                        ts.msac.rng,
                    );
                }
            } else if b_dim[2] + b_dim[3] >= 2
                && b.uv_mode() >= VERT_PRED as u8
                && b.uv_mode() <= VERT_LEFT_PRED as u8
            {
                let acdf = &mut ts.cdf.m.angle_delta[b.uv_mode() as usize - VERT_PRED as usize];
                let angle = dav1d_msac_decode_symbol_adapt8(&mut ts.msac, acdf, 6) as libc::c_int;
                *b.uv_angle_mut() = (angle - 3) as int8_t;
            }
        }

        *b.pal_sz_mut() = [0, 0];
        if frame_hdr.allow_screen_content_tools != 0 && imax(bw4, bh4) <= 16 && bw4 + bh4 >= 4 {
            let sz_ctx = b_dim[2] + b_dim[3] - 2;
            if b.y_mode() == DC_PRED as u8 {
                let pal_ctx = ((*t.a).pal_sz.0[bx4 as usize] > 0) as usize
                    + (t.l.pal_sz.0[by4 as usize] > 0) as usize;
                let use_y_pal = dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.pal_y[sz_ctx as usize][pal_ctx],
                );

                if DEBUG_BLOCK_INFO(f, t) {
                    println!("Post-y_pal[{}]: r={}", use_y_pal, ts.msac.rng);
                }

                if use_y_pal {
                    read_pal_plane(t, b, 0, sz_ctx as i32, bx4, by4);
                }
            }

            if has_chroma && b.uv_mode() == DC_PRED as u8 {
                let pal_ctx = b.pal_sz()[0] > 0;
                let use_uv_pal = dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.pal_uv[pal_ctx as usize],
                );

                if DEBUG_BLOCK_INFO(f, t) {
                    println!("Post-uv_pal[{}]: r={}", use_uv_pal, ts.msac.rng);
                }

                if use_uv_pal {
                    read_pal_uv(t, b, sz_ctx as i32, bx4, by4);
                }
            }
        }

        if b.y_mode() == DC_PRED as u8
            && b.pal_sz()[0] == 0
            && imax(b_dim[2] as libc::c_int, b_dim[3] as libc::c_int) <= 3
            && (*f.seq_hdr).filter_intra != 0
        {
            let is_filter = dav1d_msac_decode_bool_adapt(
                &mut ts.msac,
                &mut ts.cdf.m.use_filter_intra[bs as usize],
            );

            if is_filter {
                *b.y_mode_mut() = FILTER_PRED as uint8_t;
                *b.y_angle_mut() =
                    dav1d_msac_decode_symbol_adapt4(&mut ts.msac, &mut ts.cdf.m.filter_intra.0, 4)
                        as int8_t;
            }

            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-filterintramode[{}/{}]: r={}",
                    b.y_mode(),
                    b.y_angle(),
                    ts.msac.rng,
                );
            }
        }

        if b.pal_sz()[0] != 0 {
            let mut pal_idx;
            if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                let frame_thread = &mut ts.frame_thread[p as usize];
                assert!(!frame_thread.pal_idx.is_null());
                pal_idx = frame_thread.pal_idx;
                frame_thread.pal_idx = frame_thread.pal_idx.offset((bw4 * bh4 * 16) as isize);
            } else {
                pal_idx = t.scratch.c2rust_unnamed_0.pal_idx.as_mut_ptr();
            }

            read_pal_indices(t, pal_idx, b, 0, w4, h4, bw4, bh4);

            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-y-pal-indices: r={}", ts.msac.rng);
            }
        }

        if has_chroma && b.pal_sz()[1] != 0 {
            let mut pal_idx;
            if t.frame_thread.pass != 0 {
                let p = t.frame_thread.pass & 1;
                let frame_thread = &mut ts.frame_thread[p as usize];
                assert!(!(frame_thread.pal_idx).is_null());
                pal_idx = frame_thread.pal_idx;
                frame_thread.pal_idx = frame_thread.pal_idx.offset((cbw4 * cbh4 * 16) as isize);
            } else {
                pal_idx = t
                    .scratch
                    .c2rust_unnamed_0
                    .pal_idx
                    .as_mut_ptr()
                    .offset((bw4 * bh4 * 16) as isize);
            }

            read_pal_indices(t, pal_idx, b, 1, cw4, ch4, cbw4, cbh4);

            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-uv-pal-indices: r={}", ts.msac.rng);
            }
        }

        let mut t_dim;
        if frame_hdr.segmentation.lossless[b.seg_id as usize] != 0 {
            b.uvtx = TX_4X4 as uint8_t;
            *b.tx_mut() = b.uvtx;
            t_dim = &dav1d_txfm_dimensions[TX_4X4 as usize];
        } else {
            *b.tx_mut() = dav1d_max_txfm_size_for_bs[bs as usize][0];
            b.uvtx = dav1d_max_txfm_size_for_bs[bs as usize][f.cur.p.layout as usize];
            t_dim = &dav1d_txfm_dimensions[b.tx() as usize];

            if frame_hdr.txfm_mode == DAV1D_TX_SWITCHABLE && t_dim.max > TX_4X4 as u8 {
                let tctx = get_tx_ctx(&*t.a, &t.l, &*t_dim, by4, bx4);
                let tx_cdf = &mut ts.cdf.m.txsz[(t_dim.max - 1) as usize][tctx as usize];
                let mut depth = dav1d_msac_decode_symbol_adapt4(
                    &mut ts.msac,
                    tx_cdf,
                    imin(t_dim.max as libc::c_int, 2 as libc::c_int) as size_t,
                ) as libc::c_int;

                for _ in 0..depth {
                    *b.tx_mut() = t_dim.sub;
                    t_dim = &dav1d_txfm_dimensions[b.tx() as usize];
                }
            }

            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-tx[{}]: r={}", b.tx(), ts.msac.rng);
            }
        }

        // reconstruction
        if t.frame_thread.pass == 1 {
            f.bd_fn.read_coef_blocks(t, bs, b);
        } else {
            f.bd_fn.recon_b_intra(t, bs, intra_edge_flags, b);
        }

        if frame_hdr.loopfilter.level_y[0] != 0 || frame_hdr.loopfilter.level_y[1] != 0 {
            dav1d_create_lf_mask_intra(
                &mut *t.lf_mask,
                f.lf.level,
                f.b4_stride,
                &*ts.lflvl.offset(b.seg_id as isize),
                t.bx,
                t.by,
                f.w4,
                f.h4,
                bs,
                b.tx() as RectTxfmSize,
                b.uvtx as RectTxfmSize,
                f.cur.p.layout,
                &mut (*t.a).tx_lpf_y.0[bx4 as usize..],
                &mut t.l.tx_lpf_y.0[by4 as usize..],
                if has_chroma {
                    Some((
                        &mut (*t.a).tx_lpf_uv.0[cbx4 as usize..],
                        &mut t.l.tx_lpf_uv.0[cby4 as usize..],
                    ))
                } else {
                    None
                },
            );
        }

        // update contexts
        let y_mode_nofilt = if b.y_mode() == FILTER_PRED as u8 {
            DC_PRED as IntraPredMode
        } else {
            b.y_mode() as IntraPredMode
        };
        let mut set_ctx = |dir: &mut BlockContext, diridx, off, mul, rep_macro: SetCtxFn| {
            // NOTE: This corresponds to the following logic in the original C:
            //
            // ```
            // ((uint8_t *) &t_dim->lw)[diridx]
            // ```
            //
            // The original logic casts a pointer to the `lw` field to a `*const u8` and
            // then does indexing within the struct's fields. This is deeply cursed and
            // unnecessary since `diridx` will only ever be 0 or 1. So for the Rust version
            // we manually select which field we want based on `diridx` directly.
            let lw_lh = match diridx {
                0 => t_dim.lw,
                1 => t_dim.lh,
                _ => unreachable!(),
            };

            rep_macro(
                dir.tx_intra.0.as_mut_ptr() as *mut u8,
                off,
                mul * lw_lh as u64,
            );
            rep_macro(dir.tx.0.as_mut_ptr() as *mut u8, off, mul * lw_lh as u64);
            rep_macro(dir.mode.0.as_mut_ptr(), off, mul * y_mode_nofilt as u64);
            rep_macro(dir.pal_sz.0.as_mut_ptr(), off, mul * b.pal_sz()[0] as u64);
            rep_macro(dir.seg_pred.0.as_mut_ptr(), off, mul * seg_pred as u64);
            rep_macro(dir.skip_mode.0.as_mut_ptr(), off, 0);
            rep_macro(dir.intra.0.as_mut_ptr(), off, mul);
            rep_macro(dir.skip.0.as_mut_ptr(), off, mul * b.skip as u64);
            // see aomedia bug 2183 for why we use luma coordinates here
            rep_macro(
                t.pal_sz_uv[diridx].as_mut_ptr(),
                off,
                mul * if has_chroma { b.pal_sz()[1] as u64 } else { 0 },
            );
            if is_inter_or_switch(frame_hdr) {
                rep_macro(
                    dir.comp_type.0.as_mut_ptr(),
                    off,
                    mul * COMP_INTER_NONE as u64,
                );
                rep_macro(
                    dir.r#ref[0].as_mut_ptr() as *mut u8,
                    off,
                    mul * u8::MAX as u64,
                );
                rep_macro(
                    dir.r#ref[1].as_mut_ptr() as *mut u8,
                    off,
                    mul * u8::MAX as u64,
                );
                rep_macro(
                    dir.filter.0[0].as_mut_ptr(),
                    off,
                    mul * DAV1D_N_SWITCHABLE_FILTERS as u64,
                );
                rep_macro(
                    dir.filter.0[1].as_mut_ptr(),
                    off,
                    mul * DAV1D_N_SWITCHABLE_FILTERS as u64,
                );
            }
        };
        case_set(bh4, &mut t.l, 1, by4 as isize, &mut set_ctx);
        case_set(bw4, &mut *t.a, 0, bx4 as isize, &mut set_ctx);

        if b.pal_sz()[0] != 0 {
            let pal = if t.frame_thread.pass != 0 {
                let index = ((t.by >> 1) + (t.bx & 1)) as isize * (f.b4_stride >> 1)
                    + ((t.bx >> 1) + (t.by & 1)) as isize;
                &(*f.frame_thread.pal.offset(index))[0]
            } else {
                &t.scratch.c2rust_unnamed_0.pal[0]
            };

            for x in 0..bw4 {
                t.al_pal[0][(bx4 + x) as usize][0] = *pal;
            }

            for y in 0..bh4 {
                t.al_pal[1][(by4 + y) as usize][0] = *pal;
            }
        }

        if has_chroma {
            let mut set_ctx = |dir: &mut BlockContext, _diridx, off, mul, rep_macro: SetCtxFn| {
                rep_macro(dir.uvmode.0.as_mut_ptr(), off, mul * b.uv_mode() as u64);
            };
            case_set(cbh4, &mut t.l, 1, cby4 as isize, &mut set_ctx);
            case_set(cbw4, &mut *t.a, 0, cbx4 as isize, &mut set_ctx);

            if b.pal_sz()[1] != 0 {
                let pal = if t.frame_thread.pass != 0 {
                    let index = ((t.by >> 1) + (t.bx & 1)) as isize * (f.b4_stride >> 1)
                        + ((t.bx >> 1) + (t.by & 1)) as isize;
                    &*f.frame_thread.pal.offset(index)
                } else {
                    &t.scratch.c2rust_unnamed_0.pal
                };

                for pl in 1..=2 {
                    for x in 0..bw4 {
                        t.al_pal[0][(bx4 + x) as usize][pl] = pal[pl];
                    }

                    for y in 0..bh4 {
                        t.al_pal[1][(by4 + y) as usize][pl] = pal[pl];
                    }
                }
            }
        }

        if is_inter_or_switch(frame_hdr) || frame_hdr.allow_intrabc != 0 {
            splat_intraref(f.c, t, bs, bw4, bh4);
        }
    } else if is_key_or_intra(frame_hdr) {
        // intra block copy
        let mut mvstack = [refmvs_candidate::default(); 8];
        let mut n_mvs = 0;
        let mut ctx_1 = 0;
        dav1d_refmvs_find(
            &mut t.rt,
            &mut mvstack,
            &mut n_mvs,
            &mut ctx_1,
            [0, -1].into(),
            bs,
            intra_edge_flags,
            t.by,
            t.bx,
        );

        if mvstack[0].mv.mv[0] != mv::ZERO {
            b.mv_mut()[0] = mvstack[0].mv.mv[0];
        } else if mvstack[1].mv.mv[0] != mv::ZERO {
            b.mv_mut()[0] = mvstack[1].mv.mv[0];
        } else if t.by - (16 << (*f.seq_hdr).sb128) < ts.tiling.row_start {
            b.mv_mut()[0].y = 0;
            b.mv_mut()[0].x = (-(512 << (*f.seq_hdr).sb128) - 2048) as int16_t;
        } else {
            b.mv_mut()[0].y = -(512 << (*f.seq_hdr).sb128) as int16_t;
            b.mv_mut()[0].x = 0;
        }

        let r#ref = b.mv()[0];
        read_mv_residual(t, &mut b.mv_mut()[0], &mut ts.cdf.dmv, false);

        // clip intrabc motion vector to decoded parts of current tile
        let mut border_left = ts.tiling.col_start * 4;
        let mut border_top = ts.tiling.row_start * 4;
        if has_chroma {
            if bw4 < 2 && ss_hor != 0 {
                border_left += 4;
            }

            if bh4 < 2 && ss_ver != 0 {
                border_top += 4;
            }
        }

        let mut src_left = t.bx * 4 + (b.mv()[0].x as libc::c_int >> 3);
        let mut src_top = t.by * 4 + (b.mv()[0].y as libc::c_int >> 3);
        let mut src_right = src_left + bw4 * 4;
        let mut src_bottom = src_top + bh4 * 4;
        let border_right = (ts.tiling.col_end + (bw4 - 1) & !(bw4 - 1)) * 4;

        // check against left or right tile boundary and adjust if necessary
        if src_left < border_left {
            src_right += border_left - src_left;
            src_left += border_left - src_left;
        } else if src_right > border_right {
            src_left -= src_right - border_right;
            src_right -= src_right - border_right;
        }

        // check against top tile boundary and adjust if necessary
        if src_top < border_top {
            src_bottom += border_top - src_top;
            src_top += border_top - src_top;
        }

        let sbx = t.bx >> 4 + (*f.seq_hdr).sb128 << 6 + (*f.seq_hdr).sb128;
        let sby = t.by >> 4 + (*f.seq_hdr).sb128 << 6 + (*f.seq_hdr).sb128;
        let sb_size = 1 << 6 + (*f.seq_hdr).sb128;

        // check for overlap with current superblock
        if src_bottom > sby && src_right > sbx {
            if src_top - border_top >= src_bottom - sby {
                // if possible move src up into the previous suberblock row
                src_top -= src_bottom - sby;
                src_bottom -= src_bottom - sby;
            } else if src_left - border_left >= src_right - sbx {
                // if possible move src left into the previous suberblock
                src_left -= src_right - sbx;
                src_right -= src_right - sbx;
            }
        }

        // move src up if it is below current superblock row
        if src_bottom > sby + sb_size {
            src_top -= src_bottom - (sby + sb_size);
            src_bottom -= src_bottom - (sby + sb_size);
        }

        // error out if mv still overlaps with the current superblock
        if src_bottom > sby && src_right > sbx {
            return -1;
        }

        b.mv_mut()[0].x = ((src_left - t.bx * 4) * 8) as int16_t;
        b.mv_mut()[0].y = ((src_top - t.by * 4) * 8) as int16_t;

        if DEBUG_BLOCK_INFO(f, t) {
            println!(
                "Post-dmv[{}/{},ref={}/{}|{}/{}]: r={}",
                b.mv()[0].y,
                b.mv()[0].x,
                r#ref.y,
                r#ref.x,
                mvstack[0].mv.mv[0].y,
                mvstack[0].mv.mv[0].x,
                ts.msac.rng,
            );
        }

        read_vartx_tree(t, b, bs, bx4, by4);

        // reconstruction
        if t.frame_thread.pass == 1 {
            f.bd_fn.read_coef_blocks(t, bs, b);
            *b.filter2d_mut() = FILTER_2D_BILINEAR as uint8_t;
        } else if f.bd_fn.recon_b_inter(t, bs, b) != 0 {
            return -1;
        }

        splat_intrabc_mv(f.c, t, bs, b, bw4, bh4);

        let mut set_ctx = |dir: &mut BlockContext, diridx: usize, off, mul, rep_macro: SetCtxFn| {
            rep_macro(
                dir.tx_intra.0.as_mut_ptr() as *mut u8,
                off,
                mul * b_dim[2 + diridx] as u64,
            );
            rep_macro(dir.mode.0.as_mut_ptr(), off, mul * DC_PRED as u64);
            rep_macro(dir.pal_sz.0.as_mut_ptr(), off, 0);
            // see aomedia bug 2183 for why this is outside `if has_chroma {}`
            rep_macro(t.pal_sz_uv[diridx].as_mut_ptr(), off, 0);
            rep_macro(dir.seg_pred.0.as_mut_ptr(), off, mul * seg_pred as u64);
            rep_macro(dir.skip_mode.0.as_mut_ptr(), off, 0);
            rep_macro(dir.intra.0.as_mut_ptr(), off, 0);
            rep_macro(dir.skip.0.as_mut_ptr(), off, mul * b.skip as u64)
        };
        case_set(bh4, &mut t.l, 1, by4 as isize, &mut set_ctx);
        case_set(bw4, &mut *t.a, 0, bx4 as isize, &mut set_ctx);

        if has_chroma {
            let mut set_ctx = |dir: &mut BlockContext, _diridx, off, mul, rep_macro: SetCtxFn| {
                rep_macro(dir.uvmode.0.as_mut_ptr(), off, mul * DC_PRED as u64);
            };
            case_set(cbh4, &mut t.l, 1, cby4 as isize, &mut set_ctx);
            case_set(cbw4, &mut *t.a, 0, cbx4 as isize, &mut set_ctx);
        }
    } else {
        // inter-specific mode/mv coding
        let mut is_comp = false;
        let mut has_subpel_filter = false;

        if b.skip_mode != 0 {
            is_comp = true;
        } else if seg
            .map(|seg| seg.r#ref == -1 && seg.globalmv == 0 && seg.skip == 0)
            .unwrap_or(true)
            && frame_hdr.switchable_comp_refs != 0
            && imin(bw4, bh4) > 1
        {
            let ctx_2 = get_comp_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
            is_comp =
                dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.comp[ctx_2 as usize]);

            if DEBUG_BLOCK_INFO(f, t) {
                println!("Post-compflag[{}]: r={}", is_comp, ts.msac.rng);
            }
        } else {
            is_comp = false;
        }

        if b.skip_mode != 0 {
            *b.ref_mut() = [
                frame_hdr.skip_mode_refs[0] as i8,
                frame_hdr.skip_mode_refs[1] as i8,
            ];
            *b.comp_type_mut() = COMP_INTER_AVG as uint8_t;
            *b.inter_mode_mut() = NEARESTMV_NEARESTMV as uint8_t;
            *b.drl_idx_mut() = NEAREST_DRL as uint8_t;
            has_subpel_filter = false;

            let mut mvstack = [Default::default(); 8];
            let mut n_mvs_0 = 0;
            let mut ctx_3 = 0;
            dav1d_refmvs_find(
                &mut t.rt,
                &mut mvstack,
                &mut n_mvs_0,
                &mut ctx_3,
                [b.r#ref()[0] + 1, b.r#ref()[1] + 1].into(),
                bs,
                intra_edge_flags,
                t.by,
                t.bx,
            );

            *b.mv_mut() = mvstack[0].mv.mv;
            fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
            fix_mv_precision(frame_hdr, &mut b.mv_mut()[1]);

            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-skipmodeblock[mv=1:y={},x={},2:y={},x={},refs={}+{}",
                    b.mv()[0].y,
                    b.mv()[0].x,
                    b.mv()[1].y,
                    b.mv()[1].x,
                    b.r#ref()[0],
                    b.r#ref()[1],
                );
            }
        } else if is_comp {
            let dir_ctx = get_comp_dir_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
            if dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.comp_dir[dir_ctx as usize])
            {
                // bidir - first reference (fw)
                let ctx1 = av1_get_fwd_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                if dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.comp_fwd_ref[0][ctx1 as usize],
                ) {
                    let ctx2 = av1_get_fwd_ref_2_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    b.ref_mut()[0] = 2 + dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.comp_fwd_ref[2][ctx2 as usize],
                    ) as int8_t;
                } else {
                    let ctx2 = av1_get_fwd_ref_1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    b.ref_mut()[0] = dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.comp_fwd_ref[1][ctx2 as usize],
                    ) as int8_t;
                }

                // second reference (bw)
                let ctx3 = av1_get_bwd_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                if dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.comp_bwd_ref[0][ctx3 as usize],
                ) {
                    b.ref_mut()[1] = 6;
                } else {
                    let ctx4 = av1_get_bwd_ref_1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    b.ref_mut()[1] = 4 + dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.comp_bwd_ref[1][ctx4 as usize],
                    ) as int8_t;
                }
            } else {
                // unidir
                let uctx_p = av1_get_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                if dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.comp_uni_ref[0][uctx_p as usize],
                ) {
                    *b.ref_mut() = [4, 6];
                } else {
                    let uctx_p1 = av1_get_uni_p1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    *b.ref_mut() = [
                        0,
                        1 + dav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.comp_uni_ref[1][uctx_p1 as usize],
                        ) as i8,
                    ];

                    if b.r#ref()[1] == 2 {
                        let uctx_p2 =
                            av1_get_fwd_ref_2_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                        b.ref_mut()[1] += dav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.comp_uni_ref[2][uctx_p2 as usize],
                        ) as i8;
                    }
                }
            }

            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-refs[{}/{}]: r={}",
                    b.r#ref()[0],
                    b.r#ref()[1],
                    ts.msac.rng,
                );
            }

            let mut mvstack = [Default::default(); 8];
            let mut n_mvs = 0;
            let mut ctx_4 = 0;
            dav1d_refmvs_find(
                &mut t.rt,
                &mut mvstack,
                &mut n_mvs,
                &mut ctx_4,
                [b.r#ref()[0] + 1, b.r#ref()[1] + 1].into(),
                bs,
                intra_edge_flags,
                t.by,
                t.bx,
            );

            *b.inter_mode_mut() = dav1d_msac_decode_symbol_adapt8(
                &mut ts.msac,
                &mut ts.cdf.m.comp_inter_mode[ctx_4 as usize],
                N_COMP_INTER_PRED_MODES as size_t - 1,
            ) as uint8_t;

            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-compintermode[{},ctx={},n_mvs={}]: r={}",
                    b.inter_mode(),
                    ctx_4,
                    n_mvs,
                    ts.msac.rng,
                );
            }

            let im = &dav1d_comp_inter_pred_modes[b.inter_mode() as usize];
            *b.drl_idx_mut() = NEAREST_DRL as u8;
            if b.inter_mode() == NEWMV_NEWMV as u8 {
                // NEARER, NEAR or NEARISH
                if n_mvs > 1 {
                    let drl_ctx_v1 = get_drl_context(&mvstack, 0);
                    *b.drl_idx_mut() += dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.drl_bit[drl_ctx_v1 as usize],
                    ) as u8;

                    if b.drl_idx() == NEARER_DRL as u8 && n_mvs > 2 {
                        let drl_ctx_v2 = get_drl_context(&mvstack, 1);
                        *b.drl_idx_mut() += dav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.drl_bit[drl_ctx_v2 as usize],
                        ) as u8;
                    }

                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-drlidx[{},n_mvs={}]: r={}",
                            b.drl_idx(),
                            n_mvs,
                            ts.msac.rng,
                        );
                    }
                }
            } else if im[0] == NEARMV as u8 || im[1] == NEARMV as u8 {
                *b.drl_idx_mut() = NEARER_DRL as u8;

                // NEAR or NEARISH
                if n_mvs > 2 {
                    let drl_ctx_v2_0 = get_drl_context(&mvstack, 1);
                    *b.drl_idx_mut() += dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.drl_bit[drl_ctx_v2_0 as usize],
                    ) as u8;

                    if b.drl_idx() == NEAR_DRL as u8 && n_mvs > 3 {
                        let drl_ctx_v3 = get_drl_context(&mvstack, 2);
                        *b.drl_idx_mut() += dav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.drl_bit[drl_ctx_v3 as usize],
                        ) as u8;
                    }

                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-drlidx[{},n_mvs={}]: r={}",
                            b.drl_idx(),
                            n_mvs,
                            ts.msac.rng,
                        );
                    }
                }
            }

            assert!(b.drl_idx() >= NEAREST_DRL as u8 && b.drl_idx() <= NEARISH_DRL as u8);

            has_subpel_filter = imin(bw4, bh4) == 1 || b.inter_mode() != GLOBALMV_GLOBALMV as u8;

            let mut assign_comp_mv = |idx: usize| match im[idx] as InterPredMode {
                NEARMV | NEARESTMV => {
                    b.mv_mut()[idx] = mvstack[b.drl_idx() as usize].mv.mv[idx];
                    fix_mv_precision(frame_hdr, &mut b.mv_mut()[idx]);
                }

                GLOBALMV => {
                    has_subpel_filter |=
                        frame_hdr.gmv[b.r#ref()[idx] as usize].type_0 == DAV1D_WM_TYPE_TRANSLATION;
                    b.mv_mut()[idx] = get_gmv_2d(
                        &frame_hdr.gmv[b.r#ref()[idx] as usize],
                        t.bx,
                        t.by,
                        bw4,
                        bh4,
                        frame_hdr,
                    );
                }

                NEWMV => {
                    b.mv_mut()[idx] = mvstack[b.drl_idx() as usize].mv.mv[idx];
                    read_mv_residual(
                        t,
                        &mut b.mv_mut()[idx],
                        &mut ts.cdf.mv,
                        frame_hdr.force_integer_mv == 0,
                    );
                }
                _ => {}
            };
            assign_comp_mv(0);
            assign_comp_mv(1);

            if DEBUG_BLOCK_INFO(f, t) {
                println!(
                    "Post-residual_mv[1:y={},x={},2:y={},x={}]: r={}",
                    b.mv()[0].y,
                    b.mv()[0].x,
                    b.mv()[1].y,
                    b.mv()[1].x,
                    ts.msac.rng,
                );
            }

            // jnt_comp vs. seg vs. wedge
            let mut is_segwedge = false;
            if (*f.seq_hdr).masked_compound != 0 {
                let mask_ctx = get_mask_comp_ctx(&*t.a, &t.l, by4, bx4);
                is_segwedge = dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.mask_comp[mask_ctx as usize],
                );

                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-segwedge_vs_jntavg[{},ctx={}]: r={}",
                        is_segwedge, mask_ctx, ts.msac.rng,
                    );
                }
            }

            if !is_segwedge {
                if (*f.seq_hdr).jnt_comp != 0 {
                    let jnt_ctx = get_jnt_comp_ctx(
                        (*f.seq_hdr).order_hint_n_bits,
                        (*f.cur.frame_hdr).frame_offset as libc::c_uint,
                        (*f.refp[b.r#ref()[0] as usize].p.frame_hdr).frame_offset as libc::c_uint,
                        (*f.refp[b.r#ref()[1] as usize].p.frame_hdr).frame_offset as libc::c_uint,
                        &*t.a,
                        &t.l,
                        by4,
                        bx4,
                    );
                    *b.comp_type_mut() = COMP_INTER_WEIGHTED_AVG as u8
                        + dav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.jnt_comp[jnt_ctx as usize],
                        ) as u8;

                    if DEBUG_BLOCK_INFO(f, t) {
                        println!(
                            "Post-jnt_comp[{},ctx={}[ac:{},ar:{},lc:{},lr:{}]]: r={}",
                            b.comp_type() == COMP_INTER_AVG as u8,
                            jnt_ctx,
                            (*t.a).comp_type[bx4 as usize],
                            (*t.a).r#ref[0][bx4 as usize],
                            t.l.comp_type[by4 as usize],
                            t.l.r#ref[0][by4 as usize],
                            ts.msac.rng,
                        );
                    }
                } else {
                    *b.comp_type_mut() = COMP_INTER_AVG as u8;
                }
            } else {
                if wedge_allowed_mask & (1 << bs) != 0 {
                    let ctx = dav1d_wedge_ctx_lut[bs as usize] as usize;
                    *b.comp_type_mut() = COMP_INTER_WEDGE as u8
                        - dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.wedge_comp[ctx])
                            as u8;

                    if b.comp_type() == COMP_INTER_WEDGE as u8 {
                        *b.wedge_idx_mut() = dav1d_msac_decode_symbol_adapt16(
                            &mut ts.msac,
                            &mut ts.cdf.m.wedge_idx[ctx],
                            15,
                        ) as u8;
                    }
                } else {
                    *b.comp_type_mut() = COMP_INTER_SEG as u8;
                }

                *b.mask_sign_mut() = dav1d_msac_decode_bool_equi(&mut ts.msac) as u8;

                if DEBUG_BLOCK_INFO(f, t) {
                    println!(
                        "Post-seg/wedge[{},wedge_idx={},sign={}]: r={}",
                        b.comp_type() == COMP_INTER_WEDGE as u8,
                        b.wedge_idx(),
                        b.mask_sign(),
                        ts.msac.rng,
                    );
                }
            }
        } else {
            b.c2rust_unnamed.c2rust_unnamed_0.comp_type = COMP_INTER_NONE as libc::c_int as uint8_t;
            if let Some(seg) = seg.filter(|seg| seg.r#ref > 0) {
                b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0 as libc::c_int as usize] =
                    seg.r#ref as i8 - 1;
            } else if let Some(_) = seg.filter(|seg| seg.globalmv != 0 || seg.skip != 0) {
                b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] = 0 as libc::c_int as int8_t;
            } else {
                let ctx1_0 = av1_get_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                if dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.r#ref[0][ctx1_0 as usize],
                ) {
                    let ctx2_1 = av1_get_bwd_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    if dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.r#ref[1][ctx2_1 as usize],
                    ) {
                        b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0 as libc::c_int as usize] =
                            6 as libc::c_int as int8_t;
                    } else {
                        let ctx3_0 =
                            av1_get_bwd_ref_1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                        b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0 as libc::c_int as usize] =
                            (4 as libc::c_int as libc::c_uint).wrapping_add(
                                dav1d_msac_decode_bool_adapt(
                                    &mut ts.msac,
                                    &mut ts.cdf.m.r#ref[5][ctx3_0 as usize],
                                ) as libc::c_uint,
                            ) as int8_t;
                    }
                } else {
                    let ctx2_2 = av1_get_fwd_ref_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                    if dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.r#ref[2][ctx2_2 as usize],
                    ) {
                        let ctx3_1 =
                            av1_get_fwd_ref_2_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                        b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0 as libc::c_int as usize] =
                            (2 as libc::c_int as libc::c_uint).wrapping_add(
                                dav1d_msac_decode_bool_adapt(
                                    &mut ts.msac,
                                    &mut ts.cdf.m.r#ref[4][ctx3_1 as usize],
                                ) as libc::c_uint,
                            ) as int8_t;
                    } else {
                        let ctx3_2 =
                            av1_get_fwd_ref_1_ctx(&*t.a, &t.l, by4, bx4, have_top, have_left);
                        b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0 as libc::c_int as usize] =
                            dav1d_msac_decode_bool_adapt(
                                &mut ts.msac,
                                &mut ts.cdf.m.r#ref[3][ctx3_2 as usize],
                            ) as int8_t;
                    }
                }
                if DEBUG_BLOCK_INFO(f, t) {
                    printf(
                        b"Post-ref[%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int,
                        ts.msac.rng,
                    );
                }
            }
            b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] = -(1 as libc::c_int) as int8_t;
            let mut mvstack_2: [refmvs_candidate; 8] = [refmvs_candidate {
                mv: refmvs_mvpair { mv: [mv::ZERO; 2] },
                weight: 0,
            }; 8];
            let mut n_mvs_2 = 0;
            let mut ctx_6 = 0;
            dav1d_refmvs_find(
                &mut t.rt,
                &mut mvstack_2,
                &mut n_mvs_2,
                &mut ctx_6,
                refmvs_refpair {
                    r#ref: [
                        (b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int + 1) as int8_t,
                        -(1 as libc::c_int) as int8_t,
                    ],
                },
                bs,
                intra_edge_flags,
                t.by,
                t.bx,
            );
            if seg
                .map(|seg| seg.skip != 0 || seg.globalmv != 0)
                .unwrap_or(false)
                || dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.newmv_mode[(ctx_6 & 7) as usize],
                )
            {
                if seg
                    .map(|seg| seg.skip != 0 || seg.globalmv != 0)
                    .unwrap_or(false)
                    || !dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.globalmv_mode[(ctx_6 >> 3 & 1) as usize],
                    )
                {
                    b.c2rust_unnamed.c2rust_unnamed_0.inter_mode =
                        GLOBALMV as libc::c_int as uint8_t;
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0] = get_gmv_2d(
                        &frame_hdr.gmv[b.r#ref()[0] as usize],
                        t.bx,
                        t.by,
                        bw4,
                        bh4,
                        frame_hdr,
                    );
                    has_subpel_filter = imin(bw4, bh4) == 1
                        || frame_hdr.gmv[b.r#ref()[0] as usize].type_0 == DAV1D_WM_TYPE_TRANSLATION;
                } else {
                    has_subpel_filter = true;
                    if dav1d_msac_decode_bool_adapt(
                        &mut ts.msac,
                        &mut ts.cdf.m.refmv_mode[(ctx_6 >> 4 & 15) as usize],
                    ) {
                        b.c2rust_unnamed.c2rust_unnamed_0.inter_mode =
                            NEARMV as libc::c_int as uint8_t;
                        b.c2rust_unnamed.c2rust_unnamed_0.drl_idx =
                            NEARER_DRL as libc::c_int as uint8_t;
                        if n_mvs_2 > 2 {
                            let drl_ctx_v2_1 = get_drl_context(&mvstack_2, 1);
                            b.c2rust_unnamed.c2rust_unnamed_0.drl_idx =
                                (b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_uint)
                                    .wrapping_add(dav1d_msac_decode_bool_adapt(
                                        &mut ts.msac,
                                        &mut ts.cdf.m.drl_bit[drl_ctx_v2_1 as usize],
                                    )
                                        as libc::c_uint) as uint8_t
                                    as uint8_t;
                            if b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int
                                == NEAR_DRL as libc::c_int
                                && n_mvs_2 > 3
                            {
                                let drl_ctx_v3_0 = get_drl_context(&mvstack_2, 2);
                                b.c2rust_unnamed.c2rust_unnamed_0.drl_idx =
                                    (b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_uint)
                                        .wrapping_add(dav1d_msac_decode_bool_adapt(
                                            &mut ts.msac,
                                            &mut ts.cdf.m.drl_bit[drl_ctx_v3_0 as usize],
                                        )
                                            as libc::c_uint)
                                        as uint8_t as uint8_t;
                            }
                        }
                    } else {
                        b.c2rust_unnamed.c2rust_unnamed_0.inter_mode =
                            NEARESTMV as libc::c_int as uint8_t;
                        b.c2rust_unnamed.c2rust_unnamed_0.drl_idx =
                            NEAREST_DRL as libc::c_int as uint8_t;
                    }
                    if !(b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int
                        >= NEAREST_DRL as libc::c_int
                        && b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int
                            <= NEARISH_DRL as libc::c_int)
                    {
                        unreachable!();
                    }
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0] = mvstack_2[b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as usize]
                        .mv
                        .mv[0];
                    if (b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int)
                        < NEAR_DRL as libc::c_int
                    {
                        fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
                    }
                }
                if DEBUG_BLOCK_INFO(f, t) {
                    printf(
                        b"Post-intermode[%d,drl=%d,mv=y:%d,x:%d,n_mvs=%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int,
                        b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0]
                            .y as libc::c_int,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0]
                            .x as libc::c_int,
                        n_mvs_2,
                        ts.msac.rng,
                    );
                }
            } else {
                has_subpel_filter = true;
                b.c2rust_unnamed.c2rust_unnamed_0.inter_mode = NEWMV as libc::c_int as uint8_t;
                b.c2rust_unnamed.c2rust_unnamed_0.drl_idx = NEAREST_DRL as libc::c_int as uint8_t;
                if n_mvs_2 > 1 {
                    let drl_ctx_v1_0 = get_drl_context(&mvstack_2, 0);
                    b.c2rust_unnamed.c2rust_unnamed_0.drl_idx =
                        (b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_uint).wrapping_add(
                            dav1d_msac_decode_bool_adapt(
                                &mut ts.msac,
                                &mut ts.cdf.m.drl_bit[drl_ctx_v1_0 as usize],
                            ) as libc::c_uint,
                        ) as uint8_t as uint8_t;
                    if b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int
                        == NEARER_DRL as libc::c_int
                        && n_mvs_2 > 2
                    {
                        let drl_ctx_v2_2 = get_drl_context(&mvstack_2, 1);
                        b.c2rust_unnamed.c2rust_unnamed_0.drl_idx =
                            (b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_uint)
                                .wrapping_add(dav1d_msac_decode_bool_adapt(
                                    &mut ts.msac,
                                    &mut ts.cdf.m.drl_bit[drl_ctx_v2_2 as usize],
                                ) as libc::c_uint) as uint8_t
                                as uint8_t;
                    }
                }
                if !(b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int
                    >= NEAREST_DRL as libc::c_int
                    && b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int
                        <= NEARISH_DRL as libc::c_int)
                {
                    unreachable!();
                }
                if n_mvs_2 > 1 {
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0] = mvstack_2[b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as usize]
                        .mv
                        .mv[0];
                } else {
                    if b.c2rust_unnamed.c2rust_unnamed_0.drl_idx != 0 {
                        unreachable!();
                    }
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0] = mvstack_2[0].mv.mv[0];
                    fix_mv_precision(frame_hdr, &mut b.mv_mut()[0]);
                }
                if DEBUG_BLOCK_INFO(f, t) {
                    printf(
                        b"Post-intermode[%d,drl=%d]: r=%d\n\0" as *const u8 as *const libc::c_char,
                        b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int,
                        b.c2rust_unnamed.c2rust_unnamed_0.drl_idx as libc::c_int,
                        ts.msac.rng,
                    );
                }
                read_mv_residual(
                    t,
                    &mut *(b
                        .c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv)
                        .as_mut_ptr()
                        .offset(0),
                    &mut ts.cdf.mv,
                    frame_hdr.force_integer_mv == 0,
                );
                if DEBUG_BLOCK_INFO(f, t) {
                    printf(
                        b"Post-residualmv[mv=y:%d,x:%d]: r=%d\n\0" as *const u8
                            as *const libc::c_char,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0]
                            .y as libc::c_int,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0]
                            .x as libc::c_int,
                        ts.msac.rng,
                    );
                }
            }
            let ii_sz_grp = dav1d_ymode_size_context[bs as usize] as libc::c_int;
            if (*f.seq_hdr).inter_intra != 0
                && interintra_allowed_mask
                    & ((1 as libc::c_int) << bs as libc::c_uint) as libc::c_uint
                    != 0
                && dav1d_msac_decode_bool_adapt(
                    &mut ts.msac,
                    &mut ts.cdf.m.interintra[ii_sz_grp as usize],
                )
            {
                b.c2rust_unnamed
                    .c2rust_unnamed_0
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .interintra_mode = dav1d_msac_decode_symbol_adapt4(
                    &mut ts.msac,
                    &mut ts.cdf.m.interintra_mode[ii_sz_grp as usize],
                    (N_INTER_INTRA_PRED_MODES as libc::c_int - 1) as size_t,
                ) as uint8_t;
                let wedge_ctx = dav1d_wedge_ctx_lut[bs as usize] as libc::c_int;
                b.c2rust_unnamed.c2rust_unnamed_0.interintra_type =
                    (INTER_INTRA_BLEND as libc::c_int as libc::c_uint).wrapping_add(
                        dav1d_msac_decode_bool_adapt(
                            &mut ts.msac,
                            &mut ts.cdf.m.interintra_wedge[wedge_ctx as usize],
                        ) as libc::c_uint,
                    ) as uint8_t;
                if b.c2rust_unnamed.c2rust_unnamed_0.interintra_type as libc::c_int
                    == INTER_INTRA_WEDGE as libc::c_int
                {
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .wedge_idx = dav1d_msac_decode_symbol_adapt16(
                        &mut ts.msac,
                        &mut ts.cdf.m.wedge_idx[wedge_ctx as usize],
                        15 as libc::c_int as size_t,
                    ) as uint8_t;
                }
            } else {
                b.c2rust_unnamed.c2rust_unnamed_0.interintra_type =
                    INTER_INTRA_NONE as libc::c_int as uint8_t;
            }
            if DEBUG_BLOCK_INFO(f, t)
                && (*f.seq_hdr).inter_intra != 0
                && interintra_allowed_mask
                    & ((1 as libc::c_int) << bs as libc::c_uint) as libc::c_uint
                    != 0
            {
                printf(
                    b"Post-interintra[t=%d,m=%d,w=%d]: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    b.c2rust_unnamed.c2rust_unnamed_0.interintra_type as libc::c_int,
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .interintra_mode as libc::c_int,
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .wedge_idx as libc::c_int,
                    ts.msac.rng,
                );
            }
            if frame_hdr.switchable_motion_mode != 0
                && b.c2rust_unnamed.c2rust_unnamed_0.interintra_type as libc::c_int
                    == INTER_INTRA_NONE as libc::c_int
                && imin(bw4, bh4) >= 2
                && !(frame_hdr.force_integer_mv == 0
                    && b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                        == GLOBALMV as libc::c_int
                    && frame_hdr.gmv[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize].type_0
                        as libc::c_uint
                        > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint)
                && (have_left && findoddzero(&t.l.intra.0[by4 as usize..][..h4 as usize])
                    || have_top && findoddzero(&(*t.a).intra.0[bx4 as usize..][..w4 as usize]))
            {
                let mut mask: [uint64_t; 2] =
                    [0 as libc::c_int as uint64_t, 0 as libc::c_int as uint64_t];
                find_matching_ref(
                    t,
                    intra_edge_flags,
                    bw4,
                    bh4,
                    w4,
                    h4,
                    have_left,
                    have_top,
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int,
                    mask.as_mut_ptr(),
                );
                let allow_warp =
                    (f.svc[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize][0].scale == 0
                        && frame_hdr.force_integer_mv == 0
                        && frame_hdr.warp_motion != 0
                        && mask[0] | mask[1] != 0) as libc::c_int;
                b.c2rust_unnamed.c2rust_unnamed_0.motion_mode = (if allow_warp != 0 {
                    dav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.motion_mode[bs as usize],
                        2 as libc::c_int as size_t,
                    )
                } else {
                    dav1d_msac_decode_bool_adapt(&mut ts.msac, &mut ts.cdf.m.obmc[bs as usize])
                        as libc::c_uint
                }) as uint8_t;
                if b.c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                    == MM_WARP as libc::c_int
                {
                    has_subpel_filter = false;
                    t.warpmv = derive_warpmv(
                        t,
                        bw4,
                        bh4,
                        &mask,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0],
                        t.warpmv.clone(),
                    );
                    if DEBUG_BLOCK_INFO(f, t) {
                        printf(
                            b"[ %c%x %c%x %c%x\n  %c%x %c%x %c%x ]\nalpha=%c%x, beta=%c%x, gamma=%c%x, delta=%c%x, mv=y:%d,x:%d\n\0"
                                as *const u8 as *const libc::c_char,
                            if t.warpmv.matrix[0]
                                < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.matrix[0]).abs(),
                            if t.warpmv.matrix[1]
                                < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.matrix[1]).abs(),
                            if t.warpmv.matrix[2]
                                < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.matrix[2]).abs(),
                            if t.warpmv.matrix[3]
                                < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.matrix[3]).abs(),
                            if t.warpmv.matrix[4]
                                < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.matrix[4]).abs(),
                            if t.warpmv.matrix[5]
                                < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.matrix[5]).abs(),
                            if (t.warpmv.alpha() as libc::c_int) < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.alpha() as libc::c_int).abs(),
                            if (t.warpmv.beta() as libc::c_int) < 0 {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.beta() as libc::c_int).abs(),
                            if (t.warpmv.gamma() as libc::c_int) < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.gamma() as libc::c_int).abs(),
                            if (t.warpmv.delta() as libc::c_int) < 0
                            {
                                '-' as i32
                            } else {
                                ' ' as i32
                            },
                            (t.warpmv.delta() as libc::c_int).abs(),
                            b
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[0]
                                .y as libc::c_int,
                            b
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[0]
                                .x as libc::c_int,
                        );
                    }
                    if t.frame_thread.pass != 0 {
                        if t.warpmv.type_0 as libc::c_uint
                            == DAV1D_WM_TYPE_AFFINE as libc::c_int as libc::c_uint
                        {
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[0] =
                                (t.warpmv.matrix[2] - 0x10000 as libc::c_int) as int16_t;
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[1] = t.warpmv.matrix[3] as int16_t;
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[2] = t.warpmv.matrix[4] as int16_t;
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[3] =
                                (t.warpmv.matrix[5] - 0x10000 as libc::c_int) as int16_t;
                        } else {
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed_0
                                .matrix[0] = (-(32767 as libc::c_int) - 1) as int16_t;
                        }
                    }
                }
                if DEBUG_BLOCK_INFO(f, t) {
                    printf(
                        b"Post-motionmode[%d]: r=%d [mask: 0x%lx/0x%lx]\n\0" as *const u8
                            as *const libc::c_char,
                        b.c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int,
                        ts.msac.rng,
                        mask[0],
                        mask[1],
                    );
                }
            } else {
                b.c2rust_unnamed.c2rust_unnamed_0.motion_mode =
                    MM_TRANSLATION as libc::c_int as uint8_t;
            }
        }
        let mut filter_0: [Dav1dFilterMode; 2] = [DAV1D_FILTER_8TAP_REGULAR; 2];
        if frame_hdr.subpel_filter_mode as libc::c_uint
            == DAV1D_FILTER_SWITCHABLE as libc::c_int as libc::c_uint
        {
            if has_subpel_filter {
                let comp = b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int
                    != COMP_INTER_NONE as libc::c_int;
                let ctx1_1 = get_filter_ctx(
                    &*t.a,
                    &t.l,
                    comp,
                    false,
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0],
                    by4,
                    bx4,
                );
                filter_0[0] = dav1d_msac_decode_symbol_adapt4(
                    &mut ts.msac,
                    &mut ts.cdf.m.filter.0[0][ctx1_1 as usize],
                    (DAV1D_N_SWITCHABLE_FILTERS as libc::c_int - 1) as size_t,
                ) as Dav1dFilterMode;
                if (*f.seq_hdr).dual_filter != 0 {
                    let ctx2_3 = get_filter_ctx(
                        &*t.a,
                        &t.l,
                        comp,
                        true,
                        b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0],
                        by4,
                        bx4,
                    );
                    if DEBUG_BLOCK_INFO(f, t) {
                        printf(
                            b"Post-subpel_filter1[%d,ctx=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            filter_0[0] as libc::c_uint,
                            ctx1_1 as libc::c_int,
                            ts.msac.rng,
                        );
                    }
                    filter_0[1] = dav1d_msac_decode_symbol_adapt4(
                        &mut ts.msac,
                        &mut ts.cdf.m.filter.0[1][ctx2_3 as usize],
                        (DAV1D_N_SWITCHABLE_FILTERS as libc::c_int - 1) as size_t,
                    ) as Dav1dFilterMode;
                    if DEBUG_BLOCK_INFO(f, t) {
                        printf(
                            b"Post-subpel_filter2[%d,ctx=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            filter_0[1] as libc::c_uint,
                            ctx2_3 as libc::c_int,
                            ts.msac.rng,
                        );
                    }
                } else {
                    filter_0[1] = filter_0[0];
                    if DEBUG_BLOCK_INFO(f, t) {
                        printf(
                            b"Post-subpel_filter[%d,ctx=%d]: r=%d\n\0" as *const u8
                                as *const libc::c_char,
                            filter_0[0] as libc::c_uint,
                            ctx1_1 as libc::c_int,
                            ts.msac.rng,
                        );
                    }
                }
            } else {
                filter_0[1] = DAV1D_FILTER_8TAP_REGULAR;
                filter_0[0] = filter_0[1];
            }
        } else {
            filter_0[1] = frame_hdr.subpel_filter_mode;
            filter_0[0] = filter_0[1];
        }
        b.c2rust_unnamed.c2rust_unnamed_0.filter2d =
            dav1d_filter_2d[filter_0[1] as usize][filter_0[0] as usize];
        read_vartx_tree(t, b, bs, bx4, by4);
        if t.frame_thread.pass == 1 {
            (f.bd_fn.read_coef_blocks).expect("non-null function pointer")(t, bs, b);
        } else if (f.bd_fn.recon_b_inter).expect("non-null function pointer")(t, bs, b) != 0 {
            return -(1 as libc::c_int);
        }
        if frame_hdr.loopfilter.level_y[0] != 0 || frame_hdr.loopfilter.level_y[1] != 0 {
            let is_globalmv = (b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                == (if is_comp {
                    GLOBALMV_GLOBALMV as libc::c_int
                } else {
                    GLOBALMV as libc::c_int
                })) as libc::c_int;
            let lf_lvls: *const [[uint8_t; 2]; 8] =
                &*(*(*(*(ts.lflvl).offset(b.seg_id as isize)).as_ptr().offset(0))
                    .as_ptr()
                    .offset(
                        (*(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_mut_ptr()
                            .offset(0) as libc::c_int
                            + 1) as isize,
                    ))
                .as_ptr()
                .offset((is_globalmv == 0) as libc::c_int as isize)
                    as *const uint8_t as *const [[uint8_t; 2]; 8];
            let tx_split: [uint16_t; 2] = [
                b.c2rust_unnamed.c2rust_unnamed_0.tx_split0 as uint16_t,
                b.c2rust_unnamed.c2rust_unnamed_0.tx_split1,
            ];
            let mut ytx: RectTxfmSize = b.c2rust_unnamed.c2rust_unnamed_0.max_ytx as RectTxfmSize;
            let mut uvtx: RectTxfmSize = b.uvtx as RectTxfmSize;
            if frame_hdr.segmentation.lossless[b.seg_id as usize] != 0 {
                ytx = TX_4X4 as libc::c_int as RectTxfmSize;
                uvtx = TX_4X4 as libc::c_int as RectTxfmSize;
            }
            dav1d_create_lf_mask_inter(
                &mut *t.lf_mask,
                f.lf.level,
                f.b4_stride,
                lf_lvls,
                t.bx,
                t.by,
                f.w4,
                f.h4,
                b.skip != 0,
                bs,
                ytx,
                &tx_split,
                uvtx,
                f.cur.p.layout,
                &mut (*t.a).tx_lpf_y.0[bx4 as usize..],
                &mut t.l.tx_lpf_y.0[by4 as usize..],
                if has_chroma {
                    Some((
                        &mut (*t.a).tx_lpf_uv.0[cbx4 as usize..],
                        &mut t.l.tx_lpf_uv.0[cby4 as usize..],
                    ))
                } else {
                    None
                },
            );
        }
        if is_comp {
            splat_tworef_mv(f.c, t, bs, b, bw4, bh4);
        } else {
            splat_oneref_mv(f.c, t, bs, b, bw4, bh4);
        }
        match bh4 {
            1 => {
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 * seg_pred) as uint8_t;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 * b.skip_mode as libc::c_int) as uint8_t;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = 0 as libc::c_int as uint8_t;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 * b.skip as libc::c_int) as uint8_t;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = 0 as libc::c_int as uint8_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias8))
                    .u8_0 = 0 as libc::c_int as uint8_t;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut alias8))
                    .u8_0 = 0x1 * b_dim[2 + 1] as uint8_t;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int)
                    as uint8_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(filter_0[0] as libc::c_uint)
                    as uint8_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(filter_0[1] as libc::c_uint)
                    as uint8_t;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int)
                    as uint8_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut int8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int)
                    as uint8_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut int8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_int)
                    as uint8_t;
            }
            2 => {
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 * seg_pred) as uint16_t;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 * b.skip_mode as libc::c_int) as uint16_t;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = 0 as libc::c_int as uint16_t;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 * b.skip as libc::c_int) as uint16_t;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = 0 as libc::c_int as uint16_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias16))
                    .u16_0 = 0 as libc::c_int as uint16_t;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut alias16))
                    .u16_0 = 0x101 * b_dim[2 + 1] as uint16_t;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int)
                    as uint16_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int as libc::c_uint)
                    .wrapping_mul(filter_0[0] as libc::c_uint)
                    as uint16_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int as libc::c_uint)
                    .wrapping_mul(filter_0[1] as libc::c_uint)
                    as uint16_t;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int)
                    as uint16_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut int8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int)
                    as uint16_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut int8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_int)
                    as uint16_t;
            }
            4 => {
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(seg_pred as libc::c_uint);
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(b.skip_mode as libc::c_uint);
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = 0 as libc::c_int as uint32_t;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(b.skip as libc::c_uint);
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = 0 as libc::c_int as uint32_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias32))
                    .u32_0 = 0 as libc::c_int as uint32_t;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(b_dim[2 + 1] as libc::c_uint);
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_uint);
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(filter_0[0] as libc::c_uint);
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(filter_0[1] as libc::c_uint);
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_uint);
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut int8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_uint);
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut int8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_uint,
                );
            }
            8 => {
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(seg_pred as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = 0 as libc::c_int as uint64_t;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = 0 as libc::c_int as uint64_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias64))
                    .u64_0 = 0 as libc::c_int as uint64_t;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset(by4 as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b_dim[2 + 1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut uint8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset(by4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut int8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(by4 as isize) as *mut int8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_ulonglong,
                ) as uint64_t;
            }
            16 => {
                let const_val_123: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(seg_pred as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_123;
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_123;
                let const_val_124: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_124;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_124;
                let const_val_125: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_125;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_125;
                let const_val_126: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_126;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_126;
                let const_val_127: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_127;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_127;
                let const_val_128: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_128;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_128;
                let const_val_129: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b_dim[2 + 1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_129;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_129;
                let const_val_130: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_130;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_130;
                let const_val_131: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_131;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_131;
                let const_val_132: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_132;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_132;
                let const_val_133: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_133;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_133;
                let const_val_134: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_134;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_134;
                let const_val_135: uint64_t = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_ulonglong,
                ) as uint64_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_135;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_135;
            }
            32 => {
                let const_val_136: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(seg_pred as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_136;
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_136;
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_136;
                (*(&mut *(t.l.seg_pred.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_136;
                let const_val_137: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_137;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_137;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_137;
                (*(&mut *(t.l.skip_mode.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_137;
                let const_val_138: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_138;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_138;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_138;
                (*(&mut *(t.l.intra.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_138;
                let const_val_139: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_139;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_139;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_139;
                (*(&mut *(t.l.skip.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_139;
                let const_val_140: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_140;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_140;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_140;
                (*(&mut *(t.l.pal_sz.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_140;
                let const_val_141: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_141;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_141;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_141;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_141;
                let const_val_142: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b_dim[2 + 1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_142;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_142;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_142;
                (*(&mut *(t.l.tx_intra.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_142;
                let const_val_143: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_143;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_143;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_143;
                (*(&mut *(t.l.comp_type.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_143;
                let const_val_144: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_144;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_144;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_144;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_144;
                let const_val_145: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_145;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_145;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_145;
                (*(&mut *(*(t.l.filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_145;
                let const_val_146: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset((by4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_146;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset((by4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_146;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset((by4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_146;
                (*(&mut *(t.l.mode.0).as_mut_ptr().offset((by4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_146;
                let const_val_147: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_147;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_147;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 16) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_147;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((by4 + 24) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_147;
                let const_val_148: uint64_t = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_ulonglong,
                ) as uint64_t;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_148;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_148;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 16) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_148;
                (*(&mut *(*(t.l.r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((by4 + 24) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_148;
            }
            _ => {}
        }
        match bw4 {
            1 => {
                (*(&mut *((*t.a).seg_pred.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 * seg_pred) as uint8_t;
                (*(&mut *((*t.a).skip_mode.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 * b.skip_mode as libc::c_int) as uint8_t;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = 0 as libc::c_int as uint8_t;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 * b.skip as libc::c_int) as uint8_t;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = 0 as libc::c_int as uint8_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias8))
                    .u8_0 = 0 as libc::c_int as uint8_t;
                (*(&mut *((*t.a).tx_intra.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut alias8))
                    .u8_0 = 0x1 * b_dim[2 + 0];
                (*(&mut *((*t.a).comp_type.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int)
                    as uint8_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(filter_0[0] as libc::c_uint)
                    as uint8_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int as libc::c_uint)
                    .wrapping_mul(filter_0[1] as libc::c_uint)
                    as uint8_t;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int)
                    as uint8_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut int8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int)
                    as uint8_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut int8_t as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_int)
                    as uint8_t;
            }
            2 => {
                (*(&mut *((*t.a).seg_pred.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 * seg_pred) as uint16_t;
                (*(&mut *((*t.a).skip_mode.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 * b.skip_mode as libc::c_int) as uint16_t;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = 0 as libc::c_int as uint16_t;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 * b.skip as libc::c_int) as uint16_t;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = 0 as libc::c_int as uint16_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias16))
                    .u16_0 = 0 as libc::c_int as uint16_t;
                (*(&mut *((*t.a).tx_intra.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut alias16))
                    .u16_0 = 0x101 * b_dim[2 + 0] as uint16_t;
                (*(&mut *((*t.a).comp_type.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int)
                    as uint16_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int as libc::c_uint)
                    .wrapping_mul(filter_0[0] as libc::c_uint)
                    as uint16_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int as libc::c_uint)
                    .wrapping_mul(filter_0[1] as libc::c_uint)
                    as uint16_t;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int)
                    as uint16_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut int8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_int)
                    as uint16_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut int8_t as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_int)
                    as uint16_t;
            }
            4 => {
                (*(&mut *((*t.a).seg_pred.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(seg_pred as libc::c_uint);
                (*(&mut *((*t.a).skip_mode.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(b.skip_mode as libc::c_uint);
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = 0 as libc::c_int as uint32_t;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(b.skip as libc::c_uint);
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = 0 as libc::c_int as uint32_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias32))
                    .u32_0 = 0 as libc::c_int as uint32_t;
                (*(&mut *((*t.a).tx_intra.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(b_dim[2 + 0] as libc::c_uint);
                (*(&mut *((*t.a).comp_type.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_uint);
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(filter_0[0] as libc::c_uint);
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(filter_0[1] as libc::c_uint);
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_uint);
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut int8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_uint);
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut int8_t as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint).wrapping_mul(
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_uint,
                );
            }
            8 => {
                (*(&mut *((*t.a).seg_pred.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(seg_pred as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).skip_mode.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = 0 as libc::c_int as uint64_t;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = 0 as libc::c_int as uint64_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias64))
                    .u64_0 = 0 as libc::c_int as uint64_t;
                (*(&mut *((*t.a).tx_intra.0).as_mut_ptr().offset(bx4 as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b_dim[2 + 0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).comp_type.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut uint8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset(bx4 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut int8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset(bx4 as isize) as *mut int8_t as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_ulonglong,
                ) as uint64_t;
            }
            16 => {
                let const_val_149: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(seg_pred as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).seg_pred.0)
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_149;
                (*(&mut *((*t.a).seg_pred.0)
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_149;
                let const_val_150: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).skip_mode.0).as_mut_ptr().offset((bx4 + 0) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_150;
                (*(&mut *((*t.a).skip_mode.0).as_mut_ptr().offset((bx4 + 8) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_150;
                let const_val_151: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_151;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_151;
                let const_val_152: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_152;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_152;
                let const_val_153: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_153;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_153;
                let const_val_154: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_154;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_154;
                let const_val_155: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b_dim[2 + 0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).tx_intra.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_155;
                (*(&mut *((*t.a).tx_intra.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_155;
                let const_val_156: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).comp_type.0).as_mut_ptr().offset((bx4 + 0) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_156;
                (*(&mut *((*t.a).comp_type.0).as_mut_ptr().offset((bx4 + 8) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_156;
                let const_val_157: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_157;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_157;
                let const_val_158: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_158;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_158;
                let const_val_159: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_159;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_159;
                let const_val_160: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_160;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_160;
                let const_val_161: uint64_t = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_ulonglong,
                ) as uint64_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_161;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_161;
            }
            32 => {
                let const_val_162: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(seg_pred as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).seg_pred.0)
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_162;
                (*(&mut *((*t.a).seg_pred.0)
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_162;
                (*(&mut *((*t.a).seg_pred.0).as_mut_ptr().offset((bx4 + 16) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_162;
                (*(&mut *((*t.a).seg_pred.0).as_mut_ptr().offset((bx4 + 24) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_162;
                let const_val_163: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).skip_mode.0).as_mut_ptr().offset((bx4 + 0) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_163;
                (*(&mut *((*t.a).skip_mode.0).as_mut_ptr().offset((bx4 + 8) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_163;
                (*(&mut *((*t.a).skip_mode.0)
                    .as_mut_ptr()
                    .offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_163;
                (*(&mut *((*t.a).skip_mode.0)
                    .as_mut_ptr()
                    .offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_163;
                let const_val_164: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_164;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_164;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_164;
                (*(&mut *((*t.a).intra.0).as_mut_ptr().offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_164;
                let const_val_165: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.skip as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_165;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_165;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_165;
                (*(&mut *((*t.a).skip.0).as_mut_ptr().offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_165;
                let const_val_166: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_166;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_166;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_166;
                (*(&mut *((*t.a).pal_sz.0).as_mut_ptr().offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_166;
                let const_val_167: uint64_t = 0 as libc::c_int as uint64_t;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_167;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_167;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_167;
                (*(&mut *(*(t.pal_sz_uv).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_167;
                let const_val_168: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b_dim[2 + 0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).tx_intra.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_168;
                (*(&mut *((*t.a).tx_intra.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_168;
                (*(&mut *((*t.a).tx_intra.0)
                    .as_mut_ptr()
                    .offset((bx4 + 16) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_168;
                (*(&mut *((*t.a).tx_intra.0)
                    .as_mut_ptr()
                    .offset((bx4 + 24) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_168;
                let const_val_169: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).comp_type.0).as_mut_ptr().offset((bx4 + 0) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_169;
                (*(&mut *((*t.a).comp_type.0).as_mut_ptr().offset((bx4 + 8) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_169;
                (*(&mut *((*t.a).comp_type.0)
                    .as_mut_ptr()
                    .offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_169;
                (*(&mut *((*t.a).comp_type.0)
                    .as_mut_ptr()
                    .offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_169;
                let const_val_170: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_170;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_170;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_170;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_170;
                let const_val_171: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(filter_0[1] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_171;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_171;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_171;
                (*(&mut *(*((*t.a).filter.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_171;
                let const_val_172: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset((bx4 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_172;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset((bx4 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_172;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset((bx4 + 16) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_172;
                (*(&mut *((*t.a).mode.0).as_mut_ptr().offset((bx4 + 24) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val_172;
                let const_val_173: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                    .wrapping_mul(b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as libc::c_ulonglong)
                    as uint64_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_173;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_173;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 16) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_173;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(0))
                    .as_mut_ptr()
                    .offset((bx4 + 24) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_173;
                let const_val_174: uint64_t = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    b.c2rust_unnamed.c2rust_unnamed_0.r#ref[1] as uint8_t as libc::c_ulonglong,
                ) as uint64_t;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 0) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_174;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 8) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_174;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 16) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_174;
                (*(&mut *(*((*t.a).r#ref.0).as_mut_ptr().offset(1))
                    .as_mut_ptr()
                    .offset((bx4 + 24) as isize) as *mut int8_t
                    as *mut alias64))
                    .u64_0 = const_val_174;
            }
            _ => {}
        }
        if has_chroma {
            match cbh4 {
                1 => {
                    (*(&mut *(t.l.uvmode.0).as_mut_ptr().offset(cby4 as isize) as *mut uint8_t
                        as *mut alias8))
                        .u8_0 = (0x1 * DC_PRED as libc::c_int) as uint8_t;
                }
                2 => {
                    (*(&mut *(t.l.uvmode.0).as_mut_ptr().offset(cby4 as isize) as *mut uint8_t
                        as *mut alias16))
                        .u16_0 = (0x101 * DC_PRED as libc::c_int) as uint16_t;
                }
                4 => {
                    (*(&mut *(t.l.uvmode.0).as_mut_ptr().offset(cby4 as isize) as *mut uint8_t
                        as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(DC_PRED as libc::c_int as libc::c_uint);
                }
                8 => {
                    (*(&mut *(t.l.uvmode.0).as_mut_ptr().offset(cby4 as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(DC_PRED as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                }
                16 => {
                    let const_val_175: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(DC_PRED as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(t.l.uvmode.0)
                        .as_mut_ptr()
                        .offset((cby4 + 0) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_175;
                    (*(&mut *(t.l.uvmode.0)
                        .as_mut_ptr()
                        .offset((cby4 + 8) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_175;
                }
                32 => {
                    let const_val_176: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(DC_PRED as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *(t.l.uvmode.0)
                        .as_mut_ptr()
                        .offset((cby4 + 0) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_176;
                    (*(&mut *(t.l.uvmode.0)
                        .as_mut_ptr()
                        .offset((cby4 + 8) as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = const_val_176;
                    (*(&mut *(t.l.uvmode.0).as_mut_ptr().offset((cby4 + 16) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_176;
                    (*(&mut *(t.l.uvmode.0).as_mut_ptr().offset((cby4 + 24) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_176;
                }
                _ => {}
            }
            match cbw4 {
                1 => {
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset(cbx4 as isize) as *mut uint8_t
                        as *mut alias8))
                        .u8_0 = (0x1 * DC_PRED as libc::c_int) as uint8_t;
                }
                2 => {
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset(cbx4 as isize) as *mut uint8_t
                        as *mut alias16))
                        .u16_0 = (0x101 * DC_PRED as libc::c_int) as uint16_t;
                }
                4 => {
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset(cbx4 as isize) as *mut uint8_t
                        as *mut alias32))
                        .u32_0 = (0x1010101 as libc::c_uint)
                        .wrapping_mul(DC_PRED as libc::c_int as libc::c_uint);
                }
                8 => {
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset(cbx4 as isize) as *mut uint8_t
                        as *mut alias64))
                        .u64_0 = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(DC_PRED as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                }
                16 => {
                    let const_val_177: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(DC_PRED as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset((cbx4 + 0) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_177;
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset((cbx4 + 8) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_177;
                }
                32 => {
                    let const_val_178: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(DC_PRED as libc::c_int as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset((cbx4 + 0) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_178;
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset((cbx4 + 8) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_178;
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset((cbx4 + 16) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_178;
                    (*(&mut *((*t.a).uvmode.0).as_mut_ptr().offset((cbx4 + 24) as isize)
                        as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_178;
                }
                _ => {}
            }
        }
    }
    if frame_hdr.segmentation.enabled != 0 && frame_hdr.segmentation.update_map != 0 {
        let mut seg_ptr: *mut uint8_t = &mut *(f.cur_segmap)
            .offset((t.by as isize * f.b4_stride + t.bx as isize) as isize)
            as *mut uint8_t;
        match bw4 {
            1 => {
                let mut y_3 = 0;
                while y_3 < bh4 {
                    (*(&mut *seg_ptr.offset(0) as *mut uint8_t as *mut alias8)).u8_0 =
                        (0x1 * b.seg_id as libc::c_int) as uint8_t;
                    seg_ptr = seg_ptr.offset(f.b4_stride as isize);
                    y_3 += 1;
                }
            }
            2 => {
                let mut y_4 = 0;
                while y_4 < bh4 {
                    (*(&mut *seg_ptr.offset(0) as *mut uint8_t as *mut alias16)).u16_0 =
                        (0x101 * b.seg_id as libc::c_int) as uint16_t;
                    seg_ptr = seg_ptr.offset(f.b4_stride as isize);
                    y_4 += 1;
                }
            }
            4 => {
                let mut y_5 = 0;
                while y_5 < bh4 {
                    (*(&mut *seg_ptr.offset(0) as *mut uint8_t as *mut alias32)).u32_0 =
                        (0x1010101 as libc::c_uint).wrapping_mul(b.seg_id as libc::c_uint);
                    seg_ptr = seg_ptr.offset(f.b4_stride as isize);
                    y_5 += 1;
                }
            }
            8 => {
                let mut y_6 = 0;
                while y_6 < bh4 {
                    (*(&mut *seg_ptr.offset(0) as *mut uint8_t as *mut alias64)).u64_0 =
                        (0x101010101010101 as libc::c_ulonglong)
                            .wrapping_mul(b.seg_id as libc::c_ulonglong)
                            as uint64_t;
                    seg_ptr = seg_ptr.offset(f.b4_stride as isize);
                    y_6 += 1;
                }
            }
            16 => {
                let mut y_7 = 0;
                while y_7 < bh4 {
                    let const_val_179: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(b.seg_id as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *seg_ptr.offset((0 + 0) as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_179;
                    (*(&mut *seg_ptr.offset((0 + 8) as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_179;
                    seg_ptr = seg_ptr.offset(f.b4_stride as isize);
                    y_7 += 1;
                }
            }
            32 => {
                let mut y_8 = 0;
                while y_8 < bh4 {
                    let const_val_180: uint64_t = (0x101010101010101 as libc::c_ulonglong)
                        .wrapping_mul(b.seg_id as libc::c_ulonglong)
                        as uint64_t;
                    (*(&mut *seg_ptr.offset((0 + 0) as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_180;
                    (*(&mut *seg_ptr.offset((0 + 8) as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_180;
                    (*(&mut *seg_ptr.offset((0 + 16) as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_180;
                    (*(&mut *seg_ptr.offset((0 + 24) as isize) as *mut uint8_t as *mut alias64))
                        .u64_0 = const_val_180;
                    seg_ptr = seg_ptr.offset(f.b4_stride as isize);
                    y_8 += 1;
                }
            }
            _ => {}
        }
    }
    if b.skip == 0 {
        let mut noskip_mask: *mut [uint16_t; 2] = &mut *((*t.lf_mask).noskip_mask)
            .as_mut_ptr()
            .offset((by4 >> 1) as isize)
            as *mut [uint16_t; 2];
        let mask_0: libc::c_uint = !(0 as libc::c_uint) >> 32 - bw4 << (bx4 & 15);
        let bx_idx = (bx4 & 16) >> 4;
        let mut y_9 = 0;
        while y_9 < bh4 {
            (*noskip_mask)[bx_idx as usize] =
                ((*noskip_mask)[bx_idx as usize] as libc::c_uint | mask_0) as uint16_t;
            if bw4 == 32 {
                (*noskip_mask)[1] = ((*noskip_mask)[1] as libc::c_uint | mask_0) as uint16_t;
            }
            y_9 += 2 as libc::c_int;
            noskip_mask = noskip_mask.offset(1);
        }
    }
    if t.frame_thread.pass == 1
        && b.intra == 0
        && frame_hdr.frame_type as libc::c_uint & 1 as libc::c_uint != 0
    {
        let sby_0 = t.by - ts.tiling.row_start >> f.sb_shift;
        let lowest_px = &mut *ts.lowest_pixel.offset(sby_0 as isize);
        if b.c2rust_unnamed.c2rust_unnamed_0.comp_type as libc::c_int
            == COMP_INTER_NONE as libc::c_int
        {
            if imin(bw4, bh4) > 1
                && (b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                    == GLOBALMV as libc::c_int
                    && f.gmv_warp_allowed[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                        as libc::c_int
                        != 0
                    || b.c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                        == MM_WARP as libc::c_int
                        && t.warpmv.type_0 as libc::c_uint
                            > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint)
            {
                affine_lowest_px_luma(
                    t,
                    &mut lowest_px[b.r#ref()[0] as usize][0],
                    b_dim,
                    if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                        == MM_WARP as libc::c_int
                    {
                        &t.warpmv
                    } else {
                        &mut *(frame_hdr.gmv).as_mut_ptr().offset(
                            *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                .as_mut_ptr()
                                .offset(0) as isize,
                        )
                    },
                );
            } else {
                mc_lowest_px(
                    &mut lowest_px[b.r#ref()[0] as usize][0],
                    t.by,
                    bh4,
                    b.c2rust_unnamed
                        .c2rust_unnamed_0
                        .c2rust_unnamed
                        .c2rust_unnamed
                        .mv[0]
                        .y as libc::c_int,
                    0 as libc::c_int,
                    &*(*(f.svc).as_ptr().offset(
                        *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                            .as_mut_ptr()
                            .offset(0) as isize,
                    ))
                    .as_ptr()
                    .offset(1),
                );
                if b.c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                    == MM_OBMC as libc::c_int
                {
                    obmc_lowest_px(t, lowest_px, false, b_dim, bx4, by4, w4, h4);
                }
            }
            if has_chroma {
                let mut is_sub8x8 = (bw4 == ss_hor || bh4 == ss_ver) as libc::c_int;
                let mut r_1: *const *mut refmvs_block = 0 as *const *mut refmvs_block;
                if is_sub8x8 != 0 {
                    if !(ss_hor == 1) {
                        unreachable!();
                    }
                    r_1 = &mut *(t.rt.r).as_mut_ptr().offset(((t.by & 31) + 5) as isize)
                        as *mut *mut refmvs_block;
                    if bw4 == 1 {
                        is_sub8x8 &= ((*(*r_1.offset(0)).offset((t.bx - 1) as isize))
                            .0
                            .r#ref
                            .r#ref[0] as libc::c_int
                            > 0) as libc::c_int;
                    }
                    if bh4 == ss_ver {
                        is_sub8x8 &= ((*(*r_1.offset(-(1 as libc::c_int) as isize))
                            .offset(t.bx as isize))
                        .0
                        .r#ref
                        .r#ref[0] as libc::c_int
                            > 0) as libc::c_int;
                    }
                    if bw4 == 1 && bh4 == ss_ver {
                        is_sub8x8 &= ((*(*r_1.offset(-(1 as libc::c_int) as isize))
                            .offset((t.bx - 1) as isize))
                        .0
                        .r#ref
                        .r#ref[0] as libc::c_int
                            > 0) as libc::c_int;
                    }
                }
                if is_sub8x8 != 0 {
                    if !(ss_hor == 1) {
                        unreachable!();
                    }
                    if bw4 == 1 && bh4 == ss_ver {
                        let rr_1: *const refmvs_block = &mut *(*r_1
                            .offset(-(1 as libc::c_int) as isize))
                        .offset((t.bx - 1) as isize)
                            as *mut refmvs_block;
                        mc_lowest_px(
                            &mut lowest_px[(*rr_1).0.r#ref.r#ref[0] as usize - 1][1],
                            t.by - 1,
                            bh4,
                            (*rr_1).0.mv.mv[0].y as libc::c_int,
                            ss_ver,
                            &*(*(f.svc).as_ptr().offset(
                                (*((*rr_1).0.r#ref.r#ref).as_ptr().offset(0) as libc::c_int - 1)
                                    as isize,
                            ))
                            .as_ptr()
                            .offset(1),
                        );
                    }
                    if bw4 == 1 {
                        let rr_2: *const refmvs_block =
                            &mut *(*r_1.offset(0)).offset((t.bx - 1) as isize) as *mut refmvs_block;
                        mc_lowest_px(
                            &mut lowest_px[(*rr_2).0.r#ref.r#ref[0] as usize - 1][1],
                            t.by,
                            bh4,
                            (*rr_2).0.mv.mv[0].y as libc::c_int,
                            ss_ver,
                            &*(*(f.svc).as_ptr().offset(
                                (*((*rr_2).0.r#ref.r#ref).as_ptr().offset(0) as libc::c_int - 1)
                                    as isize,
                            ))
                            .as_ptr()
                            .offset(1),
                        );
                    }
                    if bh4 == ss_ver {
                        let rr_3: *const refmvs_block =
                            &mut *(*r_1.offset(-(1 as libc::c_int) as isize)).offset(t.bx as isize)
                                as *mut refmvs_block;
                        mc_lowest_px(
                            &mut lowest_px[(*rr_3).0.r#ref.r#ref[0] as usize - 1][1],
                            t.by - 1,
                            bh4,
                            (*rr_3).0.mv.mv[0].y as libc::c_int,
                            ss_ver,
                            &*(*(f.svc).as_ptr().offset(
                                (*((*rr_3).0.r#ref.r#ref).as_ptr().offset(0) as libc::c_int - 1)
                                    as isize,
                            ))
                            .as_ptr()
                            .offset(1),
                        );
                    }
                    mc_lowest_px(
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        t.by,
                        bh4,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0]
                            .y as libc::c_int,
                        ss_ver,
                        &*(*(f.svc).as_ptr().offset(
                            *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                .as_mut_ptr()
                                .offset(0) as isize,
                        ))
                        .as_ptr()
                        .offset(1),
                    );
                } else if imin(cbw4, cbh4) > 1
                    && (b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                        == GLOBALMV as libc::c_int
                        && f.gmv_warp_allowed[b.c2rust_unnamed.c2rust_unnamed_0.r#ref[0] as usize]
                            as libc::c_int
                            != 0
                        || b.c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                            == MM_WARP as libc::c_int
                            && t.warpmv.type_0 as libc::c_uint
                                > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint)
                {
                    affine_lowest_px_chroma(
                        t,
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        b_dim,
                        if (*b).c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                            == MM_WARP as libc::c_int
                        {
                            &t.warpmv
                        } else {
                            &mut *(frame_hdr.gmv).as_mut_ptr().offset(
                                *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                    .as_mut_ptr()
                                    .offset(0) as isize,
                            )
                        },
                    );
                } else {
                    mc_lowest_px(
                        &mut lowest_px[b.r#ref()[0] as usize][1],
                        t.by & !ss_ver,
                        bh4 << (bh4 == ss_ver) as libc::c_int,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[0]
                            .y as libc::c_int,
                        ss_ver,
                        &*(*(f.svc).as_ptr().offset(
                            *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                .as_mut_ptr()
                                .offset(0) as isize,
                        ))
                        .as_ptr()
                        .offset(1),
                    );
                    if b.c2rust_unnamed.c2rust_unnamed_0.motion_mode as libc::c_int
                        == MM_OBMC as libc::c_int
                    {
                        obmc_lowest_px(t, lowest_px, true, b_dim, bx4, by4, w4, h4);
                    }
                }
            }
        } else {
            let mut i_0 = 0;
            while i_0 < 2 {
                if b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                    == GLOBALMV_GLOBALMV as libc::c_int
                    && f.gmv_warp_allowed
                        [b.c2rust_unnamed.c2rust_unnamed_0.r#ref[i_0 as usize] as usize]
                        as libc::c_int
                        != 0
                {
                    affine_lowest_px_luma(
                        t,
                        &mut lowest_px[b.r#ref()[i_0 as usize] as usize][0],
                        b_dim,
                        &mut *(frame_hdr.gmv).as_mut_ptr().offset(
                            *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                .as_mut_ptr()
                                .offset(i_0 as isize) as isize,
                        ),
                    );
                } else {
                    mc_lowest_px(
                        &mut lowest_px[b.r#ref()[i_0 as usize] as usize][0],
                        t.by,
                        bh4,
                        b.c2rust_unnamed
                            .c2rust_unnamed_0
                            .c2rust_unnamed
                            .c2rust_unnamed
                            .mv[i_0 as usize]
                            .y as libc::c_int,
                        0 as libc::c_int,
                        &*(*(f.svc).as_ptr().offset(
                            *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                .as_mut_ptr()
                                .offset(i_0 as isize) as isize,
                        ))
                        .as_ptr()
                        .offset(1),
                    );
                }
                i_0 += 1;
            }
            if has_chroma {
                let mut i_1 = 0;
                while i_1 < 2 {
                    if b.c2rust_unnamed.c2rust_unnamed_0.inter_mode as libc::c_int
                        == GLOBALMV_GLOBALMV as libc::c_int
                        && imin(cbw4, cbh4) > 1
                        && f.gmv_warp_allowed
                            [b.c2rust_unnamed.c2rust_unnamed_0.r#ref[i_1 as usize] as usize]
                            as libc::c_int
                            != 0
                    {
                        affine_lowest_px_chroma(
                            t,
                            &mut lowest_px[b.r#ref()[i_1 as usize] as usize][1],
                            b_dim,
                            &mut *(frame_hdr.gmv).as_mut_ptr().offset(
                                *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                    .as_mut_ptr()
                                    .offset(i_1 as isize) as isize,
                            ),
                        );
                    } else {
                        mc_lowest_px(
                            &mut lowest_px[b.r#ref()[i_1 as usize] as usize][1],
                            t.by,
                            bh4,
                            b.c2rust_unnamed
                                .c2rust_unnamed_0
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .mv[i_1 as usize]
                                .y as libc::c_int,
                            ss_ver,
                            &*(*(f.svc).as_ptr().offset(
                                *(b.c2rust_unnamed.c2rust_unnamed_0.r#ref)
                                    .as_mut_ptr()
                                    .offset(i_1 as isize) as isize,
                            ))
                            .as_ptr()
                            .offset(1),
                        );
                    }
                    i_1 += 1;
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn decode_sb(
    t: *mut Dav1dTaskContext,
    bl: BlockLevel,
    node: *const EdgeNode,
) -> libc::c_int {
    let f: *const Dav1dFrameContext = (*t).f;
    let ts: *mut Dav1dTileState = (*t).ts;
    let hsz = 16 >> bl as libc::c_uint;
    let have_h_split = ((*f).bw > (*t).bx + hsz) as libc::c_int;
    let have_v_split = ((*f).bh > (*t).by + hsz) as libc::c_int;
    if have_h_split == 0 && have_v_split == 0 {
        if !((bl as libc::c_uint) < BL_8X8 as libc::c_int as libc::c_uint) {
            unreachable!();
        }
        return decode_sb(
            t,
            (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as BlockLevel,
            (*(node as *const EdgeBranch)).split[0],
        );
    }
    let mut pc = &mut Default::default();
    let mut bp: BlockPartition = PARTITION_NONE;
    let mut ctx = 0;
    let mut bx8 = 0;
    let mut by8 = 0;
    if (*t).frame_thread.pass != 2 as libc::c_int {
        if 0 as libc::c_int != 0 && bl as libc::c_uint == BL_64X64 as libc::c_int as libc::c_uint {
            printf(
                b"poc=%d,y=%d,x=%d,bl=%d,r=%d\n\0" as *const u8 as *const libc::c_char,
                (*(*f).frame_hdr).frame_offset,
                (*t).by,
                (*t).bx,
                bl as libc::c_uint,
                (*ts).msac.rng,
            );
        }
        bx8 = ((*t).bx & 31) >> 1;
        by8 = ((*t).by & 31) >> 1;
        ctx = get_partition_ctx(&*(*t).a, &(*t).l, bl, by8, bx8);
        pc = &mut (*ts).cdf.m.partition[bl as usize][ctx as usize];
    }
    if have_h_split != 0 && have_v_split != 0 {
        if (*t).frame_thread.pass == 2 {
            let b: *const Av1Block = &mut *((*f).frame_thread.b)
                .offset(((*t).by as isize * (*f).b4_stride + (*t).bx as isize) as isize)
                as *mut Av1Block;
            bp = (if (*b).bl as libc::c_uint == bl as libc::c_uint {
                (*b).bp as libc::c_int
            } else {
                PARTITION_SPLIT as libc::c_int
            }) as BlockPartition;
        } else {
            bp = dav1d_msac_decode_symbol_adapt16(
                &mut (*ts).msac,
                pc,
                dav1d_partition_type_count[bl as usize] as size_t,
            ) as BlockPartition;
            if (*f).cur.p.layout as libc::c_uint
                == DAV1D_PIXEL_LAYOUT_I422 as libc::c_int as libc::c_uint
                && (bp as libc::c_uint == PARTITION_V as libc::c_int as libc::c_uint
                    || bp as libc::c_uint == PARTITION_V4 as libc::c_int as libc::c_uint
                    || bp as libc::c_uint == PARTITION_T_LEFT_SPLIT as libc::c_int as libc::c_uint
                    || bp as libc::c_uint == PARTITION_T_RIGHT_SPLIT as libc::c_int as libc::c_uint)
            {
                return 1 as libc::c_int;
            }
            if DEBUG_BLOCK_INFO(&*f, &*t) {
                printf(
                    b"poc=%d,y=%d,x=%d,bl=%d,ctx=%d,bp=%d: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    (*(*f).frame_hdr).frame_offset,
                    (*t).by,
                    (*t).bx,
                    bl as libc::c_uint,
                    ctx as libc::c_int,
                    bp as libc::c_uint,
                    (*ts).msac.rng,
                );
            }
        }
        let b_0: *const uint8_t = (dav1d_block_sizes[bl as usize][bp as usize]).as_ptr();
        match bp as libc::c_uint {
            0 => {
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_NONE,
                    (*node).o,
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
            }
            1 => {
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_H,
                    (*node).h[0],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_H,
                    (*node).h[1],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by -= hsz;
            }
            2 => {
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_V,
                    (*node).v[0],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_V,
                    (*node).v[1],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx -= hsz;
            }
            3 => {
                if bl as libc::c_uint == BL_8X8 as libc::c_int as libc::c_uint {
                    let tip: *const EdgeTip = node as *const EdgeTip;
                    if !(hsz == 1) {
                        unreachable!();
                    }
                    if decode_b(&mut *t, bl, BS_4x4, PARTITION_SPLIT, (*tip).split[0]) != 0 {
                        return -(1 as libc::c_int);
                    }
                    let tl_filter: Filter2d = (*t).tl_4x4_filter;
                    (*t).bx += 1;
                    if decode_b(&mut *t, bl, BS_4x4, PARTITION_SPLIT, (*tip).split[1]) != 0 {
                        return -(1 as libc::c_int);
                    }
                    (*t).bx -= 1;
                    (*t).by += 1;
                    if decode_b(&mut *t, bl, BS_4x4, PARTITION_SPLIT, (*tip).split[2]) != 0 {
                        return -(1 as libc::c_int);
                    }
                    (*t).bx += 1;
                    (*t).tl_4x4_filter = tl_filter;
                    if decode_b(&mut *t, bl, BS_4x4, PARTITION_SPLIT, (*tip).split[3]) != 0 {
                        return -(1 as libc::c_int);
                    }
                    (*t).bx -= 1;
                    (*t).by -= 1;
                    if (*t).frame_thread.pass != 0 {
                        let p = (*t).frame_thread.pass & 1;
                        (*ts).frame_thread[p as usize].cf =
                            (((*ts).frame_thread[p as usize].cf as uintptr_t).wrapping_add(63)
                                & !(63)) as *mut libc::c_void;
                    }
                } else {
                    let branch: *const EdgeBranch = node as *const EdgeBranch;
                    if decode_sb(
                        t,
                        (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                            as BlockLevel,
                        (*branch).split[0],
                    ) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    (*t).bx += hsz;
                    if decode_sb(
                        t,
                        (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                            as BlockLevel,
                        (*branch).split[1],
                    ) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    (*t).bx -= hsz;
                    (*t).by += hsz;
                    if decode_sb(
                        t,
                        (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                            as BlockLevel,
                        (*branch).split[2],
                    ) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    (*t).bx += hsz;
                    if decode_sb(
                        t,
                        (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                            as BlockLevel,
                        (*branch).split[3],
                    ) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    (*t).bx -= hsz;
                    (*t).by -= hsz;
                }
            }
            4 => {
                let branch_0: *const EdgeBranch = node as *const EdgeBranch;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_T_TOP_SPLIT,
                    (*branch_0).tts[0],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_T_TOP_SPLIT,
                    (*branch_0).tts[1],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx -= hsz;
                (*t).by += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(1) as BlockSize,
                    PARTITION_T_TOP_SPLIT,
                    (*branch_0).tts[2],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by -= hsz;
            }
            5 => {
                let branch_1: *const EdgeBranch = node as *const EdgeBranch;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_T_BOTTOM_SPLIT,
                    (*branch_1).tbs[0],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(1) as BlockSize,
                    PARTITION_T_BOTTOM_SPLIT,
                    (*branch_1).tbs[1],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(1) as BlockSize,
                    PARTITION_T_BOTTOM_SPLIT,
                    (*branch_1).tbs[2],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx -= hsz;
                (*t).by -= hsz;
            }
            6 => {
                let branch_2: *const EdgeBranch = node as *const EdgeBranch;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_T_LEFT_SPLIT,
                    (*branch_2).tls[0],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_T_LEFT_SPLIT,
                    (*branch_2).tls[1],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by -= hsz;
                (*t).bx += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(1) as BlockSize,
                    PARTITION_T_LEFT_SPLIT,
                    (*branch_2).tls[2],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx -= hsz;
            }
            7 => {
                let branch_3: *const EdgeBranch = node as *const EdgeBranch;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_T_RIGHT_SPLIT,
                    (*branch_3).trs[0],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(1) as BlockSize,
                    PARTITION_T_RIGHT_SPLIT,
                    (*branch_3).trs[1],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by += hsz;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(1) as BlockSize,
                    PARTITION_T_RIGHT_SPLIT,
                    (*branch_3).trs[2],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by -= hsz;
                (*t).bx -= hsz;
            }
            8 => {
                let branch_4: *const EdgeBranch = node as *const EdgeBranch;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_H4,
                    (*branch_4).h4[0],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by += hsz >> 1;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_H4,
                    (*branch_4).h4[1],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by += hsz >> 1;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_H4,
                    (*branch_4).h4[2],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).by += hsz >> 1;
                if (*t).by < (*f).bh {
                    if decode_b(
                        &mut *t,
                        bl,
                        *b_0.offset(0) as BlockSize,
                        PARTITION_H4,
                        (*branch_4).h4[3],
                    ) != 0
                    {
                        return -(1 as libc::c_int);
                    }
                }
                (*t).by -= hsz * 3 >> 1;
            }
            9 => {
                let branch_5: *const EdgeBranch = node as *const EdgeBranch;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_V4,
                    (*branch_5).v4[0],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx += hsz >> 1;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_V4,
                    (*branch_5).v4[1],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx += hsz >> 1;
                if decode_b(
                    &mut *t,
                    bl,
                    *b_0.offset(0) as BlockSize,
                    PARTITION_V4,
                    (*branch_5).v4[2],
                ) != 0
                {
                    return -(1 as libc::c_int);
                }
                (*t).bx += hsz >> 1;
                if (*t).bx < (*f).bw {
                    if decode_b(
                        &mut *t,
                        bl,
                        *b_0.offset(0) as BlockSize,
                        PARTITION_V4,
                        (*branch_5).v4[3],
                    ) != 0
                    {
                        return -(1 as libc::c_int);
                    }
                }
                (*t).bx -= hsz * 3 >> 1;
            }
            _ => {
                if 0 == 0 {
                    unreachable!();
                }
            }
        }
    } else if have_h_split != 0 {
        let mut is_split: libc::c_uint = 0;
        if (*t).frame_thread.pass == 2 {
            let b_1: *const Av1Block = &mut *((*f).frame_thread.b)
                .offset(((*t).by as isize * (*f).b4_stride + (*t).bx as isize) as isize)
                as *mut Av1Block;
            is_split =
                ((*b_1).bl as libc::c_uint != bl as libc::c_uint) as libc::c_int as libc::c_uint;
        } else {
            is_split = dav1d_msac_decode_bool(&mut (*ts).msac, gather_top_partition_prob(pc, bl))
                as libc::c_uint;
            if DEBUG_BLOCK_INFO(&*f, &*t) {
                printf(
                    b"poc=%d,y=%d,x=%d,bl=%d,ctx=%d,bp=%d: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    (*(*f).frame_hdr).frame_offset,
                    (*t).by,
                    (*t).bx,
                    bl as libc::c_uint,
                    ctx as libc::c_int,
                    if is_split != 0 {
                        PARTITION_SPLIT as libc::c_int
                    } else {
                        PARTITION_H as libc::c_int
                    },
                    (*ts).msac.rng,
                );
            }
        }
        if !((bl as libc::c_uint) < BL_8X8 as libc::c_int as libc::c_uint) {
            unreachable!();
        }
        if is_split != 0 {
            let branch_6: *const EdgeBranch = node as *const EdgeBranch;
            bp = PARTITION_SPLIT;
            if decode_sb(
                t,
                (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as BlockLevel,
                (*branch_6).split[0],
            ) != 0
            {
                return 1 as libc::c_int;
            }
            (*t).bx += hsz;
            if decode_sb(
                t,
                (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as BlockLevel,
                (*branch_6).split[1],
            ) != 0
            {
                return 1 as libc::c_int;
            }
            (*t).bx -= hsz;
        } else {
            bp = PARTITION_H;
            if decode_b(
                &mut *t,
                bl,
                dav1d_block_sizes[bl as usize][PARTITION_H as libc::c_int as usize][0] as BlockSize,
                PARTITION_H,
                (*node).h[0],
            ) != 0
            {
                return -(1 as libc::c_int);
            }
        }
    } else {
        if have_v_split == 0 {
            unreachable!();
        }
        let mut is_split_0: libc::c_uint = 0;
        if (*t).frame_thread.pass == 2 {
            let b_2: *const Av1Block = &mut *((*f).frame_thread.b)
                .offset(((*t).by as isize * (*f).b4_stride + (*t).bx as isize) as isize)
                as *mut Av1Block;
            is_split_0 =
                ((*b_2).bl as libc::c_uint != bl as libc::c_uint) as libc::c_int as libc::c_uint;
        } else {
            is_split_0 = dav1d_msac_decode_bool(&mut (*ts).msac, gather_left_partition_prob(pc, bl))
                as libc::c_uint;
            if (*f).cur.p.layout as libc::c_uint
                == DAV1D_PIXEL_LAYOUT_I422 as libc::c_int as libc::c_uint
                && is_split_0 == 0
            {
                return 1 as libc::c_int;
            }
            if DEBUG_BLOCK_INFO(&*f, &*t) {
                printf(
                    b"poc=%d,y=%d,x=%d,bl=%d,ctx=%d,bp=%d: r=%d\n\0" as *const u8
                        as *const libc::c_char,
                    (*(*f).frame_hdr).frame_offset,
                    (*t).by,
                    (*t).bx,
                    bl as libc::c_uint,
                    ctx as libc::c_int,
                    if is_split_0 != 0 {
                        PARTITION_SPLIT as libc::c_int
                    } else {
                        PARTITION_V as libc::c_int
                    },
                    (*ts).msac.rng,
                );
            }
        }
        if !((bl as libc::c_uint) < BL_8X8 as libc::c_int as libc::c_uint) {
            unreachable!();
        }
        if is_split_0 != 0 {
            let branch_7: *const EdgeBranch = node as *const EdgeBranch;
            bp = PARTITION_SPLIT;
            if decode_sb(
                t,
                (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as BlockLevel,
                (*branch_7).split[0],
            ) != 0
            {
                return 1 as libc::c_int;
            }
            (*t).by += hsz;
            if decode_sb(
                t,
                (bl as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint) as BlockLevel,
                (*branch_7).split[2],
            ) != 0
            {
                return 1 as libc::c_int;
            }
            (*t).by -= hsz;
        } else {
            bp = PARTITION_V;
            if decode_b(
                &mut *t,
                bl,
                dav1d_block_sizes[bl as usize][PARTITION_V as libc::c_int as usize][0] as BlockSize,
                PARTITION_V,
                (*node).v[0],
            ) != 0
            {
                return -(1 as libc::c_int);
            }
        }
    }
    if (*t).frame_thread.pass != 2 as libc::c_int
        && (bp as libc::c_uint != PARTITION_SPLIT as libc::c_int as libc::c_uint
            || bl as libc::c_uint == BL_8X8 as libc::c_int as libc::c_uint)
    {
        match hsz {
            1 => {
                (*(&mut *((*(*t).a).partition.0).as_mut_ptr().offset(bx8 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * dav1d_al_part_ctx[0][bl as usize][bp as usize] as libc::c_int)
                    as uint8_t;
                (*(&mut *((*t).l.partition.0).as_mut_ptr().offset(by8 as isize) as *mut uint8_t
                    as *mut alias8))
                    .u8_0 = (0x1 as libc::c_int
                    * dav1d_al_part_ctx[1][bl as usize][bp as usize] as libc::c_int)
                    as uint8_t;
            }
            2 => {
                (*(&mut *((*(*t).a).partition.0).as_mut_ptr().offset(bx8 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * dav1d_al_part_ctx[0][bl as usize][bp as usize] as libc::c_int)
                    as uint16_t;
                (*(&mut *((*t).l.partition.0).as_mut_ptr().offset(by8 as isize) as *mut uint8_t
                    as *mut alias16))
                    .u16_0 = (0x101 as libc::c_int
                    * dav1d_al_part_ctx[1][bl as usize][bp as usize] as libc::c_int)
                    as uint16_t;
            }
            4 => {
                (*(&mut *((*(*t).a).partition.0).as_mut_ptr().offset(bx8 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(dav1d_al_part_ctx[0][bl as usize][bp as usize] as libc::c_uint);
                (*(&mut *((*t).l.partition.0).as_mut_ptr().offset(by8 as isize) as *mut uint8_t
                    as *mut alias32))
                    .u32_0 = (0x1010101 as libc::c_uint)
                    .wrapping_mul(dav1d_al_part_ctx[1][bl as usize][bp as usize] as libc::c_uint);
            }
            8 => {
                (*(&mut *((*(*t).a).partition.0).as_mut_ptr().offset(bx8 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    dav1d_al_part_ctx[0][bl as usize][bp as usize] as libc::c_ulonglong,
                ) as uint64_t;
                (*(&mut *((*t).l.partition.0).as_mut_ptr().offset(by8 as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    dav1d_al_part_ctx[1][bl as usize][bp as usize] as libc::c_ulonglong,
                ) as uint64_t;
            }
            16 => {
                let const_val: uint64_t = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    dav1d_al_part_ctx[0][bl as usize][bp as usize] as libc::c_ulonglong,
                ) as uint64_t;
                (*(&mut *((*(*t).a).partition.0)
                    .as_mut_ptr()
                    .offset((bx8 + 0) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val;
                (*(&mut *((*(*t).a).partition.0)
                    .as_mut_ptr()
                    .offset((bx8 + 8) as isize) as *mut uint8_t
                    as *mut alias64))
                    .u64_0 = const_val;
                let const_val_0: uint64_t = (0x101010101010101 as libc::c_ulonglong).wrapping_mul(
                    dav1d_al_part_ctx[1][bl as usize][bp as usize] as libc::c_ulonglong,
                ) as uint64_t;
                (*(&mut *((*t).l.partition.0).as_mut_ptr().offset((by8 + 0) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_0;
                (*(&mut *((*t).l.partition.0).as_mut_ptr().offset((by8 + 8) as isize)
                    as *mut uint8_t as *mut alias64))
                    .u64_0 = const_val_0;
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn reset_context(
    ctx: *mut BlockContext,
    keyframe: libc::c_int,
    pass: libc::c_int,
) {
    memset(
        ((*ctx).intra.0).as_mut_ptr() as *mut libc::c_void,
        keyframe,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    memset(
        ((*ctx).uvmode.0).as_mut_ptr() as *mut libc::c_void,
        DC_PRED as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    if keyframe != 0 {
        memset(
            ((*ctx).mode.0).as_mut_ptr() as *mut libc::c_void,
            DC_PRED as libc::c_int,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
    }
    if pass == 2 {
        return;
    }
    memset(
        ((*ctx).partition.0).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>(),
    );
    memset(
        ((*ctx).skip.0).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    memset(
        ((*ctx).skip_mode.0).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    memset(
        ((*ctx).tx_lpf_y.0).as_mut_ptr() as *mut libc::c_void,
        2 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    memset(
        ((*ctx).tx_lpf_uv.0).as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    memset(
        ((*ctx).tx_intra.0).as_mut_ptr() as *mut libc::c_void,
        -(1 as libc::c_int),
        ::core::mem::size_of::<[int8_t; 32]>(),
    );
    memset(
        ((*ctx).tx.0).as_mut_ptr() as *mut libc::c_void,
        TX_64X64 as libc::c_int,
        ::core::mem::size_of::<[int8_t; 32]>(),
    );
    if keyframe == 0 {
        memset(
            ((*ctx).r#ref.0).as_mut_ptr() as *mut libc::c_void,
            -(1 as libc::c_int),
            ::core::mem::size_of::<[[int8_t; 32]; 2]>(),
        );
        memset(
            ((*ctx).comp_type.0).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
        memset(
            ((*ctx).mode.0).as_mut_ptr() as *mut libc::c_void,
            NEARESTMV as libc::c_int,
            ::core::mem::size_of::<[uint8_t; 32]>(),
        );
    }
    memset(
        ((*ctx).lcoef.0).as_mut_ptr() as *mut libc::c_void,
        0x40 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    memset(
        ((*ctx).ccoef.0).as_mut_ptr() as *mut libc::c_void,
        0x40 as libc::c_int,
        ::core::mem::size_of::<[[uint8_t; 32]; 2]>(),
    );
    memset(
        ((*ctx).filter.0).as_mut_ptr() as *mut libc::c_void,
        DAV1D_N_SWITCHABLE_FILTERS as libc::c_int,
        ::core::mem::size_of::<[[uint8_t; 32]; 2]>(),
    );
    memset(
        ((*ctx).seg_pred.0).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    memset(
        ((*ctx).pal_sz.0).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
}
static mut ss_size_mul: [[uint8_t; 2]; 4] = [
    [4 as libc::c_int as uint8_t, 4 as libc::c_int as uint8_t],
    [6 as libc::c_int as uint8_t, 5 as libc::c_int as uint8_t],
    [8 as libc::c_int as uint8_t, 6 as libc::c_int as uint8_t],
    [12 as libc::c_int as uint8_t, 8 as libc::c_int as uint8_t],
];
unsafe extern "C" fn setup_tile(
    ts: *mut Dav1dTileState,
    f: *const Dav1dFrameContext,
    data: *const uint8_t,
    sz: size_t,
    tile_row: libc::c_int,
    tile_col: libc::c_int,
    tile_start_off: libc::c_int,
) {
    let col_sb_start = (*(*f).frame_hdr).tiling.col_start_sb[tile_col as usize] as libc::c_int;
    let col_sb128_start = col_sb_start >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int;
    let col_sb_end = (*(*f).frame_hdr).tiling.col_start_sb[(tile_col + 1) as usize] as libc::c_int;
    let row_sb_start = (*(*f).frame_hdr).tiling.row_start_sb[tile_row as usize] as libc::c_int;
    let row_sb_end = (*(*f).frame_hdr).tiling.row_start_sb[(tile_row + 1) as usize] as libc::c_int;
    let sb_shift = (*f).sb_shift;
    let size_mul: *const uint8_t = (ss_size_mul[(*f).cur.p.layout as usize]).as_ptr();
    let mut p = 0;
    while p < 2 {
        (*ts).frame_thread[p as usize].pal_idx = if !((*f).frame_thread.pal_idx).is_null() {
            &mut *((*f).frame_thread.pal_idx).offset(
                (tile_start_off as size_t)
                    .wrapping_mul(*size_mul.offset(1) as size_t)
                    .wrapping_div(4) as isize,
            ) as *mut uint8_t
        } else {
            0 as *mut uint8_t
        };
        (*ts).frame_thread[p as usize].cf = (if !((*f).frame_thread.cf).is_null() {
            ((*f).frame_thread.cf as *mut uint8_t).offset(
                ((tile_start_off as size_t).wrapping_mul(*size_mul.offset(0) as size_t)
                    >> ((*(*f).seq_hdr).hbd == 0) as libc::c_int) as isize,
            )
        } else {
            0 as *mut uint8_t
        }) as *mut libc::c_void;
        p += 1;
    }
    dav1d_cdf_thread_copy(&mut (*ts).cdf, &(*f).in_cdf);
    (*ts).last_qidx = (*(*f).frame_hdr).quant.yac;
    memset(
        ((*ts).last_delta_lf).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[int8_t; 4]>(),
    );
    dav1d_msac_init(
        &mut (*ts).msac,
        data,
        sz,
        (*(*f).frame_hdr).disable_cdf_update != 0,
    );
    (*ts).tiling.row = tile_row;
    (*ts).tiling.col = tile_col;
    (*ts).tiling.col_start = col_sb_start << sb_shift;
    (*ts).tiling.col_end = imin(col_sb_end << sb_shift, (*f).bw);
    (*ts).tiling.row_start = row_sb_start << sb_shift;
    (*ts).tiling.row_end = imin(row_sb_end << sb_shift, (*f).bh);
    let mut sb_idx = 0;
    let mut unit_idx = 0;
    if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
        sb_idx = ((*ts).tiling.row_start >> 5) * (*f).sr_sb128w;
        unit_idx = ((*ts).tiling.row_start & 16) >> 3;
    } else {
        sb_idx = ((*ts).tiling.row_start >> 5) * (*f).sb128w + col_sb128_start;
        unit_idx = (((*ts).tiling.row_start & 16) >> 3) + (((*ts).tiling.col_start & 16) >> 4);
    }
    let mut current_block_31: u64;
    let mut p_0 = 0;
    while p_0 < 3 {
        if !(((*f).lf.restore_planes >> p_0) as libc::c_uint & 1 as libc::c_uint == 0) {
            if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
                let ss_hor = (p_0 != 0
                    && (*f).cur.p.layout as libc::c_uint
                        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let d = (*(*f).frame_hdr).super_res.width_scale_denominator;
                let unit_size_log2 =
                    (*(*f).frame_hdr).restoration.unit_size[(p_0 != 0) as libc::c_int as usize];
                let rnd = ((8 as libc::c_int) << unit_size_log2) - 1;
                let shift = unit_size_log2 + 3;
                let x = (4 * (*ts).tiling.col_start * d >> ss_hor) + rnd >> shift;
                let px_x = x << unit_size_log2 + ss_hor;
                let u_idx = unit_idx + ((px_x & 64) >> 6);
                let sb128x = px_x >> 7;
                if sb128x >= (*f).sr_sb128w {
                    current_block_31 = 2370887241019905314;
                } else {
                    (*ts).lr_ref[p_0 as usize] =
                        &mut *(*((*((*f).lf.lr_mask).offset((sb_idx + sb128x) as isize)).lr)
                            .as_mut_ptr()
                            .offset(p_0 as isize))
                        .as_mut_ptr()
                        .offset(u_idx as isize) as *mut Av1RestorationUnit;
                    current_block_31 = 1608152415753874203;
                }
            } else {
                (*ts).lr_ref[p_0 as usize] =
                    &mut *(*((*((*f).lf.lr_mask).offset(sb_idx as isize)).lr)
                        .as_mut_ptr()
                        .offset(p_0 as isize))
                    .as_mut_ptr()
                    .offset(unit_idx as isize) as *mut Av1RestorationUnit;
                current_block_31 = 1608152415753874203;
            }
            match current_block_31 {
                2370887241019905314 => {}
                _ => {
                    (*(*ts).lr_ref[p_0 as usize]).filter_v[0] = 3 as libc::c_int as int8_t;
                    (*(*ts).lr_ref[p_0 as usize]).filter_v[1] = -(7 as libc::c_int) as int8_t;
                    (*(*ts).lr_ref[p_0 as usize]).filter_v[2] = 15 as libc::c_int as int8_t;
                    (*(*ts).lr_ref[p_0 as usize]).filter_h[0] = 3 as libc::c_int as int8_t;
                    (*(*ts).lr_ref[p_0 as usize]).filter_h[1] = -(7 as libc::c_int) as int8_t;
                    (*(*ts).lr_ref[p_0 as usize]).filter_h[2] = 15 as libc::c_int as int8_t;
                    (*(*ts).lr_ref[p_0 as usize]).sgr_weights[0] = -(32 as libc::c_int) as int8_t;
                    (*(*ts).lr_ref[p_0 as usize]).sgr_weights[1] = 31 as libc::c_int as int8_t;
                }
            }
        }
        p_0 += 1;
    }
    if (*(*f).c).n_tc > 1 as libc::c_uint {
        let mut p_1 = 0;
        while p_1 < 2 {
            *(&mut *((*ts).progress).as_mut_ptr().offset(p_1 as isize) as *mut atomic_int) =
                row_sb_start;
            p_1 += 1;
        }
    }
}
unsafe extern "C" fn read_restoration_info(
    t: *mut Dav1dTaskContext,
    lr: *mut Av1RestorationUnit,
    p: libc::c_int,
    frame_type: Dav1dRestorationType,
) {
    let f: *const Dav1dFrameContext = (*t).f;
    let ts: *mut Dav1dTileState = (*t).ts;
    if frame_type as libc::c_uint == DAV1D_RESTORATION_SWITCHABLE as libc::c_int as libc::c_uint {
        let filter = dav1d_msac_decode_symbol_adapt4(
            &mut (*ts).msac,
            &mut (*ts).cdf.m.restore_switchable.0,
            2 as libc::c_int as size_t,
        ) as libc::c_int;
        (*lr).type_0 = (if filter != 0 {
            if filter == 2 {
                DAV1D_RESTORATION_SGRPROJ as libc::c_int
            } else {
                DAV1D_RESTORATION_WIENER as libc::c_int
            }
        } else {
            DAV1D_RESTORATION_NONE as libc::c_int
        }) as uint8_t;
    } else {
        let type_0: libc::c_uint = dav1d_msac_decode_bool_adapt(
            &mut (*ts).msac,
            if frame_type as libc::c_uint == DAV1D_RESTORATION_WIENER as libc::c_int as libc::c_uint
            {
                &mut (*ts).cdf.m.restore_wiener.0
            } else {
                &mut (*ts).cdf.m.restore_sgrproj.0
            },
        ) as libc::c_uint;
        (*lr).type_0 = (if type_0 != 0 {
            frame_type as libc::c_uint
        } else {
            DAV1D_RESTORATION_NONE as libc::c_int as libc::c_uint
        }) as uint8_t;
    }
    if (*lr).type_0 as libc::c_int == DAV1D_RESTORATION_WIENER as libc::c_int {
        (*lr).filter_v[0] = (if p != 0 {
            0 as libc::c_int
        } else {
            dav1d_msac_decode_subexp(
                &mut (*ts).msac,
                ((*(*ts).lr_ref[p as usize]).filter_v[0] + 5) as libc::c_uint,
                16,
                1,
            ) - 5
        }) as int8_t;
        (*lr).filter_v[1] = (dav1d_msac_decode_subexp(
            &mut (*ts).msac,
            ((*(*ts).lr_ref[p as usize]).filter_v[1] + 23) as libc::c_uint,
            32,
            2,
        ) - 23) as int8_t;
        (*lr).filter_v[2] = (dav1d_msac_decode_subexp(
            &mut (*ts).msac,
            ((*(*ts).lr_ref[p as usize]).filter_v[2] + 17) as libc::c_uint,
            64,
            3,
        ) - 17) as int8_t;
        (*lr).filter_h[0] = (if p != 0 {
            0 as libc::c_int
        } else {
            dav1d_msac_decode_subexp(
                &mut (*ts).msac,
                ((*(*ts).lr_ref[p as usize]).filter_h[0] + 5) as libc::c_uint,
                16,
                1,
            ) - 5
        }) as int8_t;
        (*lr).filter_h[1] = (dav1d_msac_decode_subexp(
            &mut (*ts).msac,
            ((*(*ts).lr_ref[p as usize]).filter_h[1] + 23) as libc::c_uint,
            32,
            2,
        ) - 23) as int8_t;
        (*lr).filter_h[2] = (dav1d_msac_decode_subexp(
            &mut (*ts).msac,
            ((*(*ts).lr_ref[p as usize]).filter_h[2] + 17) as libc::c_uint,
            64,
            3,
        ) - 17) as int8_t;
        memcpy(
            ((*lr).sgr_weights).as_mut_ptr() as *mut libc::c_void,
            ((*(*ts).lr_ref[p as usize]).sgr_weights).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[int8_t; 2]>() as libc::c_ulong,
        );
        (*ts).lr_ref[p as usize] = lr;
        if DEBUG_BLOCK_INFO(&*f, &*t) {
            printf(
                b"Post-lr_wiener[pl=%d,v[%d,%d,%d],h[%d,%d,%d]]: r=%d\n\0" as *const u8
                    as *const libc::c_char,
                p,
                (*lr).filter_v[0] as libc::c_int,
                (*lr).filter_v[1] as libc::c_int,
                (*lr).filter_v[2] as libc::c_int,
                (*lr).filter_h[0] as libc::c_int,
                (*lr).filter_h[1] as libc::c_int,
                (*lr).filter_h[2] as libc::c_int,
                (*ts).msac.rng,
            );
        }
    } else if (*lr).type_0 as libc::c_int == DAV1D_RESTORATION_SGRPROJ as libc::c_int {
        let idx: libc::c_uint =
            dav1d_msac_decode_bools(&mut (*ts).msac, 4 as libc::c_int as libc::c_uint);
        let sgr_params: *const uint16_t = (dav1d_sgr_params[idx as usize]).as_ptr();
        (*lr).sgr_idx = idx as uint8_t;
        (*lr).sgr_weights[0] = (if *sgr_params.offset(0) as libc::c_int != 0 {
            dav1d_msac_decode_subexp(
                &mut (*ts).msac,
                ((*(*ts).lr_ref[p as usize]).sgr_weights[0] + 96) as libc::c_uint,
                128,
                4,
            ) - 96
        } else {
            0 as libc::c_int
        }) as int8_t;
        (*lr).sgr_weights[1] = (if *sgr_params.offset(1) as libc::c_int != 0 {
            dav1d_msac_decode_subexp(
                &mut (*ts).msac,
                ((*(*ts).lr_ref[p as usize]).sgr_weights[1] + 32) as libc::c_uint,
                128,
                4,
            ) - 32
        } else {
            95 as libc::c_int
        }) as int8_t;
        memcpy(
            ((*lr).filter_v).as_mut_ptr() as *mut libc::c_void,
            ((*(*ts).lr_ref[p as usize]).filter_v).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[int8_t; 3]>() as libc::c_ulong,
        );
        memcpy(
            ((*lr).filter_h).as_mut_ptr() as *mut libc::c_void,
            ((*(*ts).lr_ref[p as usize]).filter_h).as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[int8_t; 3]>() as libc::c_ulong,
        );
        (*ts).lr_ref[p as usize] = lr;
        if DEBUG_BLOCK_INFO(&*f, &*t) {
            printf(
                b"Post-lr_sgrproj[pl=%d,idx=%d,w[%d,%d]]: r=%d\n\0" as *const u8
                    as *const libc::c_char,
                p,
                (*lr).sgr_idx as libc::c_int,
                (*lr).sgr_weights[0] as libc::c_int,
                (*lr).sgr_weights[1] as libc::c_int,
                (*ts).msac.rng,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_tile_sbrow(t: *mut Dav1dTaskContext) -> libc::c_int {
    let f: *const Dav1dFrameContext = (*t).f;
    let root_bl: BlockLevel = (if (*(*f).seq_hdr).sb128 != 0 {
        BL_128X128 as libc::c_int
    } else {
        BL_64X64 as libc::c_int
    }) as BlockLevel;
    let ts: *mut Dav1dTileState = (*t).ts;
    let c: *const Dav1dContext = (*f).c;
    let sb_step = (*f).sb_step;
    let tile_row = (*ts).tiling.row;
    let tile_col = (*ts).tiling.col;
    let col_sb_start = (*(*f).frame_hdr).tiling.col_start_sb[tile_col as usize] as libc::c_int;
    let col_sb128_start = col_sb_start >> ((*(*f).seq_hdr).sb128 == 0) as libc::c_int;
    if (*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint != 0
        || (*(*f).frame_hdr).allow_intrabc != 0
    {
        dav1d_refmvs_tile_sbrow_init(
            &mut (*t).rt,
            &(*f).rf,
            (*ts).tiling.col_start,
            (*ts).tiling.col_end,
            (*ts).tiling.row_start,
            (*ts).tiling.row_end,
            (*t).by >> (*f).sb_shift,
            (*ts).tiling.row,
            (*t).frame_thread.pass,
        );
    }
    if (*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint != 0
        && (*c).n_fc > 1 as libc::c_uint
    {
        let sby = (*t).by - (*ts).tiling.row_start >> (*f).sb_shift;
        let lowest_px: *mut [libc::c_int; 2] =
            (*((*ts).lowest_pixel).offset(sby as isize)).as_mut_ptr();
        let mut n = 0;
        while n < 7 {
            let mut m = 0;
            while m < 2 {
                (*lowest_px.offset(n as isize))[m as usize] = -(2147483647 as libc::c_int) - 1;
                m += 1;
            }
            n += 1;
        }
    }
    reset_context(
        &mut (*t).l,
        ((*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint == 0) as libc::c_int,
        (*t).frame_thread.pass,
    );
    if (*t).frame_thread.pass == 2 {
        let off_2pass = if (*c).n_tc > 1 as libc::c_uint {
            (*f).sb128w * (*(*f).frame_hdr).tiling.rows
        } else {
            0 as libc::c_int
        };
        (*t).bx = (*ts).tiling.col_start;
        (*t).a = ((*f).a)
            .offset(off_2pass as isize)
            .offset(col_sb128_start as isize)
            .offset((tile_row * (*f).sb128w) as isize);
        while (*t).bx < (*ts).tiling.col_end {
            if ::core::intrinsics::atomic_load_acquire((*c).flush) != 0 {
                return 1 as libc::c_int;
            }
            if decode_sb(t, root_bl, (*c).intra_edge.root[root_bl as usize]) != 0 {
                return 1 as libc::c_int;
            }
            if (*t).bx & 16 != 0 || (*(*f).seq_hdr).sb128 != 0 {
                (*t).a = ((*t).a).offset(1);
            }
            (*t).bx += sb_step;
        }
        ((*f).bd_fn.backup_ipred_edge).expect("non-null function pointer")(t);
        return 0 as libc::c_int;
    }
    if (*ts).msac.cnt < -(15 as libc::c_int) {
        return 1 as libc::c_int;
    }
    if (*(*f).c).n_tc > 1 as libc::c_uint && (*(*f).frame_hdr).use_ref_frame_mvs != 0 {
        (*(*f).c)
            .refmvs_dsp
            .load_tmvs
            .expect("non-null function pointer")(
            &(*f).rf,
            (*ts).tiling.row,
            (*ts).tiling.col_start >> 1,
            (*ts).tiling.col_end >> 1,
            (*t).by >> 1,
            (*t).by + sb_step >> 1,
        );
    }
    memset(
        ((*t).pal_sz_uv[1]).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint8_t; 32]>(),
    );
    let sb128y = (*t).by >> 5;
    (*t).bx = (*ts).tiling.col_start;
    (*t).a = ((*f).a)
        .offset(col_sb128_start as isize)
        .offset((tile_row * (*f).sb128w) as isize);
    (*t).lf_mask = ((*f).lf.mask)
        .offset((sb128y * (*f).sb128w) as isize)
        .offset(col_sb128_start as isize);
    while (*t).bx < (*ts).tiling.col_end {
        if ::core::intrinsics::atomic_load_acquire((*c).flush) != 0 {
            return 1 as libc::c_int;
        }
        if root_bl as libc::c_uint == BL_128X128 as libc::c_int as libc::c_uint {
            (*t).cur_sb_cdef_idx_ptr = ((*(*t).lf_mask).cdef_idx).as_mut_ptr();
            *((*t).cur_sb_cdef_idx_ptr).offset(0) = -(1 as libc::c_int) as int8_t;
            *((*t).cur_sb_cdef_idx_ptr).offset(1) = -(1 as libc::c_int) as int8_t;
            *((*t).cur_sb_cdef_idx_ptr).offset(2) = -(1 as libc::c_int) as int8_t;
            *((*t).cur_sb_cdef_idx_ptr).offset(3) = -(1 as libc::c_int) as int8_t;
        } else {
            (*t).cur_sb_cdef_idx_ptr = &mut *((*(*t).lf_mask).cdef_idx)
                .as_mut_ptr()
                .offset(((((*t).bx & 16) >> 4) + (((*t).by & 16) >> 3)) as isize)
                as *mut int8_t;
            *((*t).cur_sb_cdef_idx_ptr).offset(0) = -(1 as libc::c_int) as int8_t;
        }
        let mut p = 0;
        while p < 3 {
            if !(((*f).lf.restore_planes >> p) as libc::c_uint & 1 as libc::c_uint == 0) {
                let ss_ver = (p != 0
                    && (*f).cur.p.layout as libc::c_uint
                        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let ss_hor = (p != 0
                    && (*f).cur.p.layout as libc::c_uint
                        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                let unit_size_log2 =
                    (*(*f).frame_hdr).restoration.unit_size[(p != 0) as libc::c_int as usize];
                let y = (*t).by * 4 >> ss_ver;
                let h = (*f).cur.p.h + ss_ver >> ss_ver;
                let unit_size = (1 as libc::c_int) << unit_size_log2;
                let mask: libc::c_uint = (unit_size - 1) as libc::c_uint;
                if !(y as libc::c_uint & mask != 0) {
                    let half_unit = unit_size >> 1;
                    if !(y != 0 && y + half_unit > h) {
                        let frame_type: Dav1dRestorationType =
                            (*(*f).frame_hdr).restoration.type_0[p as usize];
                        if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1] {
                            let w = (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
                            let n_units = imax(1 as libc::c_int, w + half_unit >> unit_size_log2);
                            let d = (*(*f).frame_hdr).super_res.width_scale_denominator;
                            let rnd = unit_size * 8 - 1;
                            let shift = unit_size_log2 + 3;
                            let x0 = (4 * (*t).bx * d >> ss_hor) + rnd >> shift;
                            let x1 = (4 * ((*t).bx + sb_step) * d >> ss_hor) + rnd >> shift;
                            let mut x = x0;
                            while x < imin(x1, n_units) {
                                let px_x = x << unit_size_log2 + ss_hor;
                                let sb_idx = ((*t).by >> 5) * (*f).sr_sb128w + (px_x >> 7);
                                let unit_idx = (((*t).by & 16) >> 3) + ((px_x & 64) >> 6);
                                let lr: *mut Av1RestorationUnit =
                                    &mut *(*((*((*f).lf.lr_mask).offset(sb_idx as isize)).lr)
                                        .as_mut_ptr()
                                        .offset(p as isize))
                                    .as_mut_ptr()
                                    .offset(unit_idx as isize)
                                        as *mut Av1RestorationUnit;
                                read_restoration_info(t, lr, p, frame_type);
                                x += 1;
                            }
                        } else {
                            let x_0 = 4 * (*t).bx >> ss_hor;
                            if !(x_0 as libc::c_uint & mask != 0) {
                                let w_0 = (*f).cur.p.w + ss_hor >> ss_hor;
                                if !(x_0 != 0 && x_0 + half_unit > w_0) {
                                    let sb_idx_0 = ((*t).by >> 5) * (*f).sr_sb128w + ((*t).bx >> 5);
                                    let unit_idx_0 = (((*t).by & 16) >> 3) + (((*t).bx & 16) >> 4);
                                    let lr_0: *mut Av1RestorationUnit =
                                        &mut *(*((*((*f).lf.lr_mask).offset(sb_idx_0 as isize)).lr)
                                            .as_mut_ptr()
                                            .offset(p as isize))
                                        .as_mut_ptr()
                                        .offset(unit_idx_0 as isize)
                                            as *mut Av1RestorationUnit;
                                    read_restoration_info(t, lr_0, p, frame_type);
                                }
                            }
                        }
                    }
                }
            }
            p += 1;
        }
        if decode_sb(t, root_bl, (*c).intra_edge.root[root_bl as usize]) != 0 {
            return 1 as libc::c_int;
        }
        if (*t).bx & 16 != 0 || (*(*f).seq_hdr).sb128 != 0 {
            (*t).a = ((*t).a).offset(1);
            (*t).lf_mask = ((*t).lf_mask).offset(1);
        }
        (*t).bx += sb_step;
    }
    if (*(*f).seq_hdr).ref_frame_mvs != 0
        && (*(*f).c).n_tc > 1 as libc::c_uint
        && (*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint != 0
    {
        dav1d_refmvs_save_tmvs(
            &(*(*f).c).refmvs_dsp,
            &mut (*t).rt,
            (*ts).tiling.col_start >> 1,
            (*ts).tiling.col_end >> 1,
            (*t).by >> 1,
            (*t).by + sb_step >> 1,
        );
    }
    if (*t).frame_thread.pass != 1 as libc::c_int {
        ((*f).bd_fn.backup_ipred_edge).expect("non-null function pointer")(t);
    }
    let mut align_h = (*f).bh + 31 & !(31 as libc::c_int);
    memcpy(
        &mut *(*((*f).lf.tx_lpf_right_edge).as_ptr().offset(0))
            .offset((align_h * tile_col + (*t).by) as isize) as *mut uint8_t
            as *mut libc::c_void,
        &mut *((*t).l.tx_lpf_y.0)
            .as_mut_ptr()
            .offset(((*t).by & 16) as isize) as *mut uint8_t as *const libc::c_void,
        sb_step as libc::c_ulong,
    );
    let ss_ver_0 = ((*f).cur.p.layout as libc::c_uint
        == DAV1D_PIXEL_LAYOUT_I420 as libc::c_int as libc::c_uint)
        as libc::c_int;
    align_h >>= ss_ver_0;
    memcpy(
        &mut *(*((*f).lf.tx_lpf_right_edge).as_ptr().offset(1))
            .offset((align_h * tile_col + ((*t).by >> ss_ver_0)) as isize) as *mut uint8_t
            as *mut libc::c_void,
        &mut *((*t).l.tx_lpf_uv.0)
            .as_mut_ptr()
            .offset((((*t).by & 16) >> ss_ver_0) as isize) as *mut uint8_t
            as *const libc::c_void,
        (sb_step >> ss_ver_0) as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame_init(f: *mut Dav1dFrameContext) -> libc::c_int {
    let mut sby = 0;
    let mut n_ts = 0;
    let mut a_sz = 0;
    let mut num_sb128 = 0;
    let mut size_mul: *const uint8_t = 0 as *const uint8_t;
    let mut hbd = 0;
    let mut y_stride: ptrdiff_t = 0;
    let mut uv_stride: ptrdiff_t = 0;
    let mut has_resize = 0;
    let mut need_cdef_lpf_copy = 0;
    let mut sb128 = 0;
    let mut num_lines = 0;
    let mut lr_mask_sz = 0;
    let mut ipred_edge_sz = 0;
    let mut re_sz = 0;
    let mut has_chroma = 0;
    let mut current_block: u64;
    let c: *const Dav1dContext = (*f).c;
    let mut retval = -(12 as libc::c_int);
    if (*f).sbh > (*f).lf.start_of_tile_row_sz {
        free((*f).lf.start_of_tile_row as *mut libc::c_void);
        (*f).lf.start_of_tile_row = malloc(
            ((*f).sbh as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
        ) as *mut uint8_t;
        if ((*f).lf.start_of_tile_row).is_null() {
            (*f).lf.start_of_tile_row_sz = 0 as libc::c_int;
            current_block = 13495985911605184990;
        } else {
            (*f).lf.start_of_tile_row_sz = (*f).sbh;
            current_block = 6873731126896040597;
        }
    } else {
        current_block = 6873731126896040597;
    }
    match current_block {
        6873731126896040597 => {
            sby = 0 as libc::c_int;
            let mut tile_row = 0;
            while tile_row < (*(*f).frame_hdr).tiling.rows {
                let fresh33 = sby;
                sby = sby + 1;
                *((*f).lf.start_of_tile_row).offset(fresh33 as isize) = tile_row as uint8_t;
                while sby
                    < (*(*f).frame_hdr).tiling.row_start_sb[(tile_row + 1) as usize] as libc::c_int
                {
                    let fresh34 = sby;
                    sby = sby + 1;
                    *((*f).lf.start_of_tile_row).offset(fresh34 as isize) =
                        0 as libc::c_int as uint8_t;
                }
                tile_row += 1;
            }
            n_ts = (*(*f).frame_hdr).tiling.cols * (*(*f).frame_hdr).tiling.rows;
            if n_ts != (*f).n_ts {
                if (*c).n_fc > 1 as libc::c_uint {
                    freep(
                        &mut (*f).frame_thread.tile_start_off as *mut *mut libc::c_int
                            as *mut libc::c_void,
                    );
                    (*f).frame_thread.tile_start_off = malloc(
                        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(n_ts as libc::c_ulong),
                    ) as *mut libc::c_int;
                    if ((*f).frame_thread.tile_start_off).is_null() {
                        (*f).n_ts = 0 as libc::c_int;
                        current_block = 13495985911605184990;
                    } else {
                        current_block = 15976848397966268834;
                    }
                } else {
                    current_block = 15976848397966268834;
                }
                match current_block {
                    13495985911605184990 => {}
                    _ => {
                        dav1d_free_aligned((*f).ts as *mut libc::c_void);
                        (*f).ts = dav1d_alloc_aligned(
                            ::core::mem::size_of::<Dav1dTileState>().wrapping_mul(n_ts as size_t),
                            32 as libc::c_int as size_t,
                        ) as *mut Dav1dTileState;
                        if ((*f).ts).is_null() {
                            current_block = 13495985911605184990;
                        } else {
                            (*f).n_ts = n_ts;
                            current_block = 11584701595673473500;
                        }
                    }
                }
            } else {
                current_block = 11584701595673473500;
            }
            match current_block {
                13495985911605184990 => {}
                _ => {
                    a_sz = (*f).sb128w
                        * (*(*f).frame_hdr).tiling.rows
                        * (1 as libc::c_int
                            + ((*c).n_fc > 1 as libc::c_uint && (*c).n_tc > 1 as libc::c_uint)
                                as libc::c_int);
                    if a_sz != (*f).a_sz {
                        freep(&mut (*f).a as *mut *mut BlockContext as *mut libc::c_void);
                        (*f).a = malloc(
                            (::core::mem::size_of::<BlockContext>() as libc::c_ulong)
                                .wrapping_mul(a_sz as libc::c_ulong),
                        ) as *mut BlockContext;
                        if ((*f).a).is_null() {
                            (*f).a_sz = 0 as libc::c_int;
                            current_block = 13495985911605184990;
                        } else {
                            (*f).a_sz = a_sz;
                            current_block = 2232869372362427478;
                        }
                    } else {
                        current_block = 2232869372362427478;
                    }
                    match current_block {
                        13495985911605184990 => {}
                        _ => {
                            num_sb128 = (*f).sb128w * (*f).sb128h;
                            size_mul = (ss_size_mul[(*f).cur.p.layout as usize]).as_ptr();
                            hbd = ((*(*f).seq_hdr).hbd != 0) as libc::c_int;
                            if (*c).n_fc > 1 as libc::c_uint {
                                let mut tile_idx = 0;
                                let mut tile_row_0 = 0;
                                while tile_row_0 < (*(*f).frame_hdr).tiling.rows {
                                    let mut row_off = (*(*f).frame_hdr).tiling.row_start_sb
                                        [tile_row_0 as usize]
                                        as libc::c_int
                                        * (*f).sb_step
                                        * 4
                                        * (*f).sb128w
                                        * 128;
                                    let mut b_diff = ((*(*f).frame_hdr).tiling.row_start_sb
                                        [(tile_row_0 + 1) as usize]
                                        as libc::c_int
                                        - (*(*f).frame_hdr).tiling.row_start_sb[tile_row_0 as usize]
                                            as libc::c_int)
                                        * (*f).sb_step
                                        * 4;
                                    let mut tile_col = 0;
                                    while tile_col < (*(*f).frame_hdr).tiling.cols {
                                        let fresh35 = tile_idx;
                                        tile_idx = tile_idx + 1;
                                        *((*f).frame_thread.tile_start_off)
                                            .offset(fresh35 as isize) = row_off
                                            + b_diff
                                                * (*(*f).frame_hdr).tiling.col_start_sb
                                                    [tile_col as usize]
                                                    as libc::c_int
                                                * (*f).sb_step
                                                * 4;
                                        tile_col += 1;
                                    }
                                    tile_row_0 += 1;
                                }
                                let lowest_pixel_mem_sz = (*(*f).frame_hdr).tiling.cols * (*f).sbh;
                                if lowest_pixel_mem_sz != (*f).tile_thread.lowest_pixel_mem_sz {
                                    free((*f).tile_thread.lowest_pixel_mem as *mut libc::c_void);
                                    (*f).tile_thread.lowest_pixel_mem =
                                        malloc((lowest_pixel_mem_sz as libc::c_ulong).wrapping_mul(
                                            ::core::mem::size_of::<[[libc::c_int; 2]; 7]>()
                                                as libc::c_ulong,
                                        ))
                                            as *mut [[libc::c_int; 2]; 7];
                                    if ((*f).tile_thread.lowest_pixel_mem).is_null() {
                                        (*f).tile_thread.lowest_pixel_mem_sz = 0 as libc::c_int;
                                        current_block = 13495985911605184990;
                                    } else {
                                        (*f).tile_thread.lowest_pixel_mem_sz = lowest_pixel_mem_sz;
                                        current_block = 10891380440665537214;
                                    }
                                } else {
                                    current_block = 10891380440665537214;
                                }
                                match current_block {
                                    13495985911605184990 => {}
                                    _ => {
                                        let mut lowest_pixel_ptr: *mut [[libc::c_int; 2]; 7] =
                                            (*f).tile_thread.lowest_pixel_mem;
                                        let mut tile_row_1 = 0;
                                        let mut tile_row_base = 0;
                                        while tile_row_1 < (*(*f).frame_hdr).tiling.rows {
                                            let tile_row_sb_h =
                                                (*(*f).frame_hdr).tiling.row_start_sb
                                                    [(tile_row_1 + 1) as usize]
                                                    as libc::c_int
                                                    - (*(*f).frame_hdr).tiling.row_start_sb
                                                        [tile_row_1 as usize]
                                                        as libc::c_int;
                                            let mut tile_col_0 = 0;
                                            while tile_col_0 < (*(*f).frame_hdr).tiling.cols {
                                                let ref mut fresh36 = (*((*f).ts)
                                                    .offset((tile_row_base + tile_col_0) as isize))
                                                .lowest_pixel;
                                                *fresh36 = lowest_pixel_ptr;
                                                lowest_pixel_ptr =
                                                    lowest_pixel_ptr.offset(tile_row_sb_h as isize);
                                                tile_col_0 += 1;
                                            }
                                            tile_row_1 += 1;
                                            tile_row_base += (*(*f).frame_hdr).tiling.cols;
                                        }
                                        let cf_sz =
                                            (num_sb128 * *size_mul.offset(0) as libc::c_int) << hbd;
                                        if cf_sz != (*f).frame_thread.cf_sz {
                                            dav1d_freep_aligned(
                                                &mut (*f).frame_thread.cf as *mut *mut libc::c_void
                                                    as *mut libc::c_void,
                                            );
                                            (*f).frame_thread.cf = dav1d_alloc_aligned(
                                                (cf_sz as size_t)
                                                    .wrapping_mul(128)
                                                    .wrapping_mul(128)
                                                    .wrapping_div(2),
                                                64 as libc::c_int as size_t,
                                            );
                                            if ((*f).frame_thread.cf).is_null() {
                                                (*f).frame_thread.cf_sz = 0 as libc::c_int;
                                                current_block = 13495985911605184990;
                                            } else {
                                                memset(
                                                    (*f).frame_thread.cf,
                                                    0 as libc::c_int,
                                                    (cf_sz as size_t)
                                                        .wrapping_mul(128)
                                                        .wrapping_mul(128)
                                                        .wrapping_div(2),
                                                );
                                                (*f).frame_thread.cf_sz = cf_sz;
                                                current_block = 10930818133215224067;
                                            }
                                        } else {
                                            current_block = 10930818133215224067;
                                        }
                                        match current_block {
                                            13495985911605184990 => {}
                                            _ => {
                                                if (*(*f).frame_hdr).allow_screen_content_tools != 0
                                                {
                                                    if num_sb128 != (*f).frame_thread.pal_sz {
                                                        dav1d_freep_aligned(
                                                            &mut (*f).frame_thread.pal
                                                                as *mut *mut [[uint16_t; 8]; 3]
                                                                as *mut libc::c_void,
                                                        );
                                                        (*f).frame_thread.pal = dav1d_alloc_aligned(
                                                            ::core::mem::size_of::<
                                                                [[uint16_t; 8]; 3],
                                                            >(
                                                            )
                                                            .wrapping_mul(num_sb128 as size_t)
                                                            .wrapping_mul(16)
                                                            .wrapping_mul(16),
                                                            64,
                                                        )
                                                            as *mut [[uint16_t; 8]; 3];
                                                        if ((*f).frame_thread.pal).is_null() {
                                                            (*f).frame_thread.pal_sz =
                                                                0 as libc::c_int;
                                                            current_block = 13495985911605184990;
                                                        } else {
                                                            (*f).frame_thread.pal_sz = num_sb128;
                                                            current_block = 8835654301469918283;
                                                        }
                                                    } else {
                                                        current_block = 8835654301469918283;
                                                    }
                                                    match current_block {
                                                        13495985911605184990 => {}
                                                        _ => {
                                                            let pal_idx_sz = num_sb128
                                                                * *size_mul.offset(1)
                                                                    as libc::c_int;
                                                            if pal_idx_sz
                                                                != (*f).frame_thread.pal_idx_sz
                                                            {
                                                                dav1d_freep_aligned(
                                                                    &mut (*f).frame_thread.pal_idx
                                                                        as *mut *mut uint8_t
                                                                        as *mut libc::c_void,
                                                                );
                                                                (*f).frame_thread.pal_idx =
                                                                    dav1d_alloc_aligned(
                                                                        ::core::mem::size_of::<
                                                                            uint8_t,
                                                                        >(
                                                                        )
                                                                        .wrapping_mul(
                                                                            pal_idx_sz as size_t,
                                                                        )
                                                                        .wrapping_mul(128)
                                                                        .wrapping_mul(128)
                                                                        .wrapping_div(4),
                                                                        64,
                                                                    )
                                                                        as *mut uint8_t;
                                                                if ((*f).frame_thread.pal_idx)
                                                                    .is_null()
                                                                {
                                                                    (*f).frame_thread.pal_idx_sz =
                                                                        0 as libc::c_int;
                                                                    current_block =
                                                                        13495985911605184990;
                                                                } else {
                                                                    (*f).frame_thread.pal_idx_sz =
                                                                        pal_idx_sz;
                                                                    current_block =
                                                                        7178192492338286402;
                                                                }
                                                            } else {
                                                                current_block = 7178192492338286402;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if !((*f).frame_thread.pal).is_null() {
                                                        dav1d_freep_aligned(
                                                            &mut (*f).frame_thread.pal
                                                                as *mut *mut [[uint16_t; 8]; 3]
                                                                as *mut libc::c_void,
                                                        );
                                                        dav1d_freep_aligned(
                                                            &mut (*f).frame_thread.pal_idx
                                                                as *mut *mut uint8_t
                                                                as *mut libc::c_void,
                                                        );
                                                        (*f).frame_thread.pal_idx_sz =
                                                            0 as libc::c_int;
                                                        (*f).frame_thread.pal_sz =
                                                            (*f).frame_thread.pal_idx_sz;
                                                    }
                                                    current_block = 7178192492338286402;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                current_block = 7178192492338286402;
                            }
                            match current_block {
                                13495985911605184990 => {}
                                _ => {
                                    y_stride = (*f).cur.stride[0];
                                    uv_stride = (*f).cur.stride[1];
                                    has_resize = ((*(*f).frame_hdr).width[0]
                                        != (*(*f).frame_hdr).width[1])
                                        as libc::c_int;
                                    need_cdef_lpf_copy = ((*c).n_tc > 1 as libc::c_uint
                                        && has_resize != 0)
                                        as libc::c_int;
                                    if y_stride * (*f).sbh as isize * 4
                                        != (*f).lf.cdef_buf_plane_sz[0] as isize
                                        || uv_stride * (*f).sbh as isize * 8
                                            != (*f).lf.cdef_buf_plane_sz[1] as isize
                                        || need_cdef_lpf_copy != (*f).lf.need_cdef_lpf_copy
                                        || (*f).sbh != (*f).lf.cdef_buf_sbh
                                    {
                                        dav1d_free_aligned(
                                            (*f).lf.cdef_line_buf as *mut libc::c_void,
                                        );
                                        let mut alloc_sz: size_t = 64 as libc::c_int as size_t;
                                        alloc_sz = alloc_sz.wrapping_add(
                                            ((y_stride as libc::c_longlong).abs() as size_t)
                                                .wrapping_mul(4)
                                                .wrapping_mul((*f).sbh as size_t)
                                                << need_cdef_lpf_copy,
                                        )
                                            as size_t
                                            as size_t;
                                        alloc_sz = alloc_sz.wrapping_add(
                                            ((uv_stride as libc::c_longlong).abs() as size_t)
                                                .wrapping_mul(8)
                                                .wrapping_mul((*f).sbh as size_t)
                                                << need_cdef_lpf_copy,
                                        )
                                            as size_t
                                            as size_t;
                                        (*f).lf.cdef_line_buf = dav1d_alloc_aligned(
                                            alloc_sz,
                                            32 as libc::c_int as size_t,
                                        )
                                            as *mut uint8_t;
                                        let mut ptr: *mut uint8_t = (*f).lf.cdef_line_buf;
                                        if ptr.is_null() {
                                            (*f).lf.cdef_buf_plane_sz[1] = 0 as libc::c_int;
                                            (*f).lf.cdef_buf_plane_sz[0] =
                                                (*f).lf.cdef_buf_plane_sz[1];
                                            current_block = 13495985911605184990;
                                        } else {
                                            ptr = ptr.offset(32);
                                            if y_stride < 0 {
                                                (*f).lf.cdef_line[0][0] = ptr.offset(
                                                    -((y_stride * ((*f).sbh * 4 - 1) as isize)
                                                        as isize),
                                                )
                                                    as *mut libc::c_void;
                                                (*f).lf.cdef_line[1][0] = ptr.offset(
                                                    -((y_stride * ((*f).sbh * 4 - 3) as isize)
                                                        as isize),
                                                )
                                                    as *mut libc::c_void;
                                            } else {
                                                (*f).lf.cdef_line[0][0] = ptr
                                                    .offset((y_stride * 0) as isize)
                                                    as *mut libc::c_void;
                                                (*f).lf.cdef_line[1][0] = ptr
                                                    .offset((y_stride * 2) as isize)
                                                    as *mut libc::c_void;
                                            }
                                            ptr = ptr.offset(
                                                ((y_stride as libc::c_longlong).abs()
                                                    * (*f).sbh as libc::c_longlong
                                                    * 4 as libc::c_longlong)
                                                    as isize,
                                            );
                                            if uv_stride < 0 {
                                                (*f).lf.cdef_line[0][1] = ptr.offset(
                                                    -((uv_stride * ((*f).sbh * 8 - 1) as isize)
                                                        as isize),
                                                )
                                                    as *mut libc::c_void;
                                                (*f).lf.cdef_line[0][2] = ptr.offset(
                                                    -((uv_stride * ((*f).sbh * 8 - 3) as isize)
                                                        as isize),
                                                )
                                                    as *mut libc::c_void;
                                                (*f).lf.cdef_line[1][1] = ptr.offset(
                                                    -((uv_stride * ((*f).sbh * 8 - 5) as isize)
                                                        as isize),
                                                )
                                                    as *mut libc::c_void;
                                                (*f).lf.cdef_line[1][2] = ptr.offset(
                                                    -((uv_stride * ((*f).sbh * 8 - 7) as isize)
                                                        as isize),
                                                )
                                                    as *mut libc::c_void;
                                            } else {
                                                (*f).lf.cdef_line[0][1] = ptr
                                                    .offset((uv_stride * 0) as isize)
                                                    as *mut libc::c_void;
                                                (*f).lf.cdef_line[0][2] = ptr
                                                    .offset((uv_stride * 2) as isize)
                                                    as *mut libc::c_void;
                                                (*f).lf.cdef_line[1][1] = ptr
                                                    .offset((uv_stride * 4) as isize)
                                                    as *mut libc::c_void;
                                                (*f).lf.cdef_line[1][2] = ptr
                                                    .offset((uv_stride * 6) as isize)
                                                    as *mut libc::c_void;
                                            }
                                            if need_cdef_lpf_copy != 0 {
                                                ptr = ptr.offset(
                                                    ((uv_stride as libc::c_longlong).abs()
                                                        * (*f).sbh as libc::c_longlong
                                                        * 8 as libc::c_longlong)
                                                        as isize,
                                                );
                                                if y_stride < 0 {
                                                    (*f).lf.cdef_lpf_line[0] = ptr.offset(
                                                        -((y_stride * ((*f).sbh * 4 - 1) as isize)
                                                            as isize),
                                                    )
                                                        as *mut libc::c_void;
                                                } else {
                                                    (*f).lf.cdef_lpf_line[0] =
                                                        ptr as *mut libc::c_void;
                                                }
                                                ptr = ptr.offset(
                                                    ((y_stride as libc::c_longlong).abs()
                                                        * (*f).sbh as libc::c_longlong
                                                        * 4 as libc::c_longlong)
                                                        as isize,
                                                );
                                                if uv_stride < 0 {
                                                    (*f).lf.cdef_lpf_line[1] = ptr.offset(
                                                        -((uv_stride * ((*f).sbh * 4 - 1) as isize)
                                                            as isize),
                                                    )
                                                        as *mut libc::c_void;
                                                    (*f).lf.cdef_lpf_line[2] = ptr.offset(
                                                        -((uv_stride * ((*f).sbh * 8 - 1) as isize)
                                                            as isize),
                                                    )
                                                        as *mut libc::c_void;
                                                } else {
                                                    (*f).lf.cdef_lpf_line[1] =
                                                        ptr as *mut libc::c_void;
                                                    (*f).lf.cdef_lpf_line[2] = ptr.offset(
                                                        (uv_stride * (*f).sbh as isize * 4)
                                                            as isize,
                                                    )
                                                        as *mut libc::c_void;
                                                }
                                            }
                                            (*f).lf.cdef_buf_plane_sz[0] =
                                                y_stride as libc::c_int * (*f).sbh * 4;
                                            (*f).lf.cdef_buf_plane_sz[1] =
                                                uv_stride as libc::c_int * (*f).sbh * 8;
                                            (*f).lf.need_cdef_lpf_copy = need_cdef_lpf_copy;
                                            (*f).lf.cdef_buf_sbh = (*f).sbh;
                                            current_block = 8140372313878014523;
                                        }
                                    } else {
                                        current_block = 8140372313878014523;
                                    }
                                    match current_block {
                                        13495985911605184990 => {}
                                        _ => {
                                            sb128 = (*(*f).seq_hdr).sb128;
                                            num_lines = if (*c).n_tc > 1 as libc::c_uint {
                                                ((*f).sbh * 4) << sb128
                                            } else {
                                                12 as libc::c_int
                                            };
                                            y_stride = (*f).sr_cur.p.stride[0];
                                            uv_stride = (*f).sr_cur.p.stride[1];
                                            if y_stride * num_lines as isize
                                                != (*f).lf.lr_buf_plane_sz[0] as isize
                                                || uv_stride * num_lines as isize * 2 as isize
                                                    != (*f).lf.lr_buf_plane_sz[1] as isize
                                            {
                                                dav1d_free_aligned(
                                                    (*f).lf.lr_line_buf as *mut libc::c_void,
                                                );
                                                let mut alloc_sz_0: size_t =
                                                    128 as libc::c_int as size_t;
                                                alloc_sz_0 = alloc_sz_0.wrapping_add(
                                                    ((y_stride as libc::c_longlong).abs()
                                                        as size_t)
                                                        .wrapping_mul(num_lines as size_t),
                                                );
                                                alloc_sz_0 = alloc_sz_0.wrapping_add(
                                                    ((uv_stride as libc::c_longlong).abs()
                                                        as size_t)
                                                        .wrapping_mul(num_lines as size_t)
                                                        .wrapping_mul(2),
                                                );
                                                (*f).lf.lr_line_buf = dav1d_alloc_aligned(
                                                    alloc_sz_0,
                                                    64 as libc::c_int as size_t,
                                                )
                                                    as *mut uint8_t;
                                                let mut ptr_0: *mut uint8_t = (*f).lf.lr_line_buf;
                                                if ptr_0.is_null() {
                                                    (*f).lf.lr_buf_plane_sz[1] = 0 as libc::c_int;
                                                    (*f).lf.lr_buf_plane_sz[0] =
                                                        (*f).lf.lr_buf_plane_sz[1];
                                                    current_block = 13495985911605184990;
                                                } else {
                                                    ptr_0 = ptr_0.offset(64);
                                                    if y_stride < 0 {
                                                        (*f).lf.lr_lpf_line[0] = ptr_0.offset(
                                                            -((y_stride * (num_lines - 1) as isize)
                                                                as isize),
                                                        )
                                                            as *mut libc::c_void;
                                                    } else {
                                                        (*f).lf.lr_lpf_line[0] =
                                                            ptr_0 as *mut libc::c_void;
                                                    }
                                                    ptr_0 = ptr_0.offset(
                                                        ((y_stride as libc::c_longlong).abs()
                                                            * num_lines as libc::c_longlong)
                                                            as isize,
                                                    );
                                                    if uv_stride < 0 {
                                                        (*f).lf.lr_lpf_line[1] = ptr_0.offset(
                                                            -((uv_stride
                                                                * (num_lines * 1 - 1) as isize)
                                                                as isize),
                                                        )
                                                            as *mut libc::c_void;
                                                        (*f).lf.lr_lpf_line[2] = ptr_0.offset(
                                                            -((uv_stride
                                                                * (num_lines * 2 - 1) as isize)
                                                                as isize),
                                                        )
                                                            as *mut libc::c_void;
                                                    } else {
                                                        (*f).lf.lr_lpf_line[1] =
                                                            ptr_0 as *mut libc::c_void;
                                                        (*f).lf.lr_lpf_line[2] = ptr_0.offset(
                                                            (uv_stride * num_lines as isize)
                                                                as isize,
                                                        )
                                                            as *mut libc::c_void;
                                                    }
                                                    (*f).lf.lr_buf_plane_sz[0] =
                                                        y_stride as libc::c_int * num_lines;
                                                    (*f).lf.lr_buf_plane_sz[1] =
                                                        uv_stride as libc::c_int * num_lines * 2;
                                                    current_block = 15456862084301247793;
                                                }
                                            } else {
                                                current_block = 15456862084301247793;
                                            }
                                            match current_block {
                                                13495985911605184990 => {}
                                                _ => {
                                                    if num_sb128 != (*f).lf.mask_sz {
                                                        freep(
                                                            &mut (*f).lf.mask as *mut *mut Av1Filter
                                                                as *mut libc::c_void,
                                                        );
                                                        freep(
                                                            &mut (*f).lf.level
                                                                as *mut *mut [uint8_t; 4]
                                                                as *mut libc::c_void,
                                                        );
                                                        (*f).lf.mask = malloc(
                                                            (::core::mem::size_of::<Av1Filter>()
                                                                as libc::c_ulong)
                                                                .wrapping_mul(
                                                                    num_sb128 as libc::c_ulong,
                                                                ),
                                                        )
                                                            as *mut Av1Filter;
                                                        (*f).lf.level = malloc(
                                                            (::core::mem::size_of::<[uint8_t; 4]>()
                                                                as libc::c_ulong)
                                                                .wrapping_mul(
                                                                    num_sb128 as libc::c_ulong,
                                                                )
                                                                .wrapping_mul(
                                                                    32 as libc::c_int
                                                                        as libc::c_ulong,
                                                                )
                                                                .wrapping_mul(
                                                                    32 as libc::c_int
                                                                        as libc::c_ulong,
                                                                )
                                                                .wrapping_add(
                                                                    3 as libc::c_int
                                                                        as libc::c_ulong,
                                                                ),
                                                        )
                                                            as *mut [uint8_t; 4];
                                                        if ((*f).lf.mask).is_null()
                                                            || ((*f).lf.level).is_null()
                                                        {
                                                            (*f).lf.mask_sz = 0 as libc::c_int;
                                                            current_block = 13495985911605184990;
                                                        } else {
                                                            if (*c).n_fc > 1 as libc::c_uint {
                                                                freep(
                                                                    &mut (*f).frame_thread.b
                                                                        as *mut *mut Av1Block
                                                                        as *mut libc::c_void,
                                                                );
                                                                freep(
                                                                    &mut (*f).frame_thread.cbi
                                                                        as *mut *mut CodedBlockInfo
                                                                        as *mut libc::c_void,
                                                                );
                                                                (*f)
                                                                    .frame_thread
                                                                    .b = malloc(
                                                                    (::core::mem::size_of::<Av1Block>() as libc::c_ulong)
                                                                        .wrapping_mul(num_sb128 as libc::c_ulong)
                                                                        .wrapping_mul(32 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_mul(32 as libc::c_int as libc::c_ulong),
                                                                ) as *mut Av1Block;
                                                                (*f).frame_thread.cbi = malloc(
                                                                    (::core::mem::size_of::<
                                                                        CodedBlockInfo,
                                                                    >(
                                                                    )
                                                                        as libc::c_ulong)
                                                                        .wrapping_mul(
                                                                            num_sb128
                                                                                as libc::c_ulong,
                                                                        )
                                                                        .wrapping_mul(
                                                                            32 as libc::c_int
                                                                                as libc::c_ulong,
                                                                        )
                                                                        .wrapping_mul(
                                                                            32 as libc::c_int
                                                                                as libc::c_ulong,
                                                                        ),
                                                                )
                                                                    as *mut CodedBlockInfo;
                                                                if ((*f).frame_thread.b).is_null()
                                                                    || ((*f).frame_thread.cbi)
                                                                        .is_null()
                                                                {
                                                                    (*f).lf.mask_sz =
                                                                        0 as libc::c_int;
                                                                    current_block =
                                                                        13495985911605184990;
                                                                } else {
                                                                    current_block =
                                                                        7923086311623215889;
                                                                }
                                                            } else {
                                                                current_block = 7923086311623215889;
                                                            }
                                                            match current_block {
                                                                13495985911605184990 => {}
                                                                _ => {
                                                                    (*f).lf.mask_sz = num_sb128;
                                                                    current_block =
                                                                        3024573345131975588;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 3024573345131975588;
                                                    }
                                                    match current_block {
                                                        13495985911605184990 => {}
                                                        _ => {
                                                            (*f).sr_sb128w =
                                                                (*f).sr_cur.p.p.w + 127 >> 7;
                                                            lr_mask_sz =
                                                                (*f).sr_sb128w * (*f).sb128h;
                                                            if lr_mask_sz != (*f).lf.lr_mask_sz {
                                                                freep(
                                                                    &mut (*f).lf.lr_mask
                                                                        as *mut *mut Av1Restoration
                                                                        as *mut libc::c_void,
                                                                );
                                                                (*f).lf.lr_mask = malloc(
                                                                    (::core::mem::size_of::<
                                                                        Av1Restoration,
                                                                    >(
                                                                    )
                                                                        as libc::c_ulong)
                                                                        .wrapping_mul(
                                                                            lr_mask_sz
                                                                                as libc::c_ulong,
                                                                        ),
                                                                )
                                                                    as *mut Av1Restoration;
                                                                if ((*f).lf.lr_mask).is_null() {
                                                                    (*f).lf.lr_mask_sz =
                                                                        0 as libc::c_int;
                                                                    current_block =
                                                                        13495985911605184990;
                                                                } else {
                                                                    (*f).lf.lr_mask_sz = lr_mask_sz;
                                                                    current_block =
                                                                        16077153431071379266;
                                                                }
                                                            } else {
                                                                current_block =
                                                                    16077153431071379266;
                                                            }
                                                            match current_block {
                                                                13495985911605184990 => {}
                                                                _ => {
                                                                    (*f)
                                                                        .lf
                                                                        .restore_planes = ((((*(*f).frame_hdr)
                                                                        .restoration
                                                                        .type_0[0] as libc::c_uint
                                                                        != DAV1D_RESTORATION_NONE as libc::c_int as libc::c_uint)
                                                                        as libc::c_int) << 0)
                                                                        + ((((*(*f).frame_hdr)
                                                                            .restoration
                                                                            .type_0[1] as libc::c_uint
                                                                            != DAV1D_RESTORATION_NONE as libc::c_int as libc::c_uint)
                                                                            as libc::c_int) << 1)
                                                                        + ((((*(*f).frame_hdr)
                                                                            .restoration
                                                                            .type_0[2] as libc::c_uint
                                                                            != DAV1D_RESTORATION_NONE as libc::c_int as libc::c_uint)
                                                                            as libc::c_int) << 2);
                                                                    if (*(*f).frame_hdr)
                                                                        .loopfilter
                                                                        .sharpness
                                                                        != (*f).lf.last_sharpness
                                                                    {
                                                                        dav1d_calc_eih(
                                                                            &mut (*f).lf.lim_lut.0,
                                                                            (*(*f).frame_hdr)
                                                                                .loopfilter
                                                                                .sharpness,
                                                                        );
                                                                        (*f).lf.last_sharpness =
                                                                            (*(*f).frame_hdr)
                                                                                .loopfilter
                                                                                .sharpness;
                                                                    }
                                                                    dav1d_calc_lf_values(
                                                                        &mut (*f).lf.lvl,
                                                                        &*(*f).frame_hdr,
                                                                        &[0, 0, 0, 0],
                                                                    );
                                                                    memset(
                                                                        (*f).lf.mask
                                                                            as *mut libc::c_void,
                                                                        0 as libc::c_int,
                                                                        (::core::mem::size_of::<
                                                                            Av1Filter,
                                                                        >(
                                                                        ))
                                                                        .wrapping_mul(
                                                                            num_sb128 as size_t,
                                                                        ),
                                                                    );
                                                                    ipred_edge_sz = (*f).sbh
                                                                        * (*f).sb128w
                                                                        << hbd;
                                                                    if ipred_edge_sz
                                                                        != (*f).ipred_edge_sz
                                                                    {
                                                                        dav1d_freep_aligned(
                                                                            &mut *((*f).ipred_edge)
                                                                                .as_mut_ptr()
                                                                                .offset(0) as *mut *mut libc::c_void
                                                                                as *mut libc::c_void,
                                                                        );
                                                                        (*f).ipred_edge[0
                                                                            as libc::c_int
                                                                            as usize] =
                                                                            dav1d_alloc_aligned(
                                                                                (ipred_edge_sz
                                                                                    * 128
                                                                                    * 3)
                                                                                    as size_t,
                                                                                64 as libc::c_int
                                                                                    as size_t,
                                                                            );
                                                                        let mut ptr_1: *mut uint8_t = (*f)
                                                                            .ipred_edge[0] as *mut uint8_t;
                                                                        if ptr_1.is_null() {
                                                                            (*f).ipred_edge_sz =
                                                                                0 as libc::c_int;
                                                                            current_block = 13495985911605184990;
                                                                        } else {
                                                                            (*f)
                                                                                .ipred_edge[1 as libc::c_int
                                                                                as usize] = ptr_1
                                                                                .offset(
                                                                                    (ipred_edge_sz * 128 * 1)
                                                                                        as isize,
                                                                                ) as *mut libc::c_void;
                                                                            (*f)
                                                                                .ipred_edge[2 as libc::c_int
                                                                                as usize] = ptr_1
                                                                                .offset(
                                                                                    (ipred_edge_sz * 128 * 2)
                                                                                        as isize,
                                                                                ) as *mut libc::c_void;
                                                                            (*f).ipred_edge_sz =
                                                                                ipred_edge_sz;
                                                                            current_block = 10265667325682070567;
                                                                        }
                                                                    } else {
                                                                        current_block =
                                                                            10265667325682070567;
                                                                    }
                                                                    match current_block {
                                                                        13495985911605184990 => {}
                                                                        _ => {
                                                                            re_sz = (*f).sb128h
                                                                                * (*(*f).frame_hdr)
                                                                                    .tiling
                                                                                    .cols;
                                                                            if re_sz
                                                                                != (*f).lf.re_sz
                                                                            {
                                                                                freep(
                                                                                    &mut *((*f).lf.tx_lpf_right_edge)
                                                                                        .as_mut_ptr()
                                                                                        .offset(0) as *mut *mut uint8_t
                                                                                        as *mut libc::c_void,
                                                                                );
                                                                                (*f)
                                                                                    .lf
                                                                                    .tx_lpf_right_edge[0 as libc::c_int
                                                                                    as usize] = malloc(
                                                                                    (re_sz * 32 * 2)
                                                                                        as libc::c_ulong,
                                                                                ) as *mut uint8_t;
                                                                                if ((*f).lf.tx_lpf_right_edge[0])
                                                                                    .is_null()
                                                                                {
                                                                                    (*f).lf.re_sz = 0 as libc::c_int;
                                                                                    current_block = 13495985911605184990;
                                                                                } else {
                                                                                    (*f)
                                                                                        .lf
                                                                                        .tx_lpf_right_edge[1 as libc::c_int
                                                                                        as usize] = ((*f)
                                                                                        .lf
                                                                                        .tx_lpf_right_edge[0])
                                                                                        .offset((re_sz * 32) as isize);
                                                                                    (*f).lf.re_sz = re_sz;
                                                                                    current_block = 5511877782510663281;
                                                                                }
                                                                            } else {
                                                                                current_block = 5511877782510663281;
                                                                            }
                                                                            match current_block {
                                                                                13495985911605184990 => {}
                                                                                _ => {
                                                                                    if (*(*f).frame_hdr).frame_type as libc::c_uint
                                                                                        & 1 as libc::c_uint != 0
                                                                                        || (*(*f).frame_hdr).allow_intrabc != 0
                                                                                    {
                                                                                        let ret = dav1d_refmvs_init_frame(
                                                                                            &mut (*f).rf,
                                                                                            (*f).seq_hdr,
                                                                                            (*f).frame_hdr,
                                                                                            ((*f).refpoc).as_mut_ptr() as *const libc::c_uint,
                                                                                            (*f).mvs,
                                                                                            ((*f).refrefpoc).as_mut_ptr() as *const [libc::c_uint; 7],
                                                                                            ((*f).ref_mvs).as_mut_ptr()
                                                                                                as *const *mut refmvs_temporal_block,
                                                                                            (*(*f).c).n_tc as libc::c_int,
                                                                                            (*(*f).c).n_fc as libc::c_int,
                                                                                        );
                                                                                        if ret < 0 {
                                                                                            current_block = 13495985911605184990;
                                                                                        } else {
                                                                                            current_block = 6662862405959679103;
                                                                                        }
                                                                                    } else {
                                                                                        current_block = 6662862405959679103;
                                                                                    }
                                                                                    match current_block {
                                                                                        13495985911605184990 => {}
                                                                                        _ => {
                                                                                            init_quant_tables(
                                                                                                &*(*f).seq_hdr,
                                                                                                &*(*f).frame_hdr,
                                                                                                (*(*f).frame_hdr).quant.yac,
                                                                                                &mut (*f).dq,
                                                                                            );
                                                                                            if (*(*f).frame_hdr).quant.qm != 0 {
                                                                                                let mut i = 0;
                                                                                                while i < N_RECT_TX_SIZES as libc::c_int {
                                                                                                    (*f)
                                                                                                        .qm[i
                                                                                                        as usize][0 as libc::c_int
                                                                                                        as usize] = dav1d_qm_tbl[(*(*f).frame_hdr).quant.qm_y
                                                                                                        as usize][0][i as usize];
                                                                                                    (*f)
                                                                                                        .qm[i
                                                                                                        as usize][1 as libc::c_int
                                                                                                        as usize] = dav1d_qm_tbl[(*(*f).frame_hdr).quant.qm_u
                                                                                                        as usize][1][i as usize];
                                                                                                    (*f)
                                                                                                        .qm[i
                                                                                                        as usize][2 as libc::c_int
                                                                                                        as usize] = dav1d_qm_tbl[(*(*f).frame_hdr).quant.qm_v
                                                                                                        as usize][1][i as usize];
                                                                                                    i += 1;
                                                                                                }
                                                                                            } else {
                                                                                                memset(
                                                                                                    ((*f).qm).as_mut_ptr() as *mut libc::c_void,
                                                                                                    0 as libc::c_int,
                                                                                                    ::core::mem::size_of::<[[*const uint8_t; 3]; 19]>(),
                                                                                                );
                                                                                            }
                                                                                            if (*(*f).frame_hdr).switchable_comp_refs != 0 {
                                                                                                let mut i_0 = 0;
                                                                                                while i_0 < 7 {
                                                                                                    let ref0poc: libc::c_uint = (*(*f)
                                                                                                        .refp[i_0 as usize]
                                                                                                        .p
                                                                                                        .frame_hdr)
                                                                                                        .frame_offset as libc::c_uint;
                                                                                                    let mut j = i_0 + 1;
                                                                                                    while j < 7 {
                                                                                                        let ref1poc: libc::c_uint = (*(*f)
                                                                                                            .refp[j as usize]
                                                                                                            .p
                                                                                                            .frame_hdr)
                                                                                                            .frame_offset as libc::c_uint;
                                                                                                        let d1: libc::c_uint = imin(
                                                                                                            get_poc_diff(
                                                                                                                (*(*f).seq_hdr).order_hint_n_bits,
                                                                                                                ref0poc as libc::c_int,
                                                                                                                (*(*f).cur.frame_hdr).frame_offset,
                                                                                                            ).abs(),
                                                                                                            31 as libc::c_int,
                                                                                                        ) as libc::c_uint;
                                                                                                        let d0: libc::c_uint = imin(
                                                                                                            get_poc_diff(
                                                                                                                (*(*f).seq_hdr).order_hint_n_bits,
                                                                                                                ref1poc as libc::c_int,
                                                                                                                (*(*f).cur.frame_hdr).frame_offset,
                                                                                                            ).abs(),
                                                                                                            31 as libc::c_int,
                                                                                                        ) as libc::c_uint;
                                                                                                        let order = (d0 <= d1) as libc::c_int;
                                                                                                        static mut quant_dist_weight: [[uint8_t; 2]; 3] = [
                                                                                                            [2 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t],
                                                                                                            [2 as libc::c_int as uint8_t, 5 as libc::c_int as uint8_t],
                                                                                                            [2 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t],
                                                                                                        ];
                                                                                                        static mut quant_dist_lookup_table: [[uint8_t; 2]; 4] = [
                                                                                                            [9 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t],
                                                                                                            [11 as libc::c_int as uint8_t, 5 as libc::c_int as uint8_t],
                                                                                                            [12 as libc::c_int as uint8_t, 4 as libc::c_int as uint8_t],
                                                                                                            [13 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t],
                                                                                                        ];
                                                                                                        let mut k = 0;
                                                                                                        k = 0 as libc::c_int;
                                                                                                        while k < 3 {
                                                                                                            let c0 = quant_dist_weight[k
                                                                                                                as usize][order as usize] as libc::c_int;
                                                                                                            let c1 = quant_dist_weight[k
                                                                                                                as usize][(order == 0) as libc::c_int as usize]
                                                                                                                as libc::c_int;
                                                                                                            let d0_c0 = d0.wrapping_mul(c0 as libc::c_uint)
                                                                                                                as libc::c_int;
                                                                                                            let d1_c1 = d1.wrapping_mul(c1 as libc::c_uint)
                                                                                                                as libc::c_int;
                                                                                                            if d0 > d1 && d0_c0 < d1_c1 || d0 <= d1 && d0_c0 > d1_c1 {
                                                                                                                break;
                                                                                                            }
                                                                                                            k += 1;
                                                                                                        }
                                                                                                        (*f)
                                                                                                            .jnt_weights[i_0
                                                                                                            as usize][j
                                                                                                            as usize] = quant_dist_lookup_table[k
                                                                                                            as usize][order as usize];
                                                                                                        j += 1;
                                                                                                    }
                                                                                                    i_0 += 1;
                                                                                                }
                                                                                            }
                                                                                            has_chroma = ((*f).cur.p.layout as libc::c_uint
                                                                                                != DAV1D_PIXEL_LAYOUT_I400 as libc::c_int as libc::c_uint)
                                                                                                as libc::c_int;
                                                                                            (*f).lf.mask_ptr = (*f).lf.mask;
                                                                                            (*f)
                                                                                                .lf
                                                                                                .p[0 as libc::c_int
                                                                                                as usize] = (*f).cur.data[0];
                                                                                            (*f)
                                                                                                .lf
                                                                                                .p[1 as libc::c_int
                                                                                                as usize] = (*f)
                                                                                                .cur
                                                                                                .data[(if has_chroma != 0 {
                                                                                                1 as libc::c_int
                                                                                            } else {
                                                                                                0 as libc::c_int
                                                                                            }) as usize];
                                                                                            (*f)
                                                                                                .lf
                                                                                                .p[2 as libc::c_int
                                                                                                as usize] = (*f)
                                                                                                .cur
                                                                                                .data[(if has_chroma != 0 {
                                                                                                2 as libc::c_int
                                                                                            } else {
                                                                                                0 as libc::c_int
                                                                                            }) as usize];
                                                                                            (*f)
                                                                                                .lf
                                                                                                .sr_p[0 as libc::c_int
                                                                                                as usize] = (*f).sr_cur.p.data[0];
                                                                                            (*f)
                                                                                                .lf
                                                                                                .sr_p[1 as libc::c_int
                                                                                                as usize] = (*f)
                                                                                                .sr_cur
                                                                                                .p
                                                                                                .data[(if has_chroma != 0 {
                                                                                                1 as libc::c_int
                                                                                            } else {
                                                                                                0 as libc::c_int
                                                                                            }) as usize];
                                                                                            (*f)
                                                                                                .lf
                                                                                                .sr_p[2 as libc::c_int
                                                                                                as usize] = (*f)
                                                                                                .sr_cur
                                                                                                .p
                                                                                                .data[(if has_chroma != 0 {
                                                                                                2 as libc::c_int
                                                                                            } else {
                                                                                                0 as libc::c_int
                                                                                            }) as usize];
                                                                                            retval = 0 as libc::c_int;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame_init_cdf(f: *mut Dav1dFrameContext) -> libc::c_int {
    let mut current_block: u64;
    let c: *const Dav1dContext = (*f).c;
    let mut retval = -(22 as libc::c_int);
    if (*(*f).frame_hdr).refresh_context != 0 {
        dav1d_cdf_thread_copy((*f).out_cdf.data.cdf, &mut (*f).in_cdf);
    }
    let mut tile_row = 0;
    let mut tile_col = 0;
    (*f).task_thread.update_set = 0 as libc::c_int;
    let mut i = 0;
    's_19: loop {
        if !(i < (*f).n_tile_data) {
            current_block = 15768484401365413375;
            break;
        }
        let mut data: *const uint8_t = (*((*f).tile).offset(i as isize)).data.data;
        let mut size: size_t = (*((*f).tile).offset(i as isize)).data.sz;
        let mut j = (*((*f).tile).offset(i as isize)).start;
        while j <= (*((*f).tile).offset(i as isize)).end {
            let mut tile_sz: size_t = 0;
            if j == (*((*f).tile).offset(i as isize)).end {
                tile_sz = size;
            } else {
                if (*(*f).frame_hdr).tiling.n_bytes as size_t > size {
                    current_block = 610192855792336318;
                    break 's_19;
                }
                tile_sz = 0 as libc::c_int as size_t;
                let mut k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while k < (*(*f).frame_hdr).tiling.n_bytes {
                    let fresh37 = data;
                    data = data.offset(1);
                    tile_sz |= ((*fresh37 as libc::c_uint) << k.wrapping_mul(8)) as size_t;
                    k = k.wrapping_add(1);
                }
                tile_sz = tile_sz.wrapping_add(1);
                size = (size as libc::c_ulong)
                    .wrapping_sub((*(*f).frame_hdr).tiling.n_bytes as libc::c_ulong)
                    as size_t as size_t;
                if tile_sz > size {
                    current_block = 610192855792336318;
                    break 's_19;
                }
            }
            let fresh38 = tile_col;
            tile_col = tile_col + 1;
            setup_tile(
                &mut *((*f).ts).offset(j as isize),
                f,
                data,
                tile_sz,
                tile_row,
                fresh38,
                if (*c).n_fc > 1 as libc::c_uint {
                    *((*f).frame_thread.tile_start_off).offset(j as isize)
                } else {
                    0 as libc::c_int
                },
            );
            if tile_col == (*(*f).frame_hdr).tiling.cols {
                tile_col = 0 as libc::c_int;
                tile_row += 1;
            }
            if j == (*(*f).frame_hdr).tiling.update && (*(*f).frame_hdr).refresh_context != 0 {
                (*f).task_thread.update_set = 1 as libc::c_int;
            }
            data = data.offset(tile_sz as isize);
            size = size.wrapping_sub(tile_sz) as size_t as size_t;
            j += 1;
        }
        i += 1;
    }
    match current_block {
        15768484401365413375 => {
            if (*c).n_tc > 1 as libc::c_uint {
                let uses_2pass = ((*c).n_fc > 1 as libc::c_uint) as libc::c_int;
                let mut n = 0;
                while n < (*f).sb128w * (*(*f).frame_hdr).tiling.rows * (1 + uses_2pass) {
                    reset_context(
                        &mut *((*f).a).offset(n as isize),
                        ((*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint == 0)
                            as libc::c_int,
                        if uses_2pass != 0 {
                            1 as libc::c_int
                                + (n >= (*f).sb128w * (*(*f).frame_hdr).tiling.rows) as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                    );
                    n += 1;
                }
            }
            retval = 0 as libc::c_int;
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame_main(f: *mut Dav1dFrameContext) -> libc::c_int {
    let mut current_block: u64;
    let c: *const Dav1dContext = (*f).c;
    let mut retval = -(22 as libc::c_int);
    if !((*(*f).c).n_tc == 1 as libc::c_uint) {
        unreachable!();
    }
    let t: *mut Dav1dTaskContext =
        &mut *((*c).tc).offset(f.offset_from((*c).fc) as isize) as *mut Dav1dTaskContext;
    (*t).f = f;
    (*t).frame_thread.pass = 0 as libc::c_int;
    let mut n = 0;
    while n < (*f).sb128w * (*(*f).frame_hdr).tiling.rows {
        reset_context(
            &mut *((*f).a).offset(n as isize),
            ((*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint == 0) as libc::c_int,
            0 as libc::c_int,
        );
        n += 1;
    }
    let mut tile_row = 0;
    's_44: loop {
        if !(tile_row < (*(*f).frame_hdr).tiling.rows) {
            current_block = 10652014663920648156;
            break;
        }
        let sbh_end = imin(
            (*(*f).frame_hdr).tiling.row_start_sb[(tile_row + 1) as usize] as libc::c_int,
            (*f).sbh,
        );
        let mut sby = (*(*f).frame_hdr).tiling.row_start_sb[tile_row as usize] as libc::c_int;
        while sby < sbh_end {
            (*t).by = sby << 4 + (*(*f).seq_hdr).sb128;
            let by_end = (*t).by + (*f).sb_step >> 1;
            if (*(*f).frame_hdr).use_ref_frame_mvs != 0 {
                (*(*f).c)
                    .refmvs_dsp
                    .load_tmvs
                    .expect("non-null function pointer")(
                    &mut (*f).rf,
                    tile_row,
                    0 as libc::c_int,
                    (*f).bw >> 1,
                    (*t).by >> 1,
                    by_end,
                );
            }
            let mut tile_col = 0;
            while tile_col < (*(*f).frame_hdr).tiling.cols {
                (*t).ts = &mut *((*f).ts)
                    .offset((tile_row * (*(*f).frame_hdr).tiling.cols + tile_col) as isize)
                    as *mut Dav1dTileState;
                if dav1d_decode_tile_sbrow(t) != 0 {
                    current_block = 3839639024989683879;
                    break 's_44;
                }
                tile_col += 1;
            }
            if (*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint != 0 {
                dav1d_refmvs_save_tmvs(
                    &(*(*f).c).refmvs_dsp,
                    &mut (*t).rt,
                    0 as libc::c_int,
                    (*f).bw >> 1,
                    (*t).by >> 1,
                    by_end,
                );
            }
            ((*f).bd_fn.filter_sbrow).expect("non-null function pointer")(f, sby);
            sby += 1;
        }
        tile_row += 1;
    }
    match current_block {
        10652014663920648156 => {
            retval = 0 as libc::c_int;
        }
        _ => {}
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame_exit(f: *mut Dav1dFrameContext, retval: libc::c_int) {
    let c: *const Dav1dContext = (*f).c;
    if !((*f).sr_cur.p.data[0]).is_null() {
        *&mut (*f).task_thread.error = 0 as libc::c_int;
    }
    if (*c).n_fc > 1 as libc::c_uint && retval != 0 && !((*f).frame_thread.cf).is_null() {
        memset(
            (*f).frame_thread.cf,
            0 as libc::c_int,
            ((*f).frame_thread.cf_sz as size_t)
                .wrapping_mul(128)
                .wrapping_mul(128)
                .wrapping_div(2),
        );
    }
    let mut i = 0;
    while i < 7 {
        if !((*f).refp[i as usize].p.frame_hdr).is_null() {
            dav1d_thread_picture_unref(&mut *((*f).refp).as_mut_ptr().offset(i as isize));
        }
        dav1d_ref_dec(&mut *((*f).ref_mvs_ref).as_mut_ptr().offset(i as isize));
        i += 1;
    }
    dav1d_picture_unref_internal(&mut (*f).cur);
    dav1d_thread_picture_unref(&mut (*f).sr_cur);
    dav1d_cdf_thread_unref(&mut (*f).in_cdf);
    if !((*f).frame_hdr).is_null() && (*(*f).frame_hdr).refresh_context != 0 {
        if !((*f).out_cdf.progress).is_null() {
            ::core::intrinsics::atomic_store_seqcst(
                (*f).out_cdf.progress,
                (if retval == 0 {
                    1 as libc::c_int
                } else {
                    2147483647 - 1
                }) as libc::c_uint,
            );
        }
        dav1d_cdf_thread_unref(&mut (*f).out_cdf);
    }
    dav1d_ref_dec(&mut (*f).cur_segmap_ref);
    dav1d_ref_dec(&mut (*f).prev_segmap_ref);
    dav1d_ref_dec(&mut (*f).mvs_ref);
    dav1d_ref_dec(&mut (*f).seq_hdr_ref);
    dav1d_ref_dec(&mut (*f).frame_hdr_ref);
    let mut i_0 = 0;
    while i_0 < (*f).n_tile_data {
        dav1d_data_unref_internal(&mut (*((*f).tile).offset(i_0 as isize)).data);
        i_0 += 1;
    }
    (*f).task_thread.retval = retval;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_decode_frame(f: *mut Dav1dFrameContext) -> libc::c_int {
    if !((*(*f).c).n_fc == 1 as libc::c_uint) {
        unreachable!();
    }
    let mut res = dav1d_decode_frame_init(f);
    if res == 0 {
        res = dav1d_decode_frame_init_cdf(f);
    }
    if res == 0 {
        if (*(*f).c).n_tc > 1 as libc::c_uint {
            res = dav1d_task_create_tile_sbrow(f, 0 as libc::c_int, 1 as libc::c_int);
            pthread_mutex_lock(&mut (*(*f).task_thread.ttd).lock);
            pthread_cond_signal(&mut (*(*f).task_thread.ttd).cond);
            if res == 0 {
                while (*f).task_thread.done[0] == 0
                    || ::core::intrinsics::atomic_load_seqcst(
                        &mut (*f).task_thread.task_counter as *mut atomic_int,
                    ) > 0
                {
                    pthread_cond_wait(
                        &mut (*f).task_thread.cond,
                        &mut (*(*f).task_thread.ttd).lock,
                    );
                }
            }
            pthread_mutex_unlock(&mut (*(*f).task_thread.ttd).lock);
            res = (*f).task_thread.retval;
        } else {
            res = dav1d_decode_frame_main(f);
            if res == 0
                && (*(*f).frame_hdr).refresh_context != 0
                && (*f).task_thread.update_set != 0
            {
                dav1d_cdf_thread_update(
                    (*f).frame_hdr,
                    (*f).out_cdf.data.cdf,
                    &mut (*((*f).ts).offset((*(*f).frame_hdr).tiling.update as isize)).cdf,
                );
            }
        }
    }
    dav1d_decode_frame_exit(f, res);
    (*f).n_tile_data = 0 as libc::c_int;
    return res;
}
unsafe extern "C" fn get_upscale_x0(
    in_w: libc::c_int,
    out_w: libc::c_int,
    step: libc::c_int,
) -> libc::c_int {
    let err = out_w * step - (in_w << 14);
    let x0 = (-(out_w - in_w << 13) + (out_w >> 1)) / out_w + 128 - err / 2;
    return x0 & 0x3fff as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_submit_frame(c: *mut Dav1dContext) -> libc::c_int {
    let mut ref_coded_width: [libc::c_int; 7] = [0; 7];
    let mut uses_2pass = 0;
    let mut cols = 0;
    let mut rows = 0;
    let mut refresh_frame_flags: libc::c_uint = 0;
    let mut current_block: u64;
    let mut f: *mut Dav1dFrameContext = 0 as *mut Dav1dFrameContext;
    let mut res = -(1 as libc::c_int);
    let mut out_delayed: *mut Dav1dThreadPicture = 0 as *mut Dav1dThreadPicture;
    if (*c).n_fc > 1 as libc::c_uint {
        pthread_mutex_lock(&mut (*c).task_thread.lock);
        let fresh39 = (*c).frame_thread.next;
        (*c).frame_thread.next = ((*c).frame_thread.next).wrapping_add(1);
        let next: libc::c_uint = fresh39;
        if (*c).frame_thread.next == (*c).n_fc {
            (*c).frame_thread.next = 0 as libc::c_int as libc::c_uint;
        }
        f = &mut *((*c).fc).offset(next as isize) as *mut Dav1dFrameContext;
        while (*f).n_tile_data > 0 {
            pthread_cond_wait(&mut (*f).task_thread.cond, &mut (*c).task_thread.lock);
        }
        out_delayed =
            &mut *((*c).frame_thread.out_delayed).offset(next as isize) as *mut Dav1dThreadPicture;
        if !((*out_delayed).p.data[0]).is_null()
            || ::core::intrinsics::atomic_load_seqcst(
                &mut (*f).task_thread.error as *mut atomic_int,
            ) != 0
        {
            let mut first: libc::c_uint =
                ::core::intrinsics::atomic_load_seqcst(&mut (*c).task_thread.first);
            if first.wrapping_add(1 as libc::c_uint) < (*c).n_fc {
                ::core::intrinsics::atomic_xadd_seqcst(
                    &mut (*c).task_thread.first,
                    1 as libc::c_uint,
                );
            } else {
                ::core::intrinsics::atomic_store_seqcst(
                    &mut (*c).task_thread.first,
                    0 as libc::c_int as libc::c_uint,
                );
            }
            let fresh40 = ::core::intrinsics::atomic_cxchg_seqcst_seqcst(
                &mut (*c).task_thread.reset_task_cur,
                *&mut first,
                (2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint),
            );
            *&mut first = fresh40.0;
            fresh40.1;
            if (*c).task_thread.cur != 0 && (*c).task_thread.cur < (*c).n_fc {
                (*c).task_thread.cur = ((*c).task_thread.cur).wrapping_sub(1);
            }
        }
        let error = (*f).task_thread.retval;
        if error != 0 {
            (*f).task_thread.retval = 0 as libc::c_int;
            (*c).cached_error = error;
            dav1d_data_props_copy(&mut (*c).cached_error_props, &mut (*out_delayed).p.m);
            dav1d_thread_picture_unref(out_delayed);
        } else if !((*out_delayed).p.data[0]).is_null() {
            let progress: libc::c_uint = ::core::intrinsics::atomic_load_relaxed(
                &mut *((*out_delayed).progress).offset(1) as *mut atomic_uint,
            );
            if ((*out_delayed).visible != 0 || (*c).output_invisible_frames != 0)
                && progress
                    != (2147483647 as libc::c_int as libc::c_uint)
                        .wrapping_mul(2 as libc::c_uint)
                        .wrapping_add(1 as libc::c_uint)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                dav1d_thread_picture_ref(&mut (*c).out, out_delayed);
                (*c).event_flags = ::core::mem::transmute::<libc::c_uint, Dav1dEventFlags>(
                    (*c).event_flags as libc::c_uint
                        | dav1d_picture_get_event_flags(out_delayed) as libc::c_uint,
                );
            }
            dav1d_thread_picture_unref(out_delayed);
        }
    } else {
        f = (*c).fc;
    }
    (*f).seq_hdr = (*c).seq_hdr;
    (*f).seq_hdr_ref = (*c).seq_hdr_ref;
    dav1d_ref_inc((*f).seq_hdr_ref);
    (*f).frame_hdr = (*c).frame_hdr;
    (*f).frame_hdr_ref = (*c).frame_hdr_ref;
    (*c).frame_hdr = 0 as *mut Dav1dFrameHeader;
    (*c).frame_hdr_ref = 0 as *mut Dav1dRef;
    (*f).dsp =
        &mut *((*c).dsp).as_mut_ptr().offset((*(*f).seq_hdr).hbd as isize) as *mut Dav1dDSPContext;
    let bpc = 8 + 2 * (*(*f).seq_hdr).hbd;
    if ((*(*f).dsp).ipred.intra_pred[DC_PRED as libc::c_int as usize]).is_none() {
        let dsp: *mut Dav1dDSPContext =
            &mut *((*c).dsp).as_mut_ptr().offset((*(*f).seq_hdr).hbd as isize)
                as *mut Dav1dDSPContext;
        match bpc {
            #[cfg(feature = "bitdepth_8")]
            8 => {
                dav1d_cdef_dsp_init_8bpc(&mut (*dsp).cdef);
                dav1d_intra_pred_dsp_init_8bpc(&mut (*dsp).ipred);
                dav1d_itx_dsp_init_8bpc(&mut (*dsp).itx, bpc);
                dav1d_loop_filter_dsp_init_8bpc(&mut (*dsp).lf);
                dav1d_loop_restoration_dsp_init_8bpc(&mut (*dsp).lr, bpc);
                dav1d_mc_dsp_init_8bpc(&mut (*dsp).mc);
                dav1d_film_grain_dsp_init_8bpc(&mut (*dsp).fg);
                current_block = 313581471991351815;
            }
            #[cfg(feature = "bitdepth_16")]
            10 | 12 => {
                dav1d_cdef_dsp_init_16bpc(&mut (*dsp).cdef);
                dav1d_intra_pred_dsp_init_16bpc(&mut (*dsp).ipred);
                dav1d_itx_dsp_init_16bpc(&mut (*dsp).itx, bpc);
                dav1d_loop_filter_dsp_init_16bpc(&mut (*dsp).lf);
                dav1d_loop_restoration_dsp_init_16bpc(&mut (*dsp).lr, bpc);
                dav1d_mc_dsp_init_16bpc(&mut (*dsp).mc);
                dav1d_film_grain_dsp_init_16bpc(&mut (*dsp).fg);
                current_block = 313581471991351815;
            }
            _ => {
                dav1d_log(
                    c,
                    b"Compiled without support for %d-bit decoding\n\0" as *const u8
                        as *const libc::c_char,
                    8 + 2 * (*(*f).seq_hdr).hbd,
                );
                res = -(92 as libc::c_int);
                current_block = 9123693364129885070;
            }
        }
    } else {
        current_block = 313581471991351815;
    }
    match current_block {
        313581471991351815 => {
            if (*(*f).seq_hdr).hbd == 0 {
                #[cfg(feature = "bitdepth_8")]
                {
                    (*f).bd_fn.recon_b_inter = Some(
                        dav1d_recon_b_inter_8bpc
                            as unsafe extern "C" fn(
                                *mut Dav1dTaskContext,
                                BlockSize,
                                *const Av1Block,
                            ) -> libc::c_int,
                    );
                    (*f).bd_fn.recon_b_intra = Some(
                        dav1d_recon_b_intra_8bpc
                            as unsafe extern "C" fn(
                                *mut Dav1dTaskContext,
                                BlockSize,
                                EdgeFlags,
                                *const Av1Block,
                            ) -> (),
                    );
                    (*f).bd_fn.filter_sbrow = Some(
                        dav1d_filter_sbrow_8bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_deblock_cols = Some(
                        dav1d_filter_sbrow_deblock_cols_8bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_deblock_rows = Some(
                        dav1d_filter_sbrow_deblock_rows_8bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_cdef = Some(
                        dav1d_filter_sbrow_cdef_8bpc
                            as unsafe extern "C" fn(*mut Dav1dTaskContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_resize = Some(
                        dav1d_filter_sbrow_resize_8bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_lr = Some(
                        dav1d_filter_sbrow_lr_8bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.backup_ipred_edge = Some(
                        dav1d_backup_ipred_edge_8bpc
                            as unsafe extern "C" fn(*mut Dav1dTaskContext) -> (),
                    );
                    (*f).bd_fn.read_coef_blocks = Some(
                        dav1d_read_coef_blocks_8bpc
                            as unsafe extern "C" fn(
                                *mut Dav1dTaskContext,
                                BlockSize,
                                *const Av1Block,
                            ) -> (),
                    );
                }
            } else {
                #[cfg(feature = "bitdepth_16")]
                {
                    (*f).bd_fn.recon_b_inter = Some(
                        dav1d_recon_b_inter_16bpc
                            as unsafe extern "C" fn(
                                *mut Dav1dTaskContext,
                                BlockSize,
                                *const Av1Block,
                            ) -> libc::c_int,
                    );
                    (*f).bd_fn.recon_b_intra = Some(
                        dav1d_recon_b_intra_16bpc
                            as unsafe extern "C" fn(
                                *mut Dav1dTaskContext,
                                BlockSize,
                                EdgeFlags,
                                *const Av1Block,
                            ) -> (),
                    );
                    (*f).bd_fn.filter_sbrow = Some(
                        dav1d_filter_sbrow_16bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_deblock_cols = Some(
                        dav1d_filter_sbrow_deblock_cols_16bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_deblock_rows = Some(
                        dav1d_filter_sbrow_deblock_rows_16bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_cdef = Some(
                        dav1d_filter_sbrow_cdef_16bpc
                            as unsafe extern "C" fn(*mut Dav1dTaskContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_resize = Some(
                        dav1d_filter_sbrow_resize_16bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.filter_sbrow_lr = Some(
                        dav1d_filter_sbrow_lr_16bpc
                            as unsafe extern "C" fn(*mut Dav1dFrameContext, libc::c_int) -> (),
                    );
                    (*f).bd_fn.backup_ipred_edge = Some(
                        dav1d_backup_ipred_edge_16bpc
                            as unsafe extern "C" fn(*mut Dav1dTaskContext) -> (),
                    );
                    (*f).bd_fn.read_coef_blocks = Some(
                        dav1d_read_coef_blocks_16bpc
                            as unsafe extern "C" fn(
                                *mut Dav1dTaskContext,
                                BlockSize,
                                *const Av1Block,
                            ) -> (),
                    );
                }
            }
            ref_coded_width = [0; 7];
            if (*(*f).frame_hdr).frame_type as libc::c_uint & 1 as libc::c_uint != 0 {
                if (*(*f).frame_hdr).primary_ref_frame != 7 as libc::c_int {
                    let pri_ref =
                        (*(*f).frame_hdr).refidx[(*(*f).frame_hdr).primary_ref_frame as usize];
                    if ((*c).refs[pri_ref as usize].p.p.data[0]).is_null() {
                        res = -(22 as libc::c_int);
                        current_block = 9123693364129885070;
                    } else {
                        current_block = 13660591889533726445;
                    }
                } else {
                    current_block = 13660591889533726445;
                }
                match current_block {
                    9123693364129885070 => {}
                    _ => {
                        let mut i = 0;
                        loop {
                            if !(i < 7) {
                                current_block = 14648606000749551097;
                                break;
                            }
                            let refidx = (*(*f).frame_hdr).refidx[i as usize];
                            if ((*c).refs[refidx as usize].p.p.data[0]).is_null()
                                || ((*(*f).frame_hdr).width[0] * 2)
                                    < (*c).refs[refidx as usize].p.p.p.w
                                || ((*(*f).frame_hdr).height * 2)
                                    < (*c).refs[refidx as usize].p.p.p.h
                                || (*(*f).frame_hdr).width[0]
                                    > (*c).refs[refidx as usize].p.p.p.w * 16
                                || (*(*f).frame_hdr).height
                                    > (*c).refs[refidx as usize].p.p.p.h * 16
                                || (*(*f).seq_hdr).layout as libc::c_uint
                                    != (*c).refs[refidx as usize].p.p.p.layout as libc::c_uint
                                || bpc != (*c).refs[refidx as usize].p.p.p.bpc
                            {
                                let mut j = 0;
                                while j < i {
                                    dav1d_thread_picture_unref(
                                        &mut *((*f).refp).as_mut_ptr().offset(j as isize),
                                    );
                                    j += 1;
                                }
                                res = -(22 as libc::c_int);
                                current_block = 9123693364129885070;
                                break;
                            } else {
                                dav1d_thread_picture_ref(
                                    &mut *((*f).refp).as_mut_ptr().offset(i as isize),
                                    &mut (*((*c).refs).as_mut_ptr().offset(refidx as isize)).p,
                                );
                                ref_coded_width[i as usize] =
                                    (*(*c).refs[refidx as usize].p.p.frame_hdr).width[0];
                                if (*(*f).frame_hdr).width[0] != (*c).refs[refidx as usize].p.p.p.w
                                    || (*(*f).frame_hdr).height
                                        != (*c).refs[refidx as usize].p.p.p.h
                                {
                                    (*f).svc[i as usize][0].scale =
                                        (((*c).refs[refidx as usize].p.p.p.w << 14)
                                            + ((*(*f).frame_hdr).width[0] >> 1))
                                            / (*(*f).frame_hdr).width[0];
                                    (*f).svc[i as usize][1].scale =
                                        (((*c).refs[refidx as usize].p.p.p.h << 14)
                                            + ((*(*f).frame_hdr).height >> 1))
                                            / (*(*f).frame_hdr).height;
                                    (*f).svc[i as usize][0].step =
                                        (*f).svc[i as usize][0].scale + 8 >> 4;
                                    (*f).svc[i as usize][1].step =
                                        (*f).svc[i as usize][1].scale + 8 >> 4;
                                } else {
                                    (*f).svc[i as usize][1].scale = 0 as libc::c_int;
                                    (*f).svc[i as usize][0].scale = (*f).svc[i as usize][1].scale;
                                }
                                (*f).gmv_warp_allowed[i as usize] =
                                    ((*(*f).frame_hdr).gmv[i as usize].type_0 as libc::c_uint
                                        > DAV1D_WM_TYPE_TRANSLATION as libc::c_int as libc::c_uint
                                        && (*(*f).frame_hdr).force_integer_mv == 0
                                        && !dav1d_get_shear_params(
                                            &mut *((*(*f).frame_hdr).gmv)
                                                .as_mut_ptr()
                                                .offset(i as isize),
                                        )
                                        && (*f).svc[i as usize][0].scale == 0)
                                        as libc::c_int
                                        as uint8_t;
                                i += 1;
                            }
                        }
                    }
                }
            } else {
                current_block = 14648606000749551097;
            }
            match current_block {
                9123693364129885070 => {}
                _ => {
                    if (*(*f).frame_hdr).primary_ref_frame == 7 {
                        dav1d_cdf_thread_init_static(&mut (*f).in_cdf, (*(*f).frame_hdr).quant.yac);
                    } else {
                        let pri_ref_0 =
                            (*(*f).frame_hdr).refidx[(*(*f).frame_hdr).primary_ref_frame as usize];
                        dav1d_cdf_thread_ref(
                            &mut (*f).in_cdf,
                            &mut *((*c).cdf).as_mut_ptr().offset(pri_ref_0 as isize),
                        );
                    }
                    if (*(*f).frame_hdr).refresh_context != 0 {
                        res = dav1d_cdf_thread_alloc(
                            c,
                            &mut (*f).out_cdf,
                            ((*c).n_fc > 1 as libc::c_uint) as libc::c_int,
                        );
                        if res < 0 {
                            current_block = 9123693364129885070;
                        } else {
                            current_block = 16037123508100270995;
                        }
                    } else {
                        current_block = 16037123508100270995;
                    }
                    match current_block {
                        9123693364129885070 => {}
                        _ => {
                            if (*f).n_tile_data_alloc < (*c).n_tile_data {
                                freep(
                                    &mut (*f).tile as *mut *mut Dav1dTileGroup as *mut libc::c_void,
                                );
                                if !((*c).n_tile_data
                                    < 2147483647
                                        / ::core::mem::size_of::<Dav1dTileGroup>() as libc::c_ulong
                                            as libc::c_int)
                                {
                                    unreachable!();
                                }
                                (*f).tile = malloc(
                                    ((*c).n_tile_data as libc::c_ulong)
                                        .wrapping_mul(::core::mem::size_of::<Dav1dTileGroup>()
                                            as libc::c_ulong),
                                )
                                    as *mut Dav1dTileGroup;
                                if ((*f).tile).is_null() {
                                    (*f).n_tile_data = 0 as libc::c_int;
                                    (*f).n_tile_data_alloc = (*f).n_tile_data;
                                    res = -(12 as libc::c_int);
                                    current_block = 9123693364129885070;
                                } else {
                                    (*f).n_tile_data_alloc = (*c).n_tile_data;
                                    current_block = 1417769144978639029;
                                }
                            } else {
                                current_block = 1417769144978639029;
                            }
                            match current_block {
                                9123693364129885070 => {}
                                _ => {
                                    memcpy(
                                        (*f).tile as *mut libc::c_void,
                                        (*c).tile as *const libc::c_void,
                                        ((*c).n_tile_data as libc::c_ulong)
                                            .wrapping_mul(::core::mem::size_of::<Dav1dTileGroup>()
                                                as libc::c_ulong),
                                    );
                                    memset(
                                        (*c).tile as *mut libc::c_void,
                                        0 as libc::c_int,
                                        ((*c).n_tile_data as size_t)
                                            .wrapping_mul(::core::mem::size_of::<Dav1dTileGroup>()),
                                    );
                                    (*f).n_tile_data = (*c).n_tile_data;
                                    (*c).n_tile_data = 0 as libc::c_int;
                                    res = dav1d_thread_picture_alloc(c, f, bpc);
                                    if !(res < 0) {
                                        if (*(*f).frame_hdr).width[0] != (*(*f).frame_hdr).width[1]
                                        {
                                            res = dav1d_picture_alloc_copy(
                                                c,
                                                &mut (*f).cur,
                                                (*(*f).frame_hdr).width[0],
                                                &mut (*f).sr_cur.p,
                                            );
                                            if res < 0 {
                                                current_block = 9123693364129885070;
                                            } else {
                                                current_block = 5409161009579131794;
                                            }
                                        } else {
                                            dav1d_picture_ref(&mut (*f).cur, &mut (*f).sr_cur.p);
                                            current_block = 5409161009579131794;
                                        }
                                        match current_block {
                                            9123693364129885070 => {}
                                            _ => {
                                                if (*(*f).frame_hdr).width[0]
                                                    != (*(*f).frame_hdr).width[1]
                                                {
                                                    (*f).resize_step[0] = (((*f).cur.p.w << 14)
                                                        + ((*f).sr_cur.p.p.w >> 1))
                                                        / (*f).sr_cur.p.p.w;
                                                    let ss_hor = ((*f).cur.p.layout as libc::c_uint
                                                        != DAV1D_PIXEL_LAYOUT_I444 as libc::c_int
                                                            as libc::c_uint)
                                                        as libc::c_int;
                                                    let in_cw = (*f).cur.p.w + ss_hor >> ss_hor;
                                                    let out_cw =
                                                        (*f).sr_cur.p.p.w + ss_hor >> ss_hor;
                                                    (*f).resize_step[1] =
                                                        ((in_cw << 14) + (out_cw >> 1)) / out_cw;
                                                    (*f).resize_start[0] = get_upscale_x0(
                                                        (*f).cur.p.w,
                                                        (*f).sr_cur.p.p.w,
                                                        (*f).resize_step[0],
                                                    );
                                                    (*f).resize_start[1] = get_upscale_x0(
                                                        in_cw,
                                                        out_cw,
                                                        (*f).resize_step[1],
                                                    );
                                                }
                                                if (*c).n_fc == 1 as libc::c_uint {
                                                    if (*(*f).frame_hdr).show_frame != 0
                                                        || (*c).output_invisible_frames != 0
                                                    {
                                                        dav1d_thread_picture_ref(
                                                            &mut (*c).out,
                                                            &mut (*f).sr_cur,
                                                        );
                                                        (*c).event_flags = ::core::mem::transmute::<
                                                            libc::c_uint,
                                                            Dav1dEventFlags,
                                                        >(
                                                            (*c).event_flags as libc::c_uint
                                                                | dav1d_picture_get_event_flags(
                                                                    &mut (*f).sr_cur,
                                                                )
                                                                    as libc::c_uint,
                                                        );
                                                    }
                                                } else {
                                                    dav1d_thread_picture_ref(
                                                        out_delayed,
                                                        &mut (*f).sr_cur,
                                                    );
                                                }
                                                (*f).w4 = (*(*f).frame_hdr).width[0] + 3 >> 2;
                                                (*f).h4 = (*(*f).frame_hdr).height + 3 >> 2;
                                                (*f).bw =
                                                    ((*(*f).frame_hdr).width[0] + 7 >> 3) << 1;
                                                (*f).bh = ((*(*f).frame_hdr).height + 7 >> 3) << 1;
                                                (*f).sb128w = (*f).bw + 31 >> 5;
                                                (*f).sb128h = (*f).bh + 31 >> 5;
                                                (*f).sb_shift = 4 + (*(*f).seq_hdr).sb128;
                                                (*f).sb_step =
                                                    (16 as libc::c_int) << (*(*f).seq_hdr).sb128;
                                                (*f).sbh =
                                                    (*f).bh + (*f).sb_step - 1 >> (*f).sb_shift;
                                                (*f).b4_stride = ((*f).bw + 31
                                                    & !(31 as libc::c_int))
                                                    as ptrdiff_t;
                                                (*f).bitdepth_max =
                                                    ((1 as libc::c_int) << (*f).cur.p.bpc) - 1;
                                                *&mut (*f).task_thread.error = 0 as libc::c_int;
                                                uses_2pass =
                                                    ((*c).n_fc > 1 as libc::c_uint) as libc::c_int;
                                                cols = (*(*f).frame_hdr).tiling.cols;
                                                rows = (*(*f).frame_hdr).tiling.rows;
                                                ::core::intrinsics::atomic_store_seqcst(
                                                    &mut (*f).task_thread.task_counter,
                                                    cols * rows + (*f).sbh << uses_2pass,
                                                );
                                                if (*(*f).frame_hdr).frame_type as libc::c_uint
                                                    & 1 as libc::c_uint
                                                    != 0
                                                    || (*(*f).frame_hdr).allow_intrabc != 0
                                                {
                                                    (*f).mvs_ref = dav1d_ref_create_using_pool(
                                                        (*c).refmvs_pool,
                                                        (::core::mem::size_of::<
                                                            refmvs_temporal_block,
                                                        >(
                                                        ))
                                                        .wrapping_mul((*f).sb128h as size_t)
                                                        .wrapping_mul(16)
                                                        .wrapping_mul(
                                                            ((*f).b4_stride >> 1) as size_t,
                                                        ),
                                                    );
                                                    if ((*f).mvs_ref).is_null() {
                                                        res = -(12 as libc::c_int);
                                                        current_block = 9123693364129885070;
                                                    } else {
                                                        (*f).mvs = (*(*f).mvs_ref).data
                                                            as *mut refmvs_temporal_block;
                                                        if (*(*f).frame_hdr).allow_intrabc == 0 {
                                                            let mut i_0 = 0;
                                                            while i_0 < 7 {
                                                                (*f).refpoc[i_0 as usize] =
                                                                    (*(*f).refp[i_0 as usize]
                                                                        .p
                                                                        .frame_hdr)
                                                                        .frame_offset
                                                                        as libc::c_uint;
                                                                i_0 += 1;
                                                            }
                                                        } else {
                                                            memset(
                                                                ((*f).refpoc).as_mut_ptr()
                                                                    as *mut libc::c_void,
                                                                0 as libc::c_int,
                                                                ::core::mem::size_of::<
                                                                    [libc::c_uint; 7],
                                                                >(
                                                                ),
                                                            );
                                                        }
                                                        if (*(*f).frame_hdr).use_ref_frame_mvs != 0
                                                        {
                                                            let mut i_1 = 0;
                                                            while i_1 < 7 {
                                                                let refidx_0 = (*(*f).frame_hdr)
                                                                    .refidx
                                                                    [i_1 as usize];
                                                                let ref_w = (ref_coded_width
                                                                    [i_1 as usize]
                                                                    + 7
                                                                    >> 3)
                                                                    << 1;
                                                                let ref_h =
                                                                    ((*f).refp[i_1 as usize].p.p.h
                                                                        + 7
                                                                        >> 3)
                                                                        << 1;
                                                                if !((*c).refs[refidx_0 as usize]
                                                                    .refmvs)
                                                                    .is_null()
                                                                    && ref_w == (*f).bw
                                                                    && ref_h == (*f).bh
                                                                {
                                                                    (*f).ref_mvs_ref
                                                                        [i_1 as usize] = (*c).refs
                                                                        [refidx_0 as usize]
                                                                        .refmvs;
                                                                    dav1d_ref_inc(
                                                                        (*f).ref_mvs_ref
                                                                            [i_1 as usize],
                                                                    );
                                                                    (*f)
                                                                        .ref_mvs[i_1
                                                                        as usize] = (*(*c).refs[refidx_0 as usize].refmvs).data
                                                                        as *mut refmvs_temporal_block;
                                                                } else {
                                                                    (*f)
                                                                        .ref_mvs[i_1 as usize] = 0 as *mut refmvs_temporal_block;
                                                                    (*f).ref_mvs_ref
                                                                        [i_1 as usize] =
                                                                        0 as *mut Dav1dRef;
                                                                }
                                                                memcpy(
                                                                    ((*f).refrefpoc[i_1 as usize])
                                                                        .as_mut_ptr()
                                                                        as *mut libc::c_void,
                                                                    ((*c).refs[refidx_0 as usize]
                                                                        .refpoc)
                                                                        .as_mut_ptr()
                                                                        as *const libc::c_void,
                                                                    ::core::mem::size_of::<
                                                                        [libc::c_uint; 7],
                                                                    >(
                                                                    )
                                                                        as libc::c_ulong,
                                                                );
                                                                i_1 += 1;
                                                            }
                                                        } else {
                                                            memset(
                                                                ((*f).ref_mvs_ref).as_mut_ptr()
                                                                    as *mut libc::c_void,
                                                                0 as libc::c_int,
                                                                ::core::mem::size_of::<
                                                                    [*mut Dav1dRef; 7],
                                                                >(
                                                                ),
                                                            );
                                                        }
                                                        current_block = 2704538829018177290;
                                                    }
                                                } else {
                                                    (*f).mvs_ref = 0 as *mut Dav1dRef;
                                                    memset(
                                                        ((*f).ref_mvs_ref).as_mut_ptr()
                                                            as *mut libc::c_void,
                                                        0 as libc::c_int,
                                                        ::core::mem::size_of::<[*mut Dav1dRef; 7]>(
                                                        ),
                                                    );
                                                    current_block = 2704538829018177290;
                                                }
                                                match current_block {
                                                    9123693364129885070 => {}
                                                    _ => {
                                                        if (*(*f).frame_hdr).segmentation.enabled
                                                            != 0
                                                        {
                                                            (*f).prev_segmap_ref =
                                                                0 as *mut Dav1dRef;
                                                            (*f).prev_segmap = 0 as *const uint8_t;
                                                            if (*(*f).frame_hdr)
                                                                .segmentation
                                                                .temporal
                                                                != 0
                                                                || (*(*f).frame_hdr)
                                                                    .segmentation
                                                                    .update_map
                                                                    == 0
                                                            {
                                                                let pri_ref_1 = (*(*f).frame_hdr)
                                                                    .primary_ref_frame;
                                                                if !(pri_ref_1 != 7 as libc::c_int)
                                                                {
                                                                    unreachable!();
                                                                }
                                                                let ref_w_0 = (ref_coded_width
                                                                    [pri_ref_1 as usize]
                                                                    + 7
                                                                    >> 3)
                                                                    << 1;
                                                                let ref_h_0 = ((*f).refp
                                                                    [pri_ref_1 as usize]
                                                                    .p
                                                                    .p
                                                                    .h
                                                                    + 7
                                                                    >> 3)
                                                                    << 1;
                                                                if ref_w_0 == (*f).bw
                                                                    && ref_h_0 == (*f).bh
                                                                {
                                                                    (*f).prev_segmap_ref = (*c)
                                                                        .refs
                                                                        [(*(*f).frame_hdr).refidx
                                                                            [pri_ref_1 as usize]
                                                                            as usize]
                                                                        .segmap;
                                                                    if !((*f).prev_segmap_ref)
                                                                        .is_null()
                                                                    {
                                                                        dav1d_ref_inc(
                                                                            (*f).prev_segmap_ref,
                                                                        );
                                                                        (*f).prev_segmap = (*(*f)
                                                                            .prev_segmap_ref)
                                                                            .data
                                                                            as *const uint8_t;
                                                                    }
                                                                }
                                                            }
                                                            if (*(*f).frame_hdr)
                                                                .segmentation
                                                                .update_map
                                                                != 0
                                                            {
                                                                (*f).cur_segmap_ref =
                                                                    dav1d_ref_create_using_pool(
                                                                        (*c).segmap_pool,
                                                                        (::core::mem::size_of::<
                                                                            uint8_t,
                                                                        >(
                                                                        ))
                                                                        .wrapping_mul(
                                                                            (*f).b4_stride
                                                                                as size_t,
                                                                        )
                                                                        .wrapping_mul(32)
                                                                        .wrapping_mul(
                                                                            (*f).sb128h as size_t,
                                                                        ),
                                                                    );
                                                                if ((*f).cur_segmap_ref).is_null() {
                                                                    dav1d_ref_dec(
                                                                        &mut (*f).prev_segmap_ref,
                                                                    );
                                                                    res = -(12 as libc::c_int);
                                                                    current_block =
                                                                        9123693364129885070;
                                                                } else {
                                                                    (*f).cur_segmap =
                                                                        (*(*f).cur_segmap_ref).data
                                                                            as *mut uint8_t;
                                                                    current_block =
                                                                        10194589593280242392;
                                                                }
                                                            } else if !((*f).prev_segmap_ref)
                                                                .is_null()
                                                            {
                                                                (*f).cur_segmap_ref =
                                                                    (*f).prev_segmap_ref;
                                                                dav1d_ref_inc((*f).cur_segmap_ref);
                                                                (*f).cur_segmap =
                                                                    (*(*f).prev_segmap_ref).data
                                                                        as *mut uint8_t;
                                                                current_block =
                                                                    10194589593280242392;
                                                            } else {
                                                                let segmap_size: size_t = (::core::mem::size_of::<uint8_t>())
                                                                    .wrapping_mul((*f).b4_stride as size_t)
                                                                    .wrapping_mul(32)
                                                                    .wrapping_mul((*f).sb128h as size_t);
                                                                (*f).cur_segmap_ref =
                                                                    dav1d_ref_create_using_pool(
                                                                        (*c).segmap_pool,
                                                                        segmap_size,
                                                                    );
                                                                if ((*f).cur_segmap_ref).is_null() {
                                                                    res = -(12 as libc::c_int);
                                                                    current_block =
                                                                        9123693364129885070;
                                                                } else {
                                                                    (*f).cur_segmap =
                                                                        (*(*f).cur_segmap_ref).data
                                                                            as *mut uint8_t;
                                                                    memset(
                                                                        (*f).cur_segmap
                                                                            as *mut libc::c_void,
                                                                        0 as libc::c_int,
                                                                        segmap_size,
                                                                    );
                                                                    current_block =
                                                                        10194589593280242392;
                                                                }
                                                            }
                                                        } else {
                                                            (*f).cur_segmap = 0 as *mut uint8_t;
                                                            (*f).cur_segmap_ref =
                                                                0 as *mut Dav1dRef;
                                                            (*f).prev_segmap_ref =
                                                                0 as *mut Dav1dRef;
                                                            current_block = 10194589593280242392;
                                                        }
                                                        match current_block {
                                                            9123693364129885070 => {}
                                                            _ => {
                                                                refresh_frame_flags = (*(*f)
                                                                    .frame_hdr)
                                                                    .refresh_frame_flags
                                                                    as libc::c_uint;
                                                                let mut i_2 = 0;
                                                                while i_2 < 8 {
                                                                    if refresh_frame_flags
                                                                        & ((1 as libc::c_int)
                                                                            << i_2)
                                                                            as libc::c_uint
                                                                        != 0
                                                                    {
                                                                        if !((*c).refs
                                                                            [i_2 as usize]
                                                                            .p
                                                                            .p
                                                                            .frame_hdr)
                                                                            .is_null()
                                                                        {
                                                                            dav1d_thread_picture_unref(
                                                                                &mut (*((*c).refs).as_mut_ptr().offset(i_2 as isize)).p,
                                                                            );
                                                                        }
                                                                        dav1d_thread_picture_ref(
                                                                            &mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_2 as isize,
                                                                                ))
                                                                            .p,
                                                                            &mut (*f).sr_cur,
                                                                        );
                                                                        dav1d_cdf_thread_unref(
                                                                            &mut *((*c).cdf)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_2 as isize,
                                                                                ),
                                                                        );
                                                                        if (*(*f).frame_hdr)
                                                                            .refresh_context
                                                                            != 0
                                                                        {
                                                                            dav1d_cdf_thread_ref(
                                                                                &mut *((*c).cdf).as_mut_ptr().offset(i_2 as isize),
                                                                                &mut (*f).out_cdf,
                                                                            );
                                                                        } else {
                                                                            dav1d_cdf_thread_ref(
                                                                                &mut *((*c).cdf).as_mut_ptr().offset(i_2 as isize),
                                                                                &mut (*f).in_cdf,
                                                                            );
                                                                        }
                                                                        dav1d_ref_dec(
                                                                            &mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_2 as isize,
                                                                                ))
                                                                            .segmap,
                                                                        );
                                                                        (*c).refs[i_2 as usize]
                                                                            .segmap =
                                                                            (*f).cur_segmap_ref;
                                                                        if !((*f).cur_segmap_ref)
                                                                            .is_null()
                                                                        {
                                                                            dav1d_ref_inc(
                                                                                (*f).cur_segmap_ref,
                                                                            );
                                                                        }
                                                                        dav1d_ref_dec(
                                                                            &mut (*((*c).refs)
                                                                                .as_mut_ptr()
                                                                                .offset(
                                                                                    i_2 as isize,
                                                                                ))
                                                                            .refmvs,
                                                                        );
                                                                        if (*(*f).frame_hdr)
                                                                            .allow_intrabc
                                                                            == 0
                                                                        {
                                                                            (*c).refs
                                                                                [i_2 as usize]
                                                                                .refmvs =
                                                                                (*f).mvs_ref;
                                                                            if !((*f).mvs_ref)
                                                                                .is_null()
                                                                            {
                                                                                dav1d_ref_inc(
                                                                                    (*f).mvs_ref,
                                                                                );
                                                                            }
                                                                        }
                                                                        memcpy(
                                                                            ((*c).refs[i_2 as usize].refpoc).as_mut_ptr()
                                                                                as *mut libc::c_void,
                                                                            ((*f).refpoc).as_mut_ptr() as *const libc::c_void,
                                                                            ::core::mem::size_of::<[libc::c_uint; 7]>() as libc::c_ulong,
                                                                        );
                                                                    }
                                                                    i_2 += 1;
                                                                }
                                                                if (*c).n_fc == 1 as libc::c_uint {
                                                                    res = dav1d_decode_frame(f);
                                                                    if res < 0 {
                                                                        dav1d_thread_picture_unref(
                                                                            &mut (*c).out,
                                                                        );
                                                                        let mut i_3 = 0;
                                                                        while i_3 < 8 {
                                                                            if refresh_frame_flags
                                                                                & ((1
                                                                                    as libc::c_int)
                                                                                    << i_3)
                                                                                    as libc::c_uint
                                                                                != 0
                                                                            {
                                                                                if !((*c).refs
                                                                                    [i_3 as usize]
                                                                                    .p
                                                                                    .p
                                                                                    .frame_hdr)
                                                                                    .is_null()
                                                                                {
                                                                                    dav1d_thread_picture_unref(
                                                                                        &mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).p,
                                                                                    );
                                                                                }
                                                                                dav1d_cdf_thread_unref(
                                                                                    &mut *((*c).cdf).as_mut_ptr().offset(i_3 as isize),
                                                                                );
                                                                                dav1d_ref_dec(
                                                                                    &mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).segmap,
                                                                                );
                                                                                dav1d_ref_dec(
                                                                                    &mut (*((*c).refs).as_mut_ptr().offset(i_3 as isize)).refmvs,
                                                                                );
                                                                            }
                                                                            i_3 += 1;
                                                                        }
                                                                        current_block =
                                                                            9123693364129885070;
                                                                    } else {
                                                                        current_block =
                                                                            8115217508953982058;
                                                                    }
                                                                } else {
                                                                    dav1d_task_frame_init(f);
                                                                    pthread_mutex_unlock(
                                                                        &mut (*c).task_thread.lock,
                                                                    );
                                                                    current_block =
                                                                        8115217508953982058;
                                                                }
                                                                match current_block {
                                                                    9123693364129885070 => {}
                                                                    _ => return 0 as libc::c_int,
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    *&mut (*f).task_thread.error = 1 as libc::c_int;
    dav1d_cdf_thread_unref(&mut (*f).in_cdf);
    if (*(*f).frame_hdr).refresh_context != 0 {
        dav1d_cdf_thread_unref(&mut (*f).out_cdf);
    }
    let mut i_4 = 0;
    while i_4 < 7 {
        if !((*f).refp[i_4 as usize].p.frame_hdr).is_null() {
            dav1d_thread_picture_unref(&mut *((*f).refp).as_mut_ptr().offset(i_4 as isize));
        }
        dav1d_ref_dec(&mut *((*f).ref_mvs_ref).as_mut_ptr().offset(i_4 as isize));
        i_4 += 1;
    }
    if (*c).n_fc == 1 as libc::c_uint {
        dav1d_thread_picture_unref(&mut (*c).out);
    } else {
        dav1d_thread_picture_unref(out_delayed);
    }
    dav1d_picture_unref_internal(&mut (*f).cur);
    dav1d_thread_picture_unref(&mut (*f).sr_cur);
    dav1d_ref_dec(&mut (*f).mvs_ref);
    dav1d_ref_dec(&mut (*f).seq_hdr_ref);
    dav1d_ref_dec(&mut (*f).frame_hdr_ref);
    dav1d_data_props_copy(&mut (*c).cached_error_props, &mut (*c).in_0.m);
    let mut i_5 = 0;
    while i_5 < (*f).n_tile_data {
        dav1d_data_unref_internal(&mut (*((*f).tile).offset(i_5 as isize)).data);
        i_5 += 1;
    }
    (*f).n_tile_data = 0 as libc::c_int;
    if (*c).n_fc > 1 as libc::c_uint {
        pthread_mutex_unlock(&mut (*c).task_thread.lock);
    }
    return res;
}
