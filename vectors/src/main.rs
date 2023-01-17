fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third = &v[2];
    println!("The third element is {third}");
    println!("The size of vector is {}",v.len());
    println!("{:?}",v);
    if v.contains(&5){
        println!("found 5");
    }
    v.remove(1);
    println!("{:?}",v);

    let v1 = vec![100, 32, 57];
    for i in &v1 {
        println!("{i}");
    }
    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        *i += 50;
        println!("{i}");
    }

}
