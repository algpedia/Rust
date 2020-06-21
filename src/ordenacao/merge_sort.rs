use std::vec::Vec;

pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
  // Se a lista tiver tamanho 1 ou menor, ela já está ordenada
  if lista.len() <= 1 {
    return lista;
  }

  let meio = lista.len() / 2;

  // Separa a lista em duas metades
  let parte1: &[i32] = &lista[0..meio];
  let parte2: &[i32] = &lista[meio..lista.len()];

  // Juntas as de maneira ordenada as duas metades e retorna
  return juntar_ordenado(merge_sort(parte1.to_vec()), merge_sort(parte2.to_vec()));
}

fn juntar_ordenado(lista1: Vec<i32>, lista2: Vec<i32>) -> Vec<i32> {
  // Cria um ponteiro no inicio de cada lista
  let mut res = Vec::new();
  let mut index_lista1 = 0;
  let mut index_lista2 = 0;

  // Vai adicionando no resultado o menor elemento de cada iteracao
  while index_lista1 < lista1.len() || index_lista2 < lista2.len() {
    if index_lista1 >= lista1.len() {
      res.push(lista2[index_lista2]);
      index_lista2 += 1;
    } else if index_lista2 >= lista2.len() {
      res.push(lista1[index_lista1]);
      index_lista1 += 1;
    } else if lista2[index_lista2] < lista1[index_lista1] {
      res.push(lista2[index_lista2]);
      index_lista2 += 1;
    } else {
      res.push(lista1[index_lista1]);
      index_lista1 += 1;
    }
  }

  return res;
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn invertidos() {
    assert_eq!(merge_sort(vec![3, 2, 1]), [1, 2, 3])
  }

  #[test]
  fn ja_ordenado() {
    assert_eq!(merge_sort(vec![1, 2, 3, 4, 5]), [1, 2, 3, 4, 5]);
  }

  #[test]
  fn misturados() {
    assert_eq!(merge_sort(vec![1, 3, 2, 7, 5]), [1, 2, 3, 5, 7]);
  }

  #[test]
  fn vetor_vazio() {
    assert_eq!(merge_sort(vec![]), []);
  }
}
