// main.rs
pub mod diameter;
use nalgebra::DMatrix;
use rand::Rng;
use std::collections::{HashMap,HashSet};

struct InfectionGraph {
    graph: DMatrix<usize>,
    size: usize,
    edges: usize,
    vertices: Vec<usize>,
    infected: HashSet<usize>,
    days_infected: HashMap<usize,usize>,
    time_recovered: HashMap<usize,usize>
}
impl InfectionGraph{
    fn get_index(&self, n: usize) -> usize{
        let current_node_index = self.vertices.iter().position(|&x| x == n);
        let index = match current_node_index{
            Some(x) => x,
            None => 0
        };
        return index;

    }
    fn get_neighbours(&self, n: usize) -> Vec<usize>{
        let index = self.get_index(n);
        let neighbours: Vec<usize> = self.graph.row(index).iter().enumerate().filter(|(_,&value)| value != 0).map(|(idx,_)| idx).collect();
        return neighbours;
    }
    fn remove_node(&mut self, n: usize){
        let index = self.get_index(n);
        let new_graph = self.graph.clone();
        let done = new_graph.remove_column(index).remove_row(index);
        //let mut nearly_there = new_graph.remove_column(index);
        //let done = nearly_there.remove_row(index);
        self.vertices.remove(index);
        self.graph = done;

    }
}




fn InfectionGraphConstructor(G: DMatrix<usize>, size: usize) -> InfectionGraph{
    let edges = (G.sum())/2;
    let infected: HashSet<usize> = HashSet::new();
    let vertices: Vec<usize> = (0..size).collect();
    let zeros: Vec<usize> = vec![0; size];
    let days_infected: HashMap<usize,usize>= vertices.iter().cloned().zip(zeros.iter().cloned()).collect();
    let time_recovered: HashMap<usize,usize>= vertices.iter().cloned().zip(zeros.iter().cloned()).collect();
    InfectionGraph{graph: G, size: size, edges: edges, vertices: vertices, infected: infected,days_infected: days_infected,time_recovered: time_recovered}
}



fn print_matrix(mat: &DMatrix<usize>){
    for i in 0..mat.nrows() {
        for j in 0..mat.ncols() {
            print!("{:6} ", mat[(i, j)]);
        }
        println!();
    }
}



fn gen_random_graph(num_vertices: usize, probability: f64)-> DMatrix<usize>{
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut adjacency_matrix = DMatrix::zeros(num_vertices, num_vertices);

    for i in 0..num_vertices {
        for j in i + 1..num_vertices {
            if rng.gen::<f64>() < probability {
                adjacency_matrix[(i, j)] = 1;
                adjacency_matrix[(j, i)] = 1;
            }
        }
    }

    adjacency_matrix
}





fn main(){
    let a = gen_random_graph(2500, 0.3);
    //println!("DONE");
    //println!("{:?}",a.data)
    //for i in a.row_iter(){
    //    println!("{}",i.sum());
    //}
    let mut R: InfectionGraph = InfectionGraphConstructor(a, 10);

    //print_matrix(&R.graph);
    //println!("{:?}",&R.vertices);
    R.remove_node(2);
    println!("DOne")
    //print_matrix(&R.graph);
    //println!("{:?}",&R.vertices);


    //println!("{:?}", R.get_neighbours(2))
    //println!("{}",all_non_zero(&a).to_string());
    //println!("{}",diameter::diameter(a))
}