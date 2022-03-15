use std::io;
use std::io::Write;
use std::process;

struct Post {
    label: char,
    rings: Vec<u32>
}

impl Post {
    fn new(label: char) -> Post {
        Post {
            label,
            rings: Vec::new()
        }
    }
}

struct Tower {
    move_count: u32,
    posts: [Post; 3]
}

impl Tower {
    fn new(rings: u32) -> Tower {
        let mut a: Post = Post::new('A');
        let b: Post = Post::new('B');
        let c: Post = Post::new('C');
        for x in (0..rings).rev() {
            a.rings.push(x + 1);
        }
        let tower = Tower {
            move_count: 0,
            posts: [a, b, c]
        };
        println!("\nTower State - {}: {:?} {}: {:?} {}: {:?}",
            tower.posts[0].label, tower.posts[0].rings,
            tower.posts[1].label, tower.posts[1].rings,
            tower.posts[2].label, tower.posts[2].rings);
        return tower
    }

    fn move_ring(&mut self, source: u8, destination: u8) {
        match self.posts[source as usize].rings.pop() {
            Some(ring_to_move) => {
                self.posts[destination as usize].rings.push(ring_to_move);
                self.move_count += 1;
                println!("== Move {}\nMove ring {} from post {} to post {}",
                    self.move_count, ring_to_move,
                    self.posts[source as usize].label, self.posts[destination as usize].label);
                println!("Tower State - {}: {:?} {}: {:?} {}: {:?}",
                    self.posts[0].label, self.posts[0].rings,
                    self.posts[1].label, self.posts[1].rings,
                    self.posts[2].label, self.posts[2].rings);
            },
            None => panic!("Failed to pop ring from stack")
        }
    }
}

fn move_rings(ring: u32, source: u8, destination: u8, intermediate: u8, tower: &mut Tower) {
    if ring > 0 {
        move_rings(ring - 1, source, intermediate, destination, tower);
        tower.move_ring(source, destination);
        move_rings(ring - 1, intermediate, destination, source, tower);
    }
}

fn main() {
    println!("How many rings to move?");
    io::stdout().flush().unwrap();
    let mut num_of_rings = String::new();
    io::stdin().read_line(&mut num_of_rings).expect("Failed to read line");

    match num_of_rings.trim().parse::<u32>() {
        Ok(rings) => {
            if rings > 0 {
                let mut tower: Tower = Tower::new(rings);
                move_rings(rings, 0, 2, 1, &mut tower);
            } else {
                println!("Number of rings must be greater than 0");
                process::exit(1);
            }
        },
        Err(_) => {
            println!("Value entered must be an integer greater than 0");
            process::exit(1);
        }
    }
}
