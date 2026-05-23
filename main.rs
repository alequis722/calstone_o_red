use std::env::args;
use std::process::exit;
use std::fs::read_to_string;

enum Ret {
 Const(Option<String>,Option<f32>),
}

impl Ret {
 fn print(&self) {
  match self {
   Ret::Const(Some(s),None)=>print!("{} ",s),
   Ret::Const(None,Some(f))=>print!("{} ",f),
   _=>{ err("Undefined result"); },
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

fn to_ast(tokens:Vec<Token>)->Vec<Ast> {
 let mut res=Vec::<Ast>::with_capacity(tokens.len());
 for token in &tokens {
  match token.kind {
   TokenType::Const=>res.push(Ast{kind:AstType::Const,sval:token.sval.clone(),fval:token.fval.clone(),args:None}),
   TokenType::Name=>res.push(Ast{kind:AstType::Name,sval:token.sval.clone(),fval:token.fval.clone(),args:None}),
   TokenType::Sep=>res.push(Ast{kind:AstType::Sep,sval:token.sval.clone(),fval:token.fval.clone(),args:None}),
   TokenType::OParen=>res.push(Ast{kind:AstType::OParen,sval:token.sval.clone(),fval:token.fval.clone(),args:None}),
   TokenType::CParen=>res.push(Ast{kind:AstType::CParen,sval:token.sval.clone(),fval:token.fval.clone(),args:None}),
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

fn run(ast:&Vec<Ast>)->Ret {
 let mut res=Ret::Const(None,Some(0.0));
 let mut i=0;
 while i<ast.len() {
  if ast[i].kind==AstType::Const {
   res=Ret::Const(ast[i].sval.clone(),ast[i].fval.clone());
  } else if ast[i].kind==AstType::Expr {
   res=run(&ast[i].args.clone().unwrap());
  } else if ast[i].kind==AstType::Call {
   let name=ast[i].sval.clone().unwrap();
   let args=ast[i].args.clone().unwrap();
   if name=="print" {
    for j in args {
     run(&vec![j]).print();
    }
    println!();
   } else if name=="add" {
    if args.len()<2 {
     err("Expected 2 arguments");
    }
    let a=run(&vec![args[0].clone()]);
    let mut b:f32=0.0;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ err("Unexpected type"); },
    }
    for j in 1..args.len() {
     let c=run(&vec![args[j].clone()]);
     match c {
      Ret::Const(None,Some(f))=>b+=f,
      _=>{ err("Unexpected type"); },
     }
    }
    res=Ret::Const(None,Some(b));
   } else if name=="sub" {
    if args.len()<2 {
     err("Expected 2 arguments");
    }
    let a=run(&vec![args[0].clone()]);
    let mut b:f32=0.0;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ err("Unexpected type"); },
    }
    for j in 1..args.len() {
     let c=run(&vec![args[j].clone()]);
     match c {
      Ret::Const(None,Some(f))=>b-=f,
      _=>{ err("Unexpected type"); },
     }
    }
    res=Ret::Const(None,Some(b));
   } else if name=="mul" {
    if args.len()<2 {
     err("Expected 2 arguments");
    }
    let a=run(&vec![args[0].clone()]);
    let mut b:f32=0.0;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ err("Unexpected type"); },
    }
    for j in 1..args.len() {
     let c=run(&vec![args[j].clone()]);
     match c {
      Ret::Const(None,Some(f))=>b*=f,
      _=>{ err("Unexpected type"); },
     }
    }
    res=Ret::Const(None,Some(b));
   } else if name=="div" {
    if args.len()<2 {
     err("Expected 2 arguments");
    }
    let a=run(&vec![args[0].clone()]);
    let mut b:f32=0.0;
    match a {
     Ret::Const(None,Some(f))=>b=f,
     _=>{ err("Unexpected type"); },
    }
    for j in 1..args.len() {
     let c=run(&vec![args[j].clone()]);
     match c {
      Ret::Const(None,Some(f))=>b/=f,
      _=>{ err("Unexpected type"); },
     }
    }
    res=Ret::Const(None,Some(b));
   } else {
    err("Undefined function");
   }
  }
  i+=1;
 }
 return res;
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
 let ast=parse_from_token(tokens);
 run(&ast);
 return;
}
