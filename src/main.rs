
#![allow(dead_code)]

mod vehicle
{
    pub mod car
    {
        pub fn specifications()
        {
          println!("Car specification will be dclared here in module car");
        }
    }
}

fn main() {

    vehicle::car::specifications();
}
