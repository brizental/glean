// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{metrics::*, CommonMetricData, Lifetime};

#[derive(Debug)]
pub struct CoreMetrics {
    pub client_id: UuidMetric,
    pub first_run_date: DatetimeMetric,
    pub os: StringMetric,
    // Provided by the language bindings
    pub os_version: StringMetric,
    pub device_manufacturer: StringMetric,
    pub device_model: StringMetric,
    pub architecture: StringMetric,
    pub app_build: StringMetric,
    pub app_display_version: StringMetric,
    pub app_channel: StringMetric,
    pub locale: StringMetric,
    #[cfg(target_os="android")]
    pub android_sdk_version: StringMetric,
}

macro_rules! new_metric (
    ( $name:expr, $type:ident, $category:expr, $send_in_pings:expr, $lifetime:ident ) => {
        $type::new(CommonMetricData {
            name: $name.into(),
            category: "".into(),
            send_in_pings: vec![$send_in_pings.into()],
            lifetime: Lifetime::$lifetime,
            disabled: false,
            dynamic_label: None,
        })
    };
    ( $name:expr, $type:ident, $category:expr, $send_in_pings:expr, $lifetime:ident, $extra:expr ) => {
        $type::new(
            CommonMetricData {
                name: $name.into(),
                category: "".into(),
                send_in_pings: vec![$send_in_pings.into()],
                lifetime: Lifetime::$lifetime,
                disabled: false,
                dynamic_label: None,
            },
            $extra
        )
    };
);

impl CoreMetrics {
    pub fn new() -> CoreMetrics {
        CoreMetrics {
            client_id: new_metric!("client_id", UuidMetric, "", "glean_client_info", User),
            first_run_date: new_metric!("first_run_date", DatetimeMetric, "", "glean_client_info", User, TimeUnit::Day),
            os: new_metric!("os", StringMetric, "", "glean_client_info", Application),
            os_version: new_metric!("os_version", StringMetric, "", "glean_client_info", Application),
            device_manufacturer: new_metric!("device_manufacturer", StringMetric, "", "glean_client_info", Application),
            device_model: new_metric!("device_model", StringMetric, "", "glean_client_info", Application),
            architecture: new_metric!("architecture", StringMetric, "", "glean_client_info", Application),
            app_build: new_metric!("app_build", StringMetric, "", "glean_client_info", Application),
            app_display_version: new_metric!("app_display_version", StringMetric, "", "glean_client_info", Application),
            app_channel: new_metric!("app_channel", StringMetric, "", "glean_client_info", Application),
            locale: new_metric!("locale", StringMetric, "", "glean_client_info", Application),
            #[cfg(target_os="android")]
            android_sdk_version: new_metric!("android_sdk_version", StringMetric, "", "glean_client_info", Application),
        }
    }
}

#[derive(Debug)]
pub struct UploadMetrics {
    pub ping_upload_failure: LabeledMetric<CounterMetric>,
    pub discarded_exceeding_pings_size: MemoryDistributionMetric,
    pub pending_pings_directory_size: MemoryDistributionMetric,
    pub deleted_pings_after_quota_hit: CounterMetric,
}

impl UploadMetrics {
    pub fn new() -> UploadMetrics {
        UploadMetrics {
            ping_upload_failure: LabeledMetric::new(
                CounterMetric::new(CommonMetricData {
                    name: "ping_upload_failure".into(),
                    category: "glean.upload".into(),
                    send_in_pings: vec!["metrics".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    dynamic_label: None,
                }),
                Some(vec![
                    "status_code_4xx".into(),
                    "status_code_5xx".into(),
                    "status_code_unknown".into(),
                    "unrecoverable".into(),
                    "recoverable".into(),
                ]),
            ),

            discarded_exceeding_pings_size: MemoryDistributionMetric::new(
                CommonMetricData {
                    name: "discarded_exceeding_ping_size".into(),
                    category: "glean.upload".into(),
                    send_in_pings: vec!["metrics".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    dynamic_label: None,
                },
                MemoryUnit::Kilobyte,
            ),

            pending_pings_directory_size: MemoryDistributionMetric::new(
                CommonMetricData {
                    name: "pending_pings_directory_size".into(),
                    category: "glean.upload".into(),
                    send_in_pings: vec!["metrics".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    dynamic_label: None,
                },
                MemoryUnit::Kilobyte,
            ),

            deleted_pings_after_quota_hit: CounterMetric::new(CommonMetricData {
                name: "deleted_pings_after_quota_hit".into(),
                category: "glean.upload".into(),
                send_in_pings: vec!["metrics".into()],
                lifetime: Lifetime::Ping,
                disabled: false,
                dynamic_label: None,
            }),
        }
    }
}

#[derive(Debug)]
pub struct DatabaseMetrics {
    pub size: MemoryDistributionMetric,
}

impl DatabaseMetrics {
    pub fn new() -> DatabaseMetrics {
        DatabaseMetrics {
            size: MemoryDistributionMetric::new(
                CommonMetricData {
                    name: "size".into(),
                    category: "glean.database".into(),
                    send_in_pings: vec!["metrics".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    dynamic_label: None,
                },
                MemoryUnit::Byte,
            ),
        }
    }
}
