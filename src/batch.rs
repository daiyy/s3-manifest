use serde::{Deserialize, Serialize};

/// checksum string from manifest.json.md5
pub struct ManifestChecksum(String);

impl From<&[u8; 33]> for ManifestChecksum {
    fn from(bytes: &[u8; 33]) -> Self {
        ManifestChecksum(String::from_utf8_lossy(&bytes[0..32]).into_owned())
    }
}

/// each result file inside of manifest.json
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResultFile {
    pub task_execution_status: String,
    pub bucket: String,
    #[serde(rename = "MD5Checksum")]
    pub md5: String,
    pub key: String,
}

/// represent manifest.json itself
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompletionReportManifest {
    pub format: String,
    pub report_creation_date: String,
    pub results: Vec<ResultFile>,
    pub report_schema: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use md5;

    #[test]
    #[cfg(feature = "batch")]
    fn test_manifest_serde() {
        let data = include_bytes!("../data/batch/manifest.json");
        let parsed: CompletionReportManifest = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CompletionReportManifest = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "batch")]
    fn test_manifest_checksum() {
        let data = include_bytes!("../data/batch/manifest.json");
        let md5 = include_bytes!("../data/batch/manifest.json.md5");
        let checksum: ManifestChecksum = md5.into();
        assert_eq!(format!("{:x}", md5::compute(&data)), checksum.0);
    }
}
