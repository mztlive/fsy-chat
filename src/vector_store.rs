use crate::models::Document;
use futures_util::stream::StreamExt;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, QueryPointsBuilder, VectorParamsBuilder,
};
use rig::embeddings::{EmbeddingModel, EmbeddingsBuilder};
use rig::vector_store::VectorStoreIndex;
use rig_qdrant::QdrantVectorStore;

pub async fn initialize_vector_store(model: impl EmbeddingModel) -> impl VectorStoreIndex {
    let docs = crate::document_loader::load_doc();

    let mut builder = EmbeddingsBuilder::new(model.clone());
    for (i, chunk) in docs.iter().enumerate() {
        builder = builder
            .document(Document {
                id: format!("doc_{}", i),
                message: chunk.to_string(),
            })
            .unwrap();
    }

    let documents = builder.build().await.unwrap();

    const COLLECTION_NAME: &str = "kf2";
    let client = Qdrant::from_url("http://localhost:6334")
        .skip_compatibility_check()
        .build()
        .unwrap();

    if !client.collection_exists(COLLECTION_NAME).await.unwrap() {
        client
            .create_collection(
                CreateCollectionBuilder::new(COLLECTION_NAME)
                    .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine)),
            )
            .await
            .unwrap();
    }

    let query_params = QueryPointsBuilder::new(COLLECTION_NAME).with_payload(true);
    let vector_store = QdrantVectorStore::new(client, model, query_params.build());

    vector_store.insert_documents(documents).await.unwrap();

    println!(
        "uuid: {:?}",
        vector_store
            .top_n::<Document>("服务uuid是多少", 1)
            .await
            .unwrap()
    );

    vector_store
}
