use petgraph::{generators::random_undirected_graph, graph6_encoder::Graph6};

fn main() {
    println!("{}", random_undirected_graph::<u32>(2, 1.).graph6_string());
    println!("{}", random_undirected_graph::<u32>(3, 1.).graph6_string());
    println!("{}", random_undirected_graph::<u32>(5, 1.).graph6_string());
    println!("{}", random_undirected_graph::<u32>(7, 1.).graph6_string());
    println!("{}", random_undirected_graph::<u32>(2, 0.5).graph6_string());
    println!("{}", random_undirected_graph::<u32>(3, 0.5).graph6_string());
    println!("{}", random_undirected_graph::<u32>(5, 0.5).graph6_string());
    println!("{}", random_undirected_graph::<u32>(7, 0.5).graph6_string());
    println!("2_000");
    println!(
        "{}",
        random_undirected_graph::<u32>(2_000, 0.3).graph6_string()
    );
    println!("6_000");
    println!(
        "{}",
        random_undirected_graph::<u32>(6_000, 0.3).graph6_string()
    );
    println!("10_000");
    println!(
        "{}",
        random_undirected_graph::<u32>(10_000, 0.3).graph6_string()
    );
    println!("30_000");
    println!(
        "{}",
        random_undirected_graph::<u32>(30_000, 0.3).graph6_string()
    );
    println!("50_000");
    println!(
        "{}",
        random_undirected_graph::<u32>(50_000, 0.5).graph6_string()
    );
    println!("258_047");
    println!(
        "{}",
        random_undirected_graph::<u32>(258_047, 0.5).graph6_string()
    );
}
