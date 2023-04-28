/*
 * Copyright (c) 1999-2023 Logitech, Inc.
 *
 * SPDX-License-Identifier: LGPL-2.1+
 */

#pragma once

#include <fwupdplugin.h>

#define FU_TYPE_LOGITECH_TAP_DEVICE (fu_logitech_tap_device_get_type())
G_DECLARE_DERIVABLE_TYPE(FuLogitechTapDevice,
			 fu_logitech_tap_device,
			 FU,
			 LOGITECH_TAP_DEVICE,
			 FuUdevDevice)

struct _FuLogitechTapDeviceClass {
	FuUdevDeviceClass parent_class;
};
