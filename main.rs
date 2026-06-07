use std::env::args;
use std::fs::read_to_string;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq)]
enum Ret {
 Const(Option<String>,Option<f32>),
}

impl Ret {
 fn print(&self) {
  match self {
   Ret::Const(Some(s),None)=>print!("{} ",s),
   Ret::Const(None,Some(f))=>print!("{} ",f),
   _=>{ panic!("Undefined result"); },
  }
 }
}

#[derive(Debug,Clone,PartialEq)]
enum AstType {
 Const,
 Expr,
 Name,
 Call,
 OParen,
 CParen,
 Sep,
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
    if c>1 { panic!("Unexpected period in number"); }
    word.push(code.chars().nth(i).unwrap());
    i+=1;
   }
   res.push(Token{kind:TokenType::Const,sval:None,fval:Some(word.parse::<f32>().unwrap())});
   word.clear();
  } else if code.chars().nth(i).unwrap()=='-' {
   let mut c=0;
   word.push(code.chars().nth(i).unwrap());
   i+=1;
   while i<len && (code.chars().nth(i).unwrap().is_ascii_digit() || code.chars().nth(i).unwrap()=='.') {
    if code.chars().nth(i).unwrap()=='.' { c+=1; }
    if c>1 { panic!("Unexpected period in number"); }
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
  } else if code.chars().nth(i).unwrap()=='#' {
   i+=1;
   if i<len && code.chars().nth(i).unwrap()=='(' {
    let mut c=1;
    i+=1;
    while i<len {
     if i<len && code.chars().nth(i).unwrap()=='(' { c+=1; }
     else if i<len && code.chars().nth(i).unwrap()==')' { c-=1; }
     if c==0 { break; }
     i+=1;
    }
    if c>0 { panic!("Unclosed comment"); }
    i+=1;
   } else {
    while i<len && code.chars().nth(i).unwrap()!='\n' {
     i+=1;
    }
   }
  } else if code.chars().nth(i).unwrap()=='$' {
   i+=1;
   while i<len && code.chars().nth(i).unwrap().is_ascii_digit() {
    word.push(code.chars().nth(i).unwrap());
    i+=1;
   }
   if word.len()==0 {
    panic!("Expected a number");
   }
   res.push(Token{kind:TokenType::Name,sval:Some(word.clone()),fval:None});
   word.clear();
  }
  if i<len {
   if code.chars().nth(i).unwrap()==';' {
    res.push(Token{kind:TokenType::Sep,sval:None,fval:None});
   } else if code.chars().nth(i).unwrap()=='(' {
    res.push(Token{kind:TokenType::OParen,sval:None,fval:None});
   } else if code.chars().nth(i).unwrap()==')' {
    res.push(Token{kind:TokenType::CParen,sval:None,fval:None});
   }
  }
  i+=1;
 }
 res.shrink_to_fit();
 return res;
}

fn to_ast(tokens:Vec<Token>)->Vec<Ast> {
 let mut res=Vec::<Ast>::with_capacity(tokens.len());
 for token in &tokens {
  match token.kind {
   TokenType::Const=>res.push(Ast{kind:AstType::Const,sval:token.sval.clone(),fval:token.fval.clone(),args:None}),
   TokenType::Name=>res.push(Ast{kind:AstType::Name,sval:token.sval.clone(),fval:None,args:None}),
   TokenType::Sep=>res.push(Ast{kind:AstType::Sep,sval:None,fval:None,args:None}),
   TokenType::OParen=>res.push(Ast{kind:AstType::OParen,sval:None,fval:None,args:None}),
   TokenType::CParen=>res.push(Ast{kind:AstType::CParen,sval:None,fval:None,args:None}),
  }
 }
 res.shrink_to_fit();
 return res;
}

fn parse_paren(ast:&Vec<Ast>)->Vec<Ast> {
 let mut res=Vec::<Ast>::with_capacity(ast.len());
 let mut i=0;
 while i<ast.len() {
  if ast[i].kind==AstType::OParen {
   let mut c=1;
   let mut a=Vec::<Ast>::with_capacity(ast.len());
   i+=1;
   while i<ast.len() {
    if ast[i].kind==AstType::OParen { c+=1; }
    else if ast[i].kind==AstType::CParen { c-=1; }
    if c==0 { break; }
    a.push(ast[i].clone());
    i+=1;
   }
   a.shrink_to_fit();
   res.push(Ast{kind:AstType::Expr,sval:None,fval:None,args:Some(parse_from_ast(&a))});
   a.clear();
  } else if ast[i].kind==AstType::CParen {
   panic!("Stray ')'");
  } else {
   res.push(ast[i].clone());
  }
  i+=1;
 }
 res.shrink_to_fit();
 return res;
}

fn parse_func(ast:&Vec<Ast>)->Vec<Ast> {
 let mut res=Vec::<Ast>::with_capacity(ast.len());
 let mut i=0;
 while i<ast.len() {
  if ast[i].kind==AstType::Name {
   let name=ast[i].sval.clone().unwrap();
   let mut a=Vec::<Ast>::with_capacity(ast.len());
   i+=1;
   while i<ast.len() && ast[i].kind!=AstType::Sep {
    a.push(ast[i].clone());
    i+=1
   }
   a.shrink_to_fit();
   res.push(Ast{kind:AstType::Call,sval:Some(name),fval:None,args:Some(a.clone())});
  } else {
   res.push(ast[i].clone());
  }
  i+=1;
 }
 return res;
}

fn parse_from_ast(ast:&Vec<Ast>)->Vec<Ast> {
 let mut res:Vec<Ast>;
 res=parse_paren(ast);
 res=parse_func(&res);
 return res;
}

fn parse_from_token(tokens:Vec<Token>)->Vec<Ast> {
 let mut res:Vec<Ast>;
 res=to_ast(tokens);
 res=parse_from_ast(&res);
 return res;
}

fn run(ast:&Vec<Ast>,var:&mut Vec<HashMap<String,Ast>>,fun:&mut Vec<HashMap<String,(i32,Ast)>>,scope:usize)->Ret {
 let mut res=Ret::Const(None,Some(0.0));
 let mut i=0;
 while i<ast.len() {
  if ast[i].kind==AstType::Const {
   res=Ret::Const(ast[i].sval.clone(),ast[i].fval.clone());
  } else if ast[i].kind==AstType::Expr {
   res=run(&ast[i].args.clone().unwrap(),var,fun,scope);
  } else if ast[i].kind==AstType::Name {
   match var[scope].get(&ast[i].sval.clone().unwrap()) {
    Some(r)=>{ res=run(&vec![r.clone()],var,fun,scope); break; },
    _=>{
     if scope==0 {
      panic!("Undefined variable '{}'",ast[i].sval.clone().unwrap());
     }
     res=run(&vec![ast[i].clone()],var,fun,scope-1);
    }
   }
  } else if ast[i].kind==AstType::Call {
   let name=ast[i].sval.clone().unwrap();
   let args=ast[i].args.clone().unwrap();
   let argc=args.len();
   if name=="print" {
    for j in args {
     res=run(&vec![j],var,fun,scope);
     res.print();
    }
    println!();
   } else if name=="add" {
    if argc<2 {
     panic!("Expected 2 arguments");
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    let mut b:f32;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ panic!("Unexpected type"); },
    }
    for j in 1..argc {
     let c=run(&vec![args[j].clone()],var,fun,scope);
     match c {
      Ret::Const(None,Some(f))=>b+=f,
      _=>{ panic!("Unexpected type"); },
     }
    }
    res=Ret::Const(None,Some(b));
   } else if name=="sub" {
    if argc<2 {
     panic!("Expected 2 arguments");
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    let mut b:f32;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ panic!("Unexpected type"); },
    }
    for j in 1..argc {
     let c=run(&vec![args[j].clone()],var,fun,scope);
     match c {
      Ret::Const(None,Some(f))=>b-=f,
      _=>{ panic!("Unexpected type"); },
     }
    }
    res=Ret::Const(None,Some(b));
   } else if name=="mul" {
    if argc<2 {
     panic!("Expected 2 arguments");
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    let mut b:f32;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ panic!("Unexpected type"); },
    }
    for j in 1..argc {
     let c=run(&vec![args[j].clone()],var,fun,scope);
     match c {
      Ret::Const(None,Some(f))=>b*=f,
      _=>{ panic!("Unexpected type"); },
     }
    }
    res=Ret::Const(None,Some(b));
   } else if name=="div" {
    if argc<2 {
     panic!("Expected 2 arguments");
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    let mut b:f32;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ panic!("Unexpected type"); },
    }
    for j in 1..argc {
     let c=run(&vec![args[j].clone()],var,fun,scope);
     match c {
      Ret::Const(None,Some(f))=>b/=f,
      _=>{ panic!("Unexpected type"); },
     }
    }
    res=Ret::Const(None,Some(b));
   } else if name=="ln" {
    if argc!=1 {
     panic!("Expected 1 argument, got {}",argc);
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    match a {
     Ret::Const(None,Some(f))=>res=Ret::Const(None,Some(f.ln())),
     _=>{ panic!("Unexpected type"); },
    }
   } else if name=="log" {
    if argc!=1 {
     panic!("Expected 1 argument, got {}",argc);
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    match a {
     Ret::Const(None,Some(f))=>res=Ret::Const(None,Some(f.log10())),
     _=>{ panic!("Unexpected type"); },
    }
   } else if name=="log_" {
    if argc!=2 {
     panic!("Expected 2 arguments, got {}",argc);
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    let b:f32;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ panic!("Unexpected type"); },
    }
    match run(&vec![args[1].clone()],var,fun,scope) {
     Ret::Const(None,Some(f))=>res=Ret::Const(None,Some(f.log(b))),
     _=>{ panic!("Unexpected type"); },
    }
   } else if name=="zero" {
    if argc!=1 {
     panic!("Expected 1 argument");
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    match a {
     Ret::Const(None,Some(0.0))=>res=Ret::Const(None,Some(1.0)),
     Ret::Const(None,Some(_))=>res=Ret::Const(None,Some(0.0)),
     _=>{ panic!("Unexpected value"); },
    }
   } else if name=="pos" {
    if argc!=1 {
     panic!("Expected 1 argument");
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    match a {
     Ret::Const(None,Some(f))=>{
      if f>0.0 { res=Ret::Const(None,Some(1.0)); }
      else { res=Ret::Const(None,Some(0.0)); }
     },
     _=>{ panic!("Unexpected value"); },
    }
   } else if name=="neg" {
    if argc!=1 {
     panic!("Expected 1 argument");
    }
    let a=run(&vec![args[0].clone()],var,fun,scope);
    match a {
     Ret::Const(None,Some(f))=>{
      if f<0.0 { res=Ret::Const(None,Some(1.0)); }
      else { res=Ret::Const(None,Some(0.0)); }
     },
     _=>{ panic!("Unexpected value"); },
    }
   } else if name=="if" {
    if argc==2 {
     let a=run(&vec![args[0].clone()],var,fun,scope);
     match a {
      Ret::Const(None,Some(1.0))=>res=run(&vec![args[1].clone()],var,fun,scope),
      Ret::Const(None,Some(0.0))=>(),
      _=>{ panic!("Unexpected value"); },
     }
    } else if argc==3 {
     let a=run(&vec![args[0].clone()],var,fun,scope);
     match a {
      Ret::Const(None,Some(1.0))=>res=run(&vec![args[1].clone()],var,fun,scope),
      Ret::Const(None,Some(0.0))=>res=run(&vec![args[2].clone()],var,fun,scope),
      _=>{ panic!("Unexpected value"); },
     }
    } else {
     panic!("Expected 2 or 3 arguments");
    }
   } else if name=="let" {
    let mut vname:String;
    if argc==2 {
     if args[0].kind.clone()!=AstType::Name {
      panic!("Expected a name");
     }
     vname=args[0].sval.clone().unwrap();
     vname.shrink_to_fit();
     let a=args[1].clone();
     var[scope].insert(vname.clone(),a);
    } else if argc==3 {
     if args[0].kind.clone()!=AstType::Name {
      panic!("Expected a name");
     } else if args[1].kind!=AstType::Const && args[1].fval.is_none() {
      panic!("Expected a number");
     }
     vname=args[0].sval.clone().unwrap();
     vname.shrink_to_fit();
     let argc=args[1].fval.clone().unwrap();
     if argc.fract()>0.0 {
      panic!("Expected a whole number, got {}",argc);
     }
     if argc<=0.0 {
      panic!("Expected a positive non-zero value");
     }
     fun[scope].insert(vname.clone(),(argc as i32,args[2].clone()));
    } else {
     panic!("Expected 2 or 3 arguments");
    }
    res=Ret::Const(Some(vname.clone()),None);
   } else {
    if fun[scope].get(&name).is_none() {
     if var[scope].get(&name).is_none() {
      if scope==0 {
       panic!("Undefined function '{}'",name);
      }
      res=run(&vec![ast[i].clone()],var,fun,scope-1);
     } else {
      res=run(&vec![var[scope].get(&name).unwrap().clone()],var,fun,scope);
     }
    } else {
     var.push(HashMap::<String,Ast>::new());
     fun.push(HashMap::<String,(i32,Ast)>::new());
     let body=fun[scope].get(&name).unwrap();
     if argc!=body.0 as usize {
      panic!("Expected {} arguments, got {}",body.0,argc);
     }
     for j in 0..body.0 {
      var[scope+1].insert(j.to_string(),args[j as usize].clone());
     }
     res=run(&vec![body.1.clone()],var,fun,scope+1);
     var.pop();
     fun.pop();
    }
   }
  }
  i+=1;
 }
 return res;
}

fn main() {
 let argv:Vec<String>=args().collect();
 if argv.len()==1 {
  panic!("Expected a file");
 } else if !argv[1].ends_with(".cor") {
  panic!("File has to end with '.cor'");
 }
 {
  let path=Path::new(&argv[1]);
  if !path.exists() {
   panic!("File not found");
  }
 }
 let code=read_to_string(argv[1].clone()).unwrap();
 let tokens=lex(code);
 let ast=parse_from_token(tokens);
 let mut var=Vec::<HashMap<String,Ast>>::with_capacity(8);
 var.push(HashMap::<String,Ast>::new());
 let mut fun=Vec::<HashMap<String,(i32,Ast)>>::with_capacity(8);
 fun.push(HashMap::<String,(i32,Ast)>::new());
 run(&ast,&mut var,&mut fun,0);
 return;
}
