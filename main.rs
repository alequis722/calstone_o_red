use std::env::args;
use std::process::exit;
use std::fs::read_to_string;

#[derive(Debug,PartialEq)]
enum Ret {
 Const(Option<String>,Option<f32>),
 Name(String),
 Expr(Vec<Ast>),
}

impl Ret {
 fn print(&self) {
  match self {
   Ret::Const(Some(s),None)=>print!("{s} "),
   Ret::Const(None,Some(i))=>print!("{i} "),
   Ret::Expr(a)=>run(a.to_vec()).print(),
   _=>(),
  }
 }
}

#[derive(Debug,Clone,PartialEq)]
enum AstType {
 Const,
 Expr,
 Call,
}

#[derive(Debug,Clone,PartialEq)]
struct Ast {
 kind:AstType,
 sval:Option<String>,
 fval:Option<f32>,
 args:Option<Vec<Ast>>,
}

#[derive(Debug,PartialEq,Clone)]
enum TokenType {
 Sep,
 Const,
 Name,
 OParen,
 CParen,
}

#[derive(Debug,Clone)]
struct Token {
 kind:TokenType,
 sval:Option<String>,
 fval:Option<f32>,
}

fn err(msg:&str) {
 println!("{}",msg);
 exit(-1);
}

fn lex(code:String)->Vec<Token> {
 let len=code.len();
 let mut i=0;
 let mut c=0;
 let mut word=String::with_capacity(len);
 let mut res=Vec::<Token>::with_capacity(len);
 while i<len {
  if code.chars().nth(i).unwrap().is_ascii_alphabetic() || code.chars().nth(i).unwrap()=='_' {
   while i<len && (code.chars().nth(i).unwrap().is_alphanumeric() || code.chars().nth(i).unwrap()=='_') {
    word.push(code.chars().nth(i).unwrap());
    i+=1;
   }
   res.push(Token{kind:TokenType::Name,sval:Some(word.clone()),fval:None});
   word.clear();
  } else if code.chars().nth(i).unwrap().is_ascii_digit() {
   let mut c=0;
   while i<len && (code.chars().nth(i).unwrap().is_ascii_digit() || code.chars().nth(i).unwrap()=='.') {
    if code.chars().nth(i).unwrap()=='.' { c+=1; }
    if c>1 { err("Unexpected period in number"); }
    word.push(code.chars().nth(i).unwrap());
    i+=1;
   }
   res.push(Token{kind:TokenType::Const,sval:None,fval:Some(word.parse::<f32>().unwrap())});
   word.clear();
  } else if code.chars().nth(i).unwrap()=='"' {
   i+=1;
   c+=1;
   while i<len {
    if code.chars().nth(i).unwrap()=='"' { c-=1; }
    if c==0 { break; }
    word.push(code.chars().nth(i).unwrap());
    i+=1;
   }
   res.push(Token{kind:TokenType::Const,sval:Some(word.clone()),fval:None});
   word.clear();
  }
  if code.chars().nth(i).unwrap()==';' {
   res.push(Token{kind:TokenType::Sep,sval:None,fval:None});
  } else if code.chars().nth(i).unwrap()=='(' {
   res.push(Token{kind:TokenType::OParen,sval:None,fval:None});
  } else if code.chars().nth(i).unwrap()==')' {
   res.push(Token{kind:TokenType::CParen,sval:None,fval:None});
  }
  i+=1;
 }
 res.shrink_to_fit();
 return res;
}

fn parse(tokens:Vec<Token>)->Vec<Ast> {
 let len=tokens.len();
 let mut i=0;
 let mut res=Vec::<Ast>::with_capacity(len);
 while i<len {
  if tokens[i].kind==TokenType::Const {
   res.push(Ast{kind:AstType::Const,sval:tokens[i].sval.clone(),fval:tokens[i].fval.clone(),args:None});
  } else if tokens[i].kind==TokenType::OParen {
   let mut a=1;
   let mut b=Vec::<Token>::with_capacity(len);
   i+=1;
   while i<len {
    if tokens[i].kind==TokenType::OParen { a+=1; }
    else if tokens[i].kind==TokenType::CParen { a-=1; }
    if a==0 { break; }
    b.push(tokens[i].clone());
    i+=1;
   }
   b.shrink_to_fit();
   if b.len()>0 {
    res.push(Ast{kind:AstType::Expr,sval:None,fval:None,args:Some(parse(b.clone()))});
   }
   b.clear();
  } else if tokens[i].kind==TokenType::Name {
   let name=tokens[i].sval.clone().unwrap();
   let mut a=Vec::<Token>::with_capacity(len);
   i+=1;
   while i<len && tokens[i].kind!=TokenType::Sep {
    a.push(tokens[i].clone());
    i+=1;
   }
   a.shrink_to_fit();
   if a.len()>0 { res.push(Ast{kind:AstType::Call,sval:Some(name.clone()),fval:None,args:Some(parse(a.clone()))}); }
   else { res.push(Ast{kind:AstType::Call,sval:Some(name.clone()),fval:None,args:None}); }
   a.clear();
  }
  i+=1;
 }
 res.shrink_to_fit();
 return res;
}

fn run(ast:Vec<Ast>)->Ret {
 let mut i=0;
 let mut ret=Ret::Const(None,Some(0.0));
 while i<ast.len() {
  if ast[i].kind==AstType::Const {
   ret=Ret::Const(ast[i].sval.clone(),ast[i].fval);
  } else if ast[i].kind==AstType::Expr {
   ret=Ret::Expr(ast[i].args.clone().unwrap());
  } else if ast[i].kind==AstType::Call {
   let name=ast[i].sval.clone().unwrap();
   if name=="print" {
    for j in ast[i].args.clone().unwrap() {
     run(vec![j]).print();
    }
    println!("");
   } else if name=="add" {
    if ast[i].args.clone().unwrap().len()==0 {
     err("Unexpected atleast 1 argument");
    }
    let c=ast[i].args.clone().unwrap();
    let d=run(vec![c[0].clone()]);
    let mut a:f32=0.0;
    match d {
     Ret::Const(None,Some(i))=>a=i,
     _=>{ err("Unexpected result"); },
    }
    if c.len()>1 {
     for j in 1..c.len() {
      let b=run(vec![c[j].clone()]);
      match b {
       Ret::Const(None,Some(i))=>a+=i,
       _=>{ err("Unexpected result"); },
      }
     }
    }
    ret=Ret::Const(None,Some(a));
   } else if name=="sub" {
    if ast[i].args.clone().unwrap().len()==0 {
     err("Unexpected atleast 1 argument");
    }
    let c=ast[i].args.clone().unwrap();
    let d=run(vec![c[0].clone()]);
    let mut a:f32=0.0;
    match d {
     Ret::Const(None,Some(i))=>a=i,
     _=>{ err("Unexpected result"); },
    }
    if c.len()>1 {
     for j in 1..c.len() {
      let b=run(vec![c[j].clone()]);
      match b {
       Ret::Const(None,Some(i))=>a-=i,
       _=>{ err("Unexpected result"); },
      }
     }
    }
    ret=Ret::Const(None,Some(a));
   } else if name=="mul" {
    if ast[i].args.clone().unwrap().len()==0 {
     err("Unexpected atleast 1 argument");
    }
    let c=ast[i].args.clone().unwrap();
    let d=run(vec![c[0].clone()]);
    let mut a:f32=0.0;
    match d {
     Ret::Const(None,Some(i))=>a=i,
     _=>{ err("Unexpected result"); },
    }
    if c.len()>1 {
     for j in 1..c.len() {
      let b=run(vec![c[j].clone()]);
      match b {
       Ret::Const(None,Some(i))=>a*=i,
       _=>{ err("Unexpected result"); },
      }
     }
    }
    ret=Ret::Const(None,Some(a));
   } else if name=="div" {
    if ast[i].args.clone().unwrap().len()==0 {
     err("Unexpected atleast 1 argument");
    }
    let c=ast[i].args.clone().unwrap();
    let d=run(vec![c[0].clone()]);
    let mut a:f32=0.0;
    match d {
     Ret::Const(None,Some(i))=>a=i,
     _=>{ err("Unexpected result"); },
    }
    if c.len()>1 {
     for j in 1..c.len() {
      let b=run(vec![c[j].clone()]);
      match b {
       Ret::Const(None,Some(i))=>a/=i,
       _=>{ err("Unexpected result"); },
      }
     }
    }
    ret=Ret::Const(None,Some(a));
   } else if name=="ln" {
    if ast[i].args.clone().unwrap().len()==0 {
     err("Unexpected atleast 1 argument");
    }
    let a=run(vec![ast[i].args.clone().unwrap()[0].clone()]);
    match a {
     Ret::Const(None,Some(f))=>ret=Ret::Const(None,Some(f.ln())),
     _=>{ err("Unexpected result"); },
    }
   } else if name=="log" {
    if ast[i].args.clone().unwrap().len()==0 {
     err("Unexpected atleast 1 argument");
    }
    let a=run(vec![ast[i].args.clone().unwrap()[0].clone()]);
    match a {
     Ret::Const(None,Some(f))=>ret=Ret::Const(None,Some(f.log10())),
     _=>{ err("Unexpected result"); },
    }
   } else if name=="log_" {
    if ast[i].args.clone().unwrap().len()<2 {
     err("Unexpected atleast 1 argument");
    }
    let a=run(vec![ast[i].args.clone().unwrap()[0].clone()]);
    let c=run(vec![ast[i].args.clone().unwrap()[1].clone()]);
    let mut b:f32=0.0;
    match c {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ err("Unexpected result"); },
    }
    match a {
     Ret::Const(None,Some(f))=>ret=Ret::Const(None,Some(b.log(f))),
     _=>{ err("Unexpected result"); },
    }
   } else {
    err("Undefined function");
   }
  }
  i+=1;
 }
 return ret;
}

fn main() {
 let argv:Vec<String>=args().collect();
 if argv.len()==1 {
  err("Expected a file");
 } else if !argv[1].ends_with(".cor") {
  err("File has to end with '.cor'");
 }
 let code=read_to_string(argv[1].clone()).unwrap();
 let tokens=lex(code);
 let ast=parse(tokens);
 run(ast);
 return;
}
