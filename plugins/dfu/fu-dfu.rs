// Copyright (C) 2023 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1+

enum DfuRequest {
    Detach,
    Dnload,
    Upload,
    Getstatus,
    Clrstatus,
    Getstate,
    Abort,
}

#[derive(ToString)]
enum DfuStatus {
    Ok,
    ErrTarget,
    ErrFile,
    ErrWrite,
    ErrErase,
    ErrCheckErased,
    ErrProg,
    ErrVerify,
    ErrAddress,
    ErrNotdone,
    ErrFirmware,
    ErrVendor,
    ErrUsbr,
    ErrPor,
    ErrUnknown,
    ErrStalldpkt,
}

#[derive(ToString)]
enum DfuState {
    AppIdle,
    AppDetach,
    DfuIdle,
    DfuDnloadSync,
    DfuDnbusy,
    DfuDnloadIdle,
    DfuManifestSync,
    DfuManifest,
    DfuManifestWaitReset,
    DfuUploadIdle,
    DfuError,
}