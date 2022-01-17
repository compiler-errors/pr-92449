#![feature(nll)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

// Passing
mod issue_60658;
mod issue_71723;
mod issue_79648;
mod issue_82921;
mod issue_90696;
mod issue_92096;

// Failing
//mod issue_64552;
//mod issue_87425;
//mod issue_90696_2;
//mod issue_92415;

fn main() {
    println!("Hello, world!");
}
