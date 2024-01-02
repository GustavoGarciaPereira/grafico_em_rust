# Projeto de Gráfico em Rust

Este projeto demonstra como ler dados de um arquivo CSV e gerar um gráfico de linha em Rust. Utilizamos as crates `csv` para a leitura dos dados e `plotters` para a criação do gráfico.

## Pré-requisitos

Certifique-se de que você tem Rust e Cargo instalados no seu sistema. Se você ainda não os instalou, siga as instruções no site oficial do Rust.

## Estrutura do Projeto

O projeto segue a estrutura padrão de um projeto Rust:

-   `src/main.rs`: O arquivo principal do projeto contendo o código para ler o arquivo CSV e gerar o gráfico.
-   `Cargo.toml`: O arquivo de configuração do Cargo, listando as dependências do projeto.

## Configuração

Para configurar o projeto, clone o repositório e navegue até o diretório do projeto. Em seguida, execute o seguinte comando para baixar e compilar as dependências:

bashCopy code

`cargo build` 

## Uso

Para usar o programa, primeiro coloque o seu arquivo de dados CSV no diretório correto. O arquivo CSV deve ter duas colunas para os dados do eixo X e do eixo Y.

Execute o programa com:

bashCopy code

`cargo run` 

Isso irá gerar um gráfico baseado nos dados do seu arquivo CSV e salvará como uma imagem PNG no diretório do projeto.

## Customização

Você pode modificar o arquivo `src/main.rs` para ajustar a forma como os dados são lidos e como o gráfico é gerado. Isso inclui mudar o intervalo dos eixos, o estilo do gráfico, entre outros.

![grafico](/grafico/grafico.png)