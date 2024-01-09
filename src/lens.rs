use serde::{Deserialize, Serialize};

/// each report file inside of manifest.json
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportFile {
    pub key: String,
    pub size: usize,
    #[serde(rename = "md5Checksum")]
    pub md5: String,
}

/// represent manifest.json itself
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsExportManifest {
    pub source_account_id: String,
    pub config_id: String,
    pub destination_bucket: String,
    pub report_version: String,
    pub report_date: String,
    pub report_format: String,
    pub report_schema: String,
    pub report_files: Vec<ReportFile>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "lens")]
    fn test_manifest_serde() {
        let data = include_bytes!("../data/lens/manifest.json");
        let parsed: MetricsExportManifest = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: MetricsExportManifest = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
