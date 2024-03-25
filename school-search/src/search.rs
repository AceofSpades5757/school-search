use milli::heed::EnvOpenOptions;
use milli::update::IndexDocuments;
use milli::update::IndexDocumentsConfig;
use milli::update::IndexerConfig;
use milli::Index;
use milli::Search;
use obkv::KvReader;
use serde::{Deserialize, Serialize};

/// Limit when getting back searches
const QUERY_LIMIT: usize = 20;
const SCHOOLS: &str = include_str!("./world_universities_and_domains.json");

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct School {
    pub id: usize,
    pub name: String,
    pub alpha_two_code: String,
    pub country: String,
    #[serde(alias = "state-province")]
    pub state_province: Option<String>,
    pub domains: Vec<String>,
    pub web_pages: Vec<String>,
}

impl From<KvReader<'_, u16>> for School {
    fn from(reader: KvReader<u16>) -> Self {
        let id: usize = serde_json::from_slice(reader.get(0u16).unwrap()).unwrap();
        let name: String = serde_json::from_slice(reader.get(1u16).unwrap()).unwrap();
        let alpha_two_code: String = serde_json::from_slice(reader.get(2u16).unwrap()).unwrap();
        let country: String = serde_json::from_slice(reader.get(3u16).unwrap()).unwrap();
        let state_province: Option<String> =
            serde_json::from_slice(reader.get(4u16).unwrap()).unwrap();
        let domains: Vec<String>;
        if let Some(data) = reader.get(5u16) {
            domains = serde_json::from_slice(data).unwrap();
        } else {
            domains = Vec::new();
        }
        let web_pages: Vec<String>;
        if let Some(data) = reader.get(6u16) {
            web_pages = serde_json::from_slice(data).unwrap();
        } else {
            web_pages = Vec::new();
        }

        School {
            id,
            name,
            alpha_two_code,
            country,
            state_province,
            domains,
            web_pages,
        }
    }
}

pub fn setup_database() -> Database {
    let data = serde_json::from_str(SCHOOLS).unwrap();

    Database::new(data)
}

pub struct Database {
    index: Index,
}

impl Database {
    pub fn new(data: Vec<School>) -> Self {
        // rtxn - Read Transaction
        // wtxn - Write Transaction

        let path = tempfile::tempdir().unwrap();
        std::fs::create_dir(&path).ok();
        let mut options = EnvOpenOptions::new();
        options.map_size(100 * 1024 * 1024); // 100 MB
        let index = Index::new(options, &path).unwrap();

        let mut wtxn = index.write_txn().unwrap();

        let documents = serde_json::json!(data);
        let mut writer = std::io::Cursor::new(Vec::new());
        let mut builder = milli::documents::DocumentBatchBuilder::new(&mut writer).unwrap();
        let documents = serde_json::to_vec(&documents).unwrap();
        builder
            .extend_from_json(std::io::Cursor::new(documents))
            .unwrap();
        builder.finish().unwrap();
        writer.set_position(0);
        let content = milli::documents::DocumentBatchReader::from_reader(writer).unwrap();

        let config = IndexerConfig::default();
        let indexing_config = IndexDocumentsConfig::default();
        let mut builder =
            IndexDocuments::new(&mut wtxn, &index, &config, indexing_config, |_| ()).unwrap();
        builder.add_documents(content).unwrap();
        builder.execute().unwrap();
        wtxn.commit().unwrap();

        Self { index }
    }

    #[allow(dead_code)]
    pub fn add_data(&self, data: Vec<School>) -> Result<(), std::io::Error> {
        let documents = serde_json::json!(data);
        let mut writer = std::io::Cursor::new(Vec::new());
        let mut builder = milli::documents::DocumentBatchBuilder::new(&mut writer).unwrap();
        let documents = serde_json::to_vec(&documents).unwrap();
        builder
            .extend_from_json(std::io::Cursor::new(documents))
            .unwrap();
        builder.finish().unwrap();
        writer.set_position(0);
        let content = milli::documents::DocumentBatchReader::from_reader(writer).unwrap();

        let mut wtxn = self.index().write_txn().unwrap();
        let config = IndexerConfig::default();
        let indexing_config = IndexDocumentsConfig::default();
        let mut builder =
            IndexDocuments::new(&mut wtxn, self.index(), &config, indexing_config, |_| ()).unwrap();

        builder.add_documents(content).unwrap();
        builder.execute().unwrap();
        wtxn.commit().unwrap();

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_document_by_external_id(&self, external_id: usize) -> Option<School> {
        let rtxn = &self.index().read_txn().unwrap();
        let idio_map = &self.index().external_documents_ids(rtxn).unwrap();

        if let Some(index) = idio_map.get(external_id.to_string()) {
            self.get_document_by_internal_id(index)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_document_by_internal_id(&self, internal_id: u32) -> Option<School> {
        #[allow(unused_mut)]
        let mut rtxn = self.index().read_txn().unwrap();
        let results = self.index().documents(&rtxn, [internal_id]);
        match results {
            Ok(rs) => {
                let obj = rs[0];
                let _id = obj.0;
                let reader = obj.1;
                let doc: School = reader.into();
                Some(doc)
            }
            Err(_) => None,
        }
    }

    pub fn index(&self) -> &Index {
        &self.index
    }

    #[allow(dead_code)]
    pub fn query(&self, query: &str) -> milli::SearchResult {
        #[allow(unused_mut)]
        let mut rtxn = self.index().read_txn().unwrap();
        let mut search = Search::new(&rtxn, self.index());
        search.query(query);
        search.limit(QUERY_LIMIT);

        search.execute().unwrap()
    }

    pub fn query_documents(&self, query: &str) -> Vec<School> {
        #[allow(unused_mut)]
        let mut rtxn = self.index().read_txn().unwrap();
        let mut search = Search::new(&rtxn, self.index());
        search.query(query);
        search.limit(QUERY_LIMIT);

        let ids = search.execute().unwrap().documents_ids;
        self.index()
            .documents(&rtxn, ids)
            .unwrap()
            .into_iter()
            .map(|(_id, reader)| reader.into())
            .rev()
            .collect()
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new(vec![])
    }
}
