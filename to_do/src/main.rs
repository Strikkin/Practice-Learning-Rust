use clap::{Arg, Command, ArgAction};

mod args;

fn main() {
      let matches = Command::new("to-do")
            .next_line_help(true)
            .arg(Arg::new("add")
                  .short('a')
                  .long("add")
                  .value_name("STRING")
                  .required(false)
                  .action(ArgAction::Set)
                  .help("add a task"))
            .arg(Arg::new("delete")
                  .short('d')
                  .long("del")
                  .value_name("NUM")
                  .required(false)
                  .action(ArgAction::Set)
                  .help("delete a task"))
            .arg(Arg::new("remove")
                  .short('r')
                  .long("rem")
                  .value_name("FILE")
                  .required(false)
                  .action(ArgAction::Set)
                  .help("remove a list"))
            .arg(Arg::new("finish")
                  .short('f')
                  .long("fin")
                  .value_name("NUM")
                  .required(false)
                  .action(ArgAction::Set)
                  .help("finish a task"))
            .arg(Arg::new("view")
                  .short('w')
                  .long("view")
                  .value_name("FILE")
                  .required(false)
                  .action(ArgAction::Set)
                  .help("view a list"))
            .get_matches();

      let add_task = matches.get_one::<String>("add");
      let del_task = matches.get_one::<String>("delete");
      let rem_list = matches.get_one::<String>("remove");
      let finish_task = matches.get_one::<String>("finish");
      let view_list = matches.get_one::<String>("view");

      println!("{:?} {:?} {:?} {:?} {:?}", add_task, del_task, rem_list, finish_task, view_list)

}

