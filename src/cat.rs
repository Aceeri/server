
use rand::Rng;

pub fn random_cat() -> &'static str {
    let mut rng = rand::thread_rng();
    let cat = rng.gen::<u64>();
    match cat % 10 {
        0 => sleepy(),
        1 => floofy(),
        2 => boxy(),
        3 => curious(),
        4 => sneeze(),
        5 => pumpkin(),
        6 => strut(),
        7 => pounce(),
        8 => food(),
        9 => fall(),
        _ => "failed to get random cat :(",
    }
}

pub fn cat(name: String) -> &'static str {
    match &*name {
        "sleepy" => sleepy(),
        "floofy" => floofy(),
        "boxy" => boxy(),
        "curious" => curious(),
        "sneeze" => sneeze(),
        "pumpkin" => pumpkin(),
        "strut" => strut(),
        "pounce" => pounce(),
        "food" => food(),
        "fall" => fall(),
        _ => "unknown kitty",
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

pub fn sneeze() -> &'static str {
r#"
            /)
           //
          //
         ((
          \\
           ))
          //
      _,-'"._
     /_  ,   \
     |   '   /
     \)    ,\'
     '|      \
     /   |   )
    /_   ,  /
   /  _  % (
   ) /    ' \
  /        \_)
  ),    )_   \_  ,
 _/    _/%     \/)
('  __/ \ ___    \,
 \   (   `"-  __  /
  '\  \   ((   >  \
    )  \   '-.-,_='~~~  ~
   ('  /         ' ~~ ~  ~~ ~~
   " ""
"#
}

pub fn pumpkin() -> &'static str {
r#"
                             .                                   
                          ,''`.         _                        
                     ,.,'''  '`--- ._,,'|                        
                   ,'                   /                        
              __.-'                    |                         
           ''                ___   ___ |                         
         ,'                  \(|\ /|)/ |                         
        ,'                 _     _     `._                       
       /     ,.......-\    `.      __     `-.                    
      /     ,' :  .:''`|    `:`.../:.`` ._   `._                 
  ,..,'  _/' .: :'     |     |      '.    \.    \                
 /      ,'  :'.:  / \  |     | / \   ':.  . \    \               
|      /  .: :' ,'  _)  ".._,;'  _)    :. :. \    |              
 |     | :'.:  /   |   .,   /   |       :  :  |   |              
 |     |:' :  /____|  /  \ /____|       :  :  |  ,'              
  |   /    '         /    \            :'   : |,/                
   \ |  '_          /______\              , : |                  
  _/ |  \'`--`.    _            ,_   ,-'''  :.|         __       
 /   |   \..   ` ./ `.   _,_  ,'  ``'  /'   :'|      _,''/       
/   /'. :   \.   _    [_]   `[_]  .__,,|   _....,--=/'  /:       
|   \_| :    `.-' `.    _.._     /     . ,'  :. ':/'  /'  `.     
`.   '`'`.         `. ,.'   ` .,'     :'/ ':..':.    |  .:' `.   
  \.      \          '               :' |    ''''      ''     `. 
    `''.   `|        ':     .      .:' ,|         .  ..':.      |
      /'   / '"-..._  :   .:'    _;:.,'  \.     .:'   :. ''.    |
     (._,.'        '`''''''''''''          \.._.:      ':  ':   /
                                              '`- ._    ,:__,,-' 
                                                    ``''
"#
}

pub fn strut() -> &'static str {
r#"
                  / )
  (\__/)         ( (
  )    (          ) )
={      }=       / /
  )     `-------/ /
 (               /
  \              |
 ,'\       ,    ,'
 `-'\  ,---\   | \
    _) )    `. \ /
   (__/       ) |
             (_/
"#
}

pub fn pounce() -> &'static str {
r#"
  (`.
   ) )
  ( (
   \ \
    \ \
  .-'  `-.
 /        `.
(      )    `-._ ,    _
 )   ,'         (.\--'(
 \  (         ) /      \
  \  \_(     / (    <6 (6
   \_)))\   (   `._  .:Y)__
    '''  \   `-._.'`---^_)))
          `-._ )))       ```
               ```
"#
}

pub fn food() -> &'static str {
r#"
                        _.---.
              |\---/|  / ) ca|
  ------------;     |-/ /|foo|---
              )     (' / `---'
  ===========(       ,'==========
  ||   _     |      |
  || o/ )    |      | o
  || ( (    /       ;
  ||  \ `._/       /
  ||   `._        /|
  ||      |\    _/||
__||_____.' )  |__||____________
 ________\  |  |_________________
          \ \  `-.
           `-`---' 
"#
}

pub fn fall() -> &'static str {
r#"
        .
       -.\_.--._.______.-------.___.---------.___
       )`.                                       `-._
      (                                              `---.
      /o                                                  `.
     (                                                      \
   _.'`.  _                                                  L
   .'/| "" """"._                                            |
      |          \             |                             J
                  \-._          \                             L
                  /   `-.        \                            J
                 /      /`-.      )_                           `
                /    .-'    `    J  """"-----.`-._             |\         
              .'   .'        L   F            `-. `-.___        \`.
           ._/   .'          )  )                `-    .'""""`.  \)
__________((  _.'__       .-'  J              _.-'   .'        `. \
                   """""""((  .'--.__________(   _.-'___________)..|----------------._____
                            ""                """               ``U'

-----._______
             `-------.__________.--
                       `-.                      ______________.---------------------------
                          `----'`---------'----'

                                                               .-.        .-.
                                                              -  )      .'  )
                                                   __        /  /      /   /
                         .--.                   .-'  """"-._/__(_____.'   /
                      .-'  _)          __     .'  ___.--'                J
                     /  `-.,   .------'  `--.J---'                      /
                     F      `-<                                       .'
                     `.        `.                                 .--'
                     .'|         \                              .'
                   .'            J                             /
                  /              /                            /
                 /              '                           -'
               .'                                          (
              /                               __.----'      \
             /                         __.---'       `.      L
          .-'     _.              .--''                \     \
         /       '             .-'                      \     \
         `--.       /`  __.---'                          \     \
             \     /  .'                                  \     \
             -`.    _/                                     \     \
              ' \_,-'-._                            _       \     L
                    \`.`-                          / )       `.    L
                                                  (   )        \   |
                                                  J   (         L   L
                                                   \   \        |   )
                                               .')_.F   \       \  /
                                               (_)'      \       ""
                                              .'         /
                                             /          /
                  F-.                       /           `.
                  )  `._                 _.'/             \
                 /  -   `-------.___'`-'`                  L
                /                  )                       |
               /_.-.                                       )
              --._                                    .' .-.`.
               \  '                                   | /   \ )
             -''`.                                    \ |    Y
           .'.'/| `--'                               .'`.\__/
            ' '      J                            .-'
                      \                         .'
                   _.-'--.           ___     _.'
                .-'       `-    )--'``  ````` 
              .'    ._    __.--'
              (  _.'  """"
               ""                                _.,._./)
                                             .-''        `-.
                  ___                      .'               `.
           ____.-'   `--.____             /                 <)
         <_.  /__---.    `.  `-""------.-'                   L
          `. //  `,                                        -.|
            \ \    )                                         `._
             | )                                                `-.
             J                                                     `-.
              L  _.      /         \                      _           `--.__
              `. \)     /           `.                     `-._             `-.___
                \    .'              |                   _.'   `-._               `-.
                 |"-'\`-._    \      |     .--._    _.--'          ``-.___           )
                 J  J\`.  `-._ \     |_.--'     """"                      `----.___.'
                  L  )        "J    J
                   `"           L   |
                                ( ...\
                                 \\\\'
                                  `-'




                                           ______  .----.___
                                       .--'      `' `-      `-.
                                    .-'                        `.
                                __.'-                            `.
                              .'                                   `.
                   _.---._   /                                       `.
                 .'       "-'                                          `.
             .--'                                              .         `.
             `._.-"                                             \._        `.
               <_`-.                            .'              |  `.        `.
                |`                   .'     .--'._            .'     `-.       `.
                |         |/         |   .-'      `---.__.---'          `.       `.
                 \``    _.-          | .'                                 `.       `.
                  L   //F `.         |/                                     `-.      \
                  |"""\J    \        |                                         `-.    \
                   |||\F   .'\       |                                            `-._/
                    |`J  .'   \|     F
                    ` F |      )    J
                     J  F      |    J
                     |  F      |    F
                     \_/       |   J
                               |   )
                               F  J
                              J   |
                             ')   F
                             /.   |                             .-""-.__
                             ', J'            _.--"""--._____.-'        `-._
                             `-''          .-'                              `.
                                         .'                        _.._       `.
                                        /                       J""    `-.      \
                                       /                         \        `-.    `.
                                     .'          |               F           \     \
                                     F           |              /             `.    \
                               _.---<_           J             J                `-._/
                             ./`.     `.          \          J F
                          __/ F  )                 \          \
                        <     |                \    >.         L
                         `-.                    |  J  `-.      )
                           J         |          | /      `-    F
                            \  o     /         (|/        //  /
                             `.    )'.          |        /'  /
                              `---'`\ `.        |        /  /
                              '|\\ \ `. `.      |       /( /
                                 `` `  \  \     |       \_'
                                        L  L    |     .' /
                                        |  |    |    (_.'
                                        J  J    F
                                        J  J   J
                                        J  J   J                                 .--.
                                        J  J   F                                J    L
                                        |  |  J                                 |    |
                                        |  F  F                                 F    F
                                        F /L  |                                J    J
                                        `' \_)'                               /    /
                                                                             /    /
                                                                           .'    /
                                                   .--""--._              /     /
                                               _.-'         `-.          /     /
                                    __       .'                `.       /     /
                                 _-'-."`-.  J                    \     /     /
                            .---(  `, _   `'|                     `.  J     /
                              `. )                                  ""     /
                               F                                          J
                               L                  |                      J
                               ` (/     /         |                      F
                                |    ._'          |                      |
                               /'`--'`.           |                      J
                               '||\   |-._        `.  ___.               |
                                 `     \  `.        |/    `-            J
                                        F   L       /       J           /
                                        |   J      J         F         J
                                        |    \     J         |        J
                                        |    |L    J         J        J
                                        |    FJ    |          L       |
                                        |   J  L   |          L\      F
                                        |   F  |   |           L\    J
                                        F  F   |   |           | L   |
                                       J  J    |   |           | |   F
                                       /  )    F  J            F F  J
                                      ( .'    )   F           J J  F
                                      `"     (   J           /.'  J
                                              `-'           ||-' |)
                                                              '-'


____________________________________________________________________________.-------------'
"#
}
