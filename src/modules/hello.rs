// for use modules you have two methods.
// the first one is add mod.rs file in the folder and add the files like
// mod modules;
// use modules::hello;
//
// the second method is how i use in this proyect, 
// 1. first create a folder and a file, example folder 'modules' and in the folder create a file 'hello.rs'
// 2. after that, in 'hello.rs' create a public function
// 3. in the root of the src, create a file with extension '.rs' with the same name how the folder --> 'modules.rs' 
// and add the definition of a module (the name of the file), like --> pub mod hello
pub fn hello_world() {
    println!("Hello world!!");
}
