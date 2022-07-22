
// 文件名为test.rs 中的变量或者函数可以这样访问
pub mod test;

mod harry_mod {

    //module中的元素默认是私有的
    fn private_fn(){ //没有你pub 修饰，私有变量
        println!("in private_fn");

    }

    //通过pub修饰符，override default visibility
    pub fn function(){
        println!("in function");
    }

    // items can access other items in the same modules


    pub fn indirect_access() {
        print!("in indirect_access \n");
        private_fn();
    }

    //Modules can also be nested, but need use such synax
    pub mod nested {
        pub fn function(){
            println!("in nested mod function ");
        }

        #[allow(dead_code)]
        fn private_fn(){
            println!("in nested mod private_fn");
        }

        //Functions declared using `pub(in path)` syntax are only visible
        //within the give path. `path` must be a parent or ancestor module

        // crate is also used to represent the absolute path of a module, where crate refers to the root of the current crate. 
        pub(in crate::harry_mod) fn public_function_in_harry_mod() {
            print!("called `harry_mod:nested:public_function_in_harry_mod()`");
            //
            public_function_in_nested();
        }


        //Functions declared using `pub(self)` syntax are only visible within the current module, wihch is the same as leaving them private
        pub(self) fn public_function_in_nested(){
            println!("called `harry_mod::nested::public_function_in_nested()`");
        }

        //Functions declared using `pub(super)` syntax are only visible within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_harry_mod() {
        println!("called `harry_mod::call_public_function_in_harry_mod`, that \n>");
        nested::public_function_in_harry_mod();
        println!("> ");
        nested::public_function_in_super_mod();
    
    }
    
    //pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `harry_mod::public_function_in_crate()`");
    }
    
    
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `harry_mod::private_nested::function()`");
        }
    
        //private parent items will still restrict the visibility of a child item, 
        //even if it is declared as visible within a bigger scope
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `harry_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function`");
}


fn main(){
    //modules allow disambiguation between items that have the same name 
    function();
    harry_mod::function();

    //public items, including those inside nested modules, can be accessed from outside the parent module.
    harry_mod::indirect_access();
    harry_mod::nested::function();
    harry_mod::call_public_function_in_harry_mod();

    //pub(crate) items can be called from anywhere in the same crate
    harry_mod::public_function_in_crate();

    //pub(in path) items can only be called from within the module specified
    //error function `public_function_in_harry_mod()`
    // harry_mod::nested::public_function_in_harry_mod();
    
    //private items of a module cannot be directly accessed, even if nested in a public module
    //Error private_function is private
    // harry_mod::private_fn();

    //private function is private
    // harry_mod::nested::private_fn();

    //Error privated_nested is a private module
    // harry_mod::private_nested::function();

    //Error privated_nested is a private module
    // harry_mod::private_nested::restricted_function();

    test::test();

    let mut ex = test::Example{
        number: 12,
    };

    test::Example::boo(); // 有点跟其他的语言不太一样 
    ex.answer(); 
    ex.answer(); 
    ex.get_number();  //定义过程中的参数，最终使用的时候，是这么使用的，有点反直觉

    println!("the get_number result {}",ex.get_number());
}
