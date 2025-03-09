use tree_sitter::{Parser, Node};
use tree_sitter_python::language;
use std::fs;
use log::{info, error};

#[derive(Debug)]
enum NodeType {
    Class(String),
    Function(String),
    Endpoint(String),
    Table(String),
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<NodeType>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, node: NodeType) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }
}

fn parse_python_file(file_path: &str, graph: &mut Graph) {
    info!("Parsing file: {}", file_path);

    let code = fs::read_to_string(file_path).expect("Failed to read file");


    let mut parser = Parser::new();

    parser.set_language(language()).expect("Failed to load Python grammar");


    let tree = parser.parse(&code, None).expect("Failed to parse code");
    let root_node = tree.root_node();


    traverse_tree(root_node, &code, graph);
}

fn traverse_tree(node: Node, code: &str, graph: &mut Graph) {

    match node.kind() {
        "class_definition" => {
            let class_name = node
                .child_by_field_name("name")
                .and_then(|n| Some(n.utf8_text(code.as_bytes()).unwrap().to_string()))
                .unwrap_or_default();

            info!("Found class: {}", class_name);

            let class_index = graph.add_node(NodeType::Class(class_name.clone()));


            let function_index = graph.add_node(NodeType::Function(class_name.clone()));
            graph.add_edge(class_index, function_index);

            if class_name.contains("Person") {
                info!("Found a table: {}", class_name);
                graph.add_node(NodeType::Table(class_name));
            }
        }
        "function_definition" => {
            let function_name = node
                .child_by_field_name("name")
                .and_then(|n| Some(n.utf8_text(code.as_bytes()).unwrap().to_string()))
                .unwrap_or_default();

            info!("Found function: {}", function_name);

            graph.add_node(NodeType::Function(function_name));
        }
        "decorated_definition" => {

            if let Some(decorator) = node.child(0) {
                if decorator.kind() == "decorator" {
                    let decorator_text = decorator.utf8_text(code.as_bytes()).unwrap_or_default();
                    if decorator_text.contains("@app.") {
                        let endpoint_name = decorator_text
                            .split('(')
                            .nth(1)
                            .and_then(|s| s.split(')').next())
                            .unwrap_or_default()
                            .trim_matches('"')
                            .to_string();

                        info!("Found endpoint: {}", endpoint_name);

                        graph.add_node(NodeType::Endpoint(endpoint_name));
                    }
                }
            }
        }
        _ => {}
    }


    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        traverse_tree(child, code, graph);
    }
}


fn parse_python_repo(repo_path: &str) -> Graph {
    let mut graph = Graph::new();


    for entry in fs::read_dir(repo_path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("py") {
            parse_python_file(path.to_str().unwrap(), &mut graph);
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::LevelFilter;
    use simplelog::{SimpleLogger};

    #[test]
    fn test_python_parsing() {
        SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()).unwrap();

        let repo_path = "python"; // Modify to your repo path
        let graph = parse_python_repo(repo_path);

        info!("Graph Nodes: {:?}", graph.nodes);
        info!("Graph Edges: {:?}", graph.edges);

        

    }
}
