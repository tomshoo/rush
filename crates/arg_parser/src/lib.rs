use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug)]
pub enum StoreAction {
    StoreBool,
    StoreValue,
    None
}

pub enum Type {
    String(Rc<RefCell<String>>),
    Boolean(Rc<RefCell<bool>>)
}

pub mod argparser {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use crate::{StoreAction, Type};

    #[derive(Debug)]
    enum ArgumentType {
        Boolean(Option<Rc<RefCell<bool>>>),
        String(Option<Rc<RefCell<String>>>),
        None
    }

    #[derive(Debug)]
    pub struct Argument<'arg> {
        name: Option<&'arg str>,
        description: Option<&'arg str>,
        reference: ArgumentType,
        action: StoreAction
    }

    impl<'arg> Argument<'arg> {
        pub fn new() -> Self {Self {
            name: None,
            description: None,
            action: StoreAction::None,
            reference: ArgumentType::None
        }}

        pub fn refer(&mut self, variable: Type) {
            if let Type::Boolean(reference) = &variable {
                self.reference = ArgumentType::Boolean(
                    Some(Rc::clone(reference))
                );
            }
            if let Type::String(reference) = &variable {
                self.reference = ArgumentType::String(
                    Some(Rc::clone(reference))
                )
            }
        }
    }
    pub struct ArgumentParser<'parser> {
        arg_map: HashMap<
            [Option<&'parser str>; 2],
            Rc<RefCell<Argument<'parser>>>
        >,
        name_map: HashMap<
            &'parser str,
            [Option<&'parser str>; 2]
        >
    }

    impl<'parser> ArgumentParser<'parser> {
        pub fn new() -> Self {Self {
            arg_map: HashMap::new(),
            name_map: HashMap::new()
            // arg_map: HashMap::new()
        }}
        pub fn add_argument(
            &mut self,
            name_vec: Vec<&'parser str>,
            description: &'parser str,
            action: StoreAction
        ) -> Rc<RefCell<Argument<'parser>>> {
            if name_vec.len() > 2 {
                panic!("Unexpected length of name_vector");
            }
            else {
                let mut names: [Option<&'parser str>; 2] = [None ;2];
                for (idx, name) in name_vec.iter().enumerate() {
                    names[idx] = Some(name);
                }
                for name in &name_vec {
                    self.name_map.insert(name, names);
                }
                let mut argument_property = Argument::new();
                argument_property.name = if let Some(y) = name_vec.get(1) {
                    Some(y.clone())
                } else if let Some (y) = name_vec.get(0) {
                    Some(y.clone())
                } else {
                    Some("")
                };
                argument_property.description = Some(description);
                argument_property.action = action.clone();
                if let StoreAction::StoreValue = action {
                    argument_property.reference = ArgumentType::String(None);
                }
                else {
                    argument_property.reference = ArgumentType::Boolean(None);
                }
                let property_cell = Rc::from(RefCell::from(argument_property));
                self.arg_map.insert(names, Rc::clone(&property_cell));
                return Rc::clone(&property_cell);
            }
        }

        pub fn parse_args(&mut self, arguments: &Vec<String>) -> Result<(), String> {
            let mut idx: usize = 0;
            while idx < arguments.len() {
                match arguments.get(idx) {
                    Some(x) => {
                        if let Some(argument) = self
                            .name_map
                            .get(x as &str)
                            {
                            if let Some(arg) = self
                                .arg_map
                                .get(argument)
                                {
                                let some_arg = Rc::clone(arg);
                                if let ArgumentType::Boolean(x) = &some_arg
                                    .borrow()
                                    .reference
                                    {
                                    if let Some(arg) = x {
                                        let final_arg = Rc::clone(arg);
                                        let temp_boolean = (*final_arg.borrow()).clone();
                                        *final_arg.borrow_mut() = !temp_boolean;
                                    }
                                }
                                else if let ArgumentType::String(opt) = &some_arg
                                    .borrow()
                                    .reference
                                    {
                                    if let Some(arg) = opt {
                                        println!("1");
                                        if let Some(value) = arguments.get(idx+1) {
                                            println!("{}", value);
                                            *arg.borrow_mut() = value.to_string();
                                            idx+=1;
                                        }
                                        else {
                                            return Err(format!("'{}' requires a value", x));
                                        }
                                    }
                                };
                            }
                        }
                        else {
                            return Err(format!("Invalid argument: {}", x));
                        }
                    }
                    None => {}
                }
                idx+=1;
            }
            return Ok(());
        }
        pub fn get_arguments(&self) {
            for x in &self.arg_map {
                println!("{:?}",x);
            }
        }
    }
}
