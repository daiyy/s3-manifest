use serde::{Deserialize, Serialize};

/// checksum string from manifest.checksum
pub struct ManifestChecksum(String);

impl From<&[u8; 33]> for ManifestChecksum {
    fn from(bytes: &[u8; 33]) -> Self {
        ManifestChecksum(String::from_utf8_lossy(&bytes[0..32]).into_owned())
    }
}

/// each inventory file inside of manifest.json
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InventoryFile {
    pub key: String,
    pub size: usize,
    #[serde(rename = "MD5checksum")]
    pub md5: String,
}

/// manifest.json itself
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryManifest {
    pub source_bucket: String,
    pub destination_bucket: String,
    pub version: String,
    pub creation_timestamp: String,
    pub file_format: String,
    pub file_schema: String,
    pub files: Vec<InventoryFile>,
}

#[cfg(test)]
mod test {
    use super::*;
    use md5;

    #[test]
    #[cfg(feature = "inventory")]
    fn test_manifest_csv_serde() {
        let data = include_bytes!("../data/inventory/csv/manifest.json");
        let parsed: InventoryManifest = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: InventoryManifest = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }

    #[test]
    #[cfg(feature = "inventory")]
    fn test_manifest_csv_checksum() {
        let data = include_bytes!("../data/inventory/csv/manifest.json");
        let md5 = include_bytes!("../data/inventory/csv/manifest.checksum");
        let checksum: ManifestChecksum = md5.into();
        assert_eq!(format!("{:x}", md5::compute(&data)), checksum.0);
    }
}
