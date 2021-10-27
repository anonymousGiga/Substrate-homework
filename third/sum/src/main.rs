fn sum(param: &[u32]) -> Option<u32> {
    let mut ret: Option<u32> = Some(0);
    for i in param {
        ret = ret.unwrap().checked_add(*i);
        if ret.is_none() {
            return ret;
        }
    }

    ret
}

fn main() {
    let data: [u32; 6] = [10, 9, 8, 7, 6, 5];
    let ret = sum(&data);
    println!("The result is {:?}", ret.unwrap());
}
