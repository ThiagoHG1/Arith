# Calculadora de expressões aritméticas com uso de AST, Parse e lexer.

## [Sobre]
**Arith** é um projeto feito para aprendizado em **rust** seguindo uma semana de projetos que defini para estudos. 
O projeto usa conceitos como parse, lexer e abstract syntax tree — servindo principalmente para aprender o algoritmo de recursive descent parse; limpeza; tokenização de input e parse com Abstract Syntax tree para que os calculos fossem feitos em ordem.

---

## [Funcionamento]
Funciona com expressões de multiplicação, divisão, subtração, resto e adição — funcionando tambem com calculos contendo numeros negativos e positivos (obs: não funciona com decimais)
**exemplos:**
- +(4--6) = 10
- -(10 + 5) * -2 = 30
- (1 + (2 * (3 + (4 % 3)))) = 9

---

## [Lexer]
Remove a maioria das coisas que não são validas — como espaços, letras e tratando tanto `\n` e `\r\n` **caso seja windows ou mac/linux**. 
Ele tambem verifica se antes de um numero ou parenteses temos o sinal de negativo, identificado se é numero negativo ou se é operador menos. 

---

## [Parse]
O token gerado pelo lexer é passado para a função **parse**, que por sua vez realiza **left-to-right** - criando as expressões na ordem matematicamente correta.
Começando pelo que está dentro dos parênteses com **parse_factor**, seguindo por operadores como multiplicação, divisão ou resto e apenas no fim adição e subtração — criando a ordem correta da AST.

**Exemplo:**
5+4*6 = Expr::Add(5, Expr::Multiply(4, 6))

---

## [Baixando e executando]
Clone o repositorio com o comando:
`git clone https://github.com/ThiagoHG1/Arith.git`

- use `cd Arith` para abrir a pasta;

- use `chmod -x script/install.sh` para permitir executar o instalador

- rode o instalador com `./script/install.sh` (talvez precise de sudo). Isso vai instalar o programa com nome Arith.

Em qualquer lugar do terminal, rode o comando `Arith` - isso fara ele rodar o projeto e pedir uma expressão, basta digitar. ele automaticamente ignora espaços, letras ou quaisquer coisa que não seja expressões e numeros validos.
