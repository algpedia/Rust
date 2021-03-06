use std::vec::Vec;
use rand::Rng;

pub fn quick_sort(lista: Vec<i32>) -> Vec<i32> {
  // Se a lista tiver tamanho 1 ou menor, ela já está ordenada
  if lista.len() <= 1 {
    return lista;
  }
  
  // Define um pivot de posição aleatória
  // É muito comum ver algoritmos definirem um pivot de posição fixa, como o primeiro elemento ou o elemento do meio
  // Mas decidimos aqui ficar com o algoritmo clássico 
  let mut rng = rand::thread_rng();
  let pivot = rng.gen_range(0, lista.len());
  
  let valor_pivot = lista[pivot];

  let mut menores = Vec::new();
  let mut maiores = Vec::new();

  // Separa a lista entre os menores e os maiores que o valor pivot
  for valor in lista {
    if valor <= valor_pivot {
      menores.push(valor);
    } else {
      maiores.push(valor);
    }
  }

  // Retorna a chamada recursiva de cada uma das listas concatenadas
  return [quick_sort(menores), quick_sort(maiores)].concat();
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn invertidos() {
    assert_eq!(quick_sort(vec![3, 2, 1]), [1, 2, 3])
  }

  #[test]
  fn ja_ordenado() {
    assert_eq!(quick_sort(vec![1, 2, 3, 4, 5]), [1, 2, 3, 4, 5]);
  }

  #[test]
  fn misturados() {
    assert_eq!(quick_sort(vec![1, 3, 2, 7, 5]), [1, 2, 3, 5, 7]);
  }

  #[test]
  fn vetor_vazio() {
    assert_eq!(quick_sort(vec![]), []);
  }
}