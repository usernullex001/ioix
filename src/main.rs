#[derive(serde::Serialize,serde::Deserialize)]
struct Filecmds{
 name:String,
 #[serde(default)]
 cmdunix:String,
 #[serde(default)]
 cmdwind:String,
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
#[derive(serde::Serialize,serde::Deserialize,Default)]
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
 desc:String,
}
const VERSION:&str=env!("version");
#[derive(clap::Subcommand)]
#[derive(PartialEq)]
enum Cmds{
    Version,
    Name,
    Fullname,
    Lang,
    Bs,
    Team,
    Ver,
    Address,
    Users,
    Usemator,/// Users + Terminator(video) xDDDDDDDDDD
    Desc,
    Cmd,
    Runcmd,
    Detect,
    CargoUpdate,
    Viv,/// Version is Ver xD
    Init,
    Build,
}
#[derive(clap::Parser)]
#[command(
    name="ioix",
    version=VERSION,
    about="Project metadata tool"
)]
struct Args{
 #[arg(short,default_value="info.json")]
 file:String,
 #[arg(short)]
 cmcd:Option<String>,
 #[command(subcommand)]
 cmd:Cmds,
}
fn version_cmp(a:&str,b:&str)->std::cmp::Ordering{
    a.split('.').map(|x|x.parse::<u32>().unwrap()).collect::<Vec<u32>>().cmp(&b.split('.').map(|x|x.parse::<u32>().unwrap()).collect::<Vec<u32>>())
}
fn read_file(filename:String)->File{
    let file:File=serde_json::from_str(&std::fs::read_to_string(&filename).unwrap()).unwrap();
    if file.magic!="IOIX"{
        eprintln!("Bro really gave me this instead of the IOIX file 💀");
    }
    if version_cmp(&file.version,VERSION)==std::cmp::Ordering::Less{
        eprintln!("Dude, can you make your words a bit more classy? You're running an outdated version. :D");
    }
    else if version_cmp(&file.version,VERSION)==std::cmp::Ordering::Greater{
        eprintln!("Oooo, I'm aging over here, dude. Time for an update. :D");
    }
    else if file.version!=VERSION{
        eprintln!("Bro, I have no idea what this version even means. :D");
    }
    else{
        return file
    }
    std::process::exit(-1);
}
enum OsType{
    #[cfg(unix)]
    Unix,
    #[cfg(windows)]
    Wind,
}
struct CmdParsed{
    cmd:String,
    os:OsType,
}
fn parse_cmds(t:&Filecmds)->CmdParsed{
  let cmd;
  let os;
  #[cfg(unix)]
  {
  cmd=t.cmdunix.clone();
  os=OsType::Unix;
  }
  #[cfg(windows)]
  {
  cmd=t.cmdwind.clone();
  os=OsType::Wind;
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
  if cmd.is_empty(){
      eprintln!("Error:':/ koo koo commandesh? xD'");
      return CmdParsed{cmd:String::new(),os};
  }
  CmdParsed{cmd,os}
}
fn run_cmd(t:String,cmds:Vec<Filecmds>){
  match cmds.iter().find(|dr|dr.name==t){
      Some(t)=>{
          let cmd=parse_cmds(t);
          if !cmd.cmd.is_empty(){
              let (nb,pc)=match cmd.os{
                  #[cfg(unix)]
                  OsType::Unix=>("sh","-c"),
                  #[cfg(windows)]
                  OsType::Wind=>("cmd","/C"),
              };
              let st=std::process::Command::new(nb).arg(pc).arg(cmd.cmd).status().unwrap();
              eprintln!("Status:{st}");
          }
      },
      None=>{eprintln!("not found! :()");}
  }
}
fn main()->std::io::Result<()>{
 let args=<Args as clap::Parser>::parse();
 let mut file=File::default();
 if args.cmd==Cmds::Init{/*lililili xD*/}
 else {file=read_file(args.file.clone());}
 match &args.cmd{
  Cmds::Version=>{println!("{VERSION}");},
  Cmds::Name=>{println!("name of project is '{}'",file.name);},
  Cmds::Fullname=>{println!("full name of project is '{}'",file.fullname)},
  Cmds::Lang=>{println!("language of project is '{}'",file.lang)},
  Cmds::Bs=>{println!("build system of project is '{}'",file.buildsystem)},
  Cmds::Team=>{println!("team of project is '{}'",file.teamname)},
  Cmds::Ver=>{println!("version of project is '{}'",file.ver)}
  Cmds::Address=>{println!("address of project is '{}'",file.address)},
  Cmds::Usemator=>{println!("{}",serde_json::to_string_pretty(&file.users)?)}
  Cmds::Users=>{
      for u in file.users{
          println!("--{}={}::{}:",u.id,u.rank,u.name);
          println!("{}",u.desc);
      }
  }
  Cmds::Desc=>{println!("desc of project is '{}'",file.desc);}
  Cmds::CargoUpdate=>{
        let mut ct:toml::Value=toml::from_str(&std::fs::read_to_string("Cargo.toml")?).unwrap();
        if let Some(t)=args.cmcd{
            match t.as_str(){
                "cvtfv"=>{ct["package"]["version"]=toml::Value::String(file.ver.to_string());},
                "fvtcv"=>{file.ver=ct["package"]["version"].to_string();},
                "cdtfd"=>{ct["package"]["description"]=toml::Value::String(file.desc.to_string());},
                "fdtcd"=>{file.desc=ct["package"]["description"].to_string();},
                "cntfn"=>{ct["package"]["name"]=toml::Value::String(file.name.to_string());},
                "fntcn"=>{file.name=ct["package"]["name"].to_string();},
                _=>{eprintln!("usage: -c (first)t(second)");}
            }
        }
        std::fs::write("Cargo.toml",toml::to_string_pretty(&ct).unwrap())?;
        std::fs::write(args.file,serde_json::to_string_pretty(&file).unwrap())?;
  }
  Cmds::Viv=>{
      file.version=file.ver.clone();
      std::fs::write(&args.file,serde_json::to_string_pretty(&file)?)?;
  }
  Cmds::Cmd=>{
   if let Some(t)=args.cmcd{
   match file.cmds.iter().find(|dr|dr.name==t){
    Some(t)=>{
        println!("{}",parse_cmds(t).cmd);
     },
     None=>{eprintln!("not Found! :(");}
    }
   }
  },
  Cmds::Runcmd=>{
      if let Some(t)=args.cmcd{
           run_cmd(t,file.cmds);
      }
  }
  Cmds::Build=>{
      run_cmd("build".into(),file.cmds);
  }
  Cmds::Detect=>{
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
  Cmds::Init=>{
      let file=File{
        magic:"IOIX".into(),
        version:VERSION.into(),
        teamname:std::env::var("USER").or_else(|_|std::env::var("USERNAME")).unwrap_or_else(|_|"unknown".into()),
        // linux: USER | windows: USERNAME | xDDDDD
        ..Default::default()
      };
      std::fs::write(&args.file,&serde_json::to_string_pretty(&file)?)?;
  }
 }
 Ok(())
}
