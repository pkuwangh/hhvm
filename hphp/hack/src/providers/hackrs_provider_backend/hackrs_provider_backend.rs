// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

pub mod naming_table;

#[cfg(test)]
mod test_naming_table;

use anyhow::Result;
use datastore::{ChangesStore, NonEvictingStore, Store};
use depgraph_api::{DepGraph, NoDepGraph};
use file_provider::{DiskProvider, FileProvider};
use hackrs::{
    decl_parser::DeclParser,
    folded_decl_provider::{FoldedDeclProvider, LazyFoldedDeclProvider},
    shallow_decl_provider::{LazyShallowDeclProvider, ShallowDeclProvider, ShallowDeclStore},
};
use naming_provider::NamingProvider;
use naming_table::NamingTable;
use ocamlrep_derive::{FromOcamlRep, ToOcamlRep};
use oxidized_by_ref::parser_options::ParserOptions;
use pos::{RelativePath, RelativePathCtx, TypeName};
use std::sync::Arc;
use ty::{decl::folded::FoldedClass, reason::BReason};

pub struct ProviderBackend {
    pub path_ctx: Arc<RelativePathCtx>,
    pub file_provider: Arc<dyn FileProvider>,
    pub decl_parser: DeclParser<BReason>,
    pub dependency_graph: Arc<dyn DepGraph>,
    pub naming_provider: Arc<dyn NamingProvider>,
    pub shallow_decl_provider: Arc<dyn ShallowDeclProvider<BReason>>,
    pub folded_decl_provider: Arc<dyn FoldedDeclProvider<BReason>>,
}

pub struct HhServerProviderBackend {
    file_store: Arc<ChangesStore<RelativePath, FileType>>,
    naming_table: Arc<NamingTable>,
    #[allow(dead_code)]
    shallow_decl_store: Arc<ShallowDeclStore<BReason>>,
    #[allow(dead_code)]
    folded_classes_store: Arc<dyn Store<TypeName, Arc<FoldedClass<BReason>>>>,
    providers: ProviderBackend,
}

impl HhServerProviderBackend {
    pub fn new(path_ctx: RelativePathCtx, popt: &ParserOptions<'_>) -> Result<Self> {
        let path_ctx = Arc::new(path_ctx);
        let file_store = Arc::new(ChangesStore::new(
            Arc::new(NonEvictingStore::new()), // TODO: make this sharedmem
        ));
        let file_provider = Arc::new(FileProviderWithChanges {
            delta_and_changes: Arc::clone(&file_store),
            disk: DiskProvider::new(Arc::clone(&path_ctx)),
        });
        let decl_parser = DeclParser::with_options(Arc::clone(&file_provider) as _, popt);
        let dependency_graph = Arc::new(NoDepGraph::new());
        let naming_table = Arc::new(NamingTable::new());

        let shallow_decl_store = Arc::new(hackrs_test_utils::store::make_shallow_decl_store::<
            BReason,
        >(
            hackrs_test_utils::serde_store::StoreOpts::Unserialized,
        ));

        let shallow_decl_provider: Arc<dyn ShallowDeclProvider<_>> =
            Arc::new(LazyShallowDeclProvider::new(
                Arc::clone(&shallow_decl_store),
                Arc::clone(&naming_table) as _,
                decl_parser.clone(),
            ));

        let folded_classes_store: Arc<dyn Store<pos::TypeName, Arc<FoldedClass<_>>>> =
            Arc::new(datastore::NonEvictingStore::new());

        let folded_decl_provider: Arc<dyn FoldedDeclProvider<_>> =
            Arc::new(LazyFoldedDeclProvider::new(
                Arc::new(Default::default()), // TODO: remove?
                Arc::clone(&folded_classes_store),
                Arc::clone(&shallow_decl_provider),
                Arc::clone(&dependency_graph) as _,
            ));

        Ok(Self {
            providers: ProviderBackend {
                path_ctx,
                file_provider,
                decl_parser,
                dependency_graph,
                naming_provider: Arc::clone(&naming_table) as _,
                shallow_decl_provider,
                folded_decl_provider,
            },
            file_store,
            naming_table,
            shallow_decl_store,
            folded_classes_store,
        })
    }

    pub fn naming_table(&self) -> &NamingTable {
        &self.naming_table
    }

    pub fn file_store(&self) -> &dyn Store<RelativePath, FileType> {
        Arc::as_ref(&self.file_store) as _
    }

    pub fn file_provider(&self) -> &dyn FileProvider {
        Arc::as_ref(&self.providers.file_provider)
    }

    pub fn shallow_decl_provider(&self) -> &dyn ShallowDeclProvider<BReason> {
        Arc::as_ref(&self.providers.shallow_decl_provider)
    }

    pub fn folded_decl_provider(&self) -> &dyn FoldedDeclProvider<BReason> {
        Arc::as_ref(&self.providers.folded_decl_provider)
    }

    pub fn push_local_changes(&self) {
        self.file_store.push_local_changes();
        self.naming_table.push_local_changes();
    }

    pub fn pop_local_changes(&self) {
        self.file_store.pop_local_changes();
        self.naming_table.pop_local_changes();
    }
}

#[derive(Clone, Debug, ToOcamlRep, FromOcamlRep)]
pub enum FileType {
    Disk(bstr::BString),
    Ide(bstr::BString),
}

#[derive(Debug)]
struct FileProviderWithChanges {
    // We could use DeltaStore here if not for the fact that the OCaml
    // implementation of `File_provider.get` does not fall back to disk when the
    // given path isn't present in sharedmem/local_changes (it only does so for
    // `File_provider.get_contents`).
    delta_and_changes: Arc<ChangesStore<RelativePath, FileType>>,
    disk: DiskProvider,
}

impl FileProvider for FileProviderWithChanges {
    fn get(&self, file: RelativePath) -> Result<bstr::BString> {
        match self.delta_and_changes.get(file)? {
            Some(FileType::Disk(contents)) => Ok(contents),
            Some(FileType::Ide(contents)) => Ok(contents),
            None => match self.disk.read(file) {
                Ok(contents) => Ok(contents),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok("".into()),
                Err(e) => Err(e.into()),
            },
        }
    }
}
