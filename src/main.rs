#[derive(serde::Serialize,serde::Deserialize)]
struct Filecmds{
 name:String,
 #[serde(default)]
 cmd:String,
 #[serde(default)]
 desc:String,
 #[serde(default)]
 error:String,
 #[serde(default)]
 warn:String,
}
#[derive(serde::Serialize,serde::Deserialize)]
struct Fileusers{
 id:String,
 #[serde(default)]
 rank:String,
 #[serde(default)]
 name:String,
 #[serde(default)]
 desc:String,
}
#[derive(serde::Serialize,serde::Deserialize)]
struct File{
 version:String,
 magic:String,
 name:String,
 ver:String,
 #[serde(default)]
 fullname:String,
 #[serde(default)]
 lang:String,
 #[serde(default)]
 buildsystem:String,
 #[serde(default)]
 teamname:String,
 #[serde(default)]
 address:String,
 #[serde(default)]
 cmds:Vec<Filecmds>,
 #[serde(default)]
 users:Vec<Fileusers>,
}
const VERSION:&str=env!("version");
fn main()->std::io::Result<()>{
 let mut filename=String::from("./info.json");
 let args:Vec<String>=std::env::args().collect();
 let mut b=String::new();
 let mut a=String::new();
 if args.len()<2{
  println!(":| usage soon :D");
  std::process::exit(-1);
 }
 for l in &args{
  if let Some(n)=l.strip_prefix("-f"){
   filename=n.to_string();
  }
  else if let Some(n)=l.strip_prefix("-c"){
   b=n.to_string();
  }
  else{a=l.to_string();}
 }
 let file:File=serde_json::from_str(&std::fs::read_to_string(filename)?)?;
 if file.magic!="IOIX"{
  println!("This file not IOIX! need magic = IOIX");
  std::process::exit(-1);
 }
 if file.version!=VERSION{
  println!("This file version is not support!");
  std::process::exit(-1);
 }
 match a.as_str(){
  "help"=>{
   println!(r#"
help: :| this :/
version: :| verson :/
name: name of project
fullname: fullname of project
lang: language of project
bs: build system of project
team: team name of project
ver: version of project
address: address of project (email phone website)
users: json of users
cmd: show project commands usage: cmd <cmd>
-f<file>: set file! default is info.json usage: -f<file>
"#);
  }
  "version"=>{println!("{VERSION}");},
  "name"=>{println!("name of project is '{}'",file.name);},
  "fullname"=>{println!("full name of project is '{}'",file.fullname)},
  "lang"=>{println!("language of project is '{}'",file.lang)},
  "bs"=>{println!("build system of project is '{}'",file.buildsystem)},
  "team"=>{println!("team of project is '{}'",file.teamname)},
  "ver"=>{println!("version of project is '{}'",file.ver)}
  "address"=>{println!("address of project is '{}'",file.address)},
  "users"=>{println!("{}",serde_json::to_string_pretty(&file.users)?)}
  "cmd"=>{
   if b.is_empty(){
    println!(":| what cmd??? :|");
    std::process::exit(-1);
   }
   match file.cmds.iter().find(|dr|dr.name==b){
    Some(t)=>{
     if !t.cmd.is_empty(){
      println!("{}",t.cmd);
     }
     if !t.desc.is_empty(){
      eprintln!("Desc:'{}'",t.desc);
     }
     if !t.error.is_empty(){
      eprintln!("Error:'{}'",t.error);
     }
     if !t.warn.is_empty(){
      eprintln!("Warn:'{}'",t.warn);
     }
    },
    None=>{println!("not fund :_(_____");}
   }
  },
  &_=>{
   println!("command not fund :(");
  }
 }
 Ok(())
}
