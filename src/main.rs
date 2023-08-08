// main.rs
pub mod diameter;
use nalgebra::DMatrix;
use rand::Rng;
use rand::seq::SliceRandom; 
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
        let index_neighbours: Vec<usize> = self.graph.row(index)
                                                .iter()
                                                .enumerate()
                                                .filter(|(_,&value)| value != 0)
                                                .map(|(idx,_)| idx).collect();
        let mut neighbours = Vec::new();
        for i in index_neighbours{
            let a = self.vertices[i];
            neighbours.push(a);
        }


        return neighbours;
    }
    fn remove_node(&mut self, n: usize){
        let index = self.get_index(n);
        let new_graph = self.graph.clone();
        let done = new_graph.remove_column(index).remove_row(index);
        //let mut nearly_there = new_graph.remove_column(index);
        //let done = nearly_there.remove_row(index);
        self.vertices.remove(index);
        self.infected.remove(&n);
        self.graph = done;

    }

    fn infect(&mut self, p: f64){
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let mut to_be_infected: Vec<usize> = Vec::new();
        
        for i in &self.infected{
            let neighbours = self.get_neighbours(*i);
            to_be_infected.extend(neighbours.iter().filter(|x| !self.infected.contains(x)));
        }
        let mut new_infected = Vec::new();
        for node in to_be_infected{
            let r_no = rng.gen::<f64>();
            if self.time_recovered.get(&node) > Some(&0){
                continue;
            }
            if r_no < p {
                new_infected.push(node);
            }

        }
        self.infected.extend(new_infected)

    }
    fn die_or_recover(&mut self, node: usize, p_r: f64){
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let r_no = rng.gen::<f64>();

        if r_no < p_r{
            self.infected.remove(&node);
            self.days_infected.insert(node,0);
            let mut time_recovered = match self.time_recovered.get(&node){
                Some(x) => *x,
                None => 0
            };
            time_recovered+=1;
            self.time_recovered.insert(node,time_recovered);

        }else {
            let test = self.infected.len();
            self.remove_node(node);            
            self.days_infected.insert(node,0);
        }



    }

}




fn InfectionGraphConstructor(G: DMatrix<usize>, size: usize) -> InfectionGraph{
    let edges = (G.sum())/2;
    let vertices: Vec<usize> = (0..size).collect();
    let mut infected: HashSet<usize> = HashSet::new();
    let unlucky = vertices.choose(&mut rand::thread_rng());
    let x = match unlucky{
        Some(d) => *d,
        None => 0
    };
    infected.insert(x);
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

fn days_infected_checker(graph: &mut InfectionGraph, p_r: f64){
    let mut days_to_be_added: Vec<usize> = Vec::new();
    let mut nodes_to_die_or_recover: Vec<usize> = Vec::new();

    for (node,_) in &graph.days_infected{
        if graph.infected.contains(node){
            days_to_be_added.push(*node)
        }
        if graph.days_infected.get(&node) > Some(&10){
            nodes_to_die_or_recover.push(*node)

        }
    }
    for nodes in days_to_be_added{
        let days = graph.days_infected.get(&nodes);
        let mut day = match days {
            Some(x) => *x,
            None => 0
        };
        day +=1;
        graph.days_infected.insert(nodes,day);
    }
    for node in nodes_to_die_or_recover{
        graph.die_or_recover(node, p_r);
    }
}



fn main(){
    let size = 500;
    let a = gen_random_graph(size, 0.5);
    //println!("DONE");
    //println!("{:?}",a.data)
    //for i in a.row_iter(){
    //    println!("{}",i.sum());
    //}
    let mut R: InfectionGraph = InfectionGraphConstructor(a, size);
    
    for _ in 0..1000{
        R.infect(0.5);
        days_infected_checker(&mut R, 0.2);
        if R.infected.len() == 0{
            if R.vertices.len() == 0{
                println!("Everybody Died");
                break;
            } else{
                let survivors = R.vertices.len();
                println!("{} People Survived!",survivors);
                break;
            }
        }
    }
    println!("DONE");


    //print_matrix(&R.graph);
    //println!("{:?}",&R.vertices);
    //println!("{:?}", R.infected);
    //R.infect(1.0);
    //println!("{:?}", R.infected);



    //print_matrix(&R.graph);
    //println!("{:?}",&R.vertices);


    //println!("{:?}", R.get_neighbours(2))
    //println!("{}",all_non_zero(&a).to_string());
    //println!("{}",diameter::diameter(a))
}