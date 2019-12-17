use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub enum MalVal {
    //    Nil,
    //    Bool(bool),
    //    Int(i64),
    //Float(f64),
    //    Str(String),
    Sym(String),
    List(Vec<MalVal>),
    //List(Rc<Vec<MalVal>>, Rc<MalVal>),
}

#[derive(Debug)]
pub enum MalErr {
    ErrString(String),
}

pub fn token(s: &mut Peekable<Chars>) -> Option<MalVal> {
    let mut tok = String::from("");

    loop {
        match s.peek() {
            None => break,
            Some(c) => match c {
                '(' => {
                    s.next();
                    let mut list = Vec::<MalVal>::with_capacity(2);
                    loop {
                        let r = token(s);
                        match r {
                            None => {
                                return Some(MalVal::List(list));
                            },
                            Some(v) => {
                                match v {
                                    MalVal::Sym(sym) =>
                                        if sym == ")" {
                                            return Some(MalVal::List(list));
                                        } else {
                                            list.push(MalVal::Sym(sym));
                                        }
                                    _ => {
                                        list.push(v);
                                    }
                                }
                            }
                        }
                    }
                },
                ')' => {
                    if tok.len() == 0 {
                        s.next();
                        tok.push(')');
                    }
                    break;
                },
                ',' | ' ' => {
                    s.next();
                    if tok.len() > 0 {
                        break;
                    }
                }
                _ => {
                    tok.push(*c);
                    s.next();
                }
            },
        }
    }
    if tok.len() == 0 {
        return None
    }
    Some(MalVal::Sym(tok))
}
