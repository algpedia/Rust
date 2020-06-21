pub fn bubble_sort(lista: &mut [i32]) {
  for i in 0..lista.len() {
    for j in i + 1..lista.len() {
      // Compara o valor da posição i com todos os valores que estão à frente.
      // Se o valor comparado é menor, troca o valor da posição i com o de posição j
      if lista[j] < lista[i] {
        troca(lista, i, j);
      }
    }
  }
}

fn troca(lista: &mut [i32], pos1: usize, pos2: usize) {
  let temp = lista[pos1];
  lista[pos1] = lista[pos2];
  lista[pos2] = temp;
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn invertidos() {
    let mut input = [3, 2, 1];
    bubble_sort(&mut input);
    assert_eq!(input, [1, 2, 3]);
  }

  #[test]
  fn ja_ordenado() {
    let mut input = [1, 2, 3, 4, 5];
    bubble_sort(&mut input);
    assert_eq!(input, [1, 2, 3, 4, 5]);
  }

  #[test]
  fn misturados() {
    let mut input = [1, 3, 2, 7, 5];
    bubble_sort(&mut input);
    assert_eq!(input, [1, 2, 3, 5, 7]);
  }

  #[test]
  fn vetor_vazio() {
    let mut input = [];
    bubble_sort(&mut input);
    assert_eq!(input, []);
  }
}
