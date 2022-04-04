fn function() {
    println!("A");
}

pub mod mod1 {
    pub fn function() {
        println!("B")
    }

    pub mod mod2 {
        pub fn function() {
            super::mod3::function();
        }
    }

    pub mod mod3 {
        pub fn function() {
            super::super::function()
        }
    }
}

fn main() {
    mod1::mod2::function();
}
