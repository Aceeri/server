
use rand::Rng;

pub fn random_cat() -> &'static str {
    let mut rng = rand::thread_rng();
    rng.gen::<i64>();
    let cat = rng.gen::<i64>();
    match cat % 4 {
        0 => sleepy(),
        1 => floofy(),
        2 => boxy(),
        3 => curious(),
        _ => "failed to get random cat :(",
    }
}

pub fn sleepy() -> &'static str {
r#"
      |\      _,,,---,,_
ZZZzz /,`.-'`'    -.  ;-;;,_
     |,4-  ) )-,_. ,\ (  `'-'
    '---''(_/--'  `-'\_)
"#
}

pub fn floofy() -> &'static str {
r#"
    /\_____/\
   /  o   o  \
  ( ==  ^  == )
   )         (
  (           )
 ( (  )   (  ) )
(__(__)___(__)__)
"#
}

pub fn boxy() -> &'static str {
r#"
  ,-.       _,---._ __  / \
 /  )    .-'       `./ /   \
(  (   ,'            `/    /|
 \  `-"             \'\   / |
  `.              ,  \ \ /  |
   /`.          ,'-`----Y   |
  (            ;        |   '
  |  ,-.    ,-'         |  /
  |  | (   |            | /
  )  |  \  `.___________|/
  `--'   `--'
"#
}

pub fn curious() -> &'static str {
r#"
_                        
\`*-.                    
 )  _`-.                 
.  : `. .                
: _   '  \               
; *` _.   `*-._          
`-.-'          `-.       
  ;       `       `.     
  :.       .        \    
  . \  .   :   .-'   .   
  '  `+.;  ;  '      :   
  :  '  |    ;       ;-. 
  ; '   : :`-:     _.`* ;
.*' /  .*' ; .*`- +'  `*' 
 `*-*   `*-*  `*-*'
"#
}
