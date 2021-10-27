trait CalcGraphAera {
    fn get_eara(&self) -> u32;
}

fn print_aera<T>(graph: &T)
where
    T: CalcGraphAera,
{
    let aera = graph.get_eara();
    println!("The eara of this graph is {:?}", aera);
}

struct Square {
    hi: u32,
    wi: u32,
}

impl CalcGraphAera for Square {
    fn get_eara(&self) -> u32 {
        return self.hi.checked_mul(self.wi).unwrap_or(0);
    }
}

fn main() {
    let s = Square { hi: 10, wi: 20 };
    print_aera(&s);
}
