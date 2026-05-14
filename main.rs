use std::env::args;
use std::process::exit;
use std::fs::read_to_string;

#[derive(Debug,Clone,PartialEq)]
enum AstType {
 Const,
 Call,
}

#[derive(Debug,Clone)]
struct Ast {
 kind:AstType,
 sval:Option<String>,
 ival:Option<i32>,
 args:Option<Vec<Ast>>,
}

#[derive(Debug,PartialEq,Clone)]
enum TokenType {
 Sep,
 Const,
 Name,
}

#[derive(Debug,Clone)]
struct Token {
 kind:TokenType,
 sval:Option<String>,
 ival:Option<i32>,
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
   res.push(Token{kind:TokenType::Name,sval:Some(word.clone()),ival:None});
   word.clear();
  } else if code.chars().nth(i).unwrap().is_ascii_digit() {
   while i<len && code.chars().nth(i).unwrap().is_ascii_digit() {
    word.push(code.chars().nth(i).unwrap());
    i+=1;
   }
   res.push(Token{kind:TokenType::Const,sval:None,ival:Some(word.parse::<i32>().unwrap())});
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
   res.push(Token{kind:TokenType::Const,sval:Some(word.clone()),ival:None});
   word.clear();
  }
  if code.chars().nth(i).unwrap()==';' {
   res.push(Token{kind:TokenType::Sep,sval:None,ival:None});
  }
  i+=1;
 }
 res.shrink_to_fit();
 return res;
}

fn parse(tokens:Vec<Token>)->Vec<Ast> {
 let len=tokens.len();
 let mut i=0;
 //let mut b=0;
 let mut res=Vec::<Ast>::with_capacity(len);
 while i<len {
  if tokens[i].kind==TokenType::Const {
   res.push(Ast{kind:AstType::Const,sval:tokens[i].sval.clone(),ival:tokens[i].ival.clone(),args:None});
  } else if tokens[i].kind==TokenType::Name {
   let name=tokens[i].sval.clone().unwrap();
   let mut a=Vec::<Token>::with_capacity(len);
   i+=1;
   while i<len && tokens[i].kind!=TokenType::Sep {
    a.push(tokens[i].clone());
    i+=1;
   }
   a.shrink_to_fit();
   if a.len()>0 { res.push(Ast{kind:AstType::Call,sval:Some(name.clone()),ival:None,args:Some(parse(a.clone()))}); }
   else { res.push(Ast{kind:AstType::Call,sval:Some(name.clone()),ival:None,args:None}); }
   a.clear();
  }
  i+=1;
 }
 res.shrink_to_fit();
 return res;
}

fn main() {
 let argv:Vec<String>=args().collect();
 if argv.len()==1 {
  err("Expected a file");
 } else if !argv[1].ends_with(".mtknfkktr") {
  err("File has to end with '.mtknfkktr'");
 }
 let code=read_to_string(argv[1].clone()).unwrap();
 let tokens=lex(code);
 let ast=parse(tokens);
 for i in ast {
  println!("- - -{:?}",i);
 }
 return;
}
