#[derive(Debug)]
#[derive(PartialEq)]
enum Gender {m, w, f} //#[derive(Copy, Clone)]

fn mwf(gender: &Gender){
    let mut SexualityChange:u8 = 0;
    
    match gender {
        m => println!("I am originally m, people may think that I have a girlfriend"),
        w => println!("I am originally w, people may think that I have a boyfriend"),
        f => println!("I am originally f, people may think that I have a significant other")
    }
    SexualityChange = 1;

    let FinalSexualityFlag = if gender == &Gender::m && SexualityChange == 1 {
        println!("People may think that I have a boyfriend...");
        Gender::w
    } else if gender == &Gender::m && SexualityChange == 2 {
        println!("people may think that I have a significant other...");
        Gender::f
    } else {
        println!("people may think that I have a girlfriend...");
        Gender::m
    };
    println!("Since I am {:?} now", FinalSexualityFlag);
}

fn main(){
    mwf(&Gender::m);
}