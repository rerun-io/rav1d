; Copyright © 2020, VideoLAN and dav1d authors
; Copyright © 2020, Two Orioles, LLC
; All rights reserved.
;
; Redistribution and use in source and binary forms, with or without
; modification, are permitted provided that the following conditions are met:
;
; 1. Redistributions of source code must retain the above copyright notice, this
;    list of conditions and the following disclaimer.
;
; 2. Redistributions in binary form must reproduce the above copyright notice,
;    this list of conditions and the following disclaimer in the documentation
;    and/or other materials provided with the distribution.
;
; THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
; ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
; WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
; DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
; ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
; (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
; ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
; (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
; SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

%include "config.asm"
%include "ext/x86/x86inc.asm"

%if ARCH_X86_64

SECTION_RODATA 64

obmc_masks:
pw_512:         times 2 dw 512
                ; 2
                db 45, 19, 64,  0
                ; 4
                db 39, 25, 50, 14, 59,  5, 64,  0
                ; 8
                db 36, 28, 42, 22, 48, 16, 53, 11, 57,  7, 61,  3, 64,  0, 64,  0
                ; 16
                db 34, 30, 37, 27, 40, 24, 43, 21, 46, 18, 49, 15, 52, 12, 54, 10
                db 56,  8, 58,  6, 60,  4, 61,  3, 64,  0, 64,  0, 64,  0, 64,  0
                ; 32
                db 33, 31, 35, 29, 36, 28, 38, 26, 40, 24, 41, 23, 43, 21, 44, 20
                db 45, 19, 47, 17, 48, 16, 50, 14, 51, 13, 52, 12, 53, 11, 55,  9
                db 56,  8, 57,  7, 58,  6, 59,  5, 60,  4, 60,  4, 61,  3, 62,  2
                db 64,  0, 64,  0, 64,  0, 64,  0, 64,  0, 64,  0, 64,  0, 64,  0

warp_8x8_permA: db  4,  5,  6,  7, 16, 17, 18, 19,  5,  6,  7,  8, 17, 18, 19, 20
                db  6,  7,  8,  9, 18, 19, 20, 21,  7,  8,  9, 10, 19, 20, 21, 22
                db  8,  9, 10, 11, 20, 21, 22, 23,  9, 10, 11, 12, 21, 22, 23, 24
                db 10, 11, 12, 13, 22, 23, 24, 25, 11, 12, 13, 14, 23, 24, 25, 26
warp_8x8_permB: db  0,  1,  2,  3, 20, 21, 22, 23,  1,  2,  3,  4, 21, 22, 23, 24
                db  2,  3,  4,  5, 22, 23, 24, 25,  3,  4,  5,  6, 23, 24, 25, 26
                db  4,  5,  6,  7, 24, 25, 26, 27,  5,  6,  7,  8, 25, 26, 27, 28
                db  6,  7,  8,  9, 26, 27, 28, 29,  7,  8,  9, 10, 27, 28, 29, 30
warp_8x8_permC: db -1,  0, -1,  1, -1,  8, -1,  9, -1,  4, -1,  5, -1, 12, -1, 13
warp_8x8_permD: db -1,  2, -1,  3, -1, 10, -1, 11, -1,  6, -1,  7, -1, 14, -1, 15
pd_0to7:        dd  0,  1,  2,  3,  4,  5,  6,  7
warp_8x8_hpack: db  3, 11,  3, 11, 35, 43, 35, 43
pd_16384:       dd 16384
pd_262144:      dd 262144
warp_8x8_end:   db  0,  4, 16, 20, 32, 36, 48, 52,  2,  6, 18, 22, 34, 38, 50, 54
warp_8x8t_end:  db  2,  3, 10, 11, 18, 19, 26, 27, 34, 35, 42, 43, 50, 51, 58, 59
                db  6,  7, 14, 15, 22, 23, 30, 31, 38, 39, 46, 47, 54, 55, 62, 63
bidir_sctr_w4:  dd  0,  1,  8,  9,  2,  3, 10, 11,  4,  5, 12, 13,  6,  7, 14, 15
wm_420_perm4:   db  1,  3,  9, 11,  5,  7, 13, 15, 17, 19, 25, 27, 21, 23, 29, 31
                db 33, 35, 41, 43, 37, 39, 45, 47, 49, 51, 57, 59, 53, 55, 61, 63
                db  0,  2,  8, 10,  4,  6, 12, 14, 16, 18, 24, 26, 20, 22, 28, 30
                db 32, 34, 40, 42, 36, 38, 44, 46, 48, 50, 56, 58, 52, 54, 60, 62
wm_420_perm8:   db  1,  3, 17, 19,  5,  7, 21, 23,  9, 11, 25, 27, 13, 15, 29, 31
                db 33, 35, 49, 51, 37, 39, 53, 55, 41, 43, 57, 59, 45, 47, 61, 63
                db  0,  2, 16, 18,  4,  6, 20, 22,  8, 10, 24, 26, 12, 14, 28, 30
                db 32, 34, 48, 50, 36, 38, 52, 54, 40, 42, 56, 58, 44, 46, 60, 62
wm_420_perm16:  db  1,  3, 33, 35,  5,  7, 37, 39,  9, 11, 41, 43, 13, 15, 45, 47
                db 17, 19, 49, 51, 21, 23, 53, 55, 25, 27, 57, 59, 29, 31, 61, 63
                db  0,  2, 32, 34,  4,  6, 36, 38,  8, 10, 40, 42, 12, 14, 44, 46
                db 16, 18, 48, 50, 20, 22, 52, 54, 24, 26, 56, 58, 28, 30, 60, 62
wm_420_mask:    db  3,  7, 11, 15, 19, 23, 27, 31, 35, 39, 43, 47, 51, 55, 59, 63
                db 67, 71, 75, 79, 83, 87, 91, 95, 99,103,107,111,115,119,123,127
                db  1,  5,  9, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61
                db 65, 69, 73, 77, 81, 85, 89, 93, 97,101,105,109,113,117,121,125
wm_422_mask:    db  2,  6, 10, 14, 18, 22, 26, 30, 34, 38, 42, 46, 50, 54, 58, 62
                db  1,  5,  9, 13, 17, 21, 25, 29, 33, 37, 41, 45, 49, 53, 57, 61
                db 66, 70, 74, 78, 82, 86, 90, 94, 98,102,106,110,114,118,122,126
                db 65, 69, 73, 77, 81, 85, 89, 93, 97,101,105,109,113,117,121,125
wm_444_mask:    db  1,  3,  5,  7,  9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31
                db 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63
                db  0,  2,  4,  6,  8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
                db 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62
bilin_h_perm16: db  0,  1,  1,  2,  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7,  8
                db  8,  9,  9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16
                db 32, 33, 33, 34, 34, 35, 35, 36, 36, 37, 37, 38, 38, 39, 39, 40
                db 40, 41, 41, 42, 42, 43, 43, 44, 44, 45, 45, 46, 46, 47, 47, 48
bilin_h_perm32: db  0,  1,  1,  2,  2,  3,  3,  4,  4,  5,  5,  6,  6,  7,  7,  8
                db  8,  9,  9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 16
                db 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24
                db 24, 25, 25, 26, 26, 27, 27, 28, 28, 29, 29, 30, 30, 31, 31, 32
bilin_v_perm8:  db  0, 16,  1, 17,  2, 18,  3, 19,  4, 20,  5, 21,  6, 22,  7, 23
                db 16, 80, 17, 81, 18, 82, 19, 83, 20, 84, 21, 85, 22, 86, 23, 87
                db 80, 32, 81, 33, 82, 34, 83, 35, 84, 36, 85, 37, 86, 38, 87, 39
                db 32, 64, 33, 65, 34, 66, 35, 67, 36, 68, 37, 69, 38, 70, 39, 71
bilin_v_perm16: db  0, 16,  1, 17,  2, 18,  3, 19,  4, 20,  5, 21,  6, 22,  7, 23
                db  8, 24,  9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31
                db 16, 64, 17, 65, 18, 66, 19, 67, 20, 68, 21, 69, 22, 70, 23, 71
                db 24, 72, 25, 73, 26, 74, 27, 75, 28, 76, 29, 77, 30, 78, 31, 79
bilin_v_perm32: db  0, 64,  1, 65,  2, 66,  3, 67,  4, 68,  5, 69,  6, 70,  7, 71
                db  8, 72,  9, 73, 10, 74, 11, 75, 12, 76, 13, 77, 14, 78, 15, 79
                db 16, 80, 17, 81, 18, 82, 19, 83, 20, 84, 21, 85, 22, 86, 23, 87
                db 24, 88, 25, 89, 26, 90, 27, 91, 28, 92, 29, 93, 30, 94, 31, 95
bilin_v_perm64: dd  0,  0,  4,  8,  1,  1,  5,  9,  2,  2,  6, 10,  3,  3,  7, 11
spel_h_perm16:  db  0,  1,  2,  3,  1,  2,  3,  4,  2,  3,  4,  5,  3,  4,  5,  6
                db  8,  9, 10, 11,  9, 10, 11, 12, 10, 11, 12, 13, 11, 12, 13, 14
                db 32, 33, 34, 35, 33, 34, 35, 36, 34, 35, 36, 37, 35, 36, 37, 38
                db 40, 41, 42, 43, 41, 42, 43, 44, 42, 43, 44, 45, 43, 44, 45, 46
spel_h_perm32:  db  0,  1,  2,  3,  1,  2,  3,  4,  2,  3,  4,  5,  3,  4,  5,  6
                db  8,  9, 10, 11,  9, 10, 11, 12, 10, 11, 12, 13, 11, 12, 13, 14
                db 16, 17, 18, 19, 17, 18, 19, 20, 18, 19, 20, 21, 19, 20, 21, 22
                db 24, 25, 26, 27, 25, 26, 27, 28, 26, 27, 28, 29, 27, 28, 29, 30
spel_v_perm8:   db  0,  8,  1,  9,  2, 10,  3, 11,  4, 12,  5, 13,  6, 14,  7, 15
                db  8, 16,  9, 17, 10, 18, 11, 19, 12, 20, 13, 21, 14, 22, 15, 23
                db 16, 24, 17, 25, 18, 26, 19, 27, 20, 28, 21, 29, 22, 30, 23, 31
                db 24, 32, 25, 33, 26, 34, 27, 35, 28, 36, 29, 37, 30, 38, 31, 39
spel_v_perm16a: db 32,  0, 33,  1, 34,  2, 35,  3, 36,  4, 37,  5, 38,  6, 39,  7
                db  0,  8,  1,  9,  2, 10,  3, 11,  4, 12,  5, 13,  6, 14,  7, 15
                db 40, 16, 41, 17, 42, 18, 43, 19, 44, 20, 45, 21, 46, 22, 47, 23
                db 16, 24, 17, 25, 18, 26, 19, 27, 20, 28, 21, 29, 22, 30, 23, 31
spel_v_perm16b: db 32,  0, 33,  1, 34,  2, 35,  3, 36,  4, 37,  5, 38,  6, 39,  7
                db 40, 16, 41, 17, 42, 18, 43, 19, 44, 20, 45, 21, 46, 22, 47, 23
                db  0,  8,  1,  9,  2, 10,  3, 11,  4, 12,  5, 13,  6, 14,  7, 15
                db 16, 24, 17, 25, 18, 26, 19, 27, 20, 28, 21, 29, 22, 30, 23, 31
spel_v_perm32:  db  0, 32,  1, 33,  2, 34,  3, 35,  4, 36,  5, 37,  6, 38,  7, 39
                db  8, 40,  9, 41, 10, 42, 11, 43, 12, 44, 13, 45, 14, 46, 15, 47
                db 16, 48, 17, 49, 18, 50, 19, 51, 20, 52, 21, 53, 22, 54, 23, 55
                db 24, 56, 25, 57, 26, 58, 27, 59, 28, 60, 29, 61, 30, 62, 31, 63
spel_hv_perm4a: db  8,  9, 16, 17, 10, 11, 18, 19, 12, 13, 20, 21, 14, 15, 22, 23
                db 16, 17, 24, 25, 18, 19, 26, 27, 20, 21, 28, 29, 22, 23, 30, 31
spel_hv_perm4b: db 24, 25, 32, 33, 26, 27, 34, 35, 28, 29, 36, 37, 30, 31, 38, 39
                db 32, 33, 40, 41, 34, 35, 42, 43, 36, 37, 44, 45, 38, 39, 46, 47
spel_hv_perm4c: db 40, 41, 48, 49, 42, 43, 50, 51, 44, 45, 52, 53, 46, 47, 54, 55
                db 48, 49, 56, 57, 50, 51, 58, 59, 52, 53, 60, 61, 54, 55, 62, 63
spel_hv_perm4d: db 18, 19,  0,  1, 22, 23,  4,  5, 26, 27,  8,  9, 30, 31, 12, 13
                db  0,  1, 16, 17,  4,  5, 20, 21,  8,  9, 24, 25, 12, 13, 28, 29
spel_hv_perm8a: db  0,  1, 16, 17,  2,  3, 18, 19,  4,  5, 20, 21,  6,  7, 22, 23
                db  8,  9, 24, 25, 10, 11, 26, 27, 12, 13, 28, 29, 14, 15, 30, 31
                db 16, 17, 32, 33, 18, 19, 34, 35, 20, 21, 36, 37, 22, 23, 38, 39
                db 24, 25, 40, 41, 26, 27, 42, 43, 28, 29, 44, 45, 30, 31, 46, 47
spel_hv_perm8b: db 34, 35,  0,  1, 38, 39,  4,  5, 42, 43,  8,  9, 46, 47, 12, 13
                db 50, 51, 16, 17, 54, 55, 20, 21, 58, 59, 24, 25, 62, 63, 28, 29
                db  0,  1, 32, 33,  4,  5, 36, 37,  8,  9, 40, 41, 12, 13, 44, 45
                db 16, 17, 48, 49, 20, 21, 52, 53, 24, 25, 56, 57, 28, 29, 60, 61
spel_hv_perm16a:db  0,  1,  2,  3, 32, 33, 34, 35,  1,  2,  3,  4, 33, 34, 35, 36
                db  2,  3,  4,  5, 34, 35, 36, 37,  3,  4,  5,  6, 35, 36, 37, 38
                db  8,  9, 10, 11, 40, 41, 42, 43,  9, 10, 11, 12, 41, 42, 43, 44
                db 10, 11, 12, 13, 42, 43, 44, 45, 11, 12, 13, 14, 43, 44, 45, 46
spel_hv_perm16b:db  0,  1,  2,  3,  1,  2,  3,  4,  4,  5,  6,  7,  5,  6,  7,  8
                db  2,  3,  4,  5,  3,  4,  5,  6,  6,  7,  8,  9,  7,  8,  9, 10
                db  8,  9, 10, 11,  9, 10, 11, 12, 12, 13, 14, 15, 13, 14, 15, 16
                db 10, 11, 12, 13, 11, 12, 13, 14, 14, 15, 16, 17, 15, 16, 17, 18
spel_hv_end16:  db  1,  3, 17, 19,  5,  7, 21, 23, 33, 35, 49, 51, 37, 39, 53, 55
                db  9, 11, 25, 27, 13, 15, 29, 31, 41, 43, 57, 59, 45, 47, 61, 63
spel_hv_end:    db  1,  3,  5,  7, 17, 19, 21, 23, 33, 35, 37, 39, 49, 51, 53, 55
deint_shuf4:    db  0,  4,  1,  5,  2,  6,  3,  7,  4,  8,  5,  9,  6, 10,  7, 11
subpel_h_shuf4: db  0,  1,  2,  3,  1,  2,  3,  4,  8,  9, 10, 11,  9, 10, 11, 12
                db  2,  3,  4,  5,  3,  4,  5,  6, 10, 11, 12, 13, 11, 12, 13, 14
subpel_h_shufA: db  0,  1,  2,  3,  1,  2,  3,  4,  2,  3,  4,  5,  3,  4,  5,  6
subpel_h_shufB: db  4,  5,  6,  7,  5,  6,  7,  8,  6,  7,  8,  9,  7,  8,  9, 10
subpel_h_shufC: db  8,  9, 10, 11,  9, 10, 11, 12, 10, 11, 12, 13, 11, 12, 13, 14
bilin_h_shuf4:  db  0,  1,  1,  2,  2,  3,  3,  4,  8,  9,  9, 10, 10, 11, 11, 12
bilin_v_shuf4:  db  0,  4,  1,  5,  2,  6,  3,  7,  4,  8,  5,  9,  6, 10,  7, 11
blend_shuf:     db  0,  1,  0,  1,  0,  1,  0,  1,  2,  3,  2,  3,  2,  3,  2,  3
rescale_mul:    dd  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15
resize_permA:   dd  0,  2,  4,  6,  8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30
resize_permB:   dd  1,  3,  5,  7,  9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31
resize_permC:   dd  0,  4,  8, 12
resize_shuf:    db  0,  0,  0,  0,  0,  1,  2,  3,  4,  5,  6,  7,  7,  7,  7,  7
pb_02461357:    db  0,  2,  4,  6,  1,  3,  5,  7

wm_420_perm64:  dq 0xfedcba9876543210
wm_sign:        dd 0x40804080, 0xc0c0c0c0, 0x40404040

pb_8x0_8x8: times 8 db 0
            times 8 db 8
pb_4:       times 4 db 4
pb_32:      times 4 db 32
pb_127:     times 4 db 127
pw_m128     times 2 dw -128
pw_m256:    times 2 dw -256
pw_1024:    times 2 dw 1024
pw_2048:    times 2 dw 2048
pw_6903:    times 2 dw 6903
pw_8192:    times 2 dw 8192
pd_32:              dd 32
pd_34:              dd 34
pd_63:              dd 63
pd_512:             dd 512

%define pb_m64 (wm_sign+4)
%define pb_64  (wm_sign+8)
%define pd_2   (pd_0to7+8)

cextern_pie mc_warp_filter,runtime_mc_warp_filter,0,"EXTERNAL"
%define mc_warp_filter runtime_mc_warp_filter

cextern_pie mc_subpel_filters,runtime_mc_subpel_filters,8,"EXTERNAL"
%define subpel_filters runtime_mc_subpel_filters

cextern_pie resize_filter,runtime_resize_filter,0,"EXTERNAL"
%define resize_filter runtime_resize_filter

%macro BASE_JMP_TABLE 3-*
    %xdefine %1_%2_table (%%table - %3)
    %xdefine %%base %1_%2
    %%table:
    %rep %0 - 2
        dw %%base %+ _w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%macro HV_JMP_TABLE 5-*
    %xdefine %%prefix mangle(private_prefix %+ _%1_%2_8bpc_%3)
    %xdefine %%base %1_%3
    %assign %%types %4
    %if %%types & 1
        %xdefine %1_%2_h_%3_table  (%%h  - %5)
        %%h:
        %rep %0 - 4
            dw %%prefix %+ .h_w%5 - %%base
            %rotate 1
        %endrep
        %rotate 4
    %endif
    %if %%types & 2
        %xdefine %1_%2_v_%3_table  (%%v  - %5)
        %%v:
        %rep %0 - 4
            dw %%prefix %+ .v_w%5 - %%base
            %rotate 1
        %endrep
        %rotate 4
    %endif
    %if %%types & 4
        %xdefine %1_%2_hv_%3_table (%%hv - %5)
        %%hv:
        %rep %0 - 4
            dw %%prefix %+ .hv_w%5 - %%base
            %rotate 1
        %endrep
    %endif
%endmacro

%macro BIDIR_JMP_TABLE 2-*
    %xdefine %1_%2_table (%%table - 2*%3)
    %xdefine %%base %1_%2_table
    %xdefine %%prefix mangle(private_prefix %+ _%1_8bpc_%2)
    %%table:
    %rep %0 - 2
        dd %%prefix %+ .w%3 - %%base
        %rotate 1
    %endrep
%endmacro

%xdefine put_avx512icl mangle(private_prefix %+ _put_bilin_8bpc_avx512icl.put)
%xdefine prep_avx512icl mangle(private_prefix %+ _prep_bilin_8bpc_avx512icl.prep)

%define table_offset(type, fn) type %+ fn %+ SUFFIX %+ _table - type %+ SUFFIX

BASE_JMP_TABLE put,  avx512icl,         2, 4, 8, 16, 32, 64, 128
BASE_JMP_TABLE prep, avx512icl,            4, 8, 16, 32, 64, 128
HV_JMP_TABLE put,  bilin, avx512icl, 7, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE prep, bilin, avx512icl, 7,    4, 8, 16, 32, 64, 128
HV_JMP_TABLE put,  6tap,  avx512icl, 2, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE put,  8tap,  avx512icl, 3, 2, 4, 8, 16, 32, 64, 128
HV_JMP_TABLE prep, 6tap,  avx512icl, 2,    4, 8, 16, 32, 64, 128
HV_JMP_TABLE prep, 8tap,  avx512icl, 3,    4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE avg, avx512icl,            4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_avg, avx512icl,          4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE mask, avx512icl,           4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_420, avx512icl,     4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_422, avx512icl,     4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE w_mask_444, avx512icl,     4, 8, 16, 32, 64, 128
BIDIR_JMP_TABLE blend, avx512icl,          4, 8, 16, 32
BIDIR_JMP_TABLE blend_v, avx512icl,     2, 4, 8, 16, 32
BIDIR_JMP_TABLE blend_h, avx512icl,     2, 4, 8, 16, 32, 64, 128

SECTION .text

%macro WRAP_YMM 1+
INIT_YMM cpuname
    %1
INIT_ZMM cpuname
%endmacro

INIT_ZMM avx512icl
cglobal put_bilin_8bpc, 4, 8, 0, dst, ds, src, ss, w, h, mxy
    movifnidn          mxyd, r6m ; mx
    lea                  r7, [put_avx512icl]
    tzcnt                wd, wm
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r7m ; my
    test               mxyd, mxyd
    jnz .v
.put:
    movzx                wd, word [r7+wq*2+table_offset(put,)]
    add                  wq, r7
    jmp                  wq
.put_w2:
    movzx               r6d, word [srcq+ssq*0]
    movzx               r7d, word [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6w
    mov        [dstq+dsq*1], r7w
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w2
    RET
.put_w4:
    mov                 r6d, [srcq+ssq*0]
    mov                 r7d, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6d
    mov        [dstq+dsq*1], r7d
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w4
    RET
.put_w8:
    mov                  r6, [srcq+ssq*0]
    mov                  r7, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mov        [dstq+dsq*0], r6
    mov        [dstq+dsq*1], r7
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w8
    RET
.put_w16:
    movu               xmm0, [srcq+ssq*0]
    movu               xmm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], xmm0
    mova       [dstq+dsq*1], xmm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w16
    RET
.put_w32:
    movu                ym0, [srcq+ssq*0]
    movu                ym1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], ym0
    mova       [dstq+dsq*1], ym1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w32
    RET
.put_w64:
    movu                 m0, [srcq+ssq*0]
    movu                 m1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    mova       [dstq+dsq*0], m0
    mova       [dstq+dsq*1], m1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w64
    RET
.put_w128:
    movu                 m0, [srcq+ssq*0+64*0]
    movu                 m1, [srcq+ssq*0+64*1]
    movu                 m2, [srcq+ssq*1+64*0]
    movu                 m3, [srcq+ssq*1+64*1]
    lea                srcq, [srcq+ssq*2]
    mova  [dstq+dsq*0+64*0], m0
    mova  [dstq+dsq*0+64*1], m1
    mova  [dstq+dsq*1+64*0], m2
    mova  [dstq+dsq*1+64*1], m3
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .put_w128
    RET
.h:
    ; (16 * src[x] + (mx * (src[x + 1] - src[x])) + 8) >> 4
    ; = ((16 - mx) * src[x] + mx * src[x + 1] + 8) >> 4
    imul               mxyd, 255
    vbroadcasti128       m4, [bilin_h_perm16]
    add                mxyd, 16
    vpbroadcastw         m5, mxyd
    mov                mxyd, r7m ; my
    test               mxyd, mxyd
    jnz .hv
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_h)]
    vpbroadcastd         m3, [pw_2048]
    add                  wq, r7
    jmp                  wq
.h_w2:
    movd               xmm0, [srcq+ssq*0]
    pinsrd             xmm0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pshufb             xmm0, xm4
    pmaddubsw          xmm0, xm5
    pmulhrsw           xmm0, xm3
    packuswb           xmm0, xmm0
    pextrw     [dstq+dsq*0], xmm0, 0
    pextrw     [dstq+dsq*1], xmm0, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2
    RET
.h_w4:
    mova               xmm4, [bilin_h_shuf4]
.h_w4_loop:
    movq               xmm0, [srcq+ssq*0]
    movhps             xmm0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb             xmm0, xmm4
    pmaddubsw          xmm0, xm5
    pmulhrsw           xmm0, xm3
    packuswb           xmm0, xmm0
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4_loop
    RET
.h_w8:
    movu                xm0, [srcq+ssq*0]
    vinserti32x4        ym0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pshufb              ym0, ym4
    pmaddubsw           ym0, ym5
    pmulhrsw            ym0, ym3
    vpmovuswb           xm0, ym0
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    mova                 m4, [bilin_h_perm16]
.h_w16_loop:
    movu                ym0, [srcq+ssq*0]
    vinserti32x8         m0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    vpermb               m0, m4, m0
    pmaddubsw            m0, m5
    pmulhrsw             m0, m3
    vpmovuswb           ym0, m0
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], ym0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w16_loop
    RET
.h_w32:
    movu                ym0, [srcq+ssq*0+8*0]
    vinserti32x8         m0, [srcq+ssq*1+8*0], 1
    movu                ym1, [srcq+ssq*0+8*1]
    vinserti32x8         m1, [srcq+ssq*1+8*1], 1
    lea                srcq, [srcq+ssq*2]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w32
    RET
.h_w64:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmulhrsw             m0, m3
    pmulhrsw             m1, m3
    packuswb             m0, m1
    add                srcq, ssq
    mova             [dstq], m0
    add                dstq, dsq
    dec                  hd
    jg .h_w64
    RET
.h_w128:
    movu                 m0, [srcq+8*0]
    movu                 m2, [srcq+8*1]
    movu                 m1, [srcq+8*8]
    movu                 m6, [srcq+8*9]
    add                srcq, ssq
    REPX  {pshufb    x, m4}, m0, m2, m1, m6
    REPX  {pmaddubsw x, m5}, m0, m2, m1, m6
    REPX  {pmulhrsw  x, m3}, m0, m2, m1, m6
    packuswb             m0, m2
    packuswb             m1, m6
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    add                dstq, dsq
    dec                  hd
    jg .h_w128
    RET
.v:
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_v)]
    imul               mxyd, 255
    vpbroadcastd         m5, [pw_2048]
    add                mxyd, 16
    add                  wq, r7
    vpbroadcastw         m4, mxyd
    jmp                  wq
.v_w2:
    movd               xmm0,       [srcq+ssq*0]
.v_w2_loop:
    pinsrw             xmm1, xmm0, [srcq+ssq*1], 1 ; 0 1
    lea                srcq,       [srcq+ssq*2]
    pinsrw             xmm0, xmm1, [srcq+ssq*0], 0 ; 2 1
    pshuflw            xmm1, xmm1, q2301           ; 1 0
    punpcklbw          xmm1, xmm0
    pmaddubsw          xmm1, xm4
    pmulhrsw           xmm1, xm5
    packuswb           xmm1, xmm1
    pextrw     [dstq+dsq*0], xmm1, 1
    pextrw     [dstq+dsq*1], xmm1, 0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movd               xmm0, [srcq+ssq*0]
.v_w4_loop:
    vpbroadcastd       xmm2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd           xmm1, xmm2, xmm0, 0x01 ; 0 1
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm2, xmm0, 0x02       ; 1 2
    punpcklbw          xmm1, xmm2
    pmaddubsw          xmm1, xm4
    pmulhrsw           xmm1, xm5
    packuswb           xmm1, xmm1
    movd       [dstq+dsq*0], xmm1
    pextrd     [dstq+dsq*1], xmm1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movq               xmm0, [srcq+ssq*0]
.v_w8_loop:
    movq               xmm2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklbw          xmm1, xmm0, xmm2
    movq               xmm0, [srcq+ssq*0]
    punpcklbw          xmm2, xmm0
    pmaddubsw          xmm1, xm4
    pmaddubsw          xmm2, xm4
    pmulhrsw           xmm1, xm5
    pmulhrsw           xmm2, xm5
    packuswb           xmm1, xmm2
    movq       [dstq+dsq*0], xmm1
    movhps     [dstq+dsq*1], xmm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    RET
.v_w16:
    movu               xmm0, [srcq+ssq*0]
.v_w16_loop:
    vbroadcasti128     ymm3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd           ymm2, ymm3, ymm0, 0x0f ; 0 1
    vbroadcasti128     ymm0, [srcq+ssq*0]
    vpblendd           ymm3, ymm0, 0xf0       ; 1 2
    punpcklbw          ymm1, ymm2, ymm3
    punpckhbw          ymm2, ymm3
    pmaddubsw          ymm1, ym4
    pmaddubsw          ymm2, ym4
    pmulhrsw           ymm1, ym5
    pmulhrsw           ymm2, ym5
    packuswb           ymm1, ymm2
    mova         [dstq+dsq*0], xmm1
    vextracti128 [dstq+dsq*1], ymm1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w16_loop
    vzeroupper
    RET
.v_w32:
    movu                ym0, [srcq+ssq*0]
    kxnorb               k1, k1, k1
.v_w32_loop:
    vbroadcasti32x8      m3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendmd        m2{k1}, m3, m0 ; 0 1
    vbroadcasti32x8      m0, [srcq+ssq*0]
    vpblendmd        m3{k1}, m0, m3 ; 1 2
    punpcklbw            m1, m2, m3
    punpckhbw            m2, m3
    pmaddubsw            m1, m4
    pmaddubsw            m2, m4
    pmulhrsw             m1, m5
    pmulhrsw             m2, m5
    packuswb             m1, m2
    mova          [dstq+dsq*0], ym1
    vextracti32x8 [dstq+dsq*1], m1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w32_loop
    RET
.v_w64:
    movu                 m0, [srcq+ssq*0]
.v_w64_loop:
    movu                 m3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklbw            m1, m0, m3
    punpckhbw            m6, m0, m3
    movu                 m0, [srcq+ssq*0]
    pmaddubsw            m1, m4
    pmaddubsw            m6, m4
    punpcklbw            m2, m3, m0
    punpckhbw            m3, m0
    pmaddubsw            m2, m4
    pmaddubsw            m3, m4
    REPX   {pmulhrsw x, m5}, m1, m6, m2, m3
    packuswb             m1, m6
    packuswb             m2, m3
    mova       [dstq+dsq*0], m1
    mova       [dstq+dsq*1], m2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w64_loop
    RET
.v_w128:
    movu                 m0, [srcq+64*0]
    movu                 m1, [srcq+64*1]
.v_w128_loop:
    add                srcq, ssq
    movu                 m2, [srcq+64*0]
    movu                 m3, [srcq+64*1]
    punpcklbw            m6, m0, m2
    pmaddubsw            m6, m4
    punpckhbw            m0, m2
    pmaddubsw            m0, m4
    punpcklbw            m7, m1, m3
    pmaddubsw            m7, m4
    punpckhbw            m1, m3
    pmaddubsw            m1, m4
    REPX   {pmulhrsw x, m5}, m6, m0, m7, m1
    packuswb             m6, m0
    mova                 m0, m2
    packuswb             m7, m1
    mova                 m1, m3
    mova        [dstq+64*0], m6
    mova        [dstq+64*1], m7
    add                dstq, dsq
    dec                  hd
    jg .v_w128_loop
    RET
.hv:
    ; (16 * src[x] + (my * (src[x + src_stride] - src[x])) + 128) >> 8
    ; = (src[x] + ((my * (src[x + src_stride] - src[x])) >> 4) + 8) >> 4
    movzx                wd, word [r7+wq*2+table_offset(put, _bilin_hv)]
    WIN64_SPILL_XMM       8
    shl                mxyd, 11 ; can't shift by 12 due to signed overflow
    vpbroadcastd         m7, [pw_2048]
    add                  wq, r7
    vpbroadcastw         m6, mxyd
    jmp                  wq
.hv_w2:
    vpbroadcastd       xmm0, [srcq+ssq*0]
    pshufb             xmm0, xm4
    pmaddubsw          xmm0, xm5
.hv_w2_loop:
    movd               xmm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pinsrd             xmm1, [srcq+ssq*0], 1
    pshufb             xmm1, xm4
    pmaddubsw          xmm1, xm5               ; 1 _ 2 _
    shufps             xmm2, xmm0, xmm1, q1032 ; 0 _ 1 _
    mova               xmm0, xmm1
    psubw              xmm1, xmm2
    paddw              xmm1, xmm1
    pmulhw             xmm1, xm6
    paddw              xmm1, xmm2
    pmulhrsw           xmm1, xm7
    packuswb           xmm1, xmm1
    pextrw     [dstq+dsq*0], xmm1, 0
    pextrw     [dstq+dsq*1], xmm1, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    mova               xmm4, [bilin_h_shuf4]
    movddup            xmm0, [srcq+ssq*0]
    pshufb             xmm0, xmm4
    pmaddubsw          xmm0, xm5
.hv_w4_loop:
    movq               xmm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps             xmm1, [srcq+ssq*0]
    pshufb             xmm1, xmm4
    pmaddubsw          xmm1, xm5               ; 1 2
    shufps             xmm2, xmm0, xmm1, q1032 ; 0 1
    mova               xmm0, xmm1
    psubw              xmm1, xmm2
    paddw              xmm1, xmm1
    pmulhw             xmm1, xm6
    paddw              xmm1, xmm2
    pmulhrsw           xmm1, xm7
    packuswb           xmm1, xmm1
    movd       [dstq+dsq*0], xmm1
    pextrd     [dstq+dsq*1], xmm1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    vbroadcasti128      ym0, [srcq+ssq*0]
    pshufb              ym0, ym4
    pmaddubsw           ym0, ym5
.hv_w8_loop:
    movu                xm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti128         ym1, [srcq+ssq*0], 1
    pshufb              ym1, ym4
    pmaddubsw           ym1, ym5            ; 1 2
    valignq             ym2, ym1, ym0, 2
    mova                ym0, ym1
    psubw               ym1, ym2
    paddw               ym1, ym1
    pmulhw              ym1, ym6
    paddw               ym1, ym2
    pmulhrsw            ym1, ym7
    vpmovuswb           xm1, ym1
    movq       [dstq+dsq*0], xm1
    movhps     [dstq+dsq*1], xm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w16:
    vbroadcasti32x8      m0, [srcq+ssq*0]
    mova                 m4, [bilin_h_perm16]
    vpermb               m0, m4, m0
    pmaddubsw            m0, m5
.hv_w16_loop:
    movu                ym1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti32x8         m1, [srcq+ssq*0], 1
    vpermb               m1, m4, m1
    pmaddubsw            m1, m5        ; 1 2
    valignq              m2, m1, m0, 4 ; 0 1
    mova                 m0, m1
    psubw                m1, m2
    paddw                m1, m1
    pmulhw               m1, m6
    paddw                m1, m2
    pmulhrsw             m1, m7
    vpmovuswb           ym1, m1
    mova          [dstq+dsq*0], xm1
    vextracti32x4 [dstq+dsq*1], ym1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    RET
.hv_w32:
    mova                 m4, [bilin_h_perm32]
    vpermb               m0, m4, [srcq+ssq*0]
    pmovzxbq             m8, [pb_02461357]
    pmaddubsw            m0, m5
.hv_w32_loop:
    vpermb               m2, m4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpermb               m3, m4, [srcq+ssq*0]
    pmaddubsw            m2, m5
    psubw                m1, m2, m0
    paddw                m1, m1
    pmulhw               m1, m6
    paddw                m1, m0
    pmaddubsw            m0, m3, m5
    psubw                m3, m0, m2
    paddw                m3, m3
    pmulhw               m3, m6
    paddw                m3, m2
    pmulhrsw             m1, m7
    pmulhrsw             m3, m7
    packuswb             m1, m3
    vpermq               m1, m8, m1
    mova          [dstq+dsq*0], ym1
    vextracti32x8 [dstq+dsq*1], m1, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w32_loop
    RET
.hv_w64:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    pshufb               m0, m4
    pshufb               m1, m4
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
.hv_w64_loop:
    add                srcq, ssq
    movu                 m2, [srcq+8*0]
    movu                 m3, [srcq+8*1]
    pshufb               m2, m4
    pshufb               m3, m4
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    psubw                m8, m2, m0
    psubw                m9, m3, m1
    paddw                m8, m8
    pmulhw               m8, m6
    paddw                m9, m9
    pmulhw               m9, m6
    paddw                m8, m0
    pmulhrsw             m8, m7
    paddw                m9, m1
    pmulhrsw             m9, m7
    mova                 m0, m2
    mova                 m1, m3
    packuswb             m8, m9
    mova             [dstq], m8
    add                dstq, dsq
    dec                  hd
    jg .hv_w64_loop
    RET
.hv_w128:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    movu                 m2, [srcq+8*8]
    movu                 m3, [srcq+8*9]
    REPX  {pshufb    x, m4}, m0, m1, m2, m3
    REPX  {pmaddubsw x, m5}, m0, m1, m2, m3
.hv_w128_loop:
    add                srcq, ssq
    movu                 m8, [srcq+8*0]
    movu                 m9, [srcq+8*1]
    movu                m10, [srcq+8*8]
    movu                m11, [srcq+8*9]
    REPX  {pshufb    x, m4}, m8, m9, m10, m11
    REPX  {pmaddubsw x, m5}, m8, m9, m10, m11
    psubw               m12, m8, m0
    psubw               m13, m9, m1
    psubw               m14, m10, m2
    psubw               m15, m11, m3
    paddw               m12, m12
    pmulhw              m12, m6
    paddw               m13, m13
    pmulhw              m13, m6
    paddw               m14, m14
    pmulhw              m14, m6
    paddw               m15, m15
    pmulhw              m15, m6
    paddw               m12, m0
    pmulhrsw            m12, m7
    paddw               m13, m1
    pmulhrsw            m13, m7
    paddw               m14, m2
    pmulhrsw            m14, m7
    paddw               m15, m3
    pmulhrsw            m15, m7
    mova                 m0, m8
    mova                 m1, m9
    mova                 m2, m10
    mova                 m3, m11
    packuswb            m12, m13
    packuswb            m14, m15
    mova        [dstq+64*0], m12
    mova        [dstq+64*1], m14
    add                dstq, dsq
    dec                  hd
    jg .hv_w128_loop
    RET

DECLARE_REG_TMP 3, 5, 6

cglobal prep_bilin_8bpc, 3, 7, 0, tmp, src, stride, w, h, mxy, stride3
    movifnidn          mxyd, r5m ; mx
    lea                  t2, [prep_avx512icl]
    tzcnt                wd, wm
    movifnidn            hd, hm
    test               mxyd, mxyd
    jnz .h
    mov                mxyd, r6m ; my
    test               mxyd, mxyd
    jnz .v
.prep:
    movzx                wd, word [t2+wq*2+table_offset(prep,)]
    add                  wq, t2
    lea            stride3q, [strideq*3]
    jmp                  wq
.prep_w4:
    movd               xmm0, [srcq+strideq*0]
    pinsrd             xmm0, [srcq+strideq*1], 1
    pinsrd             xmm0, [srcq+strideq*2], 2
    pinsrd             xmm0, [srcq+stride3q ], 3
    lea                srcq, [srcq+strideq*4]
    pmovzxbw            ym0, xmm0
    psllw               ym0, 4
    mova             [tmpq], ym0
    add                tmpq, 32
    sub                  hd, 4
    jg .prep_w4
    RET
.prep_w8:
    movq               xmm0, [srcq+strideq*0]
    movq               xmm1, [srcq+strideq*1]
    vinserti128         ym0, ymm0, [srcq+strideq*2], 1
    vinserti128         ym1, ymm1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    punpcklqdq          ym0, ym1
    pmovzxbw             m0, ym0
    psllw                m0, 4
    mova             [tmpq], m0
    add                tmpq, 32*2
    sub                  hd, 4
    jg .prep_w8
    RET
.prep_w16:
    movu               xmm0, [srcq+strideq*0]
    vinserti128         ym0, ymm0, [srcq+strideq*1], 1
    movu               xmm1, [srcq+strideq*2]
    vinserti128         ym1, ymm1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    pmovzxbw             m0, ym0
    pmovzxbw             m1, ym1
    psllw                m0, 4
    psllw                m1, 4
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    add                tmpq, 32*4
    sub                  hd, 4
    jg .prep_w16
    RET
.prep_w32:
    pmovzxbw             m0, [srcq+strideq*0]
    pmovzxbw             m1, [srcq+strideq*1]
    pmovzxbw             m2, [srcq+strideq*2]
    pmovzxbw             m3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    REPX       {psllw x, 4}, m0, m1, m2, m3
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    sub                  hd, 4
    jg .prep_w32
    RET
.prep_w64:
    pmovzxbw             m0, [srcq+strideq*0+32*0]
    pmovzxbw             m1, [srcq+strideq*0+32*1]
    pmovzxbw             m2, [srcq+strideq*1+32*0]
    pmovzxbw             m3, [srcq+strideq*1+32*1]
    lea                srcq, [srcq+strideq*2]
    REPX       {psllw x, 4}, m0, m1, m2, m3
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    sub                  hd, 2
    jg .prep_w64
    RET
.prep_w128:
    pmovzxbw             m0, [srcq+32*0]
    pmovzxbw             m1, [srcq+32*1]
    pmovzxbw             m2, [srcq+32*2]
    pmovzxbw             m3, [srcq+32*3]
    REPX       {psllw x, 4}, m0, m1, m2, m3
    mova    [tmpq+64*0], m0
    mova    [tmpq+64*1], m1
    mova    [tmpq+64*2], m2
    mova    [tmpq+64*3], m3
    add                tmpq, 64*4
    add                srcq, strideq
    dec                  hd
    jg .prep_w128
    RET
.h:
    ; 16 * src[x] + (mx * (src[x + 1] - src[x]))
    ; = (16 - mx) * src[x] + mx * src[x + 1]
    imul               mxyd, 255
    add                mxyd, 16
    vpbroadcastw         m5, mxyd
    mov                mxyd, r6m ; my
    test               mxyd, mxyd
    jnz .hv
    movzx                wd, word [t2+wq*2+table_offset(prep, _bilin_h)]
    add                  wq, t2
    lea            stride3q, [strideq*3]
    jmp                  wq
.h_w4:
    vbroadcasti32x4     ym4, [bilin_h_shuf4]
.h_w4_loop:
    movq               xmm0, [srcq+strideq*0]
    movq               xmm1, [srcq+strideq*1]
    vinserti32x4        ym0, ymm0, [srcq+strideq*2], 1
    vinserti32x4        ym1, ymm1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    punpcklqdq          ym0, ym1
    pshufb              ym0, ym4
    pmaddubsw           ym0, ym5
    mova             [tmpq], ym0
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4_loop
    RET
.h_w8:
    vbroadcasti32x4      m4, [bilin_h_perm16]
.h_w8_loop:
    movu               xmm0, [srcq+strideq*0]
    vinserti32x4        ym0, ymm0, [srcq+strideq*1], 1
    vinserti32x4         m0, [srcq+strideq*2], 2
    vinserti32x4         m0, [srcq+stride3q ], 3
    lea                srcq, [srcq+strideq*4]
    pshufb               m0, m4
    pmaddubsw            m0, m5
    mova             [tmpq], m0
    add                tmpq, 64
    sub                  hd, 4
    jg .h_w8_loop
    RET
.h_w16:
    mova                 m4, [bilin_h_perm16]
.h_w16_loop:
    movu                ym0, [srcq+strideq*0]
    vinserti32x8         m0, [srcq+strideq*1], 1
    movu                ym1, [srcq+strideq*2]
    vinserti32x8         m1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    vpermb               m0, m4, m0
    vpermb               m1, m4, m1
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    add                tmpq, 64*2
    sub                  hd, 4
    jg .h_w16_loop
    RET
.h_w32:
    mova                 m4, [bilin_h_perm32]
.h_w32_loop:
    vpermb               m0, m4, [srcq+strideq*0]
    vpermb               m1, m4, [srcq+strideq*1]
    vpermb               m2, m4, [srcq+strideq*2]
    vpermb               m3, m4, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    sub                  hd, 4
    jg .h_w32_loop
    RET
.h_w64:
    mova                 m4, [bilin_h_perm32]
.h_w64_loop:
    vpermb               m0, m4, [srcq+strideq*0+32*0]
    vpermb               m1, m4, [srcq+strideq*0+32*1]
    vpermb               m2, m4, [srcq+strideq*1+32*0]
    vpermb               m3, m4, [srcq+strideq*1+32*1]
    lea                srcq, [srcq+strideq*2]
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    sub                  hd, 2
    jg .h_w64_loop
    RET
.h_w128:
    mova                 m4, [bilin_h_perm32]
.h_w128_loop:
    vpermb               m0, m4, [srcq+32*0]
    vpermb               m1, m4, [srcq+32*1]
    vpermb               m2, m4, [srcq+32*2]
    vpermb               m3, m4, [srcq+32*3]
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
    mova        [tmpq+64*2], m2
    mova        [tmpq+64*3], m3
    add                tmpq, 64*4
    add                srcq, strideq
    dec                  hd
    jg .h_w128_loop
    RET
.v:
    WIN64_SPILL_XMM       7
    movzx                wd, word [t2+wq*2+table_offset(prep, _bilin_v)]
    imul               mxyd, 255
    add                mxyd, 16
    add                  wq, t2
    lea            stride3q, [strideq*3]
    vpbroadcastw         m6, mxyd
    jmp                  wq
.v_w4:
    vpbroadcastd        xm0, [srcq+strideq*0]
    mov                 r3d, 0x29
    vbroadcasti32x4     ym3, [bilin_v_shuf4]
    kmovb                k1, r3d
.v_w4_loop:
    vpblendmd       xm1{k1}, xm0, [srcq+strideq*1] {1to4} ; __01 ____
    vpbroadcastd        ym2, [srcq+strideq*2]
    vpbroadcastd    ym2{k1}, [srcq+stride3q ]             ; __2_ 23__
    lea                srcq, [srcq+strideq*4]
    vpbroadcastd        ym0, [srcq+strideq*0]
    punpckhqdq      ym2{k1}, ym1, ym0                     ; 012_ 234_
    pshufb              ym2, ym3
    pmaddubsw           ym2, ym6
    mova             [tmpq], ym2
    add                tmpq, 32
    sub                  hd, 4
    jg .v_w4_loop
    RET
.v_w8:
    mova                 m5, [bilin_v_perm8]
    vbroadcasti32x4     ym0, [srcq+strideq*0]
.v_w8_loop:
    vinserti32x4        ym1, ym0, [srcq+strideq*1], 1
    vpbroadcastq        ym0, [srcq+strideq*2]
    vinserti32x4         m1, [srcq+stride3q ], 2
    lea                srcq, [srcq+strideq*4]
    vinserti32x4        ym0, [srcq+strideq*0], 0
    vpermt2b             m1, m5, m0
    pmaddubsw            m1, m6
    mova             [tmpq], m1
    add                tmpq, 64
    sub                  hd, 4
    jg .v_w8_loop
    RET
.v_w16:
    mova                 m5, [bilin_v_perm16]
    movu                xm0, [srcq+strideq*0]
.v_w16_loop:
    movu                xm2, [srcq+strideq*2]
    vinserti32x4        ym1, ym0, [srcq+strideq*1], 1
    vpermt2b             m1, m5, m2
    vinserti32x4        ym2, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    movu                xm0, [srcq+strideq*0]
    vpermt2b             m2, m5, m0
    pmaddubsw            m1, m6
    pmaddubsw            m2, m6
    mova        [tmpq+64*0], m1
    mova        [tmpq+64*1], m2
    add                tmpq, 64*2
    sub                  hd, 4
    jg .v_w16_loop
    RET
.v_w32:
    mova                 m5, [bilin_v_perm32]
    movu                ym0, [srcq+strideq*0]
.v_w32_loop:
    movu                ym2, [srcq+strideq*1]
    movu                ym3, [srcq+strideq*2]
    movu                ym4, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpermt2b             m0, m5, m2
    vpermt2b             m2, m5, m3
    vpermt2b             m3, m5, m4
    pmaddubsw            m1, m0, m6
    movu                ym0, [srcq+strideq*0]
    vpermt2b             m4, m5, m0
    pmaddubsw            m2, m6
    pmaddubsw            m3, m6
    pmaddubsw            m4, m6
    mova        [tmpq+64*0], m1
    mova        [tmpq+64*1], m2
    mova        [tmpq+64*2], m3
    mova        [tmpq+64*3], m4
    add                tmpq, 64*4
    sub                  hd, 4
    jg .v_w32_loop
    RET
.v_w64:
    mova                 m5, [bilin_v_perm64]
    vpermq               m0, m5, [srcq+strideq*0]
.v_w64_loop:
    vpermq               m1, m5, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    punpcklbw            m4, m0, m1
    punpckhbw            m2, m0, m1
    vpermq               m0, m5, [srcq+strideq*0]
    punpcklbw            m3, m1, m0
    punpckhbw            m1, m0
    pmaddubsw            m4, m6
    pmaddubsw            m2, m6
    pmaddubsw            m3, m6
    pmaddubsw            m1, m6
    mova        [tmpq+64*0], m4
    mova        [tmpq+64*1], m2
    mova        [tmpq+64*2], m3
    mova        [tmpq+64*3], m1
    add                tmpq, 64*4
    sub                  hd, 2
    jg .v_w64_loop
    RET
.v_w128:
    mova                 m5, [bilin_v_perm64]
    vpermq               m0, m5, [srcq+strideq*0+ 0]
    vpermq               m1, m5, [srcq+strideq*0+64]
.v_w128_loop:
    vpermq               m2, m5, [srcq+strideq*1+ 0]
    vpermq               m3, m5, [srcq+strideq*1+64]
    lea                srcq, [srcq+strideq*2]
    punpcklbw            m4, m0, m2
    punpckhbw            m0, m2
    pmaddubsw            m4, m6
    pmaddubsw            m0, m6
    mova        [tmpq+64*0], m4
    mova        [tmpq+64*1], m0
    punpcklbw            m4, m1, m3
    punpckhbw            m1, m3
    pmaddubsw            m4, m6
    pmaddubsw            m1, m6
    mova        [tmpq+64*2], m4
    mova        [tmpq+64*3], m1
    vpermq               m0, m5, [srcq+strideq*0+ 0]
    vpermq               m1, m5, [srcq+strideq*0+64]
    punpcklbw            m4, m2, m0
    punpckhbw            m2, m0
    pmaddubsw            m4, m6
    pmaddubsw            m2, m6
    mova        [tmpq+64*4], m4
    mova        [tmpq+64*5], m2
    punpcklbw            m4, m3, m1
    punpckhbw            m3, m1
    pmaddubsw            m4, m6
    pmaddubsw            m3, m6
    mova        [tmpq+64*6], m4
    mova        [tmpq+64*7], m3
    add                tmpq, 64*8
    sub                  hd, 2
    jg .v_w128_loop
    RET
.hv:
    ; (16 * src[x] + (my * (src[x + src_stride] - src[x])) + 8) >> 4
    ; = src[x] + (((my * (src[x + src_stride] - src[x])) + 8) >> 4)
    WIN64_SPILL_XMM       7
    movzx                wd, word [t2+wq*2+table_offset(prep, _bilin_hv)]
    shl                mxyd, 11
    vpbroadcastw         m6, mxyd
    add                  wq, t2
    lea            stride3q, [strideq*3]
    jmp                  wq
.hv_w4:
    vbroadcasti32x4     ym4, [bilin_h_shuf4]
    vpbroadcastq        ym0, [srcq+strideq*0]
    pshufb              ym0, ym4
    pmaddubsw           ym0, ym5
.hv_w4_loop:
    movq               xmm1, [srcq+strideq*1]
    movq               xmm2, [srcq+strideq*2]
    vinserti32x4        ym1, ymm1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    vinserti32x4        ym2, ymm2, [srcq+strideq*0], 1
    punpcklqdq          ym1, ym2
    pshufb              ym1, ym4
    pmaddubsw           ym1, ym5         ; 1 2 3 4
    valignq             ym2, ym1, ym0, 3 ; 0 1 2 3
    mova                ym0, ym1
    psubw               ym1, ym2
    pmulhrsw            ym1, ym6
    paddw               ym1, ym2
    mova             [tmpq], ym1
    add                tmpq, 32
    sub                  hd, 4
    jg .hv_w4_loop
    RET
.hv_w8:
    vbroadcasti32x4      m4, [bilin_h_perm16]
    vbroadcasti32x4      m0, [srcq+strideq*0]
    pshufb               m0, m4
    pmaddubsw            m0, m5
.hv_w8_loop:
    movu               xmm1, [srcq+strideq*1]
    vinserti128         ym1, ymm1, [srcq+strideq*2], 1
    vinserti128          m1, [srcq+stride3q ], 2
    lea                srcq, [srcq+strideq*4]
    vinserti128          m1, [srcq+strideq*0], 3
    pshufb               m1, m4
    pmaddubsw            m1, m5        ; 1 2 3 4
    valignq              m2, m1, m0, 6 ; 0 1 2 3
    mova                 m0, m1
    psubw                m1, m2
    pmulhrsw             m1, m6
    paddw                m1, m2
    mova             [tmpq], m1
    add                tmpq, 64
    sub                  hd, 4
    jg .hv_w8_loop
    RET
.hv_w16:
    mova                 m4, [bilin_h_perm16]
    vbroadcasti32x8      m0, [srcq+strideq*0]
    vpermb               m0, m4, m0
    pmaddubsw            m0, m5
.hv_w16_loop:
    movu                ym1, [srcq+strideq*1]
    vinserti32x8         m1, [srcq+strideq*2], 1
    movu                ym2, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vinserti32x8         m2, [srcq+strideq*0], 1
    vpermb               m1, m4, m1
    vpermb               m2, m4, m2
    pmaddubsw            m1, m5            ; 1 2
    vshufi32x4           m3, m0, m1, q1032 ; 0 1
    pmaddubsw            m0, m2, m5        ; 3 4
    vshufi32x4           m2, m1, m0, q1032 ; 2 3
    psubw                m1, m3
    pmulhrsw             m1, m6
    paddw                m1, m3
    psubw                m3, m0, m2
    pmulhrsw             m3, m6
    paddw                m3, m2
    mova        [tmpq+64*0], m1
    mova        [tmpq+64*1], m3
    add                tmpq, 64*2
    sub                  hd, 4
    jg .hv_w16_loop
    RET
.hv_w32:
    mova                 m4, [bilin_h_perm32]
    vpermb               m0, m4, [srcq+strideq*0]
    pmaddubsw            m0, m5
.hv_w32_loop:
    vpermb               m1, m4, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    vpermb               m2, m4, [srcq+strideq*0]
    pmaddubsw            m1, m5
    psubw                m3, m1, m0
    pmulhrsw             m3, m6
    paddw                m3, m0
    pmaddubsw            m0, m2, m5
    psubw                m2, m0, m1
    pmulhrsw             m2, m6
    paddw                m2, m1
    mova        [tmpq+64*0], m3
    mova        [tmpq+64*1], m2
    add                tmpq, 64*2
    sub                  hd, 2
    jg .hv_w32_loop
    RET
.hv_w64:
    mova                 m4, [bilin_h_perm32]
    vpermb               m0, m4, [srcq+32*0]
    vpermb               m1, m4, [srcq+32*1]
    pmaddubsw            m0, m5
    pmaddubsw            m1, m5
.hv_w64_loop:
    add                srcq, strideq
    vpermb               m2, m4, [srcq+32*0]
    vpermb               m3, m4, [srcq+32*1]
    pmaddubsw            m2, m5
    pmaddubsw            m3, m5
    psubw                m7, m2, m0
    psubw                m8, m3, m1
    pmulhrsw             m7, m6
    pmulhrsw             m8, m6
    paddw                m7, m0
    mova                 m0, m2
    paddw                m8, m1
    mova                 m1, m3
    mova        [tmpq+64*0], m7
    mova        [tmpq+64*1], m8
    add                tmpq, 64*2
    dec                  hd
    jg .hv_w64_loop
    RET
.hv_w128:
    mova                 m4, [bilin_h_perm32]
    vpermb               m0, m4, [srcq+32*0]
    vpermb               m1, m4, [srcq+32*1]
    vpermb               m2, m4, [srcq+32*2]
    vpermb               m3, m4, [srcq+32*3]
    REPX  {pmaddubsw x, m5}, m0, m1, m2, m3
.hv_w128_loop:
    add                srcq, strideq
    vpermb               m7, m4, [srcq+32*0]
    vpermb               m8, m4, [srcq+32*1]
    vpermb               m9, m4, [srcq+32*2]
    vpermb              m10, m4, [srcq+32*3]
    REPX  {pmaddubsw x, m5}, m7, m8, m9, m10
    psubw               m11, m7, m0
    psubw               m12, m8, m1
    psubw               m13, m9, m2
    psubw               m14, m10, m3
    REPX  {pmulhrsw  x, m6}, m11, m12, m13, m14
    paddw               m11, m0
    mova                 m0, m7
    paddw               m12, m1
    mova                 m1, m8
    paddw               m13, m2
    mova                 m2, m9
    paddw               m14, m3
    mova                 m3, m10
    mova        [tmpq+64*0], m11
    mova        [tmpq+64*1], m12
    mova        [tmpq+64*2], m13
    mova        [tmpq+64*3], m14
    add                tmpq, 64*4
    dec                  hd
    jg .hv_w128_loop
    RET

; int8_t subpel_filters[5][15][8]
%assign FILTER_REGULAR (0*15 << 16) | 3*15
%assign FILTER_SMOOTH  (1*15 << 16) | 4*15
%assign FILTER_SHARP   (2*15 << 16) | 3*15

%macro FN 4-5 ; fn, type, type_h, type_v, jmp_to
cglobal %1_%2_8bpc
    mov                 t0d, FILTER_%3
%ifidn %3, %4
    mov                 t1d, t0d
%else
    mov                 t1d, FILTER_%4
%endif
%if %0 == 5 ; skip the jump in the last filter
    jmp mangle(private_prefix %+ _%5 %+ SUFFIX)
%endif
%endmacro

%macro PUT_8TAP_H 4-5 0 ; dst/src, tmp[1-3], vpermb
%if %5
    vpermb              m%2, m6, m%1
    vpermb              m%3, m7, m%1
    vpermb              m%4, m8, m%1
%else
%if %2 < %4 ; reuse a previous value if possible
    pshufb              m%2, m%1, m6
%endif
    pshufb              m%3, m%1, m7
    pshufb              m%4, m%1, m8
%endif
    mova                m%1, m5
    vpdpbusd            m%1, m%2, m9
    mova                m%2, m5
    vpdpbusd            m%2, m%3, m9
    vpdpbusd            m%1, m%3, m10
    vpdpbusd            m%2, m%4, m10
    packusdw            m%1, m%2
    psrlw               m%1, 6
%endmacro

%if WIN64
DECLARE_REG_TMP 4, 5
%else
DECLARE_REG_TMP 7, 8
%endif

; Due to the use of vpdpbusd (which does 4 pixels per instruction) in
; the horizontal filter, 6-tap is only used for the vertical filter.
%define PUT_8TAP_FN FN put_8tap,
PUT_8TAP_FN sharp_smooth,   SHARP,   SMOOTH,  put_6tap_8bpc
PUT_8TAP_FN sharp_regular,  SHARP,   REGULAR, put_6tap_8bpc
PUT_8TAP_FN smooth,         SMOOTH,  SMOOTH,  put_6tap_8bpc
PUT_8TAP_FN smooth_regular, SMOOTH,  REGULAR, put_6tap_8bpc
PUT_8TAP_FN regular_smooth, REGULAR, SMOOTH,  put_6tap_8bpc
PUT_8TAP_FN regular,        REGULAR, REGULAR

cglobal put_6tap_8bpc, 4, 9, 0, dst, ds, src, ss, w, h, mx, my, ns
%define base r8-put_avx512icl
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 6tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 6tap_v, my, 4tap_v
    lea                  r8, [put_avx512icl]
    movsxd               wq, wm
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
.put:
    tzcnt                wd, wd
    movzx                wd, word [r8+wq*2+table_offset(put,)]
    add                  wq, r8
    lea                  r6, [ssq*3]
    lea                  r7, [dsq*3]
%if WIN64
    pop                  r8
%endif
    jmp                  wq
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    tzcnt               r6d, wd
    movzx               r6d, word [r8+r6*2+table_offset(put, _6tap_v)]
    vpbroadcastd         m6, [pw_512]
    lea                 myq, [base+subpel_filters+1+myq*8]
    vpbroadcastw         m7, [myq+0]
    add                  r6, r8
    vpbroadcastw         m8, [myq+2]
    mov                 nsq, ssq
    vpbroadcastw         m9, [myq+4]
    neg                 nsq
    jmp                  r6
.v_w2:
    movd               xmm2, [srcq+nsq*2]
    pinsrw             xmm2, [srcq+nsq*1], 2
    pinsrw             xmm2, [srcq+ssq*0], 4
    pinsrw             xmm2, [srcq+ssq*1], 6  ; 0 1 2 3
    lea                srcq, [srcq+ssq*2]
    vpbroadcastd       xmm0, [srcq+ssq*0]
    palignr            xmm3, xmm0, xmm2, 4    ; 1 2 3 4
    punpcklbw          xmm1, xmm2, xmm3       ; 01 12
    punpckhbw          xmm2, xmm3             ; 23 34
.v_w2_loop:
    vpbroadcastd       xmm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw          xmm3, xmm1, xm7        ; a0 b0
    mova               xmm1, xmm2
    pmaddubsw          xmm2, xm8              ; a1 b1
    paddw              xmm3, xmm2
    vpblendd           xmm2, xmm0, xmm4, 0x02 ; 4 5
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm4, xmm0, 0x02       ; 5 6
    punpcklbw          xmm2, xmm4             ; 67 78
    pmaddubsw          xmm4, xmm2, xm9        ; a3 b3
    paddw              xmm3, xmm4
    pmulhrsw           xmm3, xm6
    packuswb           xmm3, xmm3
    pextrw     [dstq+dsq*0], xmm3, 0
    pextrw     [dstq+dsq*1], xmm3, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movd               xmm2, [srcq+nsq*2]
    pinsrd             xmm2, [srcq+nsq*1], 1
    pinsrd             xmm2, [srcq+ssq*0], 2
    pinsrd             xmm2, [srcq+ssq*1], 3  ; 0 1 2 3
    lea                srcq, [srcq+ssq*2]
    vpbroadcastd       xmm0, [srcq+ssq*0]
    palignr            xmm3, xmm0, xmm2, 4    ; 1 2 3 4
    punpcklbw          xmm1, xmm2, xmm3       ; 01 12
    punpckhbw          xmm2, xmm3             ; 23 34
.v_w4_loop:
    vpbroadcastd       xmm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw          xmm3, xmm1, xm7        ; a0 b0
    mova               xmm1, xmm2
    pmaddubsw          xmm2, xm8              ; a1 b1
    paddw              xmm3, xmm2
    vpblendd           xmm2, xmm0, xmm4, 0x02 ; 4 5
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm4, xmm0, 0x02       ; 5 6
    punpcklbw          xmm2, xmm4             ; 45 56
    pmaddubsw          xmm4, xmm2, xm9        ; a2 b2
    paddw              xmm3, xmm4
    pmulhrsw           xmm3, xm6
    packuswb           xmm3, xmm3
    movd       [dstq+dsq*0], xmm3
    pextrd     [dstq+dsq*1], xmm3, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movq               xmm1, [srcq+nsq*2]
    vpbroadcastq       ymm3, [srcq+nsq*1]
    vpbroadcastq       ymm2, [srcq+ssq*0]
    vpbroadcastq       ymm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpbroadcastq       ymm0, [srcq+ssq*0]
    vpblendd           ymm1, ymm3, 0x30
    vpblendd           ymm3, ymm2, 0x30
    punpcklbw          ymm1, ymm3      ; 01 12
    vpblendd           ymm2, ymm4, 0x30
    vpblendd           ymm4, ymm0, 0x30
    punpcklbw          ymm2, ymm4      ; 23 34
.v_w8_loop:
    vpbroadcastq       ymm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw          ymm3, ymm1, ym7 ; a0 b0
    mova               ymm1, ymm2
    pmaddubsw          ymm2, ym8       ; a1 b1
    paddw              ymm3, ymm2
    vpblendd           ymm2, ymm0, ymm4, 0x30
    vpbroadcastq       ymm0, [srcq+ssq*0]
    vpblendd           ymm4, ymm0, 0x30
    punpcklbw          ymm2, ymm4      ; 45 56
    pmaddubsw          ymm4, ymm2, ym9 ; a2 b2
    paddw              ymm3, ymm4
    pmulhrsw           ymm3, ym6
    vextracti128       xmm4, ymm3, 1
    packuswb           xmm3, xmm4
    movq       [dstq+dsq*0], xmm3
    movhps     [dstq+dsq*1], xmm3
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    vzeroupper
    RET
.v_w16:
    mova                 m5, [spel_v_perm16a]
    vbroadcasti32x4      m1, [srcq+nsq*2]
    vbroadcasti32x4     ym3, [srcq+nsq*1]
    mov                 r6d, 0x0f
    vbroadcasti32x4      m2, [srcq+ssq*0]
    kmovb                k1, r6d
    vbroadcasti32x4     ym4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vbroadcasti32x4      m0, [srcq+ssq*0]
    vshufpd          m1{k1}, m3, m2, 0xcc
    vshufpd          m2{k1}, m4, m0, 0xcc
    vpermb               m1, m5, m1 ; 01 12
    vpermb               m2, m5, m2 ; 23 34
.v_w16_loop:
    vbroadcasti32x4     ym4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw            m3, m1, m7 ; a0 b0
    mova                 m1, m2
    pmaddubsw            m2, m8     ; a1 b1
    paddw                m3, m2
    mova                 m2, m0
    vbroadcasti32x4      m0, [srcq+ssq*0]
    vshufpd          m2{k1}, m4, m0, 0xcc
    vpermb               m2, m5, m2 ; 45 56
    pmaddubsw            m4, m2, m9 ; a2 b2
    paddw                m3, m4
    pmulhrsw             m3, m6
    vextracti32x8       ym4, m3, 1
    packuswb            ym3, ym4
    mova          [dstq+dsq*0], xm3
    vextracti32x4 [dstq+dsq*1], ym3, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w16_loop
    RET
.v_w32:
    mova                m10, [spel_v_perm32]
    pmovzxbq             m5, [pb_02461357]
    vpshrdw             m11, m10, m10, 8
    movu                ym0, [srcq+nsq*2]
    vinserti32x8         m0, [srcq+nsq*1], 1
    vpermb               m1, m10, m0 ; 01
    vinserti32x8         m0, [srcq+ssq*0], 0
    vpermb               m2, m11, m0 ; 12
    vinserti32x8         m0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    vpermb               m3, m10, m0 ; 23
    vinserti32x8         m0, [srcq+ssq*0], 0
    vpermb               m4, m11, m0 ; 34
.v_w32_loop:
    vinserti32x8         m0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pmaddubsw           m12, m1, m7
    mova                 m1, m3
    pmaddubsw           m13, m2, m7
    mova                 m2, m4
    pmaddubsw           m14, m3, m8
    vpermb               m3, m10, m0 ; 45
    vinserti32x8         m0, [srcq+ssq*0], 0
    pmaddubsw           m15, m4, m8
    vpermb               m4, m11, m0 ; 56
    paddw               m12, m14
    pmaddubsw           m14, m3, m9
    paddw               m13, m15
    pmaddubsw           m15, m4, m9
    paddw               m12, m14
    paddw               m13, m15
    pmulhrsw            m12, m6
    pmulhrsw            m13, m6
    packuswb            m12, m13
    vpermq              m12, m5, m12
    mova          [dstq+dsq*0], ym12
    vextracti32x8 [dstq+dsq*1], m12, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w32_loop
    RET
.v_w64:
.v_w128:
    lea                 r6d, [hq+wq*4-256]
.v_loop0:
    movu                 m2, [srcq+nsq*2]
    movu                 m4, [srcq+nsq*1]
    lea                  r4, [srcq+ssq*2]
    movu                m11, [srcq+ssq*0]
    movu                m13, [srcq+ssq*1]
    mov                  r7, dstq
    movu                 m0, [r4  +ssq*0]
    punpcklbw            m1, m2, m4   ; 01l
    punpckhbw            m2, m4       ; 01h
    punpcklbw            m3, m4, m11  ; 12l
    punpckhbw            m4, m11      ; 12h
    punpcklbw           m10, m11, m13 ; 23l
    punpckhbw           m11, m13      ; 23h
    punpcklbw           m12, m13, m0  ; 34l
    punpckhbw           m13, m0       ; 34h
.v_loop:
    movu                 m5, [r4+ssq*1]
    pmaddubsw           m14, m1, m7   ; a0l
    mova                 m1, m10
    pmaddubsw           m10, m8       ; a1l
    lea                  r4, [r4+ssq*2]
    pmaddubsw           m15, m2, m7   ; a0h
    mova                 m2, m11
    pmaddubsw           m11, m8       ; a1h
    paddw               m14, m10
    punpcklbw           m10, m0, m5   ; 45l
    paddw               m15, m11
    punpckhbw           m11, m0, m5   ; 45h
    pmaddubsw            m0, m10, m9  ; a2l
    paddw               m14, m0
    pmaddubsw            m0, m11, m9  ; a2h
    paddw               m15, m0
    movu                 m0, [r4+ssq*0]
    pmulhrsw            m14, m6
    pmulhrsw            m15, m6
    packuswb            m14, m15
    pmaddubsw           m15, m3, m7   ; b0l
    mova                 m3, m12
    pmaddubsw           m12, m8       ; b1l
    mova         [r7+dsq*0], m14
    pmaddubsw           m14, m4, m7   ; b0h
    mova                 m4, m13
    pmaddubsw           m13, m8       ; b1h
    paddw               m15, m12
    punpcklbw           m12, m5, m0   ; 56l
    paddw               m14, m13
    punpckhbw           m13, m5, m0   ; 56h
    pmaddubsw            m5, m12, m9  ; b2l
    paddw               m15, m5
    pmaddubsw            m5, m13, m9  ; b2h
    paddw               m14, m5
    pmulhrsw            m15, m6
    pmulhrsw            m14, m6
    packuswb            m15, m14
    mova         [r7+dsq*1], m15
    lea                  r7, [r7+dsq*2]
    sub                  hd, 2
    jg .v_loop
    add                srcq, 64
    add                dstq, 64
    movzx                hd, r6b
    sub                 r6d, 256
    jg .v_loop0
    RET
.h:
    test                myd, 0xf00
    jz mangle(private_prefix %+ _put_8tap_8bpc_avx512icl).h2
.hv:
    vpbroadcastd         m9, [pd_34]
    mova               xm10, [spel_hv_end]
    pxor                xm0, xm0
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    dec                srcq
    vpbroadcastd         m7, [base+subpel_filters+mxq*8+2]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    vpbroadcastq        ym1, [base+subpel_filters+1+myq*8]
    mov                 nsq, ssq
    punpcklbw           ym0, ym1
    neg                 nsq
    psraw               ym0, 2 ; << 6
    pshufd             ym11, ym0, q0000
    pshufd             ym12, ym0, q1111
    pshufd             ym13, ym0, q2222
    cmp                  wd, 4
    je .hv_w4
    vbroadcasti128      ym5, [subpel_h_shuf4]
    movq               xmm0, [srcq+nsq*2]
    movhps             xmm0, [srcq+nsq*1]
    movq               xmm2, [srcq+ssq*0]
    movhps             xmm2, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpbroadcastq       ymm1, [srcq+ssq*0]
    vpblendd           ymm0, ymm1, 0x30
    pshufb             xmm2, xm5        ; 2 3
    pshufb             ymm0, ym5        ; 0 1   4
    mova               xmm1, xm9
    vpdpbusd           xmm1, xmm2, xm7
    mova               ymm2, ym9
    vpdpbusd           ymm2, ymm0, ym7
    packssdw           ymm2, ymm1
    psraw              ymm2, 2
    vextracti128       xmm0, ymm2, 1
    vzeroupper
    palignr            xmm0, xmm2, 4
    punpcklwd          xmm1, xmm2, xmm0 ; 01 12
    punpckhwd          xmm2, xmm0       ; 23 34
.hv_w2_loop:
    movq               xmm3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps             xmm3, [srcq+ssq*0]
    pmaddwd            xmm4, xmm1, xm11 ; a0 b0
    mova               xmm1, xmm2
    vpdpwssd           xmm4, xmm2, xm12 ; a1 b1
    pshufb             xmm3, xm5
    mova               xmm2, xm9
    vpdpbusd           xmm2, xmm3, xm7
    packssdw           xmm3, xmm2, xmm2
    psraw              xmm3, 2
    palignr            xmm2, xmm3, xmm0, 12
    mova               xmm0, xmm3
    punpcklwd          xmm2, xmm3       ; 45 56
    vpdpwssd           xmm4, xmm2, xm13 ; a2 b2
    packuswb           xmm4, xmm4
    pshufb             xmm4, xm10
    pextrw     [dstq+dsq*0], xmm4, 0
    pextrw     [dstq+dsq*1], xmm4, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    RET
.hv_w4:
    movq                xm2, [srcq+nsq*2]
    vpbroadcastq        ym1, [srcq+nsq*1]
    vinserti32x4        ym2, [srcq+ssq*0], 1
    vinserti32x4         m1, [srcq+ssq*1], 2 ; _ 1 3
    lea                srcq, [srcq+ssq*2]
    vbroadcasti32x4      m5, [subpel_h_shufA]
    vinserti32x4         m2, [srcq+ssq*0], 2 ; 0 2 4
    pshufb               m1, m5
    mova                 m0, m9
    pshufb               m2, m5
    mova                 m3, m9
    vpdpbusd             m0, m1, m7
    mova                ym1, [spel_hv_perm4a]
    vpdpbusd             m3, m2, m7
    mova                ym2, [spel_hv_perm4b]
    mov                 r6d, 0x5555
    mova                ym6, [spel_hv_perm4d]
    packssdw             m0, m3
    kmovw                k1, r6d
    psraw                m0, 2 ; _ 0   1 2   3 4   5 6
    vpermb              ym1, ym1, ym0 ; 01 12
    vpermb               m2, m2, m0   ; 23 34
.hv_w4_loop:
    movq                xm3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti32x4        ym3, [srcq+ssq*0], 1
    pmaddwd             ym4, ym1, ym11 ; a0 b0
    mova                ym1, ym2
    pshufb              ym3, ym5
    mova                ym0, ym9
    vpdpbusd            ym0, ym3, ym7
    vpdpwssd            ym4, ym2, ym12 ; a1 b1
    vpsraw          ym2{k1}, ym0, 2    ; 5 6
    vpermb              ym2, ym6, ym2  ; 45 56
    vpdpwssd            ym4, ym2, ym13 ; a2 b2
    packuswb            ym4, ym4
    vpermb              ym4, ym10, ym4
    movd       [dstq+dsq*0], xm4
    pextrd     [dstq+dsq*1], xm4, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    shr                 mxd, 16
    sub                srcq, 3
    vpbroadcastd        m11, [base+subpel_filters+mxq*8+0]
    vpbroadcastd        m12, [base+subpel_filters+mxq*8+4]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    vpbroadcastq         m1, [base+subpel_filters+1+myq*8]
    mov                 nsq, ssq
    punpcklbw            m0, m1
    neg                 nsq
    psraw                m0, 2 ; << 6
    pshufd              m13, m0, q0000
    pshufd              m14, m0, q1111
    pshufd              m15, m0, q2222
    cmp                  wd, 8
    jne .hv_w16
    movu                xm0, [srcq+nsq*2]
    vinserti32x4        ym0, [srcq+nsq*1], 1
    vbroadcasti32x4      m1, [subpel_h_shufA]
    vinserti32x4         m0, [srcq+ssq*0], 2
    vbroadcasti32x4      m4, [subpel_h_shufB]
    vinserti32x4         m0, [srcq+ssq*1], 3
    lea                srcq, [srcq+ssq*2]
    vbroadcasti32x4      m7, [subpel_h_shufC]
    vbroadcasti32x4     ym5, [srcq+ssq*0]
    vbroadcasti32x8      m6, [subpel_h_shufA]
    pshufb               m1, m0, m1   ; 0 1 2 3    0123
    mova                 m2, m9
    vpdpbusd             m2, m1, m11
    pshufb               m4, m0, m4   ; 0 1 2 3    4567
    mova                 m1, m9
    vpdpbusd             m1, m4, m11
    pshufb               m0, m7       ; 0 1 2 3    89ab
    pshufb              ym7, ym5, ym6 ; 4     0123 4567
    mova                ym3, ym9
    vpdpbusd            ym3, ym7, ym11
    vbroadcasti32x8      m7, [subpel_h_shufB]
    vpdpbusd             m2, m4, m12
    mova                 m4, [spel_hv_perm8a]
    pshufb              ym5, ym7      ; 4     4567 89ab
    vpdpbusd             m1, m0, m12
    vpaddd               m0, m4, [pb_32] {1to16}
    vpdpbusd            ym3, ym5, ym12
    mova                 m5, [spel_hv_perm8b]
    mov                  r6, 0x55555555ff00
    packssdw             m2, m1
    vpmovsdw            xm3, ym3
    kmovq                k1, r6
    psraw                m2, 2        ; 0 1 2 3
    psraw               xm3, 2        ; 4
    vpermb               m1, m4, m2   ; 01 12
    kshiftrq             k2, k1, 16
    vpermt2b             m2, m0, m3   ; 23 34
.hv_w8_loop:
    vbroadcasti32x4     ym3, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vbroadcasti32x4  m3{k1}, [srcq+ssq*0]
    pmaddwd              m0, m1, m13  ; a0 b0
    pshufb               m1, m3, m6   ; 5 6   0123 4567
    mova                 m4, m9
    vpdpbusd             m4, m1, m11
    pshufb               m3, m7       ; 5 6   4567 89ab
    vpdpwssd             m0, m2, m14  ; a1 b1
    mova                 m1, m2
    vpdpbusd             m4, m3, m12
    psraw            m2{k2}, m4, 2    ; 53 64
    vpermb               m2, m5, m2   ; 45 56
    vpdpwssd             m0, m2, m15  ; a2 b2
    packuswb             m0, m0
    vpermb               m0, m10, m0
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    RET
.hv_w16:
    movu                m19, [spel_hv_perm16a]
    vpbroadcastd         m7, [pb_4]
    lea                 r6d, [wq*2-32]
    mova                 m6, [spel_hv_perm16b]
    paddb               m20, m7, m19
    lea                 r6d, [hq+r6*8]
    paddb               m21, m7, m20
    mova               ym10, [spel_hv_end16]
    paddb                m7, m6
.hv_w16_loop0:
    movu               ym16, [srcq+nsq*2]
    vinserti32x8        m16, [srcq+nsq*1], 1
    lea                  r4, [srcq+ssq*2]
    movu               ym17, [srcq+ssq*0]
    vinserti32x8        m17, [srcq+ssq*1], 1
    mov                  r7, dstq
    movu               ym18, [r4  +ssq*0]
    vpermb               m2, m19, m16    ; 0 1   0123   89ab
    mova                 m1, m9
    vpermb               m3, m21, m16    ; 0 1   89ab   ghij
    vpdpbusd             m1, m2, m11
    mova                 m2, m9
    vpermb               m4, m19, m17    ; 2 3   0123   89ab
    vpdpbusd             m2, m3, m12
    mova                 m3, m9
    vpermb               m5, m21, m17    ; 2 3   89ab   ghij
    vpdpbusd             m3, m4, m11
    mova                 m4, m9
    vpermb               m0, m6, m18     ; 4     0145   2367   89cd   abef
    vpdpbusd             m4, m5, m12
    mova                 m5, m9
    vpermb              m16, m20, m16    ; 0 1   4567   cdef
    vpdpbusd             m5, m0, m11
    vpermb              m17, m20, m17    ; 2 3   4567   cdef
    vpdpbusd             m1, m16, m12
    vpermb              m18, m7, m18     ; 4     4589   67ab   cdgh   efij
    vpdpbusd             m2, m16, m11
    vpdpbusd             m3, m17, m12
    vpdpbusd             m4, m17, m11
    vpdpbusd             m5, m18, m12
    packssdw             m1, m2          ; 01
    packssdw             m3, m4          ; 23
    REPX       {psraw x, 2}, m1, m3, m5
    vpshrdd              m2, m1, m3, 16  ; 12
    vpshrdd              m4, m3, m5, 16  ; 34
.hv_w16_loop:
    movu               ym18, [r4+ssq*1]
    lea                  r4, [r4+ssq*2]
    vinserti32x8        m18, [r4+ssq*0], 1
    pmaddwd             m16, m1, m13     ; a0
    vpermb               m1, m19, m18    ; 5 6   0123   89ab
    pmaddwd             m17, m2, m13     ; b0
    vpermb               m2, m20, m18    ; 5 6   4567   cdef
    mova                 m0, m9
    vpdpbusd             m0, m1, m11
    vpermb              m18, m21, m18
    mova                 m1, m9
    vpdpbusd             m1, m2, m11
    vpdpwssd            m16, m3, m14    ; a1
    vpdpwssd            m17, m4, m14    ; b1
    vpdpbusd             m0, m2, m12
    mova                 m2, m4
    vpdpbusd             m1, m18, m12
    packssdw             m0, m1
    mova                 m1, m3
    psraw                m4, m0, 2      ; 5 6
    vpshrdd              m3, m2, m4, 16 ; 4 5
    vpdpwssd            m17, m4, m15    ; b2
    vpdpwssd            m16, m3, m15    ; a2
    packuswb            m16, m17
    vpermb              m16, m10, m16
    mova         [r7+dsq*0], xm16
    vextracti128 [r7+dsq*1], ym16, 1
    lea                  r7, [r7+dsq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    add                srcq, 16
    add                dstq, 16
    movzx                hd, r6b
    sub                 r6d, 1<<8
    jg .hv_w16_loop0
    vzeroupper
    RET

PUT_8TAP_FN smooth_sharp,   SMOOTH,  SHARP,   put_8tap_8bpc
PUT_8TAP_FN regular_sharp,  REGULAR, SHARP,   put_8tap_8bpc
PUT_8TAP_FN sharp,          SHARP,   SHARP

cglobal put_8tap_8bpc, 4, 9, 0, dst, ds, src, ss, w, h, mx, my, ss3
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    lea                  r8, [put_avx512icl]
    movsxd               wq, wm
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jz mangle(private_prefix %+ _put_6tap_8bpc_avx512icl).put
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    tzcnt               r6d, wd
    lea                 myq, [base+subpel_filters+myq*8]
    movzx               r6d, word [r8+r6*2+table_offset(put, _8tap_v)]
    vpbroadcastd         m7, [pw_512]
    vpbroadcastw         m8, [myq+0]
    add                  r6, r8
    vpbroadcastw         m9, [myq+2]
    lea                ss3q, [ssq*3]
    vpbroadcastw        m10, [myq+4]
    sub                srcq, ss3q
    vpbroadcastw        m11, [myq+6]
    jmp                  r6
.v_w2:
    movd               xmm2, [srcq+ssq*0]
    pinsrw             xmm2, [srcq+ssq*1], 2
    pinsrw             xmm2, [srcq+ssq*2], 4
    add                srcq, ss3q
    pinsrw             xmm2, [srcq+ssq*0], 6  ; 0 1 2 3
    movd               xmm3, [srcq+ssq*1]
    vpbroadcastd       xmm1, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm3, xmm3, xmm1, 0x02 ; 4 5
    vpblendd           xmm1, xmm1, xmm0, 0x02 ; 5 6
    palignr            xmm4, xmm3, xmm2, 4    ; 1 2 3 4
    punpcklbw          xmm3, xmm1             ; 45 56
    punpcklbw          xmm1, xmm2, xmm4       ; 01 12
    punpckhbw          xmm2, xmm4             ; 23 34
.v_w2_loop:
    pmaddubsw          xmm5, xmm1, xm8        ; a0 b0
    mova               xmm1, xmm2
    pmaddubsw          xmm2, xm9              ; a1 b1
    paddw              xmm5, xmm2
    mova               xmm2, xmm3
    pmaddubsw          xmm3, xm10             ; a2 b2
    paddw              xmm5, xmm3
    vpbroadcastd       xmm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vpblendd           xmm3, xmm0, xmm4, 0x02 ; 6 7
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm4, xmm4, xmm0, 0x02 ; 7 8
    punpcklbw          xmm3, xmm4             ; 67 78
    pmaddubsw          xmm4, xmm3, xm11       ; a3 b3
    paddw              xmm5, xmm4
    pmulhrsw           xmm5, xm7
    packuswb           xmm5, xmm5
    pextrw     [dstq+dsq*0], xmm5, 0
    pextrw     [dstq+dsq*1], xmm5, 2
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w2_loop
    RET
.v_w4:
    movd               xmm2, [srcq+ssq*0]
    pinsrd             xmm2, [srcq+ssq*1], 1
    pinsrd             xmm2, [srcq+ssq*2], 2
    add                srcq, ss3q
    pinsrd             xmm2, [srcq+ssq*0], 3  ; 0 1 2 3
    movd               xmm3, [srcq+ssq*1]
    vpbroadcastd       xmm1, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm3, xmm3, xmm1, 0x02 ; 4 5
    vpblendd           xmm1, xmm1, xmm0, 0x02 ; 5 6
    palignr            xmm4, xmm3, xmm2, 4    ; 1 2 3 4
    punpcklbw          xmm3, xmm1             ; 45 56
    punpcklbw          xmm1, xmm2, xmm4       ; 01 12
    punpckhbw          xmm2, xmm4             ; 23 34
.v_w4_loop:
    vpbroadcastd       xmm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw          xmm5, xmm1, xm8        ; a0 b0
    mova               xmm1, xmm2
    pmaddubsw          xmm2, xm9              ; a1 b1
    paddw              xmm5, xmm2
    mova               xmm2, xmm3
    pmaddubsw          xmm3, xm10             ; a2 b2
    paddw              xmm5, xmm3
    vpblendd           xmm3, xmm0, xmm4, 0x02 ; 6 7
    vpbroadcastd       xmm0, [srcq+ssq*0]
    vpblendd           xmm4, xmm4, xmm0, 0x02 ; 7 8
    punpcklbw          xmm3, xmm4             ; 67 78
    pmaddubsw          xmm4, xmm3, xm11       ; a3 b3
    paddw              xmm5, xmm4
    pmulhrsw           xmm5, xm7
    packuswb           xmm5, xmm5
    movd       [dstq+dsq*0], xmm5
    pextrd     [dstq+dsq*1], xmm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w4_loop
    RET
.v_w8:
    movq               xmm1, [srcq+ssq*0]
    vpbroadcastq       ymm0, [srcq+ssq*1]
    vpbroadcastq       ymm2, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastq       ymm5, [srcq+ssq*0]
    vpbroadcastq       ymm3, [srcq+ssq*1]
    vpbroadcastq       ymm4, [srcq+ssq*2]
    add                srcq, ss3q
    vpblendd           ymm1, ymm0, 0x30
    vpblendd           ymm0, ymm2, 0x30
    punpcklbw          ymm1, ymm0 ; 01 12
    vpbroadcastq       ymm0, [srcq+ssq*0]
    vpblendd           ymm2, ymm5, 0x30
    vpblendd           ymm5, ymm3, 0x30
    punpcklbw          ymm2, ymm5 ; 23 34
    vpblendd           ymm3, ymm4, 0x30
    vpblendd           ymm4, ymm0, 0x30
    punpcklbw          ymm3, ymm4 ; 45 56
.v_w8_loop:
    vpbroadcastq       ymm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pmaddubsw          ymm5, ymm1, ym8  ; a0 b0
    mova               ymm1, ymm2
    pmaddubsw          ymm2, ym9        ; a1 b1
    paddw              ymm5, ymm2
    mova               ymm2, ymm3
    pmaddubsw          ymm3, ym10       ; a2 b2
    paddw              ymm5, ymm3
    vpblendd           ymm3, ymm0, ymm4, 0x30
    vpbroadcastq       ymm0, [srcq+ssq*0]
    vpblendd           ymm4, ymm4, ymm0, 0x30
    punpcklbw          ymm3, ymm4       ; 67 78
    pmaddubsw          ymm4, ymm3, ym11 ; a3 b3
    paddw              ymm5, ymm4
    pmulhrsw           ymm5, ym7
    vextracti128       xmm4, ymm5, 1
    packuswb           xmm5, xmm4
    movq       [dstq+dsq*0], xmm5
    movhps     [dstq+dsq*1], xmm5
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w8_loop
    vzeroupper
    RET
.v_w16:
    mova                m12, [spel_v_perm16a]
    vbroadcasti32x4      m1, [srcq+ssq*0]
    vbroadcasti32x4     ym4, [srcq+ssq*1]
    mov                 r6d, 0x0f
    vbroadcasti32x4      m2, [srcq+ssq*2]
    add                srcq, ss3q
    vbroadcasti32x4     ym5, [srcq+ssq*0]
    kmovb                k1, r6d
    vbroadcasti32x4      m3, [srcq+ssq*1]
    vbroadcasti32x4     ym6, [srcq+ssq*2]
    add                srcq, ss3q
    vbroadcasti32x4      m0, [srcq+ssq*0]
    vshufpd          m1{k1}, m4, m2, 0xcc
    vshufpd          m2{k1}, m5, m3, 0xcc
    vshufpd          m3{k1}, m6, m0, 0xcc
    vpermb               m1, m12, m1 ; 01 12
    vpermb               m2, m12, m2 ; 23 34
    vpermb               m3, m12, m3 ; 45 56
.v_w16_loop:
    pmaddubsw            m4, m1, m8  ; a0 b0
    mova                 m1, m2
    pmaddubsw            m5, m2, m9  ; a1 b1
    mova                 m2, m3
    pmaddubsw            m6, m3, m10 ; a2 b2
    mova                 m3, m0
    paddw                m4, m5
    vbroadcasti32x4     ym5, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vbroadcasti32x4      m0, [srcq+ssq*0]
    vshufpd          m3{k1}, m5, m0, 0xcc
    vpermb               m3, m12, m3 ; 67 78
    pmaddubsw            m5, m3, m11 ; a3 b3
    paddw                m4, m6
    paddw                m4, m5
    pmulhrsw             m4, m7
    vextracti32x8       ym5, m4, 1
    packuswb            ym4, ym5
    mova          [dstq+dsq*0], xm4
    vextracti32x4 [dstq+dsq*1], ym4, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w16_loop
    RET
.v_w32:
    mova                m12, [spel_v_perm32]
    pmovzxbq            m14, [pb_02461357]
    vpshrdw             m13, m12, m12, 8
    movu                ym0, [srcq+ssq*0]
    vinserti32x8         m0, [srcq+ssq*1], 1
    vpermb               m1, m12, m0 ; 01
    vinserti32x8         m0, [srcq+ssq*2], 0
    add                srcq, ss3q
    vpermb               m2, m13, m0 ; 12
    vinserti32x8         m0, [srcq+ssq*0], 1
    vpermb               m3, m12, m0 ; 23
    vinserti32x8         m0, [srcq+ssq*1], 0
    vpermb               m4, m13, m0 ; 34
    vinserti32x8         m0, [srcq+ssq*2], 1
    add                srcq, ss3q
    vpermb               m5, m12, m0 ; 45
    vinserti32x8         m0, [srcq+ssq*0], 0
    vpermb               m6, m13, m0 ; 56
.v_w32_loop:
    vinserti32x8         m0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    pmaddubsw           m15, m1, m8
    mova                 m1, m3
    pmaddubsw           m16, m2, m8
    mova                 m2, m4
    pmaddubsw           m17, m3, m9
    mova                 m3, m5
    pmaddubsw           m18, m4, m9
    mova                 m4, m6
    pmaddubsw           m19, m5, m10
    vpermb               m5, m12, m0 ; 67
    vinserti32x8         m0, [srcq+ssq*0], 0
    pmaddubsw           m20, m6, m10
    vpermb               m6, m13, m0 ; 78
    paddw               m15, m17
    pmaddubsw           m17, m5, m11
    paddw               m16, m18
    pmaddubsw           m18, m6, m11
    paddw               m15, m19
    paddw               m16, m20
    paddw               m15, m17
    paddw               m16, m18
    pmulhrsw            m15, m7
    pmulhrsw            m16, m7
    packuswb            m15, m16
    vpermq              m15, m14, m15
    mova          [dstq+dsq*0], ym15
    vextracti32x8 [dstq+dsq*1], m15, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_w32_loop
    vzeroupper
    RET
.v_w64:
.v_w128:
    lea                 r6d, [hq+wq*4-256]
    mov                  r4, srcq
    mov                  r7, dstq
.v_loop0:
    movu                 m2, [srcq+ssq*0]
    movu                 m4, [srcq+ssq*1]
    movu                 m6, [srcq+ssq*2]
    add                srcq, ss3q
    movu                m13, [srcq+ssq*0]
    movu                m15, [srcq+ssq*1]
    movu                m17, [srcq+ssq*2]
    add                srcq, ss3q
    movu                 m0, [srcq+ssq*0]
    punpcklbw            m1, m2, m4    ; 01l
    punpckhbw            m2, m4        ; 01h
    punpcklbw            m3, m4, m6    ; 12l
    punpckhbw            m4, m6        ; 12h
    punpcklbw            m5, m6, m13   ; 23l
    punpckhbw            m6, m13       ; 23h
    punpcklbw           m12, m13, m15  ; 34l
    punpckhbw           m13, m15       ; 34h
    punpcklbw           m14, m15, m17  ; 45l
    punpckhbw           m15, m17       ; 45h
    punpcklbw           m16, m17, m0   ; 56l
    punpckhbw           m17, m0        ; 56h
.v_loop:
    pmaddubsw           m18, m1, m8    ; a0l
    mova                 m1, m5
    pmaddubsw           m19, m2, m8    ; a0h
    mova                 m2, m6
    pmaddubsw           m20, m3, m8    ; b0l
    mova                 m3, m12
    pmaddubsw           m21, m4, m8    ; b0h
    mova                 m4, m13
    pmaddubsw            m5, m9        ; a1l
    pmaddubsw            m6, m9        ; a1h
    pmaddubsw           m12, m9        ; b1l
    pmaddubsw           m13, m9        ; b1h
    paddw               m18, m5
    mova                 m5, m14
    pmaddubsw           m14, m10       ; a2l
    paddw               m19, m6
    mova                 m6, m15
    pmaddubsw           m15, m10       ; a2h
    paddw               m20, m12
    mova                m12, m16
    pmaddubsw           m16, m10       ; b2l
    paddw               m21, m13
    mova                m13, m17
    pmaddubsw           m17, m10       ; b2h
    paddw               m18, m14
    paddw               m19, m15
    paddw               m20, m16
    paddw               m21, m17
    movu                m17, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    punpcklbw           m14, m0, m17  ; 67l
    punpckhbw           m15, m0, m17  ; 67h
    pmaddubsw           m16, m14, m11 ; a3l
    pmaddubsw            m0, m15, m11 ; a3h
    paddw               m18, m16
    paddw               m19, m0
    movu                 m0, [srcq+ssq*0]
    punpcklbw           m16, m17, m0  ; 78l
    punpckhbw           m17, m0       ; 78h
    pmulhrsw            m18, m7
    pmulhrsw            m19, m7
    packuswb            m18, m19
    mova       [dstq+dsq*0], m18
    pmaddubsw           m18, m16, m11 ; b3l
    pmaddubsw           m19, m17, m11 ; b3h
    paddw               m18, m20
    paddw               m19, m21
    pmulhrsw            m18, m7
    pmulhrsw            m19, m7
    packuswb            m18, m19
    mova       [dstq+dsq*1], m18
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .v_loop
    add                  r4, 64
    add                  r7, 64
    movzx                hd, r6b
    mov                srcq, r4
    mov                dstq, r7
    sub                 r6d, 256
    jg .v_loop0
    vzeroupper
    RET
.h:
    test                myd, 0xf00
    jnz .hv
.h2:
    vpbroadcastd         m5, [pd_34] ; 2 + (8 << 2)
    cmp                  wd, 4
    jl .h_w2
    vbroadcasti128       m6, [subpel_h_shufA]
    je .h_w4
    tzcnt                wd, wd
    vbroadcasti128       m7, [subpel_h_shufB]
    vbroadcasti128       m8, [subpel_h_shufC]
    shr                 mxd, 16
    sub                srcq, 3
    movzx                wd, word [r8+wq*2+table_offset(put, _8tap_h)]
    vpbroadcastd         m9, [base+mxq*8+subpel_filters+0]
    vpbroadcastd        m10, [base+mxq*8+subpel_filters+4]
    add                  wq, r8
    jmp                  wq
.h_w2:
    movzx               mxd, mxb
    dec                srcq
    mova               xmm4, [subpel_h_shuf4]
    vpbroadcastd       xmm3, [base+mxq*8+subpel_filters+2]
.h_w2_loop:
    movq               xmm0, [srcq+ssq*0]
    movhps             xmm0, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb             xmm0, xmm4
    mova               xmm1, xm5
    vpdpbusd           xmm1, xmm0, xmm3
    packssdw           xmm0, xmm1, xmm1
    psraw              xmm0, 6
    packuswb           xmm0, xm0
    pextrw     [dstq+dsq*0], xmm0, 0
    pextrw     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w2_loop
    RET
.h_w4:
    movzx               mxd, mxb
    dec                srcq
    vpbroadcastd       xmm3, [base+mxq*8+subpel_filters+2]
.h_w4_loop:
    movq               xmm0, [srcq+ssq*0]
    movq               xmm1, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    pshufb             xmm0, xm6
    pshufb             xmm1, xm6
    mova               xmm2, xm5
    vpdpbusd           xmm2, xmm0, xmm3
    mova               xmm0, xm5
    vpdpbusd           xmm0, xmm1, xmm3
    packssdw           xmm0, xmm2, xmm0
    psraw              xmm0, 6
    packuswb           xmm0, xmm0
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w4_loop
    RET
.h_w8:
    movu                xm0, [srcq+ssq*0]
    vinserti32x4        ym0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    WRAP_YMM PUT_8TAP_H   0, 1, 2, 3
    vpmovuswb           xm0, ym0
    movq       [dstq+dsq*0], xm0
    movhps     [dstq+dsq*1], xm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w8
    RET
.h_w16:
    mova                 m6, [spel_h_perm16]
    vpbroadcastd         m8, [pb_4]
    paddb                m7, m8, m6
    paddb                m8, m7
.h_w16_loop:
    movu                ym0, [srcq+ssq*0]
    vinserti32x8         m0, [srcq+ssq*1], 1
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_H            0, 1, 2, 3, 1
    vpmovuswb           ym0, m0
    mova         [dstq+dsq*0], xm0
    vextracti128 [dstq+dsq*1], ym0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w16_loop
    RET
.h_w32:
    movu                ym0, [srcq+ssq*0+8*0]
    vinserti32x8         m0, [srcq+ssq*1+8*0], 1
    movu                ym1, [srcq+ssq*0+8*1]
    vinserti32x8         m1, [srcq+ssq*1+8*1], 1
    lea                srcq, [srcq+ssq*2]
    PUT_8TAP_H            0, 2, 3, 4
    PUT_8TAP_H            1, 4, 3, 2
    packuswb             m0, m1
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .h_w32
    RET
.h_w64:
    movu                 m0, [srcq+8*0]
    movu                 m1, [srcq+8*1]
    add                srcq, ssq
    PUT_8TAP_H            0, 2, 3, 4
    PUT_8TAP_H            1, 4, 3, 2
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, dsq
    dec                  hd
    jg .h_w64
    RET
.h_w128:
    movu                 m0, [srcq+8*0]
    movu                 m2, [srcq+8*1]
    movu                 m1, [srcq+8*8]
    movu                 m3, [srcq+8*9]
    add                srcq, ssq
    PUT_8TAP_H            0,  4, 11, 12
    PUT_8TAP_H            2, 12, 11,  4
    PUT_8TAP_H            1,  4, 11, 12
    PUT_8TAP_H            3, 12, 11,  4
    packuswb             m0, m2
    packuswb             m1, m3
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    add                dstq, dsq
    dec                  hd
    jg .h_w128
    RET
.hv:
    vpbroadcastd         m9, [pd_34]
    pxor                xm0, xm0
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    dec                srcq
    vpbroadcastd         m7, [base+subpel_filters+mxq*8+2]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    vpbroadcastq        ym1, [base+subpel_filters+myq*8]
    lea                ss3q, [ssq*3]
    mov                  r6, srcq
    punpcklbw           ym0, ym1
    sub                  r6, ss3q
    psraw               ym0, 2 ; << 6
    mova               xm14, [spel_hv_end]
    pshufd             ym10, ym0, q0000
    pshufd             ym11, ym0, q1111
    pshufd             ym12, ym0, q2222
    pshufd             ym13, ym0, q3333
    cmp                  wd, 4
    je .hv_w4
    vbroadcasti128      ym6, [subpel_h_shuf4]
    movq               xmm2, [r6+ssq*0]
    movhps             xmm2, [r6+ssq*1]
    movq               xmm0, [r6+ssq*2]
    movhps             xmm0, [srcq+ssq*0]
    vpbroadcastq       ymm3, [srcq+ssq*1]
    vpbroadcastq       ymm4, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastq       ymm1, [srcq+ssq*0]
    vpblendd           ymm2, ymm3, 0x30
    vpblendd           ymm0, ymm1, 0x30 ; 2 3   6 _
    vpblendd           ymm2, ymm4, 0xc0 ; 0 1   4 5
    pshufb             ymm2, ym6
    pshufb             ymm0, ym6
    mova               ymm1, ym9
    vpdpbusd           ymm1, ymm2, ym7
    mova               ymm2, ym9
    vpdpbusd           ymm2, ymm0, ym7
    packssdw           ymm2, ymm1, ymm2
    psraw              ymm2, 2
    vextracti128       xmm3, ymm2, 1
    palignr            xmm4, xmm3, xmm2, 4
    punpcklwd          xmm1, xmm2, xmm4 ; 01 12
    punpckhwd          xmm2, xmm4       ; 23 34
    pshufd             xmm0, xmm3, q2121
    punpcklwd          xmm3, xmm0       ; 45 56
.hv_w2_loop:
    movq               xmm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movhps             xmm4, [srcq+ssq*0]
    pmaddwd            xmm5, xmm1, xm10 ; a0 b0
    mova               xmm1, xmm2
    vpdpwssd           xmm5, xmm2, xm11 ; a1 b1
    pshufb             xmm4, xm6
    mova               xmm2, xmm3
    vpdpwssd           xmm5, xmm3, xm12 ; a2 b2
    mova               xmm3, xm9
    vpdpbusd           xmm3, xmm4, xm7
    packssdw           xmm4, xmm3, xmm3
    psraw              xmm4, 2
    palignr            xmm3, xmm4, xmm0, 12
    mova               xmm0, xmm4
    punpcklwd          xmm3, xmm4       ; 67 78
    vpdpwssd           xmm5, xmm3, xm13 ; a3 b3
    packuswb           xmm5, xmm5
    pshufb             xmm5, xm14
    pextrw     [dstq+dsq*0], xmm5, 0
    pextrw     [dstq+dsq*1], xmm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w2_loop
    vzeroupper
    RET
.hv_w4:
    movq               xmm1, [r6+ssq*0]
    vpbroadcastq        ym2, [r6+ssq*1]
    vinserti32x4        ym1, ymm1, [r6+ssq*2], 1
    vinserti32x4         m2, [srcq+ssq*0], 2
    vinserti32x4         m1, [srcq+ssq*1], 2
    vinserti32x4         m2, [srcq+ssq*2], 3 ; _ 1 3 5
    vbroadcasti32x4      m6, [subpel_h_shufA]
    add                srcq, ss3q
    vinserti32x4         m1, [srcq+ssq*0], 3 ; 0 2 4 6
    pshufb               m2, m6
    pshufb               m1, m6
    mova                 m0, m9
    vpdpbusd             m0, m2, m7
    mova                 m4, m9
    vpdpbusd             m4, m1, m7
    mova                ym1, [spel_hv_perm4a]
    mova                ym2, [spel_hv_perm4b]
    mova                ym3, [spel_hv_perm4c]
    packssdw             m0, m4
    psraw                m0, 2 ; _ 0   1 2   3 4   5 6
    mov                 r6d, 0x5555
    vpermb              ym1, ym1, ym0 ; 01 12
    vpermb               m2, m2, m0   ; 23 34
    vpermb               m3, m3, m0   ; 45 56
    kmovw                k1, r6d
    mova               ym15, [spel_hv_perm4d]
.hv_w4_loop:
    movq               xmm4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vinserti32x4        ym4, ymm4, [srcq+ssq*0], 1
    pmaddwd             ym5, ym1, ym10 ; a0 b0
    mova                ym1, ym2
    pshufb              ym4, ym6
    mova                ym0, ym9
    vpdpbusd            ym0, ym4, ym7
    vpdpwssd            ym5, ym2, ym11 ; a1 b1
    mova                ym2, ym3
    vpdpwssd            ym5, ym3, ym12 ; a2 b2
    vpsraw          ym3{k1}, ym0, 2    ; 7 8
    vpermb              ym3, ym15, ym3 ; 67 78
    vpdpwssd            ym5, ym3, ym13 ; a3 b3
    packuswb            ym5, ym5
    vpermb              ym5, ym14, ym5
    movd       [dstq+dsq*0], xm5
    pextrd     [dstq+dsq*1], xm5, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w4_loop
    RET
.hv_w8:
    shr                 mxd, 16
    sub                srcq, 3
    vpbroadcastd        m10, [base+subpel_filters+mxq*8+0]
    vpbroadcastd        m11, [base+subpel_filters+mxq*8+4]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 6
    cmovs               myd, mxd
    vpbroadcastq         m1, [base+subpel_filters+myq*8]
    punpcklbw            m0, m1
    lea                ss3q, [ssq*3]
    psraw                m0, 2 ; << 6
    pshufd              m12, m0, q0000
    pshufd              m13, m0, q1111
    pshufd              m14, m0, q2222
    pshufd              m15, m0, q3333
    cmp                  wd, 8
    jne .hv_w16
    mov                  r6, srcq
    sub                  r6, ss3q
    movu               xmm1, [r6+ssq*0]
    vinserti128        ymm1, [r6+ssq*1], 1
    movu               xmm2, [srcq+ssq*1]
    vinserti32x4         m6, zmm1, [r6+ssq*2], 2
    vinserti128        ymm2, [srcq+ssq*2], 1
    vinserti32x4         m6, [srcq+ssq*0], 3 ; 0 1 2 3
    add                srcq, ss3q
    vbroadcasti32x4      m4, [subpel_h_shufA]
    vinserti32x4         m0, zmm2, [srcq+ssq*0], 2 ; 4 5 6 _
    vbroadcasti32x4      m7, [subpel_h_shufB]
    vbroadcasti32x4      m8, [subpel_h_shufC]
    pshufb               m1, m6, m4  ; 0 1 2 3   0123
    mova                 m2, m9
    vpdpbusd             m2, m1, m10
    pshufb               m5, m6, m7  ; 0 1 2 3   4567
    mova                 m1, m9
    vpdpbusd             m1, m5, m10
    pshufb               m4, m0, m4  ; 4 5 6 _   0123
    mova                 m3, m9
    vpdpbusd             m3, m4, m10
    pshufb               m7, m0, m7  ; 4 5 6 _   4567
    mova                 m4, m9
    vpdpbusd             m4, m7, m10
    pshufb               m6, m8
    vpdpbusd             m2, m5, m11
    vpdpbusd             m1, m6, m11
    pshufb               m6, m0, m8
    vpdpbusd             m3, m7, m11
    vpdpbusd             m4, m6, m11
    mova                 m5, [spel_hv_perm8a]
    vpaddd               m0, m5, [pb_32] {1to16}
    mov                  r6, 0x55555555ff00
    packssdw             m2, m1
    packssdw             m3, m4
    mova                 m8, [spel_hv_perm8b]
    psraw                m2, 2 ; 0 1 2 3
    psraw                m3, 2 ; 4 5 6 _
    vpermb               m1, m5, m2 ; 01 12
    vbroadcasti32x8      m6, [subpel_h_shufA]
    kmovq                k1, r6
    vpermt2b             m2, m0, m3 ; 23 34
    vbroadcasti32x8      m7, [subpel_h_shufB]
    kshiftrq             k2, k1, 16
    mova               xm16, [spel_hv_end]
    vpermb               m3, m5, m3 ; 45 56
.hv_w8_loop:
    vbroadcasti32x4     ym4, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    vbroadcasti32x4  m4{k1}, [srcq+ssq*0]
    pmaddwd              m0, m1, m12 ; a0 b0
    pshufb               m1, m4, m6  ; 7 8   0123 4567
    mova                 m5, m9
    vpdpbusd             m5, m1, m10
    pshufb               m4, m7      ; 7 8   4567 89ab
    vpdpwssd             m0, m2, m13 ; a1 b1
    mova                 m1, m2
    vpdpbusd             m5, m4, m11
    mova                 m2, m3
    vpdpwssd             m0, m3, m14 ; a2 b2
    psraw            m3{k2}, m5, 2   ; 75 86
    vpermb               m3, m8, m3  ; 67 78
    vpdpwssd             m0, m3, m15 ; a3 b3
    packuswb             m0, m0
    vpermb             zmm1, m16, m0
    movq       [dstq+dsq*0], xmm1
    movhps     [dstq+dsq*1], xmm1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .hv_w8_loop
    vzeroupper
    RET
.hv_w16:
    WIN64_SPILL_XMM      23
    movu                m22, [spel_hv_perm16a]
    sub                srcq, ss3q
    vpbroadcastd         m8, [pb_4]
    lea                 r6d, [wq*2-32]
    mova                 m7, [spel_hv_perm16b]
    paddb               m20, m8, m22
    mova               ym16, [spel_hv_end16]
    paddb               m21, m8, m20
    lea                 r6d, [hq+r6*8]
    paddb                m8, m7
.hv_w16_loop0:
    movu               ym17, [srcq+ssq*0]
    vinserti32x8        m17, [srcq+ssq*1], 1 ; 0 1
    lea                  r4, [srcq+ss3q]
    movu               ym18, [srcq+ssq*2]
    vinserti32x8        m18, [r4  +ssq*0], 1 ; 2 3
    mov                  r7, dstq
    movu               ym19, [r4  +ssq*1]
    vinserti32x8        m19, [r4  +ssq*2], 1 ; 4 5
    add                  r4, ss3q
    vpermb               m2, m22, m17    ; 0 1   0123   89ab
    mova                 m1, m9
    vpermb               m3, m21, m17    ; 0 1   89ab   ghij
    vpdpbusd             m1, m2, m10
    mova                 m2, m9
    vpermb               m4, m22, m18    ; 2 3   0123   89ab
    vpdpbusd             m2, m3, m11
    mova                 m3, m9
    vpermb               m5, m21, m18    ; 2 3   89ab   ghij
    vpdpbusd             m3, m4, m10
    mova                 m4, m9
    vpermb               m6, m22, m19    ; 4 5   0123   89ab
    vpdpbusd             m4, m5, m11
    mova                 m5, m9
    vpermb              m17, m20, m17    ; 0 1   4567   cdef
    vpdpbusd             m5, m6, m10
    mova                 m6, m9
    vpermb               m0, m21, m19    ; 4 5   89ab   ghij
    vpdpbusd             m1, m17, m11
    vpdpbusd             m2, m17, m10
    movu               ym17, [r4+ssq*0]  ; 6
    vpermb              m18, m20, m18    ; 2 3   4567   cdef
    vpdpbusd             m6, m0, m11
    vpermb               m0, m7, m17     ; 6     0145   2367   89cd   abef
    vpdpbusd             m3, m18, m11
    vpermb              m19, m20, m19    ; 4 5   4567   cdef
    vpdpbusd             m4, m18, m10
    mova                m18, m9
    vpermb              m17, m8, m17     ; 6     4589   67ab   cdgh   efij
    vpdpbusd            m18, m0, m10
    packssdw             m1, m2
    vpdpbusd             m5, m19, m11
    vpdpbusd             m6, m19, m10
    packssdw             m3, m4
    vpdpbusd            m18, m17, m11
    psraw                m1, 2           ; 01
    psraw                m3, 2           ; 23
    packssdw             m5, m6
    vpshrdd              m2, m1, m3, 16  ; 12
    psraw                m5, 2           ; 45
    vpshrdd              m4, m3, m5, 16  ; 34
    psraw               m18, 2
    vpshrdd              m6, m5, m18, 16 ; 56
.hv_w16_loop:
    movu               ym19, [r4+ssq*1]
    lea                  r4, [r4+ssq*2]
    vinserti32x8        m19, [r4+ssq*0], 1
    pmaddwd             m17, m1, m12     ; a0
    vpermb               m1, m22, m19    ; 7 8   0123   89ab
    pmaddwd             m18, m2, m12     ; b0
    mova                 m0, m9
    vpermb               m2, m21, m19    ; 7 8   89ab   ghij
    vpdpbusd             m0, m1, m10
    mova                 m1, m9
    vpermb              m19, m20, m19    ; 7 8   4567   cdef
    vpdpbusd             m1, m2, m11
    mova                 m2, m4
    vpdpwssd            m17, m3, m13     ; a1
    vpdpwssd            m18, m4, m13     ; b1
    mova                 m4, m6
    vpdpbusd             m0, m19, m11
    vpdpbusd             m1, m19, m10
    vpdpwssd            m17, m5, m14     ; a2
    vpdpwssd            m18, m6, m14     ; b2
    packssdw             m0, m1
    mova                 m1, m3
    psraw                m6, m0, 2       ; 78
    mova                 m3, m5
    vpshrdd              m5, m4, m6, 16  ; 67
    vpdpwssd            m18, m6, m15     ; b3
    vpdpwssd            m17, m5, m15     ; a3
    packuswb            m17, m18
    vpermb              m17, m16, m17
    mova         [r7+dsq*0], xm17
    vextracti128 [r7+dsq*1], ym17, 1
    lea                  r7, [r7+dsq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    add                srcq, 16
    add                dstq, 16
    movzx                hd, r6b
    sub                 r6d, 1<<8
    jg .hv_w16_loop0
    RET

%if WIN64
DECLARE_REG_TMP 6, 4
%else
DECLARE_REG_TMP 6, 7
%endif

%define PREP_8TAP_FN FN prep_8tap,
PREP_8TAP_FN sharp_smooth,   SHARP,   SMOOTH,  prep_6tap_8bpc
PREP_8TAP_FN sharp_regular,  SHARP,   REGULAR, prep_6tap_8bpc
PREP_8TAP_FN smooth,         SMOOTH,  SMOOTH,  prep_6tap_8bpc
PREP_8TAP_FN smooth_regular, SMOOTH,  REGULAR, prep_6tap_8bpc
PREP_8TAP_FN regular_smooth, REGULAR, SMOOTH,  prep_6tap_8bpc
PREP_8TAP_FN regular,        REGULAR, REGULAR

cglobal prep_6tap_8bpc, 4, 8, 0, tmp, src, ss, w, h, mx, my, ss3
%define base r7-prep_avx512icl
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 6tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 6tap_v, my, 4tap_v
    lea                  r7, [prep_avx512icl]
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jnz .v
.prep:
    tzcnt                wd, wd
    movzx                wd, word [r7+wq*2+table_offset(prep,)]
    add                  wq, r7
    lea                  r6, [ssq*3]
%if WIN64
    pop                  r7
%endif
    jmp                  wq
.v:
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    tzcnt               r5d, wd
    lea                 myq, [base+subpel_filters+1+myq*8]
    movzx               r5d, word [r7+r5*2+table_offset(prep, _6tap_v)]
    vpbroadcastd         m7, [pw_8192]
    sub                srcq, ssq
    vpbroadcastw         m8, [myq+0]
    add                  r5, r7
    vpbroadcastw         m9, [myq+2]
    lea                ss3q, [ssq*3]
    vpbroadcastw        m10, [myq+4]
    sub                srcq, ssq
    jmp                  r5
.v_w4:
    movd               xmm2, [srcq+ssq*0]
    pinsrd             xmm2, [srcq+ssq*1], 1
    vpbroadcastd       ymm1, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastd       ymm3, [srcq+ssq*0]
    vpbroadcastd       ymm0, [srcq+ssq*1]
    vbroadcasti128     ymm5, [deint_shuf4]
    vpblendd           ymm1, ymm2, 0xeb
    punpcklqdq         ymm3, ymm0
    vpblendd           ymm1, ymm3, 0x60 ; 0 1 2 _   2 3 4 _
    pshufb             ymm1, ymm5       ; 01 12 23 34
.v_w4_loop:
    pinsrd             xmm0, [srcq+ssq*2], 1
    vpbroadcastd       ymm2, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    vpbroadcastd       ymm3, [srcq+ssq*0]
    vpblendd           ymm2, ymm0, 0xeb
    vpbroadcastd       ymm0, [srcq+ssq*1]
    punpcklqdq         ymm3, ymm0
    vpblendd           ymm2, ymm3, 0x60 ; 4 5 6 _   6 7 8 _
    pshufb             ymm2, ymm5       ; 45 56 67 78
    pmaddubsw          ymm3, ymm1, ym8  ; a0 b0 c0 d0
    vperm2i128         ymm1, ymm2, 0x21 ; 23 34 45 56
    pmaddubsw          ymm4, ymm2, ym10 ; a2 b2 c2 d2
    pmaddubsw          ymm1, ym9        ; a1 b1 c1 d1
    paddw              ymm3, ymm4
    paddw              ymm3, ymm1
    pmulhrsw           ymm3, ym7
    mova               ymm1, ymm2
    mova             [tmpq], ymm3
    add                tmpq, 32
    sub                  hd, 4
    jg .v_w4_loop
    vzeroupper
    RET
.v_w8:
    mova                 m6, [spel_v_perm8]
    movq                xm1, [srcq+ssq*0]
    mov                 r6d, 0x3e
    movq                xm2, [srcq+ssq*1]
    kmovb                k1, r6d
    vpbroadcastq        ym3, [srcq+ssq*2]
    add                srcq, ss3q
    vpunpcklqdq         ym2, [srcq+ssq*0] {1to4}
    vpunpcklqdq      m1{k1}, m3, [srcq+ssq*1] {1to8}
    movq                xm0, [srcq+ssq*1]
    kshiftlb             k2, k1, 2
    shufpd               m1, m2, 0x18  ; 0 1   2 3   4
    vpermb               m1, m6, m1    ; 01 12 23 34
.v_w8_loop:
    vpbroadcastq        ym3, [srcq+ss3q ]
    vpunpcklqdq     ym0{k1}, ym3, [srcq+ssq*2] {1to4}
    lea                srcq, [srcq+ssq*4]
    vpbroadcastq         m3, [srcq+ssq*1]
    vpunpcklqdq      m0{k2}, m3, [srcq+ssq*0] {1to8}
    pmaddubsw            m4, m1, m8    ; a0 b0 c0 d0
    vpermb               m2, m6, m0    ; 45 56 67 78
    mova                xm0, xm3
    vshufi32x4           m1, m2, q1032 ; 23 34 45 56
    pmaddubsw            m3, m2, m10   ; a3 b3 c3 d3
    pmaddubsw            m5, m1, m9    ; a2 b2 c2 d2
    mova                 m1, m2
    paddw                m4, m3
    paddw                m4, m5
    pmulhrsw             m4, m7
    mova             [tmpq], m4
    add                tmpq, 64
    sub                  hd, 4
    jg .v_w8_loop
    RET
.v_w16:
    mova                m11, [spel_v_perm16b]
    vbroadcasti32x4      m1, [srcq+ssq*0]
    mov                 r6d, 0x0f
    vbroadcasti32x4     ym3, [srcq+ssq*1]
    vbroadcasti32x4      m2, [srcq+ssq*2]
    kmovb                k1, r6d
    add                srcq, ss3q
    vbroadcasti32x4     ym4, [srcq+ssq*0]
    vbroadcasti32x4      m0, [srcq+ssq*1]
    vshufpd          m1{k1}, m3, m2, 0xcc
    vshufpd          m2{k1}, m4, m0, 0xcc
    vpermb               m1, m11, m1 ; 01 12
    vpermb               m2, m11, m2 ; 23 34
.v_w16_loop:
    pmaddubsw            m3, m1, m8  ; a0 b0
    pmaddubsw            m5, m2, m9  ; a1 b1
    vbroadcasti32x4     ym6, [srcq+ssq*2]
    pmaddubsw            m4, m2, m8  ; c0 d0
    vbroadcasti32x4      m2, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    vshufpd          m0{k1}, m6, m2, 0xcc
    vbroadcasti32x4     ym6, [srcq+ssq*0]
    vpermb               m1, m11, m0 ; 45 56
    vbroadcasti32x4      m0, [srcq+ssq*1]
    vshufpd          m2{k1}, m6, m0, 0xcc
    pmaddubsw            m6, m1, m9  ; c1 d1
    vpermb               m2, m11, m2 ; 67 78
    paddw                m3, m5
    pmaddubsw            m5, m1, m10 ; a2 b2
    paddw                m4, m6
    pmaddubsw            m6, m2, m10 ; c2 d2
    paddw                m3, m5
    paddw                m4, m6
    pmulhrsw             m3, m7
    pmulhrsw             m4, m7
    mova          [tmpq+ 0], m3
    mova          [tmpq+64], m4
    add                tmpq, 64*2
    sub                  hd, 4
    jg .v_w16_loop
    RET
.v_w32:
    movshdup             m6, [bilin_v_perm64]
    movu               ym16, [srcq+ssq*0]
    movu               ym17, [srcq+ssq*1]
    movu               ym18, [srcq+ssq*2]
    add                srcq, ss3q
    movu               ym19, [srcq+ssq*0]
    add                srcq, ssq
    movu               ym20, [srcq+ssq*0]
    vpermt2q            m16, m6, m18   ; 0 2
    vpermt2q            m17, m6, m19   ; 1 3
    vpermt2q            m18, m6, m20   ; 2 4
    punpcklbw            m0, m16, m17  ; 01
    punpcklbw            m1, m17, m18  ; 12
    punpckhbw            m2, m16, m17  ; 23
    punpckhbw            m3, m17, m18  ; 34
.v_w32_loop:
    movu               ym16, [srcq+ssq*1]
    lea                srcq, [srcq+ssq*2]
    movu               ym17, [srcq+ssq*0]
    pmaddubsw            m4, m0, m8    ; a0
    mova                 m0, m2
    pmaddubsw            m2, m9        ; a1
    vpermt2q            m16, m6, m17   ; 5 6
    pmaddubsw            m5, m1, m8    ; b0
    mova                 m1, m3
    pmaddubsw            m3, m9        ; b1
    shufpd              m18, m16, 0x55 ; 4 5
    paddw                m4, m2
    punpcklbw            m2, m18, m16  ; 45
    paddw                m5, m3
    punpckhbw            m3, m18, m16  ; 56
    mova                m18, m16
    pmaddubsw           m16, m2, m10   ; a2
    pmaddubsw           m17, m3, m10   ; b2
    paddw                m4, m16
    paddw                m5, m17
    pmulhrsw             m4, m7
    pmulhrsw             m5, m7
    mova          [tmpq+ 0], m4
    mova          [tmpq+64], m5
    add                tmpq, 64*2
    sub                  hd, 2
    jg .v_w32_loop
    vzeroupper
    RET
.v_w64:
.v_w128:
    mova                 m6, [bilin_v_perm64]
    add                  wd, wd
    lea                 r6d, [hq+wq]
.v_loop0:
    vpermq              m12, m6, [srcq+ssq*0]
    vpermq              m13, m6, [srcq+ssq*1]
    lea                  r5, [srcq+ssq*2]
    vpermq              m14, m6, [r5  +ssq*0]
    vpermq              m15, m6, [r5  +ssq*1]
    lea                  r5, [r5+ssq*2]
    vpermq              m16, m6, [r5  +ssq*0]
    mov                  r7, tmpq
    punpcklbw            m0, m12, m13 ; 01
    punpckhbw           m12, m13
    punpcklbw            m1, m13, m14 ; 12
    punpckhbw           m13, m14
    punpcklbw            m2, m14, m15 ; 23
    punpckhbw           m14, m15
    punpcklbw            m3, m15, m16 ; 34
    punpckhbw           m15, m16
.v_loop:
    pmaddubsw           m17, m0, m8   ; a0
    vpermq               m5, m6, [r5+ssq*1]
    pmaddubsw           m18, m12, m8
    mova                 m0, m2
    pmaddubsw            m2, m9       ; a1
    mova                m12, m14
    pmaddubsw           m14, m9
    lea                  r5, [r5+ssq*2]
    pmaddubsw           m19, m1, m8   ; b0
    pmaddubsw           m20, m13, m8
    mova                 m1, m3
    pmaddubsw            m3, m9       ; b1
    mova                m13, m15
    pmaddubsw           m15, m9
    paddw               m17, m2
    punpcklbw            m2, m16, m5  ; 67
    paddw               m18, m14
    punpckhbw           m14, m16, m5
    vpermq              m16, m6, [r5+ssq*0]
    paddw               m19, m3
    pmaddubsw            m3, m2, m10  ; a3
    paddw               m20, m15
    pmaddubsw           m15, m14, m10
    paddw               m17, m3
    punpcklbw            m3, m5, m16  ; 78
    pmaddubsw            m4, m3, m10  ; b3
    paddw               m18, m15
    punpckhbw           m15, m5, m16
    pmaddubsw            m5, m15, m10
    paddw               m19, m4
    paddw               m20, m5
    REPX   {pmulhrsw x, m7}, m17, m18, m19, m20
    mova       [r7+wq*0+ 0], m17
    mova       [r7+wq*0+64], m18
    mova       [r7+wq*1+ 0], m19
    mova       [r7+wq*1+64], m20
    lea                  r7, [r7+wq*2]
    sub                  hd, 2
    jg .v_loop
    add                srcq, 64
    add                tmpq, 128
    movzx                hd, r6b
    sub                 r6d, 1<<8
    jg .v_loop0
    vzeroupper
    RET
.h:
    test                myd, 0xf00
    jz mangle(private_prefix %+ _prep_8tap_8bpc_avx512icl).h2
.hv:
    vpbroadcastd         m8, [pd_2]
    vpbroadcastd         m9, [pd_32]
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    vpbroadcastd        m11, [base+subpel_filters+mxq*8+2]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    vpbroadcastq         m3, [base+subpel_filters+1+myq*8]
    vbroadcasti128      m10, [subpel_h_shufA]
    lea                  r6, [ssq*2+1]
    mov                 r3d, 0x30
    sub                srcq, r6
    kmovb                k1, r3d
    vpbroadcastq        ym2, [srcq+ssq*0]
    lea                ss3q, [ssq*3]
    vpbroadcastq         m1, [srcq+ssq*1]
    kaddb                k2, k1, k1
    vpbroadcastq     m2{k1}, [srcq+ssq*2]
    add                srcq, ss3q
    vpbroadcastq     m1{k2}, [srcq+ssq*0] ; _ _ 1 3
    punpcklbw            m3, m3
    vpbroadcastq     m2{k2}, [srcq+ssq*1] ; _ 0 2 4
    psraw                m3, 8 ; sign-extend
    mova                 m6, [spel_hv_perm4a]
    kshiftrb             k1, k1, 2
    movu                 m7, [spel_hv_perm4b]
    pshufb               m1, m10
    mova                 m0, m8
    vpdpbusd             m0, m1, m11
    pshufb               m2, m10
    mova                 m1, m8
    vpdpbusd             m1, m2, m11
    pshufd              m12, m3, q0000
    pshufd              m13, m3, q1111
    pshufd              m14, m3, q2222
    packssdw             m0, m1           ; _ _   _ 0   1 2   3 4
    psraw                m0, 2
    vpermb               m1, m7, m0       ; 01 12 23 34
.hv_w4_loop:
    movq                xm3, [srcq+ssq*2]
    movq                xm4, [srcq+ss3q ]
    lea                srcq, [srcq+ssq*4]
    vpbroadcastq    ym3{k1}, [srcq+ssq*0] ; 5 7
    vpbroadcastq    ym4{k1}, [srcq+ssq*1] ; 6 8
    pshufb              ym3, ym10
    mova                ym2, ym8
    vpdpbusd            ym2, ym3, ym11
    pshufb              ym4, ym10
    mova                ym3, ym8
    vpdpbusd            ym3, ym4, ym11
    mova                 m4, m9
    vpdpwssd             m4, m1, m12      ; a0 b0 c0 d0
    packssdw            ym2, ym3          ; 5 6   7 8
    psraw               ym2, 2
    vshufi32x4           m0, m2, q1032    ; _ 2   3 4   5 6   7 8
    vpermb               m2, m6, m0       ; 23 34 45 56
    vpermb               m1, m7, m0       ; 45 56 67 78
    vpdpwssd             m4, m2, m13      ; a1 b1 c1 d1
    vpdpwssd             m4, m1, m14      ; a2 b2 c2 d2
    psrad                m4, 6
    vpmovdw          [tmpq], m4
    add                tmpq, 32
    sub                  hd, 4
    jg .hv_w4_loop
    RET
.hv_w8:
    shr                 mxd, 16
    vpbroadcastd        m10, [base+subpel_filters+mxq*8+0]
    vpbroadcastd        m11, [base+subpel_filters+mxq*8+4]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    vpbroadcastq         m0, [base+subpel_filters+1+myq*8]
    lea                  r6, [ssq*2+3]
    punpcklbw            m0, m0
    sub                srcq, r6
    psraw                m0, 8 ; sign-extend
    lea                ss3q, [ssq*3]
    pshufd              m12, m0, q0000
    pshufd              m13, m0, q1111
    pshufd              m14, m0, q2222
    cmp                  wd, 8
    jg .hv_w16
    movu               xm16, [srcq+ssq*0]
    vbroadcasti32x4     m19, [subpel_h_shufA]
    vinserti128        ym16, [srcq+ssq*1], 1
    vbroadcasti32x4     m21, [subpel_h_shufC]
    vinserti32x4        m16, [srcq+ssq*2], 2
    add                srcq, ss3q
    vinserti32x4        m16, [srcq+ssq*0], 3
    movu               xm17, [srcq+ssq*1]
    vbroadcasti32x4     m20, [subpel_h_shufB]
    pshufb               m3, m16, m19   ; 0 1 2 3   0123
    mova                 m2, m8
    pshufb               m0, m16, m21   ; 0 1 2 3   89ab
    vpdpbusd             m2, m3, m10
    mova                 m3, m8
    pshufb              xm1, xm17, xm19 ; 3 4 5 6   0123
    vpdpbusd             m3, m0, m11
    mova                xm0, xm8
    pshufb             xm18, xm17, xm21 ; 3 4 5 6   89ab
    vpdpbusd            xm0, xm1, xm10
    mova                xm1, xm8
    pshufb              m16, m20        ; 0 1 2 3   4567
    vpdpbusd            xm1, xm18, xm11
    pshufb             xm17, xm20       ; 3 4 5 6   4567
    vpdpbusd             m2, m16, m11
    vpdpbusd             m3, m16, m10
    vpdpbusd            xm0, xm17, xm11
    vpdpbusd            xm1, xm17, xm10
    packssdw             m2, m3
    packssdw            xm0, xm1
    psraw                m2, 2          ; 0 1 2 3
    psraw               xm0, 2          ; 4
    valignq              m0, m2, 2      ; 1 2 3 4
    punpcklwd            m1, m2, m0     ; 01 12 23 34
    punpckhwd            m2, m0
.hv_w8_loop:
    movu               xm16, [srcq+ssq*2]
    vinserti128        ym16, [srcq+ss3q ], 1
    lea                srcq, [srcq+ssq*4]
    vinserti32x4        m16, [srcq+ssq*0], 2
    vinserti32x4        m16, [srcq+ssq*1], 3
    pshufb               m6, m16, m19   ; 5 6 7 8   0123
    mova                 m5, m8
    pshufb               m3, m16, m21   ; 5 6 7 8   89ab
    vpdpbusd             m5, m6, m10
    mova                 m6, m8
    pshufb              m16, m20        ; 5 6 7 8   4567
    vpdpbusd             m6, m3, m11
    mova                 m3, m9
    vpdpwssd             m3, m1, m12    ; a0 b0 c0 d0
    mova                 m4, m9
    vpdpwssd             m4, m2, m12
    vpdpbusd             m5, m16, m11
    vpdpbusd             m6, m16, m10
    mova                m16, m1
    packssdw             m5, m6
    mova                 m6, m2
    psraw                m5, 2          ; 5 6 7 8
    valignq              m2, m5, m0, 6  ; 4 5 6 7
    mova                 m0, m5
    punpcklwd            m1, m2, m5     ; 45 56 67 78
    punpckhwd            m2, m5
    vpdpwssd             m3, m1, m14    ; a2 b2 c2 d2
    vpdpwssd             m4, m2, m14
    vshufi32x4          m16, m1, q1032  ; 23 34 45 56
    vshufi32x4           m6, m2, q1032
    vpdpwssd             m3, m16, m13   ; a1 b1 c1 d1
    vpdpwssd             m4, m6, m13
    psrad                m3, 6
    psrad                m4, 6
    packssdw             m3, m4
    mova             [tmpq], m3
    add                tmpq, 64
    sub                  hd, 4
    jg .hv_w8_loop
    vzeroupper
    RET
.hv_w16:
    mova                m16, [spel_h_perm16]
    vpbroadcastd        m18, [pb_4]
    add                  wd, wd
    paddb               m17, m18, m16
    lea                 r6d, [hq+wq*8-256]
    paddb               m18, m17
.hv_w16_loop0:
    movu               ym19, [srcq+ssq*0]
    vinserti32x8        m19, [srcq+ssq*1], 1
    lea                  r5, [srcq+ssq*2]
    movu               ym20, [r5  +ssq*0]
    vinserti32x8        m20, [r5  +ssq*1], 1
    lea                  r5, [r5  +ssq*2]
    movu               ym21, [r5  +ssq*0]
    mov                  r7, tmpq
    vpermb               m3, m16, m19      ; 0 1   0123   89ab
    mova                 m2, m8
    vpermb               m4, m18, m19      ; 0 1   89ab   ghij
    vpdpbusd             m2, m3, m10
    mova                 m3, m8
    vpermb               m5, m16, m20      ; 2 3   0123   89ab
    vpdpbusd             m3, m4, m11
    mova                 m4, m8
    vpermb               m0, m18, m20      ; 2 3   89ab   ghij
    vpdpbusd             m4, m5, m10
    mova                 m5, m8
    vpermb              ym1, ym16, ym21    ; 4     0123   89ab
    vpdpbusd             m5, m0, m11
    mova                ym0, ym8
    vpermb              ym6, ym18, ym21    ; 4     89ab   ghij
    vpdpbusd            ym0, ym1, ym10
    mova                ym1, ym8
    vpermb              m19, m17, m19      ; 0 1   4567   cdef
    vpdpbusd            ym1, ym6, ym11
    vpermb              m20, m17, m20      ; 2 3   4567   cdef
    vpdpbusd             m2, m19, m11
    vpdpbusd             m3, m19, m10
    vpermb             ym21, ym17, ym21    ; 4     4567   cdef
    vpdpbusd             m4, m20, m11
    vpdpbusd             m5, m20, m10
    vpdpbusd            ym0, ym21, ym11
    vpdpbusd            ym1, ym21, ym10
    packssdw             m2, m3            ; 0 1
    packssdw             m4, m5            ; 2 3
    packssdw            ym0, ym1           ; 4
    REPX       {psraw x, 2}, m2, m4, ym0
    vshufi32x4           m3, m2, m4, q1032 ; 1 2
    vshufi32x4           m0, m4, m0, q1032 ; 3 4
    punpcklwd            m1, m2, m3        ; 01 12
    punpckhwd            m2, m3
    punpcklwd            m3, m4, m0        ; 23 34
    punpckhwd            m4, m0
.hv_w16_loop:
    movu               ym19, [r5+ssq*1]
    lea                  r5, [r5+ssq*2]
    vinserti32x8        m19, [r5+ssq*0], 1
    vpermb               m6, m16, m19      ; 5 6   0123   89ab
    mova                 m5, m8
    vpermb              m20, m18, m19      ; 5 6   89ab   ghij
    vpdpbusd             m5, m6, m10
    mova                 m6, m8
    vpermb              m19, m17, m19      ; 5 6   4567   cdef
    vpdpbusd             m6, m20, m11
    mova                m20, m9
    vpdpwssd            m20, m1, m12       ; a0 b0
    mova                m21, m9
    vpdpwssd            m21, m2, m12
    vpdpbusd             m5, m19, m11
    vpdpbusd             m6, m19, m10
    vpdpwssd            m20, m3, m13       ; a1 b1
    vpdpwssd            m21, m4, m13
    packssdw             m5, m6
    mova                 m1, m3
    psraw                m5, 2             ; 5 6
    mova                 m2, m4
    vshufi32x4           m4, m0, m5, q1032 ; 4 5
    mova                 m0, m5
    punpcklwd            m3, m4, m0        ; 45 56
    punpckhwd            m4, m0
    vpdpwssd            m20, m3, m14       ; a2 b2
    vpdpwssd            m21, m4, m14
    psrad               m20, 6
    psrad               m21, 6
    packssdw            m20, m21
    mova          [r7+wq*0], ym20
    vextracti32x8 [r7+wq*1], m20, 1
    lea                  r7, [r7+wq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    add                srcq, 16
    add                tmpq, 32
    movzx                hd, r6b
    sub                 r6d, 1<<8
    jg .hv_w16_loop0
    vzeroupper
    RET

%macro PREP_8TAP_H 0
    vpermb              m10, m5, m0
    vpermb              m11, m5, m1
    vpermb              m12, m6, m0
    vpermb              m13, m6, m1
    vpermb              m14, m7, m0
    vpermb              m15, m7, m1
    mova                 m0, m4
    vpdpbusd             m0, m10, m8
    mova                 m2, m4
    vpdpbusd             m2, m12, m8
    mova                 m1, m4
    vpdpbusd             m1, m11, m8
    mova                 m3, m4
    vpdpbusd             m3, m13, m8
    vpdpbusd             m0, m12, m9
    vpdpbusd             m2, m14, m9
    vpdpbusd             m1, m13, m9
    vpdpbusd             m3, m15, m9
    packssdw             m0, m2
    packssdw             m1, m3
    psraw                m0, 2
    psraw                m1, 2
    mova        [tmpq+64*0], m0
    mova        [tmpq+64*1], m1
%endmacro

PREP_8TAP_FN smooth_sharp,   SMOOTH,  SHARP,   prep_8tap_8bpc
PREP_8TAP_FN regular_sharp,  REGULAR, SHARP,   prep_8tap_8bpc
PREP_8TAP_FN sharp,          SHARP,   SHARP

cglobal prep_8tap_8bpc, 4, 8, 0, tmp, src, stride, w, h, mx, my, stride3
    imul                mxd, mxm, 0x010101
    add                 mxd, t0d ; 8tap_h, mx, 4tap_h
    imul                myd, mym, 0x010101
    add                 myd, t1d ; 8tap_v, my, 4tap_v
    lea                  r7, [prep_avx512icl]
    movifnidn            hd, hm
    test                mxd, 0xf00
    jnz .h
    test                myd, 0xf00
    jz mangle(private_prefix %+ _prep_6tap_8bpc_avx512icl).prep
.v:
    movzx               mxd, myb ; Select 4-tap/8-tap filter multipliers.
    shr                 myd, 16  ; Note that the code is 8-tap only, having
    cmp                  hd, 4   ; a separate 4-tap code path for (4|8|16)x4
    cmove               myd, mxd ; had a negligible effect on performance.
    tzcnt               r5d, wd
    lea                 myq, [base+subpel_filters+myq*8]
    movzx               r5d, word [r7+r5*2+table_offset(prep, _8tap_v)]
    vpbroadcastd         m7, [pw_8192]
    vpbroadcastw         m8, [myq+0]
    add                  r5, r7
    vpbroadcastw         m9, [myq+2]
    lea            stride3q, [strideq*3]
    vpbroadcastw        m10, [myq+4]
    sub                srcq, stride3q
    vpbroadcastw        m11, [myq+6]
    jmp                  r5
.v_w4:
    movd               xmm0, [srcq+strideq*0]
    vpbroadcastd       ymm1, [srcq+strideq*2]
    vpbroadcastd       xmm2, [srcq+strideq*1]
    vpbroadcastd       ymm3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpblendd           ymm1, ymm0, 0x01       ; 0 2 2 _   2 _ _ _
    vpblendd           ymm3, ymm2, 0x03       ; 1 1 3 3   3 3 _ _
    vpbroadcastd       ymm0, [srcq+strideq*0]
    vpbroadcastd       ymm2, [srcq+strideq*1]
    vpblendd           ymm1, ymm0, 0x68       ; 0 2 2 4   2 4 4 _
    vpbroadcastd       ymm0, [srcq+strideq*2]
    vbroadcasti128     ymm5, [deint_shuf4]
    vpblendd           ymm3, ymm2, 0xc0       ; 1 1 3 3   3 3 5 5
    vpblendd           ymm2, ymm3, ymm1, 0x55 ; 0 1 2 3   2 3 4 5
    vpblendd           ymm3, ymm1, 0xaa       ; 1 2 3 4   3 4 5 _
    punpcklbw          ymm1, ymm2, ymm3       ; 01  12    23  34
    vpblendd           ymm3, ymm0, 0x80       ; 1 2 3 4   3 4 5 6
    punpckhbw          ymm2, ymm3             ; 23  34    45  56
.v_w4_loop:
    pinsrd             xmm0, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    vpbroadcastd       ymm3, [srcq+strideq*0]
    vpbroadcastd       ymm4, [srcq+strideq*1]
    vpblendd           ymm3, ymm4, 0x20       ; _ _ 8 _   8 9 _ _
    vpblendd           ymm3, ymm0, 0x03       ; 6 7 8 _   8 9 _ _
    vpbroadcastd       ymm0, [srcq+strideq*2]
    vpblendd           ymm3, ymm0, 0x40       ; 6 7 8 _   8 9 a _
    pshufb             ymm3, ymm5             ; 67  78    89  9a
    pmaddubsw          ymm4, ymm1, ym8
    vperm2i128         ymm1, ymm2, ymm3, 0x21 ; 45  56    67  78
    pmaddubsw          ymm2, ym9
    paddw              ymm4, ymm2
    mova               ymm2, ymm3
    pmaddubsw          ymm3, ym11
    paddw              ymm3, ymm4
    pmaddubsw          ymm4, ymm1, ym10
    paddw              ymm3, ymm4
    pmulhrsw           ymm3, ym7
    mova             [tmpq], ymm3
    add                tmpq, 32
    sub                  hd, 4
    jg .v_w4_loop
    vzeroupper
    RET
.v_w8:
    mova                 m6, [spel_v_perm8]
    movq                xm1, [srcq+strideq*0]
    mov                 r6d, 0x3e
    movq                xm2, [srcq+strideq*1]
    vpbroadcastq        ym3, [srcq+strideq*2]
    kmovb                k1, r6d
    vpbroadcastq        ym4, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpunpcklqdq      m1{k1}, m3, [srcq+strideq*0] {1to8}
    vpunpcklqdq      m2{k1}, m4, [srcq+strideq*1] {1to8}
    movq                xm0, [srcq+strideq*2]
    kshiftlb             k2, k1, 2
    shufpd               m1, m2, 0x30      ; 0 1   2 3   4 5
    vshufi32x4           m2, m1, m0, q0021 ; 2 3   4 5   6 _
    vpermb               m1, m6, m1        ; 01 12 23 34
    vpermb               m2, m6, m2        ; 23 34 45 56
.v_w8_loop:
    vpbroadcastq        ym3, [srcq+strideq*4]
    vpunpcklqdq     ym0{k1}, ym3, [srcq+stride3q] {1to4}
    lea                srcq, [srcq+strideq*4]
    vpbroadcastq         m3, [srcq+strideq*2]
    vpunpcklqdq      m0{k2}, m3, [srcq+strideq*1] {1to8}
    pmaddubsw            m4, m1, m8        ; a0 b0 c0 d0
    mova                 m1, m2
    pmaddubsw            m5, m2, m9        ; a1 b1 c1 d1
    vpermb               m2, m6, m0        ; 67 78 89 9a
    mova                xm0, xm3
    vshufi32x4           m1, m2, q1032     ; 45 56 67 78
    pmaddubsw            m3, m2, m11       ; a3 b3 c3 d3
    paddw                m4, m5
    pmaddubsw            m5, m1, m10       ; a2 b2 c2 d2
    paddw                m4, m3
    paddw                m4, m5
    pmulhrsw             m4, m7
    mova             [tmpq], m4
    add                tmpq, 64
    sub                  hd, 4
    jg .v_w8_loop
    RET
.v_w16:
    mova                m12, [spel_v_perm16b]
    vbroadcasti32x4      m1, [srcq+strideq*0]
    mov                 r6d, 0x0f
    vbroadcasti32x4     ym4, [srcq+strideq*1]
    vbroadcasti32x4      m2, [srcq+strideq*2]
    kmovb                k1, r6d
    vbroadcasti32x4     ym5, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vbroadcasti32x4      m3, [srcq+strideq*0]
    vbroadcasti32x4     ym6, [srcq+strideq*1]
    vbroadcasti32x4      m0, [srcq+strideq*2]
    vshufpd          m1{k1}, m4, m2, 0xcc
    vshufpd          m2{k1}, m5, m3, 0xcc
    vshufpd          m3{k1}, m6, m0, 0xcc
    vpermb               m1, m12, m1 ; 01 12
    vpermb               m2, m12, m2 ; 23 34
    vpermb               m3, m12, m3 ; 45 56
.v_w16_loop:
    pmaddubsw            m4, m1, m8  ; a0 b0
    mova                 m1, m3
    pmaddubsw           m13, m2, m9  ; a1 b1
    vbroadcasti32x4     ym6, [srcq+stride3q ]
    pmaddubsw            m5, m2, m8  ; c0 d0
    lea                srcq, [srcq+strideq*4]
    pmaddubsw           m14, m3, m9  ; c1 d1
    vbroadcasti32x4      m3, [srcq+strideq*0]
    vshufpd          m0{k1}, m6, m3, 0xcc
    vbroadcasti32x4     ym6, [srcq+strideq*1]
    vpermb               m2, m12, m0 ; 67 78
    vbroadcasti32x4      m0, [srcq+strideq*2]
    vshufpd          m3{k1}, m6, m0, 0xcc
    paddw                m4, m13
    pmaddubsw           m13, m1, m10 ; a2 b2
    vpermb               m3, m12, m3 ; 89 9a
    paddw                m5, m14
    pmaddubsw           m14, m2, m10 ; c2 d2
    pmaddubsw           m15, m2, m11 ; a3 b3
    pmaddubsw            m6, m3, m11 ; c3 d3
    paddw                m4, m13
    paddw                m5, m14
    paddw                m4, m15
    paddw                m5, m6
    pmulhrsw             m4, m7
    pmulhrsw             m5, m7
    mova          [tmpq+ 0], m4
    mova          [tmpq+64], m5
    add                tmpq, 64*2
    sub                  hd, 4
    jg .v_w16_loop
    RET
.v_w32:
    movshdup            m21, [bilin_v_perm64]
    movu               ym16, [srcq+strideq*0]
    movu               ym17, [srcq+strideq*1]
    movu               ym18, [srcq+strideq*2]
    add                srcq, stride3q
    movu               ym19, [srcq+strideq*0]
    vpermt2q            m16, m21, m19  ; 0 3
    movu               ym20, [srcq+strideq*1]
    vpermt2q            m17, m21, m20  ; 1 4
    movu               ym20, [srcq+strideq*2]
    add                srcq, stride3q
    vpermt2q            m18, m21, m20  ; 2 5
    movu               ym20, [srcq+strideq*0]
    vpermt2q            m19, m21, m20  ; 3 6
    punpcklbw            m0, m16, m17  ; 01
    punpcklbw            m1, m17, m18  ; 12
    punpcklbw            m2, m18, m19  ; 23
    punpckhbw            m3, m16, m17  ; 34
    punpckhbw            m4, m17, m18  ; 45
    punpckhbw            m5, m18, m19  ; 56
.v_w32_loop:
    movu               ym16, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    movu               ym17, [srcq+strideq*0]
    pmaddubsw           m14, m0, m8
    mova                 m0, m2
    pmaddubsw           m15, m1, m8
    mova                 m1, m3
    pmaddubsw            m2, m9
    vpermt2q            m16, m21, m17  ; 7 8
    pmaddubsw            m3, m9
    pmaddubsw           m12, m4, m10
    pmaddubsw           m13, m5, m10
    shufpd              m19, m16, 0x55 ; 6 7
    paddw               m14, m2
    mova                 m2, m4
    punpcklbw            m4, m19, m16  ; 67
    paddw               m15, m3
    mova                 m3, m5
    punpckhbw            m5, m19, m16  ; 78
    paddw               m14, m12
    paddw               m15, m13
    pmaddubsw           m12, m4, m11
    pmaddubsw           m13, m5, m11
    mova                m19, m16
    paddw               m14, m12
    paddw               m15, m13
    pmulhrsw            m14, m7
    pmulhrsw            m15, m7
    mova          [tmpq+ 0], m14
    mova          [tmpq+64], m15
    add                tmpq, 64*2
    sub                  hd, 2
    jg .v_w32_loop
    vzeroupper
    RET
.v_w64:
.v_w128:
    WIN64_SPILL_XMM      24
    mova                m23, [bilin_v_perm64]
    add                  wd, wd
    lea                 r6d, [hq+wq]
.v_loop0:
    vpermq              m12, m23, [srcq+strideq*0]
    vpermq              m13, m23, [srcq+strideq*1]
    lea                  r5, [srcq+strideq*2]
    vpermq              m14, m23, [r5  +strideq*0]
    vpermq              m15, m23, [r5  +strideq*1]
    lea                  r5, [r5+strideq*2]
    vpermq              m16, m23, [r5  +strideq*0]
    vpermq              m17, m23, [r5  +strideq*1]
    lea                  r5, [r5+strideq*2]
    vpermq              m18, m23, [r5  +strideq*0]
    mov                  r7, tmpq
    punpcklbw            m0, m12, m13 ; 01
    punpckhbw           m12, m13
    punpcklbw            m1, m13, m14 ; 12
    punpckhbw           m13, m14
    punpcklbw            m2, m14, m15 ; 23
    punpckhbw           m14, m15
    punpcklbw            m3, m15, m16 ; 34
    punpckhbw           m15, m16
    punpcklbw            m4, m16, m17 ; 45
    punpckhbw           m16, m17
    punpcklbw            m5, m17, m18 ; 56
    punpckhbw           m17, m18
.v_loop:
    pmaddubsw           m19, m0, m8   ; a0
    vpermq               m6, m23, [r5+strideq*1]
    pmaddubsw           m20, m12, m8
    mova                 m0, m2
    pmaddubsw            m2, m9       ; a1
    mova                m12, m14
    pmaddubsw           m14, m9
    lea                  r5, [r5+strideq*2]
    pmaddubsw           m21, m1, m8   ; b0
    pmaddubsw           m22, m13, m8
    mova                 m1, m3
    pmaddubsw            m3, m9       ; b1
    mova                m13, m15
    pmaddubsw           m15, m9
    paddw               m19, m2
    mova                 m2, m4
    pmaddubsw            m4, m10      ; a2
    paddw               m20, m14
    mova                m14, m16
    pmaddubsw           m16, m10
    paddw               m21, m3
    mova                 m3, m5
    pmaddubsw            m5, m10      ; b2
    paddw               m22, m15
    mova                m15, m17
    pmaddubsw           m17, m10
    paddw               m19, m4
    punpcklbw            m4, m18, m6  ; 67
    paddw               m20, m16
    punpckhbw           m16, m18, m6
    vpermq              m18, m23, [r5+strideq*0]
    paddw               m21, m5
    pmaddubsw            m5, m4, m11  ; a3
    paddw               m22, m17
    pmaddubsw           m17, m16, m11
    paddw               m19, m5
    punpcklbw            m5, m6, m18  ; 78
    paddw               m20, m17
    punpckhbw           m17, m6, m18
    pmaddubsw            m6, m5, m11  ; b3
    paddw               m21, m6
    pmaddubsw            m6, m17, m11
    paddw               m22, m6
    REPX   {pmulhrsw x, m7}, m19, m20, m21, m22
    mova       [r7+wq*0+ 0], m19
    mova       [r7+wq*0+64], m20
    mova       [r7+wq*1+ 0], m21
    mova       [r7+wq*1+64], m22
    lea                  r7, [r7+wq*2]
    sub                  hd, 2
    jg .v_loop
    add                srcq, 64
    add                tmpq, 128
    movzx                hd, r6b
    sub                 r6d, 1<<8
    jg .v_loop0
    RET
.h:
    RESET_STACK_STATE
    test                myd, 0xf00
    jnz .hv
.h2:
    vpbroadcastd         m4, [pd_2]
    cmp                  wd, 4
    je .h_w4
    tzcnt                wd, wd
    shr                 mxd, 16
    sub                srcq, 3
    movzx                wd, word [r7+wq*2+table_offset(prep, _8tap_h)]
    vpbroadcastd         m8, [base+subpel_filters+mxq*8+0]
    vpbroadcastd         m9, [base+subpel_filters+mxq*8+4]
    add                  wq, r7
    jmp                  wq
.h_w4:
    movzx               mxd, mxb
    vbroadcasti128      ym5, [subpel_h_shufA]
    mov                 r3d, 0x4
    dec                srcq
    vpbroadcastd        ym6, [base+subpel_filters+mxq*8+2]
    kmovb                k1, r3d
    lea            stride3q, [strideq*3]
.h_w4_loop:
    movq                xm2, [srcq+strideq*0]
    movq                xm3, [srcq+strideq*1]
    vpbroadcastq    ym2{k1}, [srcq+strideq*2]
    vpbroadcastq    ym3{k1}, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    pshufb              ym2, ym5
    pshufb              ym3, ym5
    mova                ym0, ym4
    vpdpbusd            ym0, ym2, ym6
    mova                ym1, ym4
    vpdpbusd            ym1, ym3, ym6
    packssdw            ym0, ym1
    psraw               ym0, 2
    mova             [tmpq], ym0
    add                tmpq, 32
    sub                  hd, 4
    jg .h_w4_loop
    RET
.h_w8:
    vbroadcasti128       m5, [subpel_h_shufA]
    vbroadcasti128       m6, [subpel_h_shufB]
    vbroadcasti128       m7, [subpel_h_shufC]
    lea            stride3q, [strideq*3]
.h_w8_loop:
    movu               xmm3, [srcq+strideq*0]
    vinserti128         ym3, ymm3, [srcq+strideq*1], 1
    vinserti128          m3, [srcq+strideq*2], 2
    vinserti128          m3, [srcq+stride3q ], 3
    lea                srcq, [srcq+strideq*4]
    pshufb               m1, m3, m5
    pshufb               m2, m3, m6
    mova                 m0, m4
    vpdpbusd             m0, m1, m8
    mova                 m1, m4
    vpdpbusd             m1, m2, m8
    pshufb               m3, m7
    vpdpbusd             m0, m2, m9
    vpdpbusd             m1, m3, m9
    packssdw             m0, m1
    psraw                m0, 2
    mova             [tmpq], m0
    add                tmpq, 64
    sub                  hd, 4
    jg .h_w8_loop
    RET
.h_w16:
    mova                 m5, [spel_h_perm16]
    vpbroadcastd         m7, [pb_4]
    lea            stride3q, [strideq*3]
    paddb                m6, m7, m5
    paddb                m7, m6
.h_w16_loop:
    movu                ym0, [srcq+strideq*0]
    movu                ym1, [srcq+strideq*2]
    vinserti32x8         m0, [srcq+strideq*1], 1
    vinserti32x8         m1, [srcq+stride3q ], 1
    lea                srcq, [srcq+strideq*4]
    PREP_8TAP_H
    add                tmpq, 64*2
    sub                  hd, 4
    jg .h_w16_loop
    RET
.h_w32:
    mova                 m5, [spel_h_perm32]
    vpbroadcastd         m7, [pb_4]
    paddb                m6, m7, m5
    paddb                m7, m6
.h_w32_loop:
    movu                 m0, [srcq+strideq*0]
    movu                 m1, [srcq+strideq*1]
    lea                srcq, [srcq+strideq*2]
    PREP_8TAP_H
    add                tmpq, 64*2
    sub                  hd, 2
    jg .h_w32_loop
    RET
.h_w64:
    xor                 r6d, r6d
    jmp .h_start
.h_w128:
    mov                  r6, -64*1
.h_start:
    mova                 m5, [spel_h_perm32]
    vpbroadcastd         m7, [pb_4]
    sub                srcq, r6
    paddb                m6, m7, m5
    paddb                m7, m6
.h_loop0:
    mov                  r5, r6
.h_loop:
    movu                 m0, [srcq+r5+32*0]
    movu                 m1, [srcq+r5+32*1]
    PREP_8TAP_H
    add                tmpq, 64*2
    add                  r5, 64
    jle .h_loop
    add                srcq, strideq
    dec                  hd
    jg .h_loop0
    RET
.hv:
    RESET_STACK_STATE
    vpbroadcastd         m8, [pd_2]
    vpbroadcastd         m9, [pd_32]
    cmp                  wd, 4
    jg .hv_w8
    movzx               mxd, mxb
    dec                srcq
    vpbroadcastd        m11, [base+subpel_filters+mxq*8+2]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    vpbroadcastq         m0, [base+subpel_filters+myq*8]
    lea            stride3q, [strideq*3]
    sub                srcq, stride3q
    mov                 r3d, 0x04
    kmovb                k1, r3d
    kshiftlb             k2, k1, 2
    kshiftlb             k3, k1, 4
    vbroadcasti128      m10, [subpel_h_shufA]
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    pshufd              m12, m0, q0000
    pshufd              m13, m0, q1111
    pshufd              m14, m0, q2222
    pshufd              m15, m0, q3333
    movq                xm3, [srcq+strideq*0]
    vpbroadcastq        ym2, [srcq+strideq*1]
    vpbroadcastq    ym3{k1}, [srcq+strideq*2]
    vpbroadcastq     m2{k2}, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vpbroadcastq     m3{k2}, [srcq+strideq*0]
    vpbroadcastq     m2{k3}, [srcq+strideq*1]
    vpbroadcastq     m3{k3}, [srcq+strideq*2]
    mova                 m6, [spel_hv_perm4a]
    movu                 m7, [spel_hv_perm4b]
    mova                 m0, m8
    mova                 m1, m8
    pshufb               m2, m10
    pshufb               m3, m10
    vpdpbusd             m0, m2, m11
    vpdpbusd             m1, m3, m11
    packssdw             m0, m1        ; _ 0  1 2  3 4  5 6
    psraw                m0, 2
    vpermb               m1, m6, m0    ; 01 12 23 34
    vpermb               m2, m7, m0    ; 23 34 45 56
.hv_w4_loop:
    movq                xm3, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    movq                xm4, [srcq+strideq*0]
    vpbroadcastq    ym3{k1}, [srcq+strideq*1]
    vpbroadcastq    ym4{k1}, [srcq+strideq*2]
    mova                 m5, m9
    pshufb              ym3, ym10
    vpdpwssd             m5, m1, m12   ; a0 b0 c0 d0
    mova                ym1, ym8
    pshufb              ym4, ym10
    vpdpbusd            ym1, ym3, ym11
    mova                ym3, ym8
    vpdpbusd            ym3, ym4, ym11
    vpdpwssd             m5, m2, m13   ; a1 b1 c1 d1
    packssdw            ym1, ym3       ; 7 8  9 a
    psraw               ym1, 2
    vshufi32x4           m0, m1, q1032 ; _ 4  5 6  7 8  9 a
    vpermb               m1, m6, m0    ; 45 56 67 78
    vpermb               m2, m7, m0    ; 67 78 89 9a
    vpdpwssd             m5, m1, m14   ; a2 b2 c2 d2
    vpdpwssd             m5, m2, m15   ; a3 b3 c3 d3
    psrad                m5, 6
    vpmovdw          [tmpq], m5
    add                tmpq, 32
    sub                  hd, 4
    jg .hv_w4_loop
    RET
.hv_w8:
    shr                 mxd, 16
    sub                srcq, 3
    vpbroadcastd        m10, [base+subpel_filters+mxq*8+0]
    vpbroadcastd        m11, [base+subpel_filters+mxq*8+4]
    movzx               mxd, myb
    shr                 myd, 16
    cmp                  hd, 4
    cmove               myd, mxd
    vpbroadcastq         m0, [base+subpel_filters+myq*8]
    lea            stride3q, [strideq*3]
    sub                srcq, stride3q
    punpcklbw            m0, m0
    psraw                m0, 8 ; sign-extend
    pshufd              m12, m0, q0000
    pshufd              m13, m0, q1111
    pshufd              m14, m0, q2222
    pshufd              m15, m0, q3333
    cmp                  wd, 8
    jg .hv_w16
    vbroadcasti32x4     m17, [srcq+stride3q ]
    vinserti32x4        m16, m17, [srcq+strideq*0], 0
    vbroadcasti32x4     m19, [subpel_h_shufA]
    vinserti32x4        m16, [srcq+strideq*1], 1
    vbroadcasti32x4     m21, [subpel_h_shufC]
    vinserti32x4        m16, [srcq+strideq*2], 2
    lea                srcq, [srcq+strideq*4]
    vinserti128        ym17, [srcq+strideq*0], 1
    vbroadcasti32x4     m20, [subpel_h_shufB]
    vinserti32x4        m17, [srcq+strideq*1], 2
    vinserti32x4        m17, [srcq+strideq*2], 3
    pshufb               m3, m16, m19      ; 0 1 2 3   0123
    mova                 m2, m8
    pshufb               m0, m16, m21      ; 0 1 2 3   89ab
    vpdpbusd             m2, m3, m10
    mova                 m3, m8
    pshufb               m1, m17, m19      ; 3 4 5 6   0123
    vpdpbusd             m3, m0, m11
    mova                 m0, m8
    pshufb               m4, m17, m21      ; 3 4 5 6   89ab
    vpdpbusd             m0, m1, m10
    mova                 m1, m8
    pshufb              m16, m20           ; 0 1 2 3   4567
    vpdpbusd             m1, m4, m11
    pshufb              m17, m20           ; 3 4 5 6   4567
    vpdpbusd             m2, m16, m11
    vpdpbusd             m3, m16, m10
    vpdpbusd             m0, m17, m11
    vpdpbusd             m1, m17, m10
    packssdw             m2, m3
    packssdw             m0, m1
    psraw                m2, 2             ; 0 1 2 3
    psraw                m0, 2             ; 3 4 5 6
    vshufi32x4           m4, m2, m0, q2132 ; 2 3 4 5
    vshufi32x4           m5, m2, m0, q1021 ; 1 2 3 4
    punpcklwd            m3, m4, m0        ; 23 34 45 56
    punpckhwd            m4, m0
    punpcklwd            m1, m2, m5        ; 01 12 23 34
    punpckhwd            m2, m5
.hv_w8_loop:
    movu               xm18, [srcq+stride3q ]
    lea                srcq, [srcq+strideq*4]
    vinserti128        ym18, [srcq+strideq*0], 1
    vinserti32x4        m18, [srcq+strideq*1], 2
    vinserti32x4        m18, [srcq+strideq*2], 3
    pshufb              m17, m18, m19      ; 7 8 9 a   0123
    mova                m16, m8
    pshufb               m5, m18, m21      ; 7 8 9 a   89ab
    vpdpbusd            m16, m17, m10
    mova                m17, m8
    pshufb              m18, m20           ; 7 8 9 a   4567
    vpdpbusd            m17, m5, m11
    mova                 m5, m9
    vpdpwssd             m5, m3, m13       ; a1 b1 c1 d1
    mova                 m6, m9
    vpdpwssd             m6, m4, m13
    vpdpbusd            m16, m18, m11
    vpdpbusd            m17, m18, m10
    vpdpwssd             m5, m1, m12       ; a0 b0 c0 d0
    mova                 m1, m3
    vpdpwssd             m6, m2, m12
    mova                 m2, m4
    packssdw            m16, m17
    psraw               m16, 2             ; 7 8 9 a
    valignq              m4, m16, m0, 6    ; 6 7 8 9
    mova                 m0, m16
    punpcklwd            m3, m4, m16       ; 67 78 89 9a
    punpckhwd            m4, m16
    vpdpwssd             m5, m3, m15       ; a3 b3 c3 d3
    vpdpwssd             m6, m4, m15
    vshufi32x4           m1, m3, q1032     ; 45 56 67 78
    vshufi32x4           m2, m4, q1032
    vpdpwssd             m5, m1, m14       ; a2 b2 c2 d2
    vpdpwssd             m6, m2, m14
    psrad                m5, 6
    psrad                m6, 6
    packssdw             m5, m6
    mova             [tmpq], m5
    add                tmpq, 64
    sub                  hd, 4
    jg .hv_w8_loop
    vzeroupper
    RET
.hv_w16:
    WIN64_SPILL_XMM      23
    mova                m16, [spel_h_perm16]
    vpbroadcastd        m18, [pb_4]
    add                  wd, wd
    paddb               m17, m18, m16
    lea                 r6d, [hq+wq*8-256]
    paddb               m18, m17
.hv_w16_loop0:
    movu               ym19, [srcq+strideq*0]
    vinserti32x8        m19, [srcq+strideq*1], 1
    lea                  r5, [srcq+strideq*2]
    movu               ym20, [r5  +strideq*0]
    vinserti32x8        m20, [r5  +strideq*1], 1
    lea                  r5, [r5  +strideq*2]
    movu               ym21, [r5  +strideq*0]
    vinserti32x8        m21, [r5  +strideq*1], 1
    lea                  r5, [r5  +strideq*2]
    movu               ym22, [r5  +strideq*0]
    mov                  r7, tmpq
    vpermb               m3, m16, m19      ; 0 1   0123   89ab
    mova                 m2, m8
    vpermb               m4, m18, m19      ; 0 1   89ab   ghij
    vpdpbusd             m2, m3, m10
    mova                 m3, m8
    vpermb               m5, m16, m20      ; 2 3   0123   89ab
    vpdpbusd             m3, m4, m11
    mova                 m4, m8
    vpermb               m6, m18, m20      ; 2 3   89ab   ghij
    vpdpbusd             m4, m5, m10
    mova                 m5, m8
    vpermb               m7, m16, m21      ; 4 5   0123   89ab
    vpdpbusd             m5, m6, m11
    mova                 m6, m8
    vpermb               m0, m18, m21      ; 4 5   89ab   ghij
    vpdpbusd             m6, m7, m10
    mova                 m7, m8
    vpermb              ym1, ym16, ym22    ; 6     0123   89ab
    vpdpbusd             m7, m0, m11
    mova                ym0, ym8
    vpermb              m19, m17, m19      ; 0 1   4567   cdef
    vpdpbusd            ym0, ym1, ym10
    vpermb              ym1, ym18, ym22    ; 6     89ab   ghij
    vpdpbusd             m2, m19, m11
    vpdpbusd             m3, m19, m10
    mova               ym19, ym8
    vpermb              m20, m17, m20      ; 2 3   4567   cdef
    vpdpbusd           ym19, ym1, ym11
    vpermb              m21, m17, m21      ; 4 5   4567   cdef
    vpdpbusd             m4, m20, m11
    vpdpbusd             m5, m20, m10
    vpermb             ym22, ym17, ym22    ; 6     4567   cdef
    vpdpbusd             m6, m21, m11
    vpdpbusd             m7, m21, m10
    packssdw             m2, m3            ; 0 1
    vpdpbusd            ym0, ym22, ym11
    packssdw             m4, m5            ; 2 3
    vpdpbusd           ym19, ym22, ym10
    packssdw             m6, m7            ; 4 5
    packssdw            ym0, ym19          ; 6
    REPX       {psraw x, 2}, m2, m4, m6, ym0
    vshufi32x4           m3, m2, m4, q1032 ; 1 2
    vshufi32x4           m5, m4, m6, q1032 ; 3 4
    vshufi32x4           m0, m6, m0, q1032 ; 5 6
    punpcklwd            m1, m2, m3  ; 01 12
    punpckhwd            m2, m3
    punpcklwd            m3, m4, m5  ; 23 34
    punpckhwd            m4, m5
    punpcklwd            m5, m6, m0  ; 45 56
    punpckhwd            m6, m0
.hv_w16_loop:
    movu               ym19, [r5+strideq*1]
    lea                  r5, [r5+strideq*2]
    vinserti32x8        m19, [r5+strideq*0], 1
    mova                m20, m9
    vpdpwssd            m20, m1, m12 ; a0
    vpermb               m1, m16, m19
    mova                m21, m9
    vpdpwssd            m21, m2, m12 ; b0
    vpermb               m2, m17, m19
    mova                m22, m8
    vpdpbusd            m22, m1, m10
    mova                 m1, m8
    vpermb              m19, m18, m19
    vpdpbusd             m1, m2, m10
    vpdpwssd            m20, m3, m13 ; a1
    vpdpwssd            m21, m4, m13 ; b1
    vpdpbusd            m22, m2, m11
    mova                 m2, m4
    vpdpbusd             m1, m19, m11
    mova                 m4, m6
    vpdpwssd            m20, m5, m14 ; a2
    vpdpwssd            m21, m6, m14 ; b2
    packssdw            m22, m1
    mova                 m1, m3
    psraw               m22, 2              ; 7 8
    mova                 m3, m5
    vshufi32x4           m6, m0, m22, q1032 ; 6 7
    mova                 m0, m22
    punpcklwd            m5, m6, m0  ; 67 78
    punpckhwd            m6, m0
    vpdpwssd            m20, m5, m15 ; a3
    vpdpwssd            m21, m6, m15 ; b3
    psrad               m20, 6
    psrad               m21, 6
    packssdw            m20, m21
    mova          [r7+wq*0], ym20
    vextracti32x8 [r7+wq*1], m20, 1
    lea                  r7, [r7+wq*2]
    sub                  hd, 2
    jg .hv_w16_loop
    add                srcq, 16
    add                tmpq, 32
    movzx                hd, r6b
    sub                 r6d, 1<<8
    jg .hv_w16_loop0
    RET

cglobal warp_affine_8x8t_8bpc, 4, 7, 22, tmp, ts
    vpbroadcastd         m9, [pd_16384]
    mova               ym15, [warp_8x8t_end]
    call mangle(private_prefix %+ _warp_affine_8x8_8bpc_avx512icl).main
    jmp .start
.loop:
    call mangle(private_prefix %+ _warp_affine_8x8_8bpc_avx512icl).main2
    lea                tmpq, [tmpq+tsq*4]
.start:
    paddd               m16, m16
    vpermb              m16, m15, m16
    mova         [tmpq+tsq*0], xm16
    vextracti128 [tmpq+tsq*2], ym16, 1
    sub                 r6d, 0x1800
    jg .loop
    RET

cglobal warp_affine_8x8_8bpc, 4, 7, 22, dst, ds, src, ss, abcd, filter
    vpbroadcastd         m9, [pd_262144]
    mova               xm15, [warp_8x8_end]
    call .main
    jmp .start
.loop:
    call .main2
    lea                dstq, [dstq+dsq*2]
.start:
    psrad               m16, 19
    packuswb            m16, m16
    vpermb              m16, m15, m16
    movq       [dstq+dsq*0], xm16
    movhps     [dstq+dsq*1], xm16
    sub                 r6d, 0x1800
    jg .loop
    RET
ALIGN function_align
.main:
    vpbroadcastd         m1, [pd_512]
%if WIN64
    mov               abcdq, r5mp
    vpaddd             ym18, ym1, r6m {1to8} ; mx
%else
    add                 r5d, 512
    vpbroadcastd       ym18, r5d
%endif
    vpaddd             ym20, ym1, r7m {1to8} ; my
    mova               ym16, [pd_0to7]
    vpbroadcastd       ym19, [abcdq+4*0]
    vpbroadcastd       ym21, [abcdq+4*1]
    lea                  r4, [ssq*3+3]
    mova                m10, [warp_8x8_permA]
    mov                 r6d, 0x5555
    mova                m11, [warp_8x8_permB]
    lea             filterq, [mc_warp_filter+64*8]
    vpbroadcastq        m12, [warp_8x8_hpack]
    sub                srcq, r4               ; src -= src_stride*3 + 3
    vbroadcasti32x4     m13, [warp_8x8_permC]
    kxnorb               k2, k2, k2
    vbroadcasti32x4     m14, [warp_8x8_permD]
    vpdpwssd           ym18, ym19, ym16       ; alpha
    vpdpwssd           ym20, ym21, ym16       ; gamma
    vbroadcasti32x4      m0, [srcq]
    psrad              ym19, 16               ; beta
    psrad              ym21, 16               ; delta
    kmovw                k1, r6d
    psrad              ym16, ym18, 10
    kmovb                k3, k2
    paddd              ym18, ym19
    vpgatherdq       m2{k2}, [filterq+ym16*8] ; filter_x0
    psrld                m1, 8                ; pd_2
    pshufb               m0, m11
    paddd                m8, m1, m1           ; pd_4
    vpdpbusd             m1, m0, m2
    call .h
    psllq                m2, m1, 45
    pslld                m1, 13
    paddd                m1, m2
    vpshrdq              m1, m0, 48           ; 01 12
    call .h
    vpshrdq              m2, m1, m0, 48       ; 23 34
    call .h
    vpshrdq              m3, m2, m0, 48       ; 45 56
.main2:
    call .h
    psrad              ym17, ym20, 10
    kmovb                k2, k3
    paddd              ym20, ym21
    vpgatherdq       m7{k3}, [filterq+ym17*8] ; filter_y0
    psrad              ym16, ym20, 10
    kmovb                k3, k2
    paddd              ym20, ym21
    vpgatherdq      m17{k2}, [filterq+ym16*8] ; filter_y1
    shufps               m5, m7, m17, q2020   ; a0 a1 a2 a3 b0 b1 b2 b3 A0 A1 A2 A3 B0 B1 B2 B3
    mova                m16, m9
    pshufb               m4, m5, m13          ;    a0    a1    A0    A1    b0    b1    B0    B1
    vpdpwssd            m16, m1, m4
    pshufb               m5, m14              ;    a2    a3    A2    A3    b2    b3    B2    B3
    mova                 m1, m2
    vpdpwssd            m16, m2, m5
    shufps               m5, m7, m17, q3131   ; a4 a5 a6 a7 b4 b5 b6 b7 A4 A5 A6 A7 B4 B5 B6 B7
    mova                 m2, m3
    pshufb               m4, m5, m13          ;    a4    a5    A4    A5    b4    b5    B4    B5
    vpdpwssd            m16, m3, m4
    vpshrdq              m3, m0, 48           ; 67 78
    pshufb               m5, m14              ;    a6    a7    A6    A7    b6    b7    B6    B7
    vpdpwssd            m16, m3, m5
    ret
ALIGN function_align
.h:
    movu                xm5, [srcq+ssq*1]
    psrad              ym16, ym18, 10
    lea                srcq, [srcq+ssq*2]
    vinserti32x4        ym5, [srcq+ssq*0], 1
    kmovb                k2, k3
    paddd              ym18, ym19
    vpgatherdq       m6{k3}, [filterq+ym16*8] ; filter_x1
    psrad              ym17, ym18, 10
    kmovb                k3, k2
    paddd              ym18, ym19
    vpgatherdq      m16{k2}, [filterq+ym17*8] ; filter_x2
    mova                 m0, m8
    vpermb               m4, m10, m5          ; a4 b0 a5 b1   a6 b2 a7 b3   a8 b4 a9 b5   aa b6 ab b7
    vpshldq             m17, m16, m6, 32      ; a4 a5 a6 a7   b0 b1 b2 b3
    vpdpbusd             m0, m4, m17
    vpermb               m5, m11, m5          ; a0 b4 a1 b5   a2 b6 a3 b7   a4 b8 a5 b9   a6 ba a7 bb
    vmovdqa32       m16{k1}, m6               ; a0 a1 a2 a3   b4 b5 b6 b7
    vpdpbusd             m0, m5, m16
    vpmultishiftqb       m0, m12, m0          ; 1 1 2 2 (>> 3)
    ret

%macro BIDIR_FN 1 ; op
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    cmp                  hd, 8
    jg .w4_h16
    WRAP_YMM %1           0
    vextracti32x4       xm1, ym0, 1
    movd   [dstq          ], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    jl .w4_ret
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq          ], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
.w4_ret:
    RET
.w4_h16:
    vpbroadcastd         m7, strided
    pmulld               m7, [bidir_sctr_w4]
    %1                    0
    kxnorw               k1, k1, k1
    vpscatterdd [dstq+m7]{k1}, m0
    RET
.w8:
    cmp                  hd, 4
    jne .w8_h8
    WRAP_YMM %1           0
    vextracti32x4       xm1, ym0, 1
    movq   [dstq          ], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    RET
.w8_loop:
    %1_INC_PTR            2
    lea                dstq, [dstq+strideq*4]
.w8_h8:
    %1                    0
    vextracti32x4       xm1, ym0, 1
    vextracti32x4       xm2, m0, 2
    vextracti32x4       xm3, m0, 3
    movq   [dstq          ], xm0
    movq   [dstq+strideq*1], xm1
    movq   [dstq+strideq*2], xm2
    movq   [dstq+stride3q ], xm3
    lea                dstq, [dstq+strideq*4]
    movhps [dstq          ], xm0
    movhps [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm3
    sub                  hd, 8
    jg .w8_loop
    RET
.w16_loop:
    %1_INC_PTR            2
    lea                dstq, [dstq+strideq*4]
.w16:
    %1                    0
    vpermq               m0, m0, q3120
    mova          [dstq          ], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    sub                  hd, 4
    jg .w16_loop
    RET
.w32:
    pmovzxbq             m7, [pb_02461357]
.w32_loop:
    %1                    0
    %1_INC_PTR            2
    vpermq               m0, m7, m0
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_loop
    RET
.w64:
    pmovzxbq             m7, [pb_02461357]
.w64_loop:
    %1                    0
    %1_INC_PTR            2
    vpermq               m0, m7, m0
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET
.w128:
    pmovzxbq             m7, [pb_02461357]
.w128_loop:
    %1                    0
    vpermq               m6, m7, m0
    %1                    2
    mova        [dstq+64*0], m6
    %1_INC_PTR            4
    vpermq               m6, m7, m0
    mova        [dstq+64*1], m6
    add                dstq, strideq
    dec                  hd
    jg .w128_loop
    RET
%endmacro

%macro AVG 1 ; src_offset
    mova                 m0, [tmp1q+(%1+0)*mmsize]
    paddw                m0, [tmp2q+(%1+0)*mmsize]
    mova                 m1, [tmp1q+(%1+1)*mmsize]
    paddw                m1, [tmp2q+(%1+1)*mmsize]
    pmulhrsw             m0, m4
    pmulhrsw             m1, m4
    packuswb             m0, m1
%endmacro

%macro AVG_INC_PTR 1
    add               tmp1q, %1*mmsize
    add               tmp2q, %1*mmsize
%endmacro

cglobal avg_8bpc, 4, 7, 5, dst, stride, tmp1, tmp2, w, h, stride3
%define base r6-avg_avx512icl_table
    lea                  r6, [avg_avx512icl_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, dword [r6+wq*4]
    vpbroadcastd         m4, [base+pw_1024]
    add                  wq, r6
    BIDIR_FN            AVG

%macro W_AVG 1 ; src_offset
    ; (a * weight + b * (16 - weight) + 128) >> 8
    ; = ((a - b) * weight + (b << 4) + 128) >> 8
    ; = ((((a - b) * ((weight-16) << 12)) >> 16) + a + 8) >> 4
    ; = ((((b - a) * (-weight     << 12)) >> 16) + b + 8) >> 4
    mova                 m0,     [tmp1q+(%1+0)*mmsize]
    psubw                m2, m0, [tmp2q+(%1+0)*mmsize]
    mova                 m1,     [tmp1q+(%1+1)*mmsize]
    psubw                m3, m1, [tmp2q+(%1+1)*mmsize]
    pmulhw               m2, m4
    pmulhw               m3, m4
    paddw                m0, m2
    paddw                m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
%endmacro

%define W_AVG_INC_PTR AVG_INC_PTR

cglobal w_avg_8bpc, 4, 7, 6, dst, stride, tmp1, tmp2, w, h, stride3
%define base r6-w_avg_avx512icl_table
    lea                  r6, [w_avg_avx512icl_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    vpbroadcastw         m4, r6m ; weight
    movsxd               wq, dword [r6+wq*4]
    vpbroadcastd         m5, [base+pw_2048]
    psllw                m4, 12 ; (weight-16) << 12 when interpreted as signed
    add                  wq, r6
    cmp           dword r6m, 7
    jg .weight_gt7
    mov                  r6, tmp1q
    pxor                 m0, m0
    mov               tmp1q, tmp2q
    psubw                m4, m0, m4 ; -weight
    mov               tmp2q, r6
.weight_gt7:
    BIDIR_FN          W_AVG

%macro MASK 1 ; src_offset
    ; (a * m + b * (64 - m) + 512) >> 10
    ; = ((a - b) * m + (b << 6) + 512) >> 10
    ; = ((((b - a) * (-m << 10)) >> 16) + b + 8) >> 4
%if mmsize == 64
    vpermq               m3, m8, [maskq+%1*32]
%else
    vpermq               m3,     [maskq+%1*16], q3120
%endif
    mova                 m0,     [tmp2q+(%1+0)*mmsize]
    psubw                m1, m0, [tmp1q+(%1+0)*mmsize]
    psubb                m3, m4, m3
    paddw                m1, m1     ; (b - a) << 1
    paddb                m3, m3
    punpcklbw            m2, m4, m3 ; -m << 9
    pmulhw               m1, m2
    paddw                m0, m1
    mova                 m1,     [tmp2q+(%1+1)*mmsize]
    psubw                m2, m1, [tmp1q+(%1+1)*mmsize]
    paddw                m2, m2
    punpckhbw            m3, m4, m3
    pmulhw               m2, m3
    paddw                m1, m2
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
%endmacro

%macro MASK_INC_PTR 1
    add               maskq, %1*32
    add               tmp2q, %1*64
    add               tmp1q, %1*64
%endmacro

cglobal mask_8bpc, 4, 8, 6, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-mask_avx512icl_table
    lea                  r7, [mask_avx512icl_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    mov               maskq, maskmp
    movsxd               wq, dword [r7+wq*4]
    pxor                 m4, m4
    mova                 m8, [base+bilin_v_perm64]
    vpbroadcastd         m5, [base+pw_2048]
    add                  wq, r7
    BIDIR_FN           MASK

%macro W_MASK 4-5 0 ; dst, mask, tmp_offset[1-2], 4:4:4
    mova                m%1, [tmp1q+mmsize*%3]
    mova                 m1, [tmp2q+mmsize*%3]
    psubw                m1, m%1
    pabsw               m%2, m1
    psubusw             m%2, m6, m%2
    psrlw               m%2, 8 ; 64 - m
    psllw                m2, m%2, 10
    pmulhw               m1, m2
    paddw               m%1, m1
    mova                 m1, [tmp1q+mmsize*%4]
    mova                 m2, [tmp2q+mmsize*%4]
    psubw                m2, m1
    pabsw                m3, m2
    psubusw              m3, m6, m3
    vpshldw             m%2, m3, 8
    psllw                m3, m%2, 10
%if %5
    psubb               m%2, m5, m%2
%endif
    pmulhw               m2, m3
    paddw                m1, m2
    pmulhrsw            m%1, m7
    pmulhrsw             m1, m7
    packuswb            m%1, m1
%endmacro

cglobal w_mask_420_8bpc, 4, 8, 16, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_420_avx512icl_table
    lea                  r7, [w_mask_420_avx512icl_table]
    tzcnt                wd, wm
    mov                 r6d, r7m ; sign
    movifnidn            hd, hm
    movsxd               wq, [r7+wq*4]
    vpbroadcastd         m6, [base+pw_6903] ; ((64 - 38) << 8) + 255 - 8
    vpbroadcastd         m7, [base+pw_2048]
    vpbroadcastd         m9, [base+pb_m64]             ; -1 << 6
    mova               ym10, [base+wm_420_mask+32]
    vpbroadcastd         m8, [base+wm_sign+r6*8] ; (258 - sign) << 6
    add                  wq, r7
    mov               maskq, maskmp
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    mova                 m5, [wm_420_perm4]
    cmp                  hd, 8
    jg .w4_h16
    WRAP_YMM W_MASK       0, 4, 0, 1
    vinserti128         ym5, [wm_420_perm4+32], 1
    vpermb              ym4, ym5, ym4
    vpdpbusd            ym8, ym4, ym9
    vextracti32x4       xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
.w4_end:
    vpermb              ym8, ym10, ym8
    movq            [maskq], xm8
    RET
.w4_h16:
    vpbroadcastd        m11, strided
    pmulld              m11, [bidir_sctr_w4]
    W_MASK                0, 4, 0, 1
    vpermb               m4, m5, m4
    vpdpbusd             m8, m4, m9
    kxnorw               k1, k1, k1
    vpermb               m8, m10, m8
    mova            [maskq], xm8
    vpscatterdd [dstq+m11]{k1}, m0
    RET
.w8:
    mova                 m5, [wm_420_perm8]
    cmp                  hd, 4
    jne .w8_h8
    WRAP_YMM W_MASK       0, 4, 0, 1
    vinserti128         ym5, [wm_420_perm8+32], 1
    vpermb              ym4, ym5, ym4
    vpdpbusd            ym8, ym4, ym9
    vpermb               m8, m10, m8
    mova            [maskq], xm8
    vextracti32x4       xm1, ym0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    RET
.w8_loop:
    add               tmp1q, 128
    add               tmp2q, 128
    add               maskq, 16
    lea                dstq, [dstq+strideq*4]
.w8_h8:
    W_MASK                0, 4, 0, 1
    vpermb               m4, m5, m4
    mova                 m1, m8
    vpdpbusd             m1, m4, m9
    vpermb               m1, m10, m1
    mova            [maskq], xm1
    vextracti32x4       xm1, ym0, 1
    vextracti32x4       xm2, m0, 2
    vextracti32x4       xm3, m0, 3
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movq   [dstq+strideq*2], xm2
    movq   [dstq+stride3q ], xm3
    lea                dstq, [dstq+strideq*4]
    movhps [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm3
    sub                  hd, 8
    jg .w8_loop
    RET
.w16:
    mova                 m5, [wm_420_perm16]
.w16_loop:
    W_MASK                0, 4, 0, 1
    vpermb               m4, m5, m4
    mova                 m1, m8
    vpdpbusd             m1, m4, m9
    add               tmp1q, 128
    add               tmp2q, 128
    vpermb               m1, m10, m1
    vpermq               m0, m0, q3120
    mova            [maskq], xm1
    add               maskq, 16
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    lea                dstq, [dstq+strideq*4]
    sub                  hd, 4
    jg .w16_loop
    RET
.w32:
    pmovzxbq             m5, [pb_02461357]
.w32_loop:
    W_MASK                0, 4, 0, 1
    mova                 m1, m8
    vpdpbusd             m1, m4, m9
    add               tmp1q, 128
    add               tmp2q, 128
    vpermb               m1, m10, m1
    vpermq               m0, m5, m0
    mova            [maskq], xm1
    add               maskq, 16
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_loop
    RET
.w64:
    pmovzxbq            m12, [wm_420_perm64] ; 0, 2, 4, 6, 8, 10, 12, 14
    psrlq               m13, m12, 4          ; 1, 3, 5, 7, 9, 11, 13, 15
.w64_loop:
    W_MASK                0, 4, 0, 2
    W_MASK               11, 5, 1, 3
    mova                 m2, m8
    vpdpbusd             m2, m4, m9
    mova                 m3, m8
    vpdpbusd             m3, m5, m9
    add               tmp1q, 256
    add               tmp2q, 256
    vpermt2b             m2, m10, m3
    mova                 m1, m0
    vpermt2q             m0, m12, m11
    vpermt2q             m1, m13, m11
    mova            [maskq], ym2
    add               maskq, 32
    mova   [dstq+strideq*0], m0
    mova   [dstq+strideq*1], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w64_loop
    RET
.w128:
    pmovzxbq            m14, [wm_420_perm64]
    mova                m10, [wm_420_mask]
    psrlq               m15, m14, 4
.w128_loop:
    W_MASK                0, 12, 0, 4
    W_MASK               11, 13, 1, 5
    mova                 m4, m8
    vpdpbusd             m4, m12, m9
    mova                 m5, m8
    vpdpbusd             m5, m13, m9
    mova                 m1, m0
    vpermt2q             m0, m14, m11
    vpermt2q             m1, m15, m11
    mova [dstq+strideq*0+64*0], m0
    mova [dstq+strideq*1+64*0], m1
    W_MASK                0, 12, 2, 6
    W_MASK               11, 13, 3, 7
    vprold               m4, 16
    vprold               m5, 16
    vpdpbusd             m4, m12, m9
    vpdpbusd             m5, m13, m9
    add               tmp1q, 512
    add               tmp2q, 512
    vpermt2b             m4, m10, m5
    mova                 m1, m0
    vpermt2q             m0, m14, m11
    vpermt2q             m1, m15, m11
    mova            [maskq], m4
    add               maskq, 64
    mova [dstq+strideq*0+64*1], m0
    mova [dstq+strideq*1+64*1], m1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w128_loop
    RET

cglobal w_mask_422_8bpc, 4, 8, 14, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_422_avx512icl_table
    lea                  r7, [w_mask_422_avx512icl_table]
    tzcnt                wd, wm
    mov                 r6d, r7m ; sign
    movifnidn            hd, hm
    movsxd               wq, dword [r7+wq*4]
    vpbroadcastd         m6, [base+pw_6903] ; ((64 - 38) << 8) + 255 - 8
    vpbroadcastd         m7, [base+pw_2048]
    vpbroadcastd         m9, [base+pw_m128]
    mova                m10, [base+wm_422_mask]
    vpbroadcastd        m11, [base+pb_127]
    add                  wq, r7
    vpbroadcastd         m8, [base+wm_sign+4+r6*4]
    mov               maskq, maskmp
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    cmp                  hd, 8
    jg .w4_h16
    WRAP_YMM W_MASK       0, 4, 0, 1
    movhps             xm10, [wm_422_mask+16]
    vpdpwssd            ym8, ym4, ym9
    vpermb              ym8, ym10, ym8
    vextracti32x4       xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
.w4_end:
    pand                xm8, xm11
    mova            [maskq], xm8
    RET
.w4_h16:
    vpbroadcastd         m5, strided
    pmulld               m5, [bidir_sctr_w4]
    W_MASK                0, 4, 0, 1
    vpdpwssd             m8, m4, m9
    kxnorw               k1, k1, k1
    vpermb               m8, m10, m8
    pand                ym8, ym11
    mova            [maskq], ym8
    vpscatterdd [dstq+m5]{k1}, m0
    RET
.w8:
    cmp                  hd, 4
    jne .w8_h8
    WRAP_YMM W_MASK       0, 4, 0, 1
    movhps             xm10, [wm_422_mask+16]
    vpdpwssd            ym8, ym4, ym9
    vpermb              ym8, ym10, ym8
    pand                xm8, xm11
    mova            [maskq], xm8
    vextracti32x4       xm1, ym0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    RET
.w8_loop:
    add               tmp1q, 128
    add               tmp2q, 128
    add               maskq, 32
    lea                dstq, [dstq+strideq*4]
.w8_h8:
    W_MASK                0, 4, 0, 1
    mova                 m1, m8
    vpdpwssd             m1, m4, m9
    vpermb               m1, m10, m1
    pand                ym1, ym11
    mova            [maskq], ym1
    vextracti32x4       xm1, ym0, 1
    vextracti32x4       xm2, m0, 2
    vextracti32x4       xm3, m0, 3
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movq   [dstq+strideq*2], xm2
    movq   [dstq+stride3q ], xm3
    lea                dstq, [dstq+strideq*4]
    movhps [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm3
    sub                  hd, 8
    jg .w8_loop
    RET
.w16_loop:
    add               tmp1q, 128
    add               tmp2q, 128
    add               maskq, 32
    lea                dstq, [dstq+strideq*4]
.w16:
    W_MASK                0, 4, 0, 1
    mova                 m1, m8
    vpdpwssd             m1, m4, m9
    vpermb               m1, m10, m1
    vpermq               m0, m0, q3120
    pand                ym1, ym11
    mova            [maskq], ym1
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    sub                  hd, 4
    jg .w16_loop
    RET
.w32:
    pmovzxbq             m5, [pb_02461357]
.w32_loop:
    W_MASK                0, 4, 0, 1
    mova                 m1, m8
    vpdpwssd             m1, m4, m9
    add               tmp1q, 128
    add               tmp2q, 128
    vpermb               m1, m10, m1
    vpermq               m0, m5, m0
    pand                ym1, ym11
    mova            [maskq], ym1
    add               maskq, 32
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_loop
    RET
.w64:
    pmovzxbq             m5, [pb_02461357]
.w64_loop:
    W_MASK                0, 4, 0, 1
    mova                 m1, m8
    vpdpwssd             m1, m4, m9
    add               tmp1q, 128
    add               tmp2q, 128
    vpermb               m1, m10, m1
    vpermq               m0, m5, m0
    pand                ym1, ym11
    mova            [maskq], ym1
    add               maskq, 32
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET
.w128:
    pmovzxbq            m13, [pb_02461357]
.w128_loop:
    W_MASK                0, 4, 0, 1
    W_MASK               12, 5, 2, 3
    mova                 m2, m8
    vpdpwssd             m2, m4, m9
    mova                 m3, m8
    vpdpwssd             m3, m5, m9
    add               tmp1q, 256
    add               tmp2q, 256
    vpermt2b             m2, m10, m3
    vpermq               m0, m13, m0
    vpermq               m1, m13, m12
    pand                 m2, m11
    mova            [maskq], m2
    add               maskq, 64
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    add                dstq, strideq
    dec                  hd
    jg .w128_loop
    RET

cglobal w_mask_444_8bpc, 4, 8, 12, dst, stride, tmp1, tmp2, w, h, mask, stride3
%define base r7-w_mask_444_avx512icl_table
    lea                  r7, [w_mask_444_avx512icl_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, dword [r7+wq*4]
    vpbroadcastd         m6, [base+pw_6903] ; ((64 - 38) << 8) + 255 - 8
    vpbroadcastd         m5, [base+pb_64]
    vpbroadcastd         m7, [base+pw_2048]
    mova                 m8, [base+wm_444_mask]
    add                  wq, r7
    mov               maskq, maskmp
    lea            stride3q, [strideq*3]
    jmp                  wq
.w4:
    cmp                  hd, 8
    jg .w4_h16
    WRAP_YMM W_MASK       0, 4, 0, 1, 1
    vinserti128         ym8, [wm_444_mask+32], 1
    vpermb              ym4, ym8, ym4
    mova            [maskq], ym4
    vextracti32x4      xm1, m0, 1
    movd   [dstq+strideq*0], xm0
    pextrd [dstq+strideq*1], xm0, 1
    movd   [dstq+strideq*2], xm1
    pextrd [dstq+stride3q ], xm1, 1
    jl .w4_end
    lea                dstq, [dstq+strideq*4]
    pextrd [dstq+strideq*0], xm0, 2
    pextrd [dstq+strideq*1], xm0, 3
    pextrd [dstq+strideq*2], xm1, 2
    pextrd [dstq+stride3q ], xm1, 3
.w4_end:
    RET
.w4_h16:
    vpbroadcastd         m9, strided
    pmulld               m9, [bidir_sctr_w4]
    W_MASK                0, 4, 0, 1, 1
    vpermb               m4, m8, m4
    kxnorw               k1, k1, k1
    mova            [maskq], m4
    vpscatterdd [dstq+m9]{k1}, m0
    RET
.w8:
    cmp                  hd, 4
    jne .w8_h8
    WRAP_YMM W_MASK       0, 4, 0, 1, 1
    vinserti128         ym8, [wm_444_mask+32], 1
    vpermb              ym4, ym8, ym4
    mova            [maskq], ym4
    vextracti32x4       xm1, ym0, 1
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm0
    movhps [dstq+stride3q ], xm1
    RET
.w8_loop:
    add               tmp1q, 128
    add               tmp2q, 128
    add               maskq, 64
    lea                dstq, [dstq+strideq*4]
.w8_h8:
    W_MASK                0, 4, 0, 1, 1
    vpermb               m4, m8, m4
    mova            [maskq], m4
    vextracti32x4       xm1, ym0, 1
    vextracti32x4       xm2, m0, 2
    vextracti32x4       xm3, m0, 3
    movq   [dstq+strideq*0], xm0
    movq   [dstq+strideq*1], xm1
    movq   [dstq+strideq*2], xm2
    movq   [dstq+stride3q ], xm3
    lea                dstq, [dstq+strideq*4]
    movhps [dstq+strideq*0], xm0
    movhps [dstq+strideq*1], xm1
    movhps [dstq+strideq*2], xm2
    movhps [dstq+stride3q ], xm3
    sub                  hd, 8
    jg .w8_loop
    RET
.w16_loop:
    add               tmp1q, 128
    add               tmp2q, 128
    add               maskq, 64
    lea                dstq, [dstq+strideq*4]
.w16:
    W_MASK                0, 4, 0, 1, 1
    vpermb               m4, m8, m4
    vpermq               m0, m0, q3120
    mova            [maskq], m4
    mova          [dstq+strideq*0], xm0
    vextracti32x4 [dstq+strideq*1], m0, 2
    vextracti32x4 [dstq+strideq*2], ym0, 1
    vextracti32x4 [dstq+stride3q ], m0, 3
    sub                  hd, 4
    jg .w16_loop
    RET
.w32:
    pmovzxbq             m9, [pb_02461357]
.w32_loop:
    W_MASK                0, 4, 0, 1, 1
    vpermb               m4, m8, m4
    add               tmp1q, 128
    add               tmp2q, 128
    vpermq               m0, m9, m0
    mova            [maskq], m4
    add               maskq, 64
    mova          [dstq+strideq*0], ym0
    vextracti32x8 [dstq+strideq*1], m0, 1
    lea                dstq, [dstq+strideq*2]
    sub                  hd, 2
    jg .w32_loop
    RET
.w64:
    pmovzxbq             m9, [pb_02461357]
.w64_loop:
    W_MASK                0, 4, 0, 1, 1
    vpermb               m4, m8, m4
    add               tmp1q, 128
    add               tmp2q, 128
    vpermq               m0, m9, m0
    mova            [maskq], m4
    add               maskq, 64
    mova             [dstq], m0
    add                dstq, strideq
    dec                  hd
    jg .w64_loop
    RET
.w128:
    pmovzxbq            m11, [pb_02461357]
.w128_loop:
    W_MASK                0, 4, 0, 1, 1
    W_MASK               10, 9, 2, 3, 1
    vpermb               m4, m8, m4
    vpermb               m9, m8, m9
    add               tmp1q, 256
    add               tmp2q, 256
    vpermq               m0, m11, m0
    vpermq              m10, m11, m10
    mova       [maskq+64*0], m4
    mova       [maskq+64*1], m9
    add               maskq, 128
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m10
    add                dstq, strideq
    dec                  hd
    jg .w128_loop
    RET

cglobal blend_8bpc, 3, 7, 8, dst, ds, tmp, w, h, mask
%define base r6-blend_avx512icl_table
    lea                  r6, [blend_avx512icl_table]
    tzcnt                wd, wm
    movifnidn         maskq, maskmp
    movifnidn            hd, hm
    movsxd               wq, [r6+wq*4]
    vpbroadcastd         m6, [base+pb_64]
    vpbroadcastd         m7, [base+pw_512]
    sub                tmpq, maskq
    add                  wq, r6
    lea                  r6, [dsq*3]
    jmp                  wq
.w4:
    movd               xmm0, [dstq+dsq*0]
    pinsrd             xmm0, [dstq+dsq*1], 1
    vpbroadcastd       xmm1, [dstq+dsq*2]
    pinsrd             xmm1, [dstq+r6   ], 3
    mova               xmm4, [maskq]
    mova               xmm5, [maskq+tmpq]
    add               maskq, 4*4
    psubb              xmm3, xm6, xmm4
    punpcklbw          xmm0, xmm5
    punpcklbw          xmm2, xmm3, xmm4
    punpckhbw          xmm1, xmm5
    punpckhbw          xmm3, xmm4
    pmaddubsw          xmm0, xmm2
    pmaddubsw          xmm1, xmm3
    pmulhrsw           xmm0, xm7
    pmulhrsw           xmm1, xm7
    packuswb           xmm0, xmm1
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    pextrd     [dstq+dsq*2], xmm0, 2
    pextrd     [dstq+r6   ], xmm0, 3
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w4
    RET
.w8:
    movq               xmm0, [dstq+dsq*0]
    vpbroadcastq       xmm1, [dstq+dsq*1]
    vpbroadcastq       ymm2, [dstq+dsq*2]
    vpbroadcastq       ymm3, [dstq+r6   ]
    mova               ymm4, [maskq]
    mova               ymm5, [maskq+tmpq]
    add               maskq, 8*4
    vpblendd           ymm0, ymm2, 0x30
    vpblendd           ymm1, ymm3, 0xc0
    psubb              ymm3, ym6, ymm4
    punpcklbw          ymm0, ymm5
    punpcklbw          ymm2, ymm3, ymm4
    punpckhbw          ymm1, ymm5
    punpckhbw          ymm3, ymm4
    pmaddubsw          ymm0, ymm2
    pmaddubsw          ymm1, ymm3
    pmulhrsw           ymm0, ym7
    pmulhrsw           ymm1, ym7
    packuswb           ymm0, ymm1
    vextracti128       xmm1, ymm0, 1
    movq       [dstq+dsq*0], xmm0
    movhps     [dstq+dsq*1], xmm0
    movq       [dstq+dsq*2], xmm1
    movhps     [dstq+r6   ], xmm1
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w8
    vzeroupper
    RET
.w16:
    mova                xm1, [dstq+dsq*0]
    vinserti32x4        ym1, [dstq+dsq*1], 1
    vinserti32x4         m1, [dstq+dsq*2], 2
    mova                 m4, [maskq]
    vinserti32x4         m1, [dstq+r6   ], 3
    mova                 m5, [maskq+tmpq]
    add               maskq, 16*4
    psubb                m3, m6, m4
    punpcklbw            m0, m1, m5
    punpcklbw            m2, m3, m4
    punpckhbw            m1, m5
    punpckhbw            m3, m4
    pmaddubsw            m0, m2
    pmaddubsw            m1, m3
    pmulhrsw             m0, m7
    pmulhrsw             m1, m7
    packuswb             m0, m1
    mova          [dstq+dsq*0], xm0
    vextracti32x4 [dstq+dsq*1], ym0, 1
    vextracti32x4 [dstq+dsq*2], m0, 2
    vextracti32x4 [dstq+r6   ], m0, 3
    lea                dstq, [dstq+dsq*4]
    sub                  hd, 4
    jg .w16
    RET
.w32:
    mova                ym1, [dstq+dsq*0]
    vinserti32x8         m1, [dstq+dsq*1], 1
    mova                 m4, [maskq]
    mova                 m5, [maskq+tmpq]
    add               maskq, 32*2
    psubb                m3, m6, m4
    punpcklbw            m0, m1, m5
    punpcklbw            m2, m3, m4
    punpckhbw            m1, m5
    punpckhbw            m3, m4
    pmaddubsw            m0, m2
    pmaddubsw            m1, m3
    pmulhrsw             m0, m7
    pmulhrsw             m1, m7
    packuswb             m0, m1
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w32
    RET

cglobal blend_v_8bpc, 3, 6, 6, dst, ds, tmp, w, h, mask
%define base r5-blend_v_avx512icl_table
    lea                  r5, [blend_v_avx512icl_table]
    tzcnt                wd, wm
    movifnidn            hd, hm
    movsxd               wq, [r5+wq*4]
    vpbroadcastd         m5, [base+pw_512]
    add                  wq, r5
    add               maskq, obmc_masks-blend_v_avx512icl_table
    jmp                  wq
.w2:
    vpbroadcastd       xmm2, [maskq+2*2]
.w2_s0_loop:
    movd               xmm0, [dstq+dsq*0]
    pinsrw             xmm0, [dstq+dsq*1], 1
    movd               xmm1, [tmpq]
    add                tmpq, 2*2
    punpcklbw          xmm0, xmm1
    pmaddubsw          xmm0, xmm2
    pmulhrsw           xmm0, xm5
    packuswb           xmm0, xmm0
    pextrw     [dstq+dsq*0], xmm0, 0
    pextrw     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w2_s0_loop
    RET
.w4:
    vpbroadcastq       xmm2, [maskq+4*2]
.w4_loop:
    movd               xmm0, [dstq+dsq*0]
    pinsrd             xmm0, [dstq+dsq*1], 1
    movq               xmm1, [tmpq]
    add                tmpq, 4*2
    punpcklbw          xmm0, xmm1
    pmaddubsw          xmm0, xmm2
    pmulhrsw           xmm0, xm5
    packuswb           xmm0, xmm0
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w4_loop
    RET
.w8:
    mova               xmm3, [maskq+8*2]
.w8_loop:
    movq               xmm0, [dstq+dsq*0]
    vpbroadcastq       xmm1, [dstq+dsq*1]
    mova               xmm2, [tmpq]
    add                tmpq, 8*2
    punpcklbw          xmm0, xmm2
    punpckhbw          xmm1, xmm2
    pmaddubsw          xmm0, xmm3
    pmaddubsw          xmm1, xmm3
    pmulhrsw           xmm0, xm5
    pmulhrsw           xmm1, xm5
    packuswb           xmm0, xmm1
    movq       [dstq+dsq*0], xmm0
    movhps     [dstq+dsq*1], xmm0
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w8_loop
    RET
.w16:
    vbroadcasti32x4     ym3, [maskq+16*2]
    vbroadcasti32x4     ym4, [maskq+16*3]
.w16_loop:
    mova                xm1, [dstq+dsq*0]
    vinserti32x4        ym1, [dstq+dsq*1], 1
    mova                ym2, [tmpq]
    add                tmpq, 16*2
    punpcklbw           ym0, ym1, ym2
    punpckhbw           ym1, ym2
    pmaddubsw           ym0, ym3
    pmaddubsw           ym1, ym4
    pmulhrsw            ym0, ym5
    pmulhrsw            ym1, ym5
    packuswb            ym0, ym1
    mova          [dstq+dsq*0], xm0
    vextracti32x4 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w16_loop
    RET
.w32:
    mova                 m4, [maskq+32*2]
    vshufi32x4           m3, m4, m4, q2020
    vshufi32x4           m4, m4, q3131
.w32_loop:
    mova                ym1, [dstq+dsq*0]
    vinserti32x8         m1, [dstq+dsq*1], 1
    mova                 m2, [tmpq]
    add                tmpq, 32*2
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    pmaddubsw            m0, m3
    pmaddubsw            m1, m4
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    sub                  hd, 2
    jg .w32_loop
    RET

cglobal blend_h_8bpc, 3, 7, 6, dst, ds, tmp, w, h, mask
%define base r6-blend_h_avx512icl_table
    lea                  r6, [blend_h_avx512icl_table]
    tzcnt                wd, wm
    mov                  hd, hm
    movsxd               wq, [r6+wq*4]
    lea               maskq, [base+obmc_masks+hq*2]
    vpbroadcastd         m5, [base+pw_512]
    lea                  hd, [hq*3]
    add                  wq, r6
    shr                  hd, 2 ; h * 3/4
    lea               maskq, [maskq+hq*2]
    neg                  hq
    jmp                  wq
.w2:
    movd               xmm0, [dstq+dsq*0]
    pinsrw             xmm0, [dstq+dsq*1], 1
    movd               xmm2, [maskq+hq*2]
    movd               xmm1, [tmpq]
    add                tmpq, 2*2
    punpcklwd          xmm2, xmm2
    punpcklbw          xmm0, xmm1
    pmaddubsw          xmm0, xmm2
    pmulhrsw           xmm0, xm5
    packuswb           xmm0, xmm0
    pextrw     [dstq+dsq*0], xmm0, 0
    pextrw     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w2
    RET
.w4:
    mova               xmm3, [blend_shuf]
.w4_loop:
    movd               xmm0, [dstq+dsq*0]
    pinsrd             xmm0, [dstq+dsq*1], 1
    movd               xmm2, [maskq+hq*2]
    movq               xmm1, [tmpq]
    add                tmpq, 4*2
    pshufb             xmm2, xmm3
    punpcklbw          xmm0, xmm1
    pmaddubsw          xmm0, xmm2
    pmulhrsw           xmm0, xm5
    packuswb           xmm0, xmm0
    movd       [dstq+dsq*0], xmm0
    pextrd     [dstq+dsq*1], xmm0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w4_loop
    RET
.w8:
    vbroadcasti128     ymm4, [blend_shuf]
    shufpd             ymm4, ymm4, 0x03
.w8_loop:
    vpbroadcastq       ymm1, [dstq+dsq*0]
    movq               xmm0, [dstq+dsq*1]
    vpblendd           ymm0, ymm1, 0x30
    vpbroadcastd       ymm3, [maskq+hq*2]
    movq               xmm1, [tmpq+8*1]
    vinserti128        ymm1, [tmpq+8*0], 1
    add                tmpq, 8*2
    pshufb             ymm3, ymm4
    punpcklbw          ymm0, ymm1
    pmaddubsw          ymm0, ymm3
    pmulhrsw           ymm0, ym5
    vextracti128       xmm1, ymm0, 1
    packuswb           xmm0, xmm1
    movhps     [dstq+dsq*0], xmm0
    movq       [dstq+dsq*1], xmm0
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w8_loop
    vzeroupper
    RET
.w16:
    vbroadcasti32x4     ym4, [blend_shuf]
    shufpd              ym4, ym4, 0x0c
.w16_loop:
    mova                xm1, [dstq+dsq*0]
    vinserti32x4        ym1, [dstq+dsq*1], 1
    vpbroadcastd        ym3, [maskq+hq*2]
    mova                ym2, [tmpq]
    add                tmpq, 16*2
    pshufb              ym3, ym4
    punpcklbw           ym0, ym1, ym2
    punpckhbw           ym1, ym2
    pmaddubsw           ym0, ym3
    pmaddubsw           ym1, ym3
    pmulhrsw            ym0, ym5
    pmulhrsw            ym1, ym5
    packuswb            ym0, ym1
    mova          [dstq+dsq*0], xm0
    vextracti32x4 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w16_loop
    RET
.w32:
    vbroadcasti32x4      m4, [blend_shuf]
    shufpd               m4, m4, 0xf0
.w32_loop:
    mova                ym1, [dstq+dsq*0]
    vinserti32x8         m1, [dstq+dsq*1], 1
    vpbroadcastd         m3, [maskq+hq*2]
    mova                 m2, [tmpq]
    add                tmpq, 32*2
    pshufb               m3, m4
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    pmaddubsw            m0, m3
    pmaddubsw            m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova          [dstq+dsq*0], ym0
    vextracti32x8 [dstq+dsq*1], m0, 1
    lea                dstq, [dstq+dsq*2]
    add                  hq, 2
    jl .w32_loop
    RET
.w64:
    vpbroadcastw         m3, [maskq+hq*2]
    mova                 m1, [dstq]
    mova                 m2, [tmpq]
    add                tmpq, 32*2
    punpcklbw            m0, m1, m2
    punpckhbw            m1, m2
    pmaddubsw            m0, m3
    pmaddubsw            m1, m3
    pmulhrsw             m0, m5
    pmulhrsw             m1, m5
    packuswb             m0, m1
    mova             [dstq], m0
    add                dstq, dsq
    inc                  hq
    jl .w64
    RET
.w128:
    vpbroadcastw         m6, [maskq+hq*2]
    mova                 m2, [dstq+64*0]
    mova                 m1, [tmpq+64*0]
    mova                 m3, [dstq+64*1]
    mova                 m4, [tmpq+64*1]
    add                tmpq, 64*2
    punpcklbw            m0, m2, m1
    punpckhbw            m2, m1
    pmaddubsw            m0, m6
    pmaddubsw            m2, m6
    punpcklbw            m1, m3, m4
    punpckhbw            m3, m4
    pmaddubsw            m1, m6
    pmaddubsw            m3, m6
    REPX   {pmulhrsw x, m5}, m0, m2, m1, m3
    packuswb             m0, m2
    packuswb             m1, m3
    mova        [dstq+64*0], m0
    mova        [dstq+64*1], m1
    add                dstq, dsq
    inc                  hq
    jl .w128
    RET

cglobal resize_8bpc, 6, 12, 19, dst, dst_stride, src, src_stride, \
                                dst_w, h, src_w, dx, mx0
    sub          dword mx0m, 4<<14
    sub        dword src_wm, 8
    mov                  r6, ~0
    vpbroadcastd         m5, dxm
    vpbroadcastd         m8, mx0m
    vpbroadcastd         m6, src_wm
    kmovq                k3, r6
 DEFINE_ARGS dst, dst_stride, src, src_stride, dst_w, h, x
    LEA                  r7, $$
%define base r7-$$
    vpbroadcastd         m3, [base+pw_m256]
    vpbroadcastd         m7, [base+pd_63]
    vbroadcasti32x4     m15, [base+pb_8x0_8x8]
    vpdpwssd             m8, m5, [base+rescale_mul] ; mx+dx*[0-15]
    pslld                m5, 4                      ; dx*16
    pslld                m6, 14
    pxor                 m2, m2
    mova                m16, [base+resize_permA]
    mova                m17, [base+resize_permB]
    mova               xm18, [base+resize_permC]
.loop_y:
    xor                  xd, xd
    mova                 m4, m8     ; per-line working version of mx
.loop_x:
    pmaxsd               m0, m4, m2
    psrad                m9, m4, 8  ; filter offset (unmasked)
    pminsd               m0, m6     ; iclip(mx, 0, src_w-8)
    psubd                m1, m4, m0 ; pshufb offset
    psrad                m0, 14     ; clipped src_x offset
    psrad                m1, 14     ; pshufb edge_emu offset
    vptestmd             k4, m1, m1
    pand                 m9, m7     ; filter offset (masked)
    ktestw               k4, k4
    jz .load
    vextracti32x8      ym12, m0, 1
    vextracti32x8      ym13, m1, 1
    kmovq                k1, k3
    kmovq                k2, k3
    vpgatherdq      m10{k1}, [srcq+ym0]
    vpgatherdq      m11{k2}, [srcq+ym12]
    kmovq                k1, k3
    kmovq                k2, k3
    vpgatherdq      m14{k1}, [base+resize_shuf+4+ym1]
    vpgatherdq       m0{k2}, [base+resize_shuf+4+ym13]
    mova                m12, m16
    mova                m13, m17
    paddb               m14, m15
    paddb                m0, m15
    pshufb              m10, m14
    pshufb              m11, m0
    vpermi2d            m12, m10, m11
    vpermi2d            m13, m10, m11
    jmp .filter
.load:
    kmovq                k1, k3
    kmovq                k2, k3
    vpgatherdd      m12{k1}, [srcq+m0+0]
    vpgatherdd      m13{k2}, [srcq+m0+4]
.filter:
    kmovq                k1, k3
    kmovq                k2, k3
    vpgatherdd      m10{k1}, [base+resize_filter+m9*8+0]
    vpgatherdd      m11{k2}, [base+resize_filter+m9*8+4]
    mova                m14, m2
    vpdpbusd            m14, m12, m10
    vpdpbusd            m14, m13, m11
    packssdw            m14, m14
    pmulhrsw            m14, m3
    packuswb            m14, m14
    vpermd              m14, m18, m14
    mova          [dstq+xq], xm14
    paddd                m4, m5
    add                  xd, 16
    cmp                  xd, dst_wd
    jl .loop_x
    add                dstq, dst_strideq
    add                srcq, src_strideq
    dec                  hd
    jg .loop_y
    RET

%endif ; ARCH_X86_64
