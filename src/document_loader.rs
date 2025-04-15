use rig::loaders::FileLoader;
use text_splitter::TextSplitter;

pub fn load_doc() -> Vec<String> {
    let max_characters = 100;
    let splitter = TextSplitter::new(max_characters);

    FileLoader::with_glob("docs/*.csv")
        .unwrap()
        .read()
        .into_iter()
        .filter_map(|result| {
            result
                .map_err(|e| {
                    eprintln!("Error reading document: {}", e);
                    e
                })
                .ok()
        })
        .flat_map(|content| {
            // // 使用正则表达式或字符串分割按 ## 分块
            // let chunks = content
            //     .split("\n## ")
            //     .map(|chunk| {
            //         if !chunk.starts_with("## ") && chunk.starts_with("# ") {
            //             chunk.to_string()
            //         } else {
            //             format!("## {}", chunk)
            //         }
            //     })
            //     .collect::<Vec<_>>();

            // let chunks = splitter
            //     .chunks(&content)
            //     .map(|chunk| chunk.to_string())
            //     .collect::<Vec<_>>();

            let chunks = content
                .split("\n")
                .map(|chunk| chunk.to_string())
                .collect::<Vec<_>>();

            println!("Loaded chunks: {:?}", chunks); // 查看实际加载的文档内容

            chunks
        })
        .collect::<Vec<_>>()
}
