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
 #[serde(default)]
 desc:String
}
const VERSION:&str=env!("version");
#[derive(clap::Parser)]
#[command(
    name="ioix",
    version=VERSION,
    about="Project metadata tool"
)]
struct Args{
 #[arg(short,default_value="info.json")]
 file:String,
 cmd:String,
 #[arg(short)]
 cmcd:Option<String>,
}
fn main()->std::io::Result<()>{
 let args=<Args as clap::Parser>::parse();
 let file:File=serde_json::from_str(&std::fs::read_to_string(args.file)?)?;
 if file.magic!="IOIX"{
  println!("This file not IOIX! need magic = IOIX");
  std::process::exit(-1);
 }
 if file.version!=VERSION{
  println!("This file version is not support!");
  std::process::exit(-1);
 }
 match args.cmd.as_str(){
  "version"=>{println!("{VERSION}");},
  "name"=>{println!("name of project is '{}'",file.name);},
  "fullname"=>{println!("full name of project is '{}'",file.fullname)},
  "lang"=>{println!("language of project is '{}'",file.lang)},
  "bs"=>{println!("build system of project is '{}'",file.buildsystem)},
  "team"=>{println!("team of project is '{}'",file.teamname)},
  "ver"=>{println!("version of project is '{}'",file.ver)}
  "address"=>{println!("address of project is '{}'",file.address)},
  "users"=>{println!("{}",serde_json::to_string_pretty(&file.users)?)}
  "desc"=>{println!("desc of project is '{}'",file.desc);}
  "cmd"=>{
   if let Some(t)=args.cmcd{
   match file.cmds.iter().find(|dr|dr.name==t){
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
     None=>{eprintln!("not Found! :(");}
    }
   }
  },
  "detect"=>{
   println!(
r#"Project:
name:{}
fullname:{}
lang:{}
buildSystem:{}
version:{}
team:{}
address:{}
desc:{}
"#,file.name,file.fullname,file.lang,file.buildsystem,file.ver,file.teamname,file.address,file.desc);
  },
  &_=>{
   println!("command not fund :(");
  }
 }
 Ok(())
}
